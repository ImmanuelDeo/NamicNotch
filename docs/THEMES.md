# Theme and layout

The expanded notch is one surface with six panes. This document lists the values
that define how it looks, so a change can be made once and stay consistent.

## Surface

| Layer | Value |
| --- | --- |
| Base gradient | `#050609` a 0.97 → `#0A0D14` a 0.95 → `#0D121D` a 0.96, vertical |
| Top fade | first 18% of the shape fades from transparent so the seam stays black |
| Specular sheen | 26 dip, white a 0.08 → 0 |
| Rim | 1.15 px stroke, white 0.24 → cyan 0.14 → white 0.07, diagonal |
| Ambient tint | radial, a 0.13 at the bottom centre, colour per pane |

Accent per pane: cyan for Hub and Control, indigo for Music, amber for Focus,
emerald for Control while charging, teal for Shelf, blue for Clips.

## Geometry

| Value | Size |
| --- | --- |
| Expanded pill | 450 x 168 dip, radius 24 |
| Collapsed pill | 168 x 34 dip, radius 15 |
| Content padding | 14 dip |
| Tab strip | y 8..30, six equal slots |
| Pager dots | y bottom − 12, 13 dip apart, active pill 12 x 4 |
| Feather (top flare) | 44 dip |
| Shoulder | 34 dip |
| Shape offset | −26 dip above the screen edge |

## Type

DirectWrite formats are created once in `NotchRenderer::new`:

- `fmt_clock_hero` — 32 pt bold, tabular figures for the clock and countdown
- `fmt_title` — 12.5 pt semibold
- `fmt_body` — 12.5 pt medium
- `fmt_caption` — 10.5 pt regular, used for labels and timestamps

Legibility rule used in every pane: body text at alpha 0.85 or higher, secondary
text never below 0.5.

## Adding a pane

1. Add a variant to `NotchState` and to `ordered_tabs()`.
2. Add dimensions to `target_dimensions_for` — expanded panes are all 450 x 168.
3. Write `draw_<name>_glass`, call `draw_glass_header` first, keep content inside
   y 38..146 so the pager dots stay clear.
4. Add the hit regions to `handle_panel_click` with the same coordinates.
5. Add the arm to `draw_glass_panel`.

## Anti-card rule

Panes do not use nested cards. Content sits on the glass and is separated by
1 px hairlines at `rgba(255,255,255,0.08)`. A box inside a box inside a box is
what makes desktop widgets look like web dashboards.
