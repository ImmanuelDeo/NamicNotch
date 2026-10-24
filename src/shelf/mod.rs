//! File shelf for Nusa Notch Clone - drag & drop file storage

use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{info, warn};
use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::settings::Settings;

/// Shelf item
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShelfItem {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Local>,
    pub item_type: ShelfItemType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum ShelfItemType {
    File,
    Folder,
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Other,
}

impl ShelfItem {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let metadata = std::fs::metadata(&path)?;
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
        
        let item_type = Self::detect_type(&path);
        
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name,
            path: path.clone(),
            size: metadata.len(),
            modified: metadata.modified()?.into(),
            item_type,
        })
    }
    
    fn detect_type(path: &PathBuf) -> ShelfItemType {
        if path.is_dir() {
            return ShelfItemType::Folder;
        }
        
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        match ext.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "ico" | "svg" => ShelfItemType::Image,
            "mp4" | "mkv" | "avi" | "mov" | "webm" | "flv" => ShelfItemType::Video,
            "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" => ShelfItemType::Audio,
            "pdf" | "doc" | "docx" | "txt" | "md" | "rtf" | "odt" => ShelfItemType::Document,
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => ShelfItemType::Archive,
            _ => ShelfItemType::File,
        }
    }
}

/// Shelf events
#[derive(Debug, Clone)]
pub enum ShelfEvent {
    AddItem(PathBuf),
    RemoveItem(String),
    Clear,
    OpenItem(String),
    OpenFolder(String),
}

/// Shelf manager
pub struct Shelf {
    settings: Arc<RwLock<Settings>>,
    event_tx: mpsc::Sender<ShelfEvent>,
    event_rx: mpsc::Receiver<ShelfEvent>,
    items: Vec<ShelfItem>,
    shelf_dir: PathBuf,
    watcher: Option<notify::RecommendedWatcher>,
}

impl Shelf {
    pub fn new(settings: Arc<RwLock<Settings>>) -> Result<Self> {
        let (event_tx, event_rx) = mpsc::channel(32);
        
        // Shelf directory
        let shelf_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("NusaShelf")
            .join("Shelf");
        
        // Create directory if needed
        std::fs::create_dir_all(&shelf_dir)?;
        
        // Load existing items
        let items = Self::load_items(&shelf_dir)?;
        
        Ok(Self {
            settings,
            event_tx,
            event_rx,
            items,
            shelf_dir,
            watcher: None,
        })
    }
    
    pub async fn run(mut self) -> Result<()> {
        info!("Shelf started at {}", self.shelf_dir.display());
        
        // Start file watcher
        self.start_watcher()?;
        
        // Cleanup interval (remove items for deleted files)
        let mut cleanup_interval = interval(Duration::from_secs(60));
        
        loop {
            tokio::select! {
                event = self.event_rx.recv() => {
                    if let Some(event) = event {
                        self.handle_event(event)?;
                    } else {
                        break;
                    }
                }
                _ = cleanup_interval.tick() => {
                    self.cleanup_missing()?;
                }
            }
        }
        
        // Stop watcher
        if let Some(watcher) = self.watcher.take() {
            drop(watcher);
        }
        
        info!("Shelf stopped");
        Ok(())
    }
    
    fn start_watcher(&mut self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(32);
        let shelf_dir = self.shelf_dir.clone();
        
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;
        
        watcher.watch(&shelf_dir, RecursiveMode::NonRecursive)?;
        self.watcher = Some(watcher);
        
        // Spawn watcher handler
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                Self::handle_watch_event(&shelf_dir, event).await;
            }
        });
        
        Ok(())
    }
    
    async fn handle_watch_event(shelf_dir: &PathBuf, event: Event) {
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                for path in event.paths {
                    if path.is_file() {
                        // New file added or modified
                        info!("File added/modified: {:?}", path);
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in event.paths {
                    info!("File removed: {:?}", path);
                }
            }
            _ => {}
        }
    }
    
    fn handle_event(&mut self, event: ShelfEvent) -> Result<()> {
        match event {
            ShelfEvent::AddItem(path) => {
                let item = ShelfItem::from_path(path)?;
                if self.items.len() >= self.settings.read().unwrap().shelf_max_items {
                    self.items.remove(0); // Remove oldest
                }
                self.items.push(item);
                self.save_items()?;
            }
            ShelfEvent::RemoveItem(id) => {
                self.items.retain(|i| i.id != id);
                self.save_items()?;
            }
            ShelfEvent::Clear => {
                self.items.clear();
                self.save_items()?;
            }
            ShelfEvent::OpenItem(id) => {
                if let Some(item) = self.items.iter().find(|i| i.id == id) {
                    open_file(&item.path)?;
                }
            }
            ShelfEvent::OpenFolder(id) => {
                if let Some(item) = self.items.iter().find(|i| i.id == id) {
                    let folder = if item.item_type == ShelfItemType::Folder {
                        &item.path
                    } else {
                        item.path.parent().unwrap_or(&self.shelf_dir)
                    };
                    open_folder(folder)?;
                }
            }
        }
        Ok(())
    }
    
    fn load_items(shelf_dir: &PathBuf) -> Result<Vec<ShelfItem>> {
        let mut items = Vec::new();
        
        if shelf_dir.exists() {
            for entry in std::fs::read_dir(shelf_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |e| e == "json") {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(item) = serde_json::from_str::<ShelfItem>(&content) {
                            // Verify file still exists
                            if item.path.exists() {
                                items.push(item);
                            }
                        }
                    }
                }
            }
        }
        
        info!("Loaded {} shelf items", items.len());
        Ok(items)
    }
    
    fn save_items(&self) -> Result<()> {
        std::fs::create_dir_all(&self.shelf_dir)?;
        
        for item in &self.items {
            let item_path = self.shelf_dir.join(format!("{}.json", item.id));
            let content = serde_json::to_string_pretty(item)?;
            std::fs::write(item_path, content)?;
        }
        
        Ok(())
    }
    
    fn cleanup_missing(&mut self) -> Result<()> {
        let initial_len = self.items.len();
        self.items.retain(|item| item.path.exists());
        
        if self.items.len() != initial_len {
            self.save_items()?;
            info!("Cleaned up {} missing shelf items", initial_len - self.items.len());
        }
        
        Ok(())
    }
    
    pub fn get_items(&self) -> &[ShelfItem] {
        &self.items
    }
}

fn open_file(path: &PathBuf) -> Result<()> {
    use std::process::Command;
    
    #[cfg(windows)]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &path.to_string_lossy()])
            .spawn()?;
    }
    
    Ok(())
}

fn open_folder(path: &Path) -> Result<()> {
    use std::process::Command;
    
    #[cfg(windows)]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_shelf_item_type_detection() {
        assert_eq!(ShelfItem::detect_type(&PathBuf::from("test.jpg")), ShelfItemType::Image);
        assert_eq!(ShelfItem::detect_type(&PathBuf::from("test.mp4")), ShelfItemType::Video);
        assert_eq!(ShelfItem::detect_type(&PathBuf::from("test.mp3")), ShelfItemType::Audio);
        assert_eq!(ShelfItem::detect_type(&PathBuf::from("test.pdf")), ShelfItemType::Document);
        assert_eq!(ShelfItem::detect_type(&PathBuf::from("test.zip")), ShelfItemType::Archive);
        assert_eq!(ShelfItem::detect_type(&PathBuf::from("test.txt")), ShelfItemType::Document);
        assert_eq!(ShelfItem::detect_type(&PathBuf::from("test.unknown")), ShelfItemType::File);
    }
}