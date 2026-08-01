# two

A generator for flat fractal tilings. One of five designs - carpet, net, htree, vtree, void - is repeated inside itself, *number* cells per side and *level* layers deep, so the default 5 by 5 carpet at level 2 fills a 25 by 25 grid. A census keeps count of the filled and empty cells as the pattern changes.

## Dials

- Page forward and back through the five designs.
- *number* is 3, 5, 7, or 9; *level* starts at 1 and climbs as far as the grid fits.
- *skin* dresses the grid: solid paints the cells in named colors, glyphs swaps the colors for single characters.
- *fill* and *void* set the two cell faces - colors on the solid skin, characters on glyphs.
- The solid skin runs to 256 cells per side; glyphs is budgeted to 2,500 cells in all.
