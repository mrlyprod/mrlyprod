# mrlylab

The laboratory of Mrly: the campaign machinery that explores and measures the designs. It rests on mrlycore and mrlymath, and unlike them it is free to break its API, because studies land here one module at a time as the research promotes them.

Three studies live here today. The moire study samples a design's residue rule over a pixel grid at one side number after another and stacks those layers into a field whose interference it renders to png. The sequence press reads the integers through the same rule, one digit per residue corner, keeps the ones every digit lands inside, and weighs every design of a universe in a single sweep.

## Studies

- **moire** layers one design at many scales into an interference field.
- **moire::sample** unpacks a code into its residue table and lays the square or hexagonal lattice under it.
- **moire::layer** samples that table over the pixel grid at one side number, to any fractal depth.
- **moire::stack** sums, intersects, or xors the layers; **moire::field** holds the result and normalizes it.
- **moire::presets** names four recipes: the heatmap, the weave, the hive, and the carpet.
- **moire::render** quantizes a field into colored levels and encodes the png.
- **moire::pairs** correlates two flat carpet layers exactly on their lcm grid and reads the primes off the clear rows.
- **press** keeps the integers whose digit vectors all lie in a design and counts them below any limit.
- **press** also tallies every design of a universe in one pass and profiles magic words without enumerating a cell.
- **ledger** reads every measure of every design as an integer sequence along the level and the odd-side axes, keeps the curated OEIS records with their shifts, identifies typed terms against them, and renders the sequences page.
