# mrlyfont

MrlyFont, the stroked pixel alphabet: paths, rasters, animation. Every glyph is a small grid of on/off cells - five rows for most, seven for the descenders - and every glyph knows the order its cells are drawn in, so text can be painted finished or written stroke by stroke.

The alphabet holds 108 characters: the uppers, their corner-rounded lowers, the digits, the punctuation and arrows, and four seven-row specials. Every glyph carries its own hand-penned stroke order in `pens.rs`, the single source of truth.

## Parts

- **glyphs** holds the raw bitmaps; **letters** builds them into glyphs.
- **pens** holds every glyph's hand-penned strokes; **paths** reads them and drafts new ones.
- **raster** lays a text out as one 0/1 grid.
- **animate** writes a text cell by cell, folds it into a stack, and loops the cycle.
- **serializer** renders glyphs as strings, lists, or JSON.

## Previews

- `cargo run -p mrlyfont --example pen` prints the pen tables; `-- X` drafts one glyph and its stroke floor.
- `cargo run -p mrlyfont --example strip` prints every glyph's stroke frames as JSON.
- `uv run python utils/logos.py motion` draws the loop into `files/logos`.
