# graph

An orbit-camera viewer for the skeleton inside a three-dimensional fractal tiling. A cube grows from one of six designs - carpet, net, xtree, ytree, ztree, void - and one of three extractors lifts a network from it: the core threads the filled sites, the tunnel threads the empty ones, the edge traces the lattice between them. Nodes are drawn as particles colored by role, branches as segments in the fill, and the solid can ride beside them as translucent glass.

## Playing

- Turn, zoom, and pan the camera; *ortho* toggles perspective.
- *design*, *number* (3, 5, 7, or 9) and *level* choose the fractal, capped at 27 cells per side.
- *kind* picks the network: core, tunnel, or edge.
- *particles* draws the nodes, *segments* the branches, *axes* the reference frame.
- *ghost* fades the solid in behind the bones, *alpha* sets how far.
- A census counts the nodes, branches, tips, junctions, components, total length, and box-counting dimension.
