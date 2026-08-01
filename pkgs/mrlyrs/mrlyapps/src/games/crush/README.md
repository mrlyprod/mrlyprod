# crush

A match-3 played like a falling-block game. Tiles drop one at a time onto a 9x9 board; you steer each one into place, and whenever three or more of a kind touch they light up, ready to be crushed for a point apiece. Boards run from 4x4 up to 16x16 with 2 to 8 tile kinds, and every round comes from a seed, so the same seed drops the same tiles.

## Playing

- Nudge the falling tile left or right; each nudge also carries it down a row.
- Drop sends it straight to the floor and locks it.
- Three or more touching tiles of one kind become crushable. Crushing clears them for a point each, and the columns above fall to fill the gaps.
- A new tile spawns at the top; when the top row is blocked, the round is over.
- Board size, tile kinds, speed and the tile design are all dials; changing one starts a fresh round.
