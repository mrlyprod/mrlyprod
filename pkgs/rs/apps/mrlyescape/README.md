# escape

A maze chase on an 11x11 walled grid. You eat your way across the floor while ghosts hunt you down the shortest path, and once every crumb is gone the four doors on the edges open onto the next maze. Each run comes from a seed, so the maze, the colors and every ghost decision replay exactly.

## Playing

- A turn holds until the next one: point up, down, left or right and you keep walking. Walls simply stop you.
- Every floor cell starts with food, and eating one scores a point.
- Ghosts move every few steps - the gap is the *ghost_ratio* dial, 1 to 4 - and each new level trims it until they match you stride for stride.
- Stepping through a door starts the next level; with *map* on random each level deals a different one of the three mazes.
- Touching a ghost ends the run.
