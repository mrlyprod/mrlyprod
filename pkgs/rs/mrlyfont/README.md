# mrlyfont

MrlyFont, the stroked pixel alphabet: paths, rasters, animation. Every glyph is a small grid of on/off cells - five rows for most, seven for the descenders - and every glyph knows the order its cells are drawn in, so text can be painted finished or written stroke by stroke.

The alphabet holds 108 characters: the uppers, their corner-rounded lowers, the digits, the punctuation and arrows, and four seven-row specials. The wordmark letters carry hand-penned stroke orders; every other glyph derives its own by walking its lit cells.

## Parts

- **glyphs** holds the raw bitmaps; **letters** builds them into glyphs.
- **paths** orders each glyph's cells into strokes.
- **raster** lays a text out as one 0/1 grid.
- **animate** writes a text cell by cell, folds it into a stack, and loops the cycle.
- **serializer** renders glyphs as strings, lists, or JSON.

## Previews

- `cargo run -p mrlyfont --example cycle` prints the wordmark's write-and-hold loop as JSON.
- `cargo run -p mrlyfont --example strip` prints every glyph's stroke frames as JSON.
- `uv run python utils/logos.py motion` draws the loop into `files/logos`.
