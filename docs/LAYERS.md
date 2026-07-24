# Layers

The layers are crates. Cargo enforces the DAG through the
`pkgs/mrlyrs/*` manifests: core ← math ← ui and core ← font ← ui;
core ← music ← apps; os leans only on core; apps sees all of those;
net sees everything and owns the card. Nothing imports net.

## The one lint

`utils/layers.py` enforces the single rule Cargo cannot: apps never
import other apps. Shared app code goes in a kit, not a sibling.
Run it before shipping (ship.py does).

## Inside a crate

Module boundaries inside a merged crate (math/physics/crypto in
mrlymath) are convention, not compiler-checked. Families keep
talking through their mod.rs face.
