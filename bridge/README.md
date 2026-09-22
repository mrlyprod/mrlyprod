# BRIDGE

- `scripts/bridge.sh` runs `cargo run -q -p bridge` under the cargo lock: it parses `pkgs/mrlyrs/src` (not `src/bin`) with `syn`, writes `bridge/manifest.json`, prints the skip report and the per-unit counts, then runs every backend listed in `BACKENDS` in `main.rs`.
- It exits 1 on a name collision, on a skippable written function missing from `skip.txt`, or on a `skip.txt` line naming nothing; the manifest is written first either way.
- `model.rs` is the contract: `Manifest { krate, version, modules, types, consts, functions }`, serde with `preserve_order`; read the JSON or the types.

## NAMES

- A path is the item's shortest public path after `pub use` (paths, groups, renames, globs); ties go to the defining module; `core::Tensor`, `math::two::census`, `Error`.
- A function reachable only through a private module with no re-export is `private`: listed, never exported.
- Every public module is listed; `items` counts the entries homed there. A module with items may not share a path with a function; fix it in `mrlyrs` by re-exporting the module's leftovers.
- Hand types export their methods as free functions at the type's defining module, self first: `Tensor::of` is `core::tensor::of`; `Cell6d::width` is `math::six::width`, the nearest public module.
- Plain types and enums keep associated functions under the type name, self first when there is one: `gen::recipe::Tile::new`, `life::Boundary::wrap(boundary)`, `life::Fate::all()`.
- A class keeps its methods as methods: `life::Life::step`.
- Two entries with one path and disjoint `dims` are one exported name that dispatches on the cell's dimension, 2 or 3; group by `path`.
- A public trait of the crate (`math::name::Named`) adds every method it declares, required or default, to each type with an `impl Named for X`: `X::to_json` and `X::checked` self first, `X::from_json` and `X::from_url` static, `Self` read as X, docs and `defined_at` from the trait fn, `source: trait`, and `trait` holding the trait's path. Trait consts (`KIND`, `LISTS`, `BARE`) do not cross.
- A backend calls a trait method through the trait, never as an inherent method: `<mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_json(text)` and `<Rule as Named>::to_json(&rule)`; `use mrlyrs::math::name::Named;` in scope also works. A type with trait self methods is a class.
- `Display` and `FromStr` are std traits and do not cross; `named_enum!` types cross as their word instead.

## KINDS

- `hand`: Tensor, Cell, CellNd (Cell2d, Cell3d), Cell6d, Color, Code, Rng; each `hand.rs` owns their crossing per the plan's CROSSING.
- `class`: a struct, or an enum with data, that has a self-taking method; holds the Rust value, public fields as getters, methods as methods; never leaves its wasm unit. A class deriving Deserialize should get a from-data constructor in each backend.
- A field under `#[serde(skip)]`, `skip_serializing` or `skip_deserializing` cannot round-trip as data, so its type is a `class` too; the field records `serde_skip`, its getter carries it, the data form (`to_dict`, `toJSON`, the CLI's JSON) drops it as serde does.
- `plain`: a struct or data enum with Serialize and Deserialize, no self methods and no skipped field; a dict, an object, JSON; no per-type code.
- `enum`: a fieldless enum with Serialize and Deserialize; a string; `named` carries the `named_enum!` words, and `all()` crosses.
- `uncrossable`: anything else (`Error`, `Result`, `Pen`); a function touching one is skipped with the reason.
- A function is `ok`, `skip` with a reason, or `private`. Skips: a type generic, a closure, `impl Trait`, a fn pointer or private type, a `&'static` or explicit-lifetime return, a `&mut` borrow returned, an iterator, a `&mut` plain or slice argument, a class from another unit.
- An elided borrow return (`fn shape(&self) -> &[usize]`) is copied out and crosses; a `&mut Rng`, `&mut Tensor` or `&mut Class` argument is mutated in place.
- The macro-written `name()` returns `&'static str` and is skipped by rule, no `skip.txt` line: the enum is its word. `skip.txt` lists written functions only, one path then one clause per line.
- Consts are listed with a `Ty` and skip on the same rules (`font::pens::UPPERS` holds `&'static`); no `skip.txt` line for a const.

## TY

- `unit`, `scalar` (bool u8 u16 u32 u64 usize i8 i16 i32 i64 f32 f64 char), `str`, `string`, `u128`, `i128`, `code`, `json` (serde_json Value or Map).
- `vec`, `slice` (`&[T]`, `mutable`), `option`, `tuple`, `array` (`len`), `ref` (`mutable`, `lifetime`), `result` (the crate's `Result<T>`), `map` (HashMap or BTreeMap, scalar or string keys), `set` (BTreeSet, a sorted list).
- `hand` {name, dim}, `plain` {path}, `enum` {path}, `class` {path, dim}, `opaque` {path, an uncrossable public type}, `unknown` {text, forces skip}.
- `u128`, `i128` and `code` are a Python int and a decimal string in JS and the CLI; `char` is a one-character string.

## PLUG IN

- Write `bridge/src/<py|js|cli>.rs` with `pub fn write(manifest: &Manifest, root: &Path) -> Result<()>` (`model::Result`, a `String` error); add `mod x;` and `("x", x::write)` to `BACKENDS` in `main.rs`.
- `root` is the workspace root; write generated files under `pkgs/` and never hand-edit them; a wrong line is a generator fix.
- `units.txt` names the wasm units, one per line; a unit is the first segment of a path; `all` is every unit.
- `docs` are the `///` lines for docstrings; `defined_at` is `file:line` under `pkgs/mrlyrs/src`; `derives` and `serde` attributes are recorded verbatim.
- `cargo test -p bridge`: one test per rule in `src/tests.rs`, a determinism test, and the count test: entries = `pub fn` grep lines - lines inside `macro_rules` + `pub const fn` + 2 per named enum + trait fns x `impl Named` blocks.
