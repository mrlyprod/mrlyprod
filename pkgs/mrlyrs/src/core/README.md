# core

The substrate of Mrly: tensors, cells, colors, images, seeded chance, json. A tensor is a small grid of bytes. A cell dresses a tensor in colors and tags, paint spreads a palette over it, and an image holds the paletted pixels and knows how to become a png or a gif.

Chance is seeded, so a grid grown from one seed grows again from the same seed. The random stream is owned on purpose, so every draw replays bit for bit wherever it runs; the png and gif codecs and the json value are rented from the png, gif and serde_json crates, the value re-exported as Json and Map.

## Parts

- **tensor** makes and tallies the byte grids.
- **cell**, **paint**, and **colors** dress a grid in ink.
- **ramp** turns counter values into colors; **resample** rescales pixels and squashes them for hex.
- **image** and **codec** turn colors into pngs and gifs, and back into pixels.
- **rng** deals seeded chance: one stream, passed by hand, never global.
- **error** and **named** carry the plumbing: the one error, its Result, the json parser and the macro that names an enum.
