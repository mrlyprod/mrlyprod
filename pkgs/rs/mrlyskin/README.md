# mrlyskin

Grids of role integers in, dressed pixels out. A grid says only what each cell *is* - a small number per cell - and a skin says what that number *looks like*, so the same board can be plain blocks, woven patterns, letters, or emoji without the grid changing a byte.

A skin is a list of visuals, one per role. A visual is a background ink (a literal color or a pen index resolved at paint time), an optional motif woven into it, an optional face, and a tint for that face. Baking a skin at some tile size gives an atlas: one tile image per role, plus the faces too large or too fancy to bake, which travel alongside as glyph facts. An atlas over a grid is a raster, and a raster composites to an image or reports itself as a fact.

## Dressing

- **ink** is the ground: a hex color, or a pen the caller supplies.
- **motif** weaves the ground into carpet, net, htree, or vtree.
- **face** is a glyph, an emoji, or a sprite of on/off rows.
- **tint** colors the face, from the ink or from a pen.
- Faces that the font covers bake into pixels; the rest ride out as glyphs.
