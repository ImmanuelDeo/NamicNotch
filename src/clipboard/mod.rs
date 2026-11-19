//! Clipboard history + quick notes store for Namic Notch.
//!
//! Pure data structures with no Win32 calls: fully unit-testable on any host.
//! Persistence lives in `%LOCALAPPDATA%\NusaShelf\clipboard.json`.

pub mod win;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Maximum entries kept in history (oldest are dropped).
pub const HISTORY_CAP: usize = 25;
/// Maximum characters kept per clipboard entry (long pastes are truncated).
pub const TEXT_CAP: usize = 4000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipEntry {
    pub text: String,
    /// Unix seconds when the entry was captured.
    pub ts: i64,
}

impl ClipEntry {
    /// Single-line preview capped at `n` chars with an ellipsis when cut.
    pub fn preview(&self, n: usize) -> String {
        preview_of(&self.text, n)
    }
}

/// Collapse whitespace runs to single spaces and cap at `n` chars.
pub fn preview_of(text: &str, n: usize) -> String {
    let flat: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= n {
        flat
    } else {
        let cut: String = flat.chars().take(n.saturating_sub(1)).collect();
        format!("{}…", cut)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClipStore {
    pub history: Vec<ClipEntry>,
    pub notes: Vec<String>,
    /// Sequence number of the last clipboard write we performed ourselves,
    /// so the listener can skip its own echo.
    #[serde(default)]
    pub last_own_seq: u32,
}

impl ClipStore {
    pub fn store_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("NusaShelf")
            .join("clipboard.json")
    }

    pub fn load() -> Self {
        let path = Self::store_path();
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::store_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, serde_json::to_string(self)?)?;
        Ok(())
    }

    /// Push a new clipboard text. Returns true when it was actually added
    /// (dedupe against the newest entry; empty/whitespace ignored; capped).
    pub fn push_clip(&mut self, text: &str, ts: i64) -> bool {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return false;
        }
        if self.history.first().map(|e| e.text.as_str()) == Some(trimmed) {
            return false;
        }
        let mut entry_text: String = trimmed.chars().take(TEXT_CAP).collect();
        if trimmed.chars().count() > TEXT_CAP {
            entry_text.push('…');
        }
        self.history.insert(0, ClipEntry { text: entry_text, ts });
        self.history.truncate(HISTORY_CAP);
        true
    }

    /// Move an entry to the front (used when the user re-copies from history).
    pub fn promote(&mut self, idx: usize) -> Option<ClipEntry> {
        if idx >= self.history.len() {
            return None;
        }
        let e = self.history.remove(idx);
        self.history.insert(0, e.clone());
        Some(e)
    }

    pub fn add_note(&mut self, text: &str) -> bool {
        let t = text.trim();
        if t.is_empty() {
            return false;
        }
        self.notes.insert(0, t.chars().take(TEXT_CAP).collect());
        self.notes.truncate(HISTORY_CAP);
        true
    }

    pub fn remove_note(&mut self, idx: usize) -> bool {
        if idx < self.notes.len() {
            self.notes.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_dedupes_consecutive_duplicates() {
        let mut s = ClipStore::default();
        assert!(s.push_clip("hello", 1));
        assert!(!s.push_clip("hello", 2));
        assert!(!s.push_clip("  hello  ", 3)); // trim-equal
        assert!(s.push_clip("world", 4));
        assert_eq!(s.history.len(), 2);
        assert_eq!(s.history[0].text, "world");
    }

    #[test]
    fn push_ignores_empty() {
        let mut s = ClipStore::default();
        assert!(!s.push_clip("", 1));
        assert!(!s.push_clip("   \n\t ", 2));
        assert!(s.history.is_empty());
    }

    #[test]
    fn history_is_capped() {
        let mut s = ClipStore::default();
        for i in 0..(HISTORY_CAP + 10) {
            assert!(s.push_clip(&format!("item {}", i), i as i64));
        }
        assert_eq!(s.history.len(), HISTORY_CAP);
        assert_eq!(s.history[0].text, format!("item {}", HISTORY_CAP + 9));
    }

    #[test]
    fn long_text_is_truncated() {
        let mut s = ClipStore::default();
        let long = "x".repeat(TEXT_CAP + 50);
        assert!(s.push_clip(&long, 1));
        assert_eq!(s.history[0].text.chars().count(), TEXT_CAP + 1); // + ellipsis
    }

    #[test]
    fn preview_flattens_and_caps() {
        assert_eq!(preview_of("a\n b\t c", 20), "a b c");
        assert_eq!(preview_of("abcdefgh", 4), "abc…");
        assert_eq!(preview_of("abc", 4), "abc");
    }

    #[test]
    fn promote_moves_to_front() {
        let mut s = ClipStore::default();
        s.push_clip("one", 1);
        s.push_clip("two", 2);
        s.push_clip("three", 3);
        let e = s.promote(2).unwrap();
        assert_eq!(e.text, "one");
        assert_eq!(s.history[0].text, "one");
        assert_eq!(s.history.len(), 3);
        assert!(s.promote(99).is_none());
    }

    #[test]
    fn notes_add_remove() {
        let mut s = ClipStore::default();
        assert!(s.add_note("  beli susu  "));
        assert_eq!(s.notes[0], "beli susu");
        assert!(!s.add_note("   "));
        assert!(s.remove_note(0));
        assert!(!s.remove_note(0));
    }
}
