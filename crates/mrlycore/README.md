# mrlycore

The substrate of Mrly: tensors, atoms, cells, tiles, colors, images, seeded chance, json. A tensor is a small grid of bytes, an atom fills one with a carpet, a net, beams, or noise, and a tile is the recipe that says which atoms stack at which sizes, levels, and turns. A cell dresses a tensor in colors and tags, paint spreads a palette over it, and an image holds the paletted pixels and knows how to become a png or a gif.

Chance is seeded and seekable, so a grid grown from one seed grows again from the same seed. The ChaCha8 keystream and the natural logarithm are owned on purpose, so every draw replays bit for bit wherever it runs; the png and gif codecs and the json value are rented from the png, gif and serde_json crates, the value re-exported as Json and Map.

## Parts

- **tensor** and **atoms** make and pattern the byte grids.
- **tile** holds the recipe for a stack of atoms: its group, parity, sources, and sizes.
- **cell**, **paint**, **enums**, and **colors** dress a grid in ink.
- **ramp** turns counter values into colors; **resample** rescales pixels and squashes them for hex.
- **image**, **codec**, and **io** turn colors into pngs and gifs, and back into pixels.
- **chacha**, **rng**, and **state** deal seeded chance, as one stream or one global.
- **json**, **errors**, **logs**, and **trig** carry the plumbing.
