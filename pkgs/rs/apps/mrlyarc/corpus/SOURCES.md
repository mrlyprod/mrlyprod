# SOURCES

- `one.bin` packs the task JSON of https://github.com/fchollet/ARC-AGI
  at commit `399030444e0ab0cc8b4e199870fb20b863846f34`.
- `two.bin` packs the task JSON of https://github.com/arcprize/ARC-AGI-2
  at commit `f3283f727488ad98fe575ea6a5ac981e4a188e49`.
- Both upstreams are Apache-2.0; the verbatim texts ride along as
  `LICENSE-ONE` and `LICENSE-TWO`.
- Repacked by `examples/vendor.rs` from fresh clones: cells nibble-packed
  two per byte, then zlib-deflated; task ids kept for provenance.
