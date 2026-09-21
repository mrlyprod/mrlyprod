# math

The mathematics of Mrly: design codes, their symmetries, counts, tiles, automata and renderings. A small integer code picks the filled corners of a hypercube, that seed grows level by level into a fractal design, and the same code always unfolds into the same shape, so a design can be named, counted and drawn again from its number alone.

Half the crate generates: bang enumerates the codes and their symmetry classes, the dimension pipelines build designs and random tiles in the plane, the cube and the hexagon, and `mrlyrs::life` steps grids through their generations. The other half measures: censuses count fills, voids and exposed faces, graphs trace cores, edges and tunnels, and counts gives the same counts in closed form. Everything rests on the tensors and cells of `mrlyrs::core` and the sequences of `mrlyrs::num`, and name pins one canonical JSON object on every design, rule, tile and word.

## Modules

- **bang** enumerates the design codes, their symmetries and their counts; **rules** marks the cells of a hypercube whose coordinate residues satisfy a rule.
- **atoms** fills a tensor with a carpet, a net, beams or noise.
- **cell** holds the N-dimensional cell and the pipeline the fixed dimensions share.
- **two**, **three** and **six** run that pipeline for flat cells, cubes and hexagons: designs, censuses, graphs and renderings.
- **counts** counts fills, grids, surfaces, hex slices and the carry ladder in closed form, without rendering.
- **graph** lifts a grid into a network of nodes and branches and takes its census; **spectrum** reads the Laplacian spectra off it.
- **spirograph** rolls a byte grid as a wheel and traces its pencils; **roulette** counts where the curves cross.
- **spin** spins a raster about its centre; **tourbillon** stacks the turned parity carpets.
- **moire** layers one design at many scales into an interference field; **press** weighs the integers a design's digit rule keeps.
- **name** prints and parses the one canonical JSON object of every bang, rule, tile and word, and cuts its url, file and prose views.

## Running

- `cargo test -p mrlyrs` runs the tests.
- `cargo run -p mrlyrs --example paints` prints one painted tile per edition as json.
