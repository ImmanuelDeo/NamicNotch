//! Media manager for Nusa Notch Clone

use anyhow::Result;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{info, warn, debug};
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionPlaybackStatus,
    GlobalSystemMediaTransportControlsSession,
    IGlobalSystemMediaTransportControlsSessionManager,
    IGlobalSystemMediaTransportControlsSession,
};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use windows::Foundation::TimeSpan;
use crate::settings::Settings;

/// Media info structure
#[derive(Debug, Clone, Default)]
pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: u32,
    pub position: u32,
    pub playing: bool,
    pub artwork: Option<Vec<u8>>,
    pub source_app: String,
}

/// Media events
#[derive(Debug, Clone)]
pub enum MediaEvent {
    Update(MediaInfo),
    PlayPause,
    Next,
    Previous,
    Seek(u32),
    SetVolume(f32),
}

/// Media manager for Windows Media Session integration
pub struct MediaManager {
    event_tx: mpsc::Sender<MediaEvent>,
    event_rx: mpsc::Receiver<MediaEvent>,
    settings: Arc<RwLock<Settings>>,
    session_manager: Option<GlobalSystemMediaTransportControlsSessionManager>,
    current_session: Option<GlobalSystemMediaTransportControlsSession>,
    current_info: MediaInfo,
    lyrics_cache: std::collections::HashMap<String, Vec<String>>,
}

impl MediaManager {
    pub fn new(
        event_tx: mpsc::Sender<MediaEvent>,
        event_rx: mpsc::Receiver<MediaEvent>,
        settings: Arc<RwLock<Settings>>,
    ) -> Result<Self> {
        Ok(Self {
            event_tx,
            event_rx,
            settings,
            session_manager: None,
            current_session: None,
            current_info: MediaInfo::default(),
            lyrics_cache: std::collections::HashMap::new(),
        })
    }
    
    pub async fn run(mut self) -> Result<()> {
        info!("Media manager started");
        
        // Initialize COM for Windows Media Control
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        }
        
        // Get session manager
        if let Ok(manager) = self.get_session_manager() {
            self.session_manager = Some(manager);
            self.current_session = self.get_current_session();
        }
        
        // Polling interval for media updates
        let mut poll_interval = interval(Duration::from_millis(500));
        
        loop {
            tokio::select! {
                event = self.event_rx.recv() => {
                    if let Some(event) = event {
                        self.handle_event(event).await?;
                    } else {
                        break;
                    }
                }
                _ = poll_interval.tick() => {
                    self.poll_media().await?;
                }
            }
        }
        
        // Cleanup COM
        unsafe { CoUninitialize(); }
        
        info!("Media manager stopped");
        Ok(())
    }
    
    fn get_session_manager(&self) -> Result<GlobalSystemMediaTransportControlsSessionManager> {
        let op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?;
        let manager = op.get()?;
        Ok(manager)
    }
    
    fn get_current_session(&self) -> Option<GlobalSystemMediaTransportControlsSession> {
        let manager = self.session_manager.as_ref()?;
        manager.GetCurrentSession().ok()
    }
    
    async fn poll_media(&mut self) -> Result<()> {
        if self.current_session.is_none() {
            self.current_session = self.get_current_session();
        }
        let Some(session) = &self.current_session else { return Ok(()); };
        
        // Get playback info
        if let Ok(playback_info) = session.GetPlaybackInfo() {
            if let Ok(status) = playback_info.PlaybackStatus() {
                self.current_info.playing = matches!(
                    status,
                    GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing
                );
            }
        }
        
        // Get timeline properties
        if let Ok(timeline) = session.GetTimelineProperties() {
            if let Ok(duration) = timeline.EndTime() {
                self.current_info.duration = (duration.Duration / 10_000_000) as u32; // 100ns to seconds
            }
            if let Ok(position) = timeline.Position() {
                self.current_info.position = (position.Duration / 10_000_000) as u32;
            }
        }
        
        // Get media properties (title, artist, etc.)
        if let Ok(props) = session.TryGetMediaPropertiesAsync().and_then(|op| op.get()) {
            let mut changed = false;
            
            if let Ok(title) = props.Title() {
                let title_str = title.to_string();
                if self.current_info.title != title_str {
                    self.current_info.title = title_str;
                    changed = true;
                }
            }
            
            if let Ok(artist) = props.Artist() {
                let artist_str = artist.to_string();
                if self.current_info.artist != artist_str {
                    self.current_info.artist = artist_str;
                    changed = true;
                }
            }
            
            if let Ok(album) = props.AlbumTitle() {
                let album_str = album.to_string();
                if self.current_info.album != album_str {
                    self.current_info.album = album_str;
                    changed = true;
                }
            }
            
            if changed {
                self.fetch_lyrics().await;
            }
        }
        
        let _ = self.event_tx.try_send(MediaEvent::Update(self.current_info.clone()));
        
        Ok(())
    }
    
    async fn fetch_lyrics(&mut self) {
        let title = &self.current_info.title;
        let artist = &self.current_info.artist;
        
        if title.is_empty() || artist.is_empty() {
            return;
        }
        
        let cache_key = format!("{} - {}", artist, title);
        
        // Check cache
        if self.lyrics_cache.contains_key(&cache_key) {
            return;
        }
        
        // Fetch from LRCLIB
        if let Ok(lyrics) = Self::fetch_from_lrclib(title, artist).await {
            if !lyrics.is_empty() {
                self.lyrics_cache.insert(cache_key, lyrics);
            }
        }
    }
    
    async fn fetch_from_lrclib(title: &str, artist: &str) -> Result<Vec<String>> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://lrclib.net/api/search?track_name={}&artist_name={}",
            urlencoding::encode(title),
            urlencoding::encode(artist)
        );
        
        let response = client.get(&url).send().await?;
        let results: Vec<LrclibResult> = response.json().await?;
        
        if let Some(first) = results.first() {
            if let Some(synced_lyrics) = &first.synced_lyrics {
                return Ok(synced_lyrics.lines().map(|s| s.to_string()).collect());
            }
        }
        
        Ok(Vec::new())
    }
    
    async fn handle_event(&mut self, event: MediaEvent) -> Result<()> {
        match event {
            MediaEvent::Update(info) => {
                self.current_info = info;
            }
            MediaEvent::PlayPause => {
                if let Some(session) = &self.current_session {
                    if self.current_info.playing {
                        let _ = session.TryPauseAsync().map(|op| op.get());
                    } else {
                        let _ = session.TryPlayAsync().map(|op| op.get());
                    }
                }
            }
            MediaEvent::Next => {
                if let Some(session) = &self.current_session {
                    let _ = session.TrySkipNextAsync().map(|op| op.get());
                }
            }
            MediaEvent::Previous => {
                if let Some(session) = &self.current_session {
                    let _ = session.TrySkipPreviousAsync().map(|op| op.get());
                }
            }
            MediaEvent::Seek(pos) => {
                if let Some(session) = &self.current_session {
                    let requested_pos = pos as i64 * 10_000_000;
                    let _ = session.TryChangePlaybackPositionAsync(requested_pos).map(|op| op.get());
                }
            }
            MediaEvent::SetVolume(vol) => {
                // Volume control would require IAudioEndpointVolume
                // Not implemented via media session
            }
        }
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
struct LrclibResult {
    id: u64,
    track_name: String,
    artist_name: String,
    album_name: Option<String>,
    duration: Option<f64>,
    instrumental: bool,
    plain_lyrics: Option<String>,
    synced_lyrics: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_media_info_default() {
        let info = MediaInfo::default();
        assert!(!info.playing);
        assert_eq!(info.duration, 0);
    }
}