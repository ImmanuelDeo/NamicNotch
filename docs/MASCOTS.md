# Mascots

Six characters idle behind the pill. Five are drawn entirely in code; three also
use a PNG head overlay for detail that would be tedious to draw with paths.

## Files

| Character | Module | Head overlay |
| --- | --- | --- |
| Asta (Devil Union) | `asta.rs` | `01_asta/head_cap.png` |
| Yuji Itadori | `sukuna.rs` | `02_yuji/head_cap.png` |
| Ryomen Sukuna | `sukuna.rs` | `03_sukuna/head_cap.png` |
| Katsuki Bakugo | `bakugo.rs` | `04_bakugo/head_cap.png` |
| Eren (human) | `eren.rs` | `05_eren_human/head_cap.png` |
| Eren (Attack Titan) | `eren.rs` | `06_eren_titan/head_cap.png` |
| Okarun | `okarun.rs` | `07_okarun/head_cap.png` |

Head overlays are 240x160 RGBA. A gesture overlay, `05_eren_human/gesture_bite.png`
(200x160), is drawn during the transformation window before the titan shift.

## Drawing rules

These exist because the earlier polygon-only versions looked flat next to the
obsidian background:

- Glow is a real radial gradient with four stops fading to alpha 0, never a
  stroked circle.
- Effect ribbons are closed tapered shapes. Constant-width strokes for lightning
  or flame read as wireframe.
- Eyes are one shared capsule routine: no `+` pupils, no vertical slit, no black
  pupil line on top of a glow.
- Head bases stay within a few percent of the background luminance, so the
  silhouette reads without turning into a grey oval.

## Anchors

Every character shares the same eye anchor so the renderer can overlay live eyes
later: centre `(296, 94)`, left eye `(268, 102)`, right eye `(324, 102)`,
`rx 11.7`, `ry 15.6` at 3x scale. The PNG heads are authored against those
anchors.

## Loading

`atlas.rs` decodes head PNGs through WIC into `ID2D1Bitmap` and caches them per
thread. Resolution order: next to the executable, one level up, working
directory, then the embedded bytes. Missing artwork never crashes the renderer —
it just skips that overlay.

## Frames

`docs/img/mascot-grid.png` is produced by `snaptop/`, which renders each frame to
a bitmap without a window. Rebuild it after changing draw code:

```powershell
cd snaptop
cargo run --release -- ..\snapshots
```
