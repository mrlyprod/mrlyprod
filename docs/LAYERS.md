# Layers

Every top folder under `pkgs/mrlyrs/src` is a layer. `utils/layers.py`
holds the dependency graph (`ALLOWED`) and enforces it; that dict is the
source of truth, not this doc. Run the linter before shipping.

## Rules

- Lower layers never import upward. `core` sits at the bottom and
  imports nothing.
- Families inside `math` and `crypto` talk through their `mod.rs` face,
  never each other's internals.
- Apps never import other apps. Shared app code goes in a kit, not a
  sibling.

## No exceptions

If the linter fails, one of two things is true: the code is wrong (move
it) or the rules are wrong (amend `ALLOWED` in its own diff, with a
reason in the commit). There is no third option, no pardon list. A rule
change grants power to a whole layer, so it should feel heavy; if it
feels too heavy, that is the signal the code belongs elsewhere.
