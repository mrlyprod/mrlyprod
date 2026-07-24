# The default face

A generic CPU renderer in `mrlyui::face`: any app's view - state, actions,
canvas - becomes pixels, then a PNG. Terminal-esque, uniform, deliberately
plain. It is the agent's eyes: a screenshot of any app on any tick, no
browser in the loop.

## The two shot kinds

- **Canvas shot** (`Os::snapshot`, `sys.shot`, cli `shot`) - the app's own
  `frame` fact rendered raw. Apps define availability: no frame fact, no
  shot. Frameless apps fail with "nothing to shoot here". Unchanged.
- **Face shot** (`mrlynet::face::face_png`, cli `face`) - works for every
  app. Renders the whole view and embeds the canvas raster in its body when
  one exists.

## Geometry

320 wide, height derived from content and clamped to [160, 512], rendered
at scale 3 - PNGs are 960 wide. Pure function of the input: no clock, no
RNG, byte-identical across runs.

## Layout

- **Title bar** (20px): 8x8 accent chip (emoji stand-in), title at scale 2,
  beat verb right-aligned muted, hairline rule.
- **Body**: route params, then the canvas embed (integer scale fitting
  308x192; undecodable, oversized, or empty facts degrade to a muted
  `frame WxH` row), then state rows in map order (`frame`, `shade`, `md`
  skipped): scalars as key/value rows, all-scalar arrays inline, number
  grids as `key 16x16` summaries, object arrays listed capped at 12 with
  `+ n more`, nested objects one indent level then brief summaries,
  `data:` strings as `[png Nb]` tokens. A `md` string renders text-only:
  H1 scale 2, H2 accent, paragraphs pixel-wrapped, lists with hanging
  indent, code lines on a faint backdrop; emphasis and links strip to
  plain text.
- **Action bar** (pinned bottom): accent verb names with muted arg hints,
  capped at 8 plus `+ n more`; zero actions reads `no actions`.

## Theme

Board and ink follow darkmode; muted and faint are mixes between them. The
accent is `ROLLABLE[hash(route) % 13]` - deterministic per app. Text uses
the bitmap font; unsupported glyphs (emoji included) draw a hollow box,
never blank. No emoji atlas yet - the sprite atlas is a later phase.

## Totality

The face never fails: null state renders `no state`, hostile or oversized
state truncates with `..` and `+ n more` against the height budget, and
every string passes through the same per-char engine. `face_png` is total
over anything `peek` returns.

## Surfaces

- Rust: `mrlyui::face::{face, face_png, decode}` over a plain `FaceInput`;
  glue in `mrlynet::face::{face_frame, face_rgba, face_png, canvas_rgba}`.
- CLI: `mrlycli face --out f.png` (replay script, focused app), repl
  `:face [path]`. `shot` is the canvas kind and is untouched.
- Python: `mrlypy.peek/capture/capture_png/face/face_png`; ndarrays via
  `np.frombuffer(buf, np.uint8).reshape(h, w, 4)`.
- Example: `cargo run -p mrlynet --example face` writes every route plus
  six lived-in gauntlet shots to `data/face/`.

One pixel pin guards the whole stack (`face_pixels_are_pinned` in mrlyui);
re-record it on deliberate layout changes.
