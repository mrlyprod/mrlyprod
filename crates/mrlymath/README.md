# mrlymath

The mathematics of Mrly: design codes, their symmetries, counts, tiles, automata and renderings. A small integer code picks the filled corners of a hypercube, that seed grows level by level into a fractal design, and the same code always unfolds into the same shape, so a design can be named, counted and drawn again from its number alone.

Half the crate generates: bang enumerates the codes and their symmetry classes, the dimension pipelines build designs and random tiles in the plane, the cube and the hexagon, and life steps grids through their generations. The other half measures: censuses count fills, voids and exposed faces, graphs trace cores, edges and tunnels, and formulas gives the same counts in closed form. Everything rests on the tensors and cells of mrlycore and the sequences, censuses and graphs of mrlynum, and name pins one canonical string on every design, rule and tile.

## Modules

- **bang** enumerates the design codes, their symmetries and their counts; **rules** marks the cells of a hypercube whose coordinate residues satisfy a rule.
- **dim** holds the N-dimensional cell and the pipeline the fixed dimensions share.
- **two**, **three** and **six** run that pipeline for flat cells, cubes and hexagons: designs, tiles, censuses, graphs and renderings.
- **life** steps, records and renders cellular automata and their stories.
- **formulas** counts fills, grids and surfaces in closed form, without rendering.
- **name** prints and parses the one canonical string of every bang, rule and tile.
- **space** holds the vectors, solids and the packed wire format of 3d scenes.

## Running

- `cargo test -p mrlymath` runs the tests.
- `cargo run -p mrlymath --example paints` prints one painted tile per edition as json.
