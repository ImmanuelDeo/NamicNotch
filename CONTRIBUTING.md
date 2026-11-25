# Contributing

## Setup

1. Install Rust (stable, MSVC toolchain) and the Visual Studio Build Tools with
   the Desktop C++ workload.
2. `cargo build --release` from the repository root.
3. Run `target\release\nusa-clone.exe`. Boot messages go to
   `%LOCALAPPDATA%\NamicNotch\boot.log`.

## Before opening a pull request

- `cargo build --release` must pass with no errors. Warnings are tolerated but
  new ones should have a reason.
- Test the change on a real desktop, at 100% and at least one scaled DPI
  setting. The notch is positioned from logical pixels, and DPI regressions are
  easy to miss.
- If you touch `src/notch/renderer.rs`, run the headless renderer in `snaptop/`
  and check the mascot frames still look right.
- Keep the note about draw order intact: the glass surface is painted before the
  content mask on purpose, so the seam at the top edge matches the body.

## What not to send

- Screenshots of fabricated UI. Run the app and capture it.
- Generated art committed alongside code changes. Mascot PNGs live in
  `assets/mascots/` and change only when the artwork changes.
- Large refactors mixed with feature work. Split them.

## Style

Match the surrounding code. The renderer uses raw Win32 and Direct2D calls with
short helper functions; keep that shape rather than introducing an abstraction
layer for a single call site. Comments explain why, not what.
