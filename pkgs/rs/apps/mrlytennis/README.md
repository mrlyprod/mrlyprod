# tennis

Paddle-and-ball against a wall of blocks. The ball moves in fractions of a cell across a square board, 18 cells a side by default, and the wall does not wait: every dozen or so paddle hits it creeps a row closer and a fresh, partly-filled row appears above it, quicker as your score grows. The serve and every new row come from a seed, so a round replays exactly.

## Playing

- Steer the paddle up, down, left or right; it keeps drifting until re-aimed, and it can roam the bottom two-fifths of the board.
- Where the ball lands on the paddle sets the bounce - the centre sends it straight up, the edges send it wide.
- Breaking a block scores a point.
- Letting the ball past the bottom edge ends the round.
- Board size (8 to 40), paddle width, block size, rows and *physics* - the ball's pace - are all dials; changing one serves fresh.
