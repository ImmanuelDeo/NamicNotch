# Nusa Notch - Reverse Engineering Documentation

## Overview
Nusa is a native Windows application written in Rust that creates a "living notch" at the top of the screen showing:
- Music playback with controls
- Synced lyrics (via LRCLIB)
- Focus timer (Pomodoro)
- Water reminder
- File shelf (drag & drop)
- To-do list
- Battery, Wi-Fi, headphones status
- Charging status
- Volume, brightness, keyboard overlays

## Technical Stack
- **Language**: Rust (x86_64-pc-windows-msvc)
- **GUI**: Custom Direct2D/DirectWrite rendering (no Electron/Tauri/WebView)
- **Windows APIs**: `windows` crate v0.58.0
- **System Tray**: Shell_NotifyIconW
- **Hooks**: SetWindowsHookExW for global hotkeys
- **Media Control**: GlobalSystemMediaTransportControlsSessionManager
- **Settings Storage**: `%LOCALAPPDATA%\NusaShelf\settings.txt`

## Installation
- Inno Setup installer (NusaSetup.exe)
- Installs to `%LOCALAPPDATA%\Programs\Nusa\nusa.exe`
- Uninstaller: unins000.exe
- Settings directory: `%LOCALAPPDATA%\NusaShelf\`

## Key Features Identified from Binary Analysis

### 1. System Tray Menu
- Start with Windows (registry Run key)
- Nusa Mascot (idle animation with eyes)
- Minimal notch (thin line for thick bezels)
- About Nusa
- Exit

### 2. Notch States (data-s attribute)
- `pet` - Resting mascot with eyes
- `idle` - Closed notch with time/battery
- `music` - Now playing with art, title, artist, progress, controls
- `lyrics` - Split view with synced lyrics
- `pomo` - Pomodoro timer with ring, phases
- `water` - Water tracker with goal, chart, drink button
- `shelf` - File drop zone, file cards
- `todo` - Task list with checkboxes
- `bt` - Headphones battery ring
- `charge` - Charging battery ring

### 3. Settings (from strings)
- `LOCALAPPDATANusaShelf\settings.txt`
- Format appears to be INI-like or simple key=value

### 4. External Dependencies
- LRCLIB API: `https://lrclib.net/api/search?` for lyrics
- Windows Media Transport Controls for music metadata
- Windows Bluetooth APIs for headphones

## Clone Strategy

Since the source code is NOT publicly available (the GitHub repo only contains the marketing website), we need to build a functional clone from scratch using:

1. **Rust** with `windows` crate for native Windows APIs
2. **Direct2D/DirectWrite** for hardware-accelerated rendering
3. **System Tray** via `Shell_NotifyIconW`
4. **Global Hooks** for media keys, volume, brightness
5. **Media Session Manager** for now-playing info
6. **LRCLIB** integration for lyrics
6. **File System Watcher** for shelf

## Project Structure (Planned)
```
nusa-clone/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, tray, message loop
│   ├── notch/
│   │   ├── mod.rs           # Notch window management
│   │   ├── renderer.rs      # Direct2D rendering
│   │   ├── states.rs        # Notch state machine (pet, idle, music, etc.)
│   │   └── layout.rs        # Layout calculations
│   ├── tray/
│   │   ├── mod.rs           # System tray icon & menu
│   │   └── menu.rs          # Menu items & actions
│   ├── media/
│   │   ├── mod.rs           # Media session integration
│   │   └── lrclib.rs        # Lyrics fetching
│   ├── hooks/
│   │   ├── mod.rs           # Global keyboard hooks
│   │   └── hotkeys.rs       # Volume, brightness, media keys
│   ├── shelf/
│   │   ├── mod.rs           # File shelf (drag & drop)
│   │   └── watcher.rs       # File system watcher
│   ├── timers/
│   │   ├── mod.rs           # Pomodoro, water reminder
│   │   └── pomodoro.rs
│   ├── settings/
│   │   ├── mod.rs           # Settings persistence
│   │   └── store.rs         # settings.txt handling
│   ├── mascot/
│   │   ├── mod.rs           # Idle mascot animation
│   │   └── eyes.rs
│   └── utils/
│       ├── mod.rs
│       └── win32.rs         # Win32 helpers
├── resources/
│   └── icon.ico
└── build.rs                 # Resource compilation
```

## Next Steps
1. Initialize Cargo project
2. Add dependencies (windows, direct2d, etc.)
3. Implement basic tray icon
4. Create notch window (WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST)
5. Implement Direct2D renderer
6. Build state machine for notch states
7. Add media session integration
8. Add global hooks
9. Implement settings persistence
10. Add shelf functionality
11. Add LRCLIB lyrics integration
