# Build

## Prerequisites

- Windows 10/11 x64
- Rust stable with the `x86_64-pc-windows-msvc` target (default on Windows)
- Visual Studio Build Tools with "Desktop development with C++"
- Inno Setup 6, only if you want the installer

## Compile

```powershell
git clone https://github.com/ImmanuelDeo/NamicNotch.git
cd NamicNotch
cargo build --release
```

The binary is `target\release\nusa-clone.exe`. Rename it to `nusa.exe` if you
prefer, the application does not depend on its file name.

Run it and check `%LOCALAPPDATA%\NamicNotch\boot.log`:

```
--- start ---
workers started; creating UI
render target: hardware
```

`render target: software` means DXGI was unavailable. The UI still draws, at a
higher CPU cost.

## Assets

Head overlays live in `assets/mascots/<name>/head_cap.png` and are embedded at
compile time. The loader prefers a real file next to the executable, then one
level above, then the working directory, then the embedded copy. That means you
can swap artwork without rebuilding, but a clean checkout still runs.

## Installer

```powershell
iscc installer\NamicNotchSetup.iss
```

Default install location is `D:\NamicNotch`; change `DefaultDirName` in the
script for a different target. The installer writes the autostart registry value
and the uninstaller removes it.

## Snapshot renderer

`snaptop/` builds a separate binary that draws mascot frames without a window:

```powershell
cd snaptop
cargo run --release -- ..\snapshots
```

It shares `src/mascot/*.rs` through `#[path]` attributes, so a mascot change is
visible in the next snapshot run without duplicating code.

## Test

```powershell
cargo test
```

The test suite covers fullscreen classification and the state transitions around
the closed pill. Rendering itself is verified by the snapshot renderer, not by
assertions.
