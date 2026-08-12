# mines

Minesweeper with a guaranteed safe opening: the mines are only placed *after* your first dig, so the first move can never lose. Boards run from 4x4 up to 30x30 with as many as 200 mines, and the field can be drawn as emoji, as digits, or as generated tiles. Each layout comes from a seed, so a board you liked can be played again exactly.

## Playing

- Tap a covered cell to dig it. A cell with no mines beside it opens its neighbours in a flood.
- Switch to the flag tool to mark a suspected mine. Flagged cells refuse to be dug.
- The counter shows mines left, which is simply the mine count minus the flags you have placed.
- Uncover every safe cell to win. Dig a mine and the whole field is revealed.
- Rows, columns and mine count are adjustable; changing any of them deals a new board.
