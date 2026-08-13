# three

An orbit-camera viewer for three-dimensional fractal tilings. A cube grows from one of six designs - carpet, net, xtree, ytree, ztree, void - and is rendered as a lit mesh whose faces are stepped into four bands of shade. The camera turns, zooms, and pans freely, and the mesh can be handed to a GPU shader that re-uploads geometry only when the shape itself changes.

## Playing

- Turn, zoom, and pan the camera; *view* snaps it back to iso, front, or top, and *ortho* toggles perspective.
- *design*, *number* (3, 5, 7, or 9) and *level* choose the fractal, capped at 32 cells per side.
- *fill* is any named color; *alpha* fades the faces toward glass.
- *edges* overlays the lattice lines, *wireframe* keeps only them, *axes* draws the reference frame.
- *anti* flips the seed before it deepens, growing the design's opposite.
- A census counts the grid, the filled cells, and the void.
