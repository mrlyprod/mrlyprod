# MrlyProd

- The MrlyMath crates: the mathematics of designs, and the instruments that measure them.
- Five crates, one chain: `mrlycore` <- `mrlynum` <- `mrlymath` <- `mrlylab` <- `mrlyweb`.
- `crates/mrlycore` the substrate: tensors, atoms, cells, tiles, colors, images, seeded chance, json.
- `crates/mrlynum` the instruments: primes, divisors, series, recurrences, fft, lattices, networks.
- `crates/mrlymath` the definitions: design codes, symmetries, counts, names, tiles, automata, renderings.
- `crates/mrlylab` the laboratory: the sequence press and the moire stacks.
- `crates/mrlyweb` the eyes: the wasm bridge that hands the designs to a browser.
- `demos/` the pages that draw them; `demos/README.md` says how to run them under bun.
- `cargo test --workspace` runs every test; `cargo doc --workspace --no-deps --open` reads the crates.
- MIT. This is the way. Why is the secret.
