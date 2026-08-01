# pixel

A pixel canvas where every painted cell is stamped with a little motif - carpet, net, vtree, htree or solid - in an ink color rolled from the round's seed. The grid starts at 24 by 24 cells with each stamp drawn three pixels wide, so even a plain stroke comes out looking patterned.

## Playing

- A stroke is a list of cells; every cell it touches turns on and shows the tile.
- *width* and *height* run from 4 to 64 cells, *tile* scales the stamp from 1 to 8 pixels.
- *design* picks the stamp: carpet, net, vtree, htree or solid.
- **clear** wipes the drawing but keeps the ink; **reset** rolls a new seed and a fresh ink color.
- Changing any setting restarts the round on the same seed, so the ink color holds.
