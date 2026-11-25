# Architecture

## Window

The notch is a layered, top-most, non-activating window:

- `WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_ACCEPTFILES | WS_EX_NOACTIVATE`
- click-through while collapsed via `WS_EX_TRANSPARENT`
- `WM_MOUSEACTIVATE` returns `MA_NOACTIVATE` so clicking the notch never takes
  focus away from a game or a fullscreen video
- position is `y = 0`, size is `540 x 220` logical pixels, the pill is centred
  inside it

The window is larger than the pill so the expanded panes and the glow have room
without resizing the window on hover.

## Render pipeline

`NotchRenderer` owns a single `ID2D1HwndRenderTarget` (hardware, falling back to
software when DXGI is unavailable, which happens over RDP and in some VMs) and
the DirectWrite formats. One frame:

1. clear to transparent
2. fill the notch silhouette in `#000` — this is what makes the shape read as
   part of the screen edge
3. paint the glass surface inside the same silhouette
4. `PushLayer` with a geometric mask for the content area
5. paint the active pane
6. `PopLayer`

Two details matter and are easy to break:

- **Draw order.** The glass surface is drawn *before* the content mask. When it
  was drawn inside the mask, the flare at the top stayed pure black while the
  body was dark blue, and the notch looked like a separate box glued on.
- **Masking.** Content is clipped with `PushLayer` + `ID2D1RoundedRectangleGeometry`,
  not `PushAxisAlignedClip`. An axis-aligned clip is a rectangle, so glow leaked
  through the four rounded corners onto the desktop.

## Silhouette

`create_merged_notch_geometry` builds the expanded shape: the top edge spans a
`feather` of 44 dip beyond the pill, then two cubic shoulders (34 dip) bring the
outline down to the pill sides before the rounded bottom. The geometry is built
26 dip above `y = 0`, so the top of the shape is off-screen and the visible part
starts flush with the bezel.

The collapsed pill uses `create_top_bezel_notch_geometry`, the older and simpler
variant with a straight top and small corner radius.

## State

`NotchController` holds a `SpringValue` for width, height, corner radius and
content alpha. Section changes animate those four values; nothing measures text
at runtime.

`NotchState` is a flat enum: `Pet`, `Idle`, `Hub`, `Music`, `Control`, `Pomo`,
`Shelf`, `Clips`. `ordered_tabs()` defines the navigation order, and both the
tab strip and the pager dots read from it, so adding a pane means adding one
enum variant and one draw function.

## Input

`handle_panel_click` maps logical pointer coordinates to pane-local actions.
Each pane defines its own hit regions with the same constants the renderer uses.
Drag support for the sliders and the seek bar comes from `WM_MOUSEMOVE` plus
`GetAsyncKeyState(VK_LBUTTON)`, which calls `drag_panel`. A drag that leaves the
notch keeps updating until the button is released, because the window does not
capture the mouse while collapsed.

## Media

`spawn_gsmtc_media_poller` polls the global system media transport controls every
second on its own thread:

- title, artist, playback status
- timeline position and duration
- thumbnail bytes, read through `OpenReadAsync` + `DataReader` and passed to the
  renderer, which decodes them with the `image` crate into a cached D2D bitmap
- position freeze detection: when GSMTC reports the same position for several
  polls while playing, the snapshot sends `-1` and the controller advances with
  its own clock instead of seeking backwards

Lyrics are fetched from LRCLIB on title change and parsed into timestamped lines.
`has_real_lyrics` distinguishes real synced lyrics from the placeholder list, so
an instrumental track shows "No synced lyrics available" instead of scrolling
fake text.

## Audio and brightness

Volume goes through `IAudioEndpointVolume` on the default render endpoint;
brightness through `WmiMonitorBrightnessMethods`. Both are applied on drag, not
on release, with a short poll suppression window so the value the user is
dragging does not get overwritten by the next poll.

## Mascots

`src/mascot/` has one module per character, a shared `core.rs` with the drawing
primitives (tapered ribbons, radial gradients, capsule eyes) and `vfx.rs` with
the effect layer. `atlas.rs` loads PNG head overlays through WIC and keeps them
in a thread-local cache; if a file is missing it decodes an embedded copy, so the
binary runs without an `assets` folder.

The headless renderer in `snaptop/` includes the same mascot modules through
`#[path]` and renders them into an `IWICBitmap`, which is how the frames in
`docs/img/` are produced without a desktop session.
