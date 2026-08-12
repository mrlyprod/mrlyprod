# matrix

The digital rain. Columns of glyphs stream down the screen, bright at the head and fading along the tail, every character rendered from a built-in pixel font. A column that runs off the bottom waits on a 1-in-40 chance each step to fall again in a fresh color. The rain comes from a seed, so the same seed pours the same storm.

## Dials

- *cols* and *rows* size the field, from 4x4 up to 128x96 glyph cells.
- *charset* is the pool of characters, any string you like; the default is 0 through 9 and A through Z.
- *speed* is how many rows a head falls per step (1 to 4), *trail* how long a glyph glows before fading (2 to 32).
- *palette* is the list of colors columns draw from; empty it and every column invents its own.
- Changing a setting pauses the rain for a look; *play* sets it pouring again.
