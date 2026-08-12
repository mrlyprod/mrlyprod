# snake

Snake on a square grid, written from scratch in Rust. Every round starts from a seed, so the same seed lays out the same apples and replays move for move. The snake, its body and the food are each drawn from a tile you pick yourself, so a round can look like anything from a plain block to a woven pattern.

## Playing

- Arrow keys, WASD, or the on-screen pad turn the snake. You cannot turn back into your own neck.
- The snake moves on a clock. Speed sets how many cells it covers per beat.
- Eating an apple grows you by one segment and drops a new apple on a free cell.
- With wrap on you slide off one edge and back in the other. With it off, the wall ends the round.
- Grid size, apple count, wrap and self collision are all dials; changing one starts a fresh round.
