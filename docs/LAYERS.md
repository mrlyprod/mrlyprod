# Layers

The layers are crates. Cargo enforces the DAG through the
`pkgs/mrlyrs/*` manifests: core ← math ← ui; os leans only on core;
apps sees all four; net sees everything. Nothing imports net.

## The one lint

`utils/layers.py` enforces the single rule Cargo cannot: apps never
import other apps. Shared app code goes in a kit, not a sibling.
Run it before shipping (ship.py does).

## Inside a crate

Module boundaries inside a merged crate (math/physics/crypto in
mrlymath, font/music in mrlyui) are convention, not compiler-checked.
Families keep talking through their mod.rs face.
