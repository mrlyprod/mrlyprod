# tile

A studio for designing square tiles. A tile is staged from one of five groups - general, fractal, magic, special, mosaic - with sources drawn from the classic designs or a catalog of canonical base-2 codes, and every knob snaps to the nearest legal combination so the tile always builds. Rolls and paint jobs are seeded, so a design you liked can be rolled again exactly.

## Using

- **roll** dices a fresh tile within the current catalog, parity, and budget; **paint** dices a coating and **strip** takes it off.
- *budget* caps tiles at 16, 32, or 64 cells across; *parity* keeps their numbers even, odd, or both.
- Paint is staged by *edition*, *scheme*, *target*, and *primary* ink.
- Fractal tiles preview a thumbnail for every level the budget allows.
- A special tile always draws a vtree figure; its source is only a rotation mask laid over it, and *flip* swaps the mask.
- The library starts with carpet, net, htree, and vtree on its 12-slot shelf, so **save** adds up to 8 more; **name** and **drop** edit entries, and a tile whose canonical name is already shelved is refused.
- **reset** restores the fresh studio, starters and all, and clears everything saved.
