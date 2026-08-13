# mrlycore

The zero-dependency core of Mrly: atoms, cells, tensors, images, colors, sound, rng, json. Everything here is written from scratch - the tensor of bytes every board grows from, the patterned atoms that seed it, the png codec, the json value, the ChaCha keystream - so nothing sits below this crate.

A tensor is a small grid of bytes; an atom fills one with a carpet, a net, stripes, or noise. A cell dresses a tensor in colors and tags, an image holds paletted pixels and knows how to become a real png, and the audio module renders notes into samples. Randomness is seeded and replayable, and json is the one interchange value everything speaks.

## Parts

- **tensor** and **atoms** make and pattern the byte grids.
- **cell**, **paint**, **ramp**, and **colors** dress them in ink.
- **image**, **codec**, and **resample** turn colors into pngs and gifs and back.
- **audio**, **rng**, **chacha**, and **state** make sound and seeded chance.
- **json**, **errors**, **io**, **time**, **trig**, and **logs** carry the plumbing.
