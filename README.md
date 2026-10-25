# Nusa Notch Clone

A living notch for Windows - reverse engineered clone of Nusa Notch.

## Features

- **Dynamic Hit-Testing**: Smart click-through with `WS_EX_TRANSPARENT` toggle
- **Spring-Damper Animation**: Critically-damped spring physics (120 FPS)
- **Neutral Slate-Blue Anti-Magic Theme**: Obsidian Slate, Dark Slate, Cool Steel Grey, Slate-Blue, Muted Wine-Rose
- **Asta Devil Union Eyes**: Awakening/Active/Dozing/WakeSurge phases, pupil plus-shape, aura smoky slate-charcoal
- **Chibi Liebe**: Selective appearance in Music (Headbop), Pomo (FocusGuard), Shelf (CatchingFiles)
- **System Tray**: Start with Windows, Mascot toggle, Minimal Notch, About, Exit
- **WinRT Media Integration**: Spotify/YouTube/Music player via Windows Media Session
- **LRCLIB Lyrics**: Synced lyrics search
- **Pomodoro Timer**: Work/Short Break/Long Break cycles
- **Water Reminder**: Hydration tracking with goals
- **File Shelf**: Drag & drop file management

## Installation

Run `NusaNotchSetup_v1.0.0.exe` to install. Requires Windows 10/11 x64.

## Building

```bash
cargo build --release
iscc NusaNotchSetup.iss
```

## License

MIT License