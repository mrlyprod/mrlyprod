# BRIDGE

- `scripts/bridge.sh` runs `cargo run -q -p bridge` under the cargo lock: it parses `crates/mrly/src/lib.rs` and every `crates/mrly-*/src` with `syn`, writes `crates/bridge/manifest.json`, prints the skip report and the per-unit counts, runs every backend listed in `BACKENDS` in `main.rs`, then writes the root `.gitattributes`.
- It exits 1 on a name collision, on a skip missing from `skip.txt`, or on a `skip.txt` line naming nothing; the manifest is written first either way.
- `scripts/bridge.sh check` writes nothing: it prints `stale: <path>` for every file a run would change, the manifest and the version stamps included, and exits 1 on any, or on a refusal above; run it before a commit, so a new `pub fn` cannot leave the bridge stale.
- The root `.gitattributes` lists every file a run wrote, each `linguist-generated=true`; it is the list of files never to hand-edit.
- A line there without that mark is hand-written and kept; a line ending in it is the bridge's and rewritten each run.
- A file the old list marked that a run no longer writes is an orphan: the run deletes it and its emptied folders, and `check` prints `orphan: <path>`.
- The Python tests run on a fresh wheel: `uv run --with ./bindings/py` rebuilds it whenever a Rust source changes, through the `[tool.uv] cache-keys` in `bindings/py/pyproject.toml`.
- `model.rs` is the contract: `Manifest { krate, version, modules, types, consts, functions }`, serde with `preserve_order`; read the JSON or the types.
- The one version is the `[workspace.package]` `version` of the root `Cargo.toml`: every crate takes it through `version.workspace`, the manifest and the generated `bindings/js/package.json` copy it, and `version.rs` rewrites every `"=X.Y.Z"` pin in the root `Cargo.toml` to it; `scripts/bump.sh` moves it, and `bridge bump` gates a publish against the manifest of the highest other `v*` tag, read from the tag at `crates/bridge/manifest.json`; with no such tag it passes as a first release.

## NAMES

- A path is the item's shortest public path after `pub use` (paths, groups, renames, globs); ties go to the defining module; `core::Tensor`, `math::two::census`, `Error`.
- A function reachable only through a private module with no re-export is `private`: listed, never exported.
- Every public module is listed; `items` counts the entries homed there. A module with items may not share a path with a function; fix it in `mrly` by re-exporting the module's leftovers.
- Hand types export their methods as free functions at the type's defining module, self first: `Tensor::of` is `core::tensor::of`; `Cell6d::width` is `math::six::width`, the nearest public module.
- Plain types and enums keep associated functions under the type name, self first when there is one: `math::moire::Spec::new`, `life::Boundary::wrap(boundary)`, `life::Fate::all()`.
- A class keeps its methods as methods: `num::prime::Sieve::step`.
- Two entries with one path and disjoint `dims` are one exported name that dispatches on the cell's dimension, 2 or 3; group by `path`.
- A public trait of the crate (`core::name::Named`) adds every method it declares, required or default, to each type with an `impl Named for X`: `X::to_json` and `X::checked` self first, `X::from_json` and `X::from_url` static, `Self` read as X, docs and `defined_at` from the trait fn, `source: trait`, and `trait` holding the trait's path. Trait consts (`KIND`, `LISTS`, `BARE`) do not cross.
- A backend calls a trait method through the trait, never as an inherent method: `<mrly::life::Rule as mrly::core::name::Named>::from_json(text)` and `<Rule as Named>::to_json(&rule)`; `use mrly::core::name::Named;` in scope also works. A type with trait self methods is a class.
- `Display` and `FromStr` are std traits and do not cross; `named_enum!` types cross as their word instead.
- `Default`, derived or an `impl Default for X`, adds a static `X::default`, `source: default`: a class returns an instance, plain data its dict or object, an enum its word; the CLI door is `X.default`.
- A const-generic type's `default` lands on each alias that fixes N (`tile::build::Config2d::default`); hand types get none.

## KINDS

- `hand`: Tensor, Cell, CellNd (Cell2d, Cell3d), Cell6d, Image, Color, Code, Rng, one row each in the `HAND` table of `model.rs`; `bindings/js/hand/src/lib.rs` and `bindings/py/src/hand.rs` own their crossing.
- `Image` is the one carrier of rendered pixels and crosses with its shape: a numpy `uint8` array `(h, w, 4)` in Python, `{shape: [h, w], colors: Uint8Array}` in JS, serde's `{width, height, colors}` in the CLI; a `Vec<Image>` is a list of those, and an image crossing in is refused unless its colors fill its shape. A bare `Vec<[u8; 4]>` still crosses as `(n, 4)`.
- `Rng.choice(seq)` and `Rng.shuffle(list)` are written in both hand files: Rust's `choice` and `shuffle` run on the index list, so a seed draws what Rust draws; the generic Rust fns stay in `skip.txt`.
- `class`: a struct, or an enum with data, that has a self-taking method; holds the Rust value, public fields as getters and setters, methods as methods; never leaves its wasm unit. A class deriving Deserialize should get a from-data constructor in each backend.
- A class field whose type is a class of another unit is left out of js and listed in `skip.txt`: a wasm unit holds only its own classes, and class objects never cross subpaths; python keeps it.
- A setter reads its value the way a parameter of that type crosses in; a field holding a borrow (`Preset::name: &'static str`) stays read-only.
- A field under `#[serde(skip)]`, `skip_serializing` or `skip_deserializing` cannot round-trip as data, so its type is a `class` too; the field records `serde_skip`, its getter carries it, the data form (`to_dict`, `toJSON`, the CLI's JSON) drops it as serde does.
- `plain`: a struct or data enum with Serialize and Deserialize, no self methods and no skipped field; a dict, an object, JSON; no per-type code.
- Plain input is strict: a missing key is an error. serde fills missing keys from `Default` only for a type carrying `#[serde(default)]`, in every bridge; no type carries it today, so start from `X.default()` and change keys.
- `enum`: a fieldless enum with Serialize and Deserialize; a string; `named` carries the `named_enum!` words, and `all()` crosses.
- `uncrossable`: anything else (`Error`, `Result`); a function touching one is skipped with the reason.
- A function is `ok`, `skip` with a reason, or `private`. Skips: a type generic, a closure, an error constructor, `impl Trait`, a fn pointer or private type, a map keyed by a non-scalar, a `&'static` return other than `&'static str`, an explicit-lifetime return, a `&mut` borrow returned, an iterator, a `&mut` plain or slice argument, a class from another unit.
- An elided borrow return (`fn shape(&self) -> &[usize]`) is copied out and crosses, and a written `&'static str` return (`math::dissection::reach`) crosses as a string; a `&mut Rng`, `&mut Tensor` or `&mut Class` argument is mutated in place.
- The macro-written `name()` returns `&'static str` and is skipped by rule, no `skip.txt` line: the enum is its word. `skip.txt` lists written functions and the class fields js leaves out, one path then one clause per line.
- Consts are listed with a `Ty` and skip on the same rules (`font::pens::UPPERS` holds `&'static`); no `skip.txt` line for a const.

## TY

- `unit`, `scalar` (bool u8 u16 u32 u64 usize i8 i16 i32 i64 f32 f64 char), `str`, `string`, `u128`, `i128`, `code`, `json` (serde_json Value or Map).
- `vec`, `slice` (`&[T]`, `mutable`), `option`, `tuple`, `array` (`len`), `ref` (`mutable`, `lifetime`), `result` (the crate's `Result<T>`), `map` (HashMap or BTreeMap, scalar or string keys), `set` (BTreeSet, a sorted list).
- `hand` {name, dim}, `plain` {path}, `enum` {path}, `class` {path, dim}, `opaque` {path, an uncrossable public type}, `unknown` {text, forces skip}.
- `u128`, `i128` and `code` are a Python int and a decimal string in JS and the CLI; `char` is a one-character string.

## PLUG IN

- Write `crates/bridge/src/<py|js|cli>.rs` with `pub fn write(manifest: &Manifest, root: &Path) -> Result<()>` (`model::Result`, a `String` error); add `mod x;` and `("x", x::write)` to `BACKENDS` in `main.rs`.
- `root` is the workspace root; write generated files under `crates/` and `bindings/` and never hand-edit them; a wrong line is a generator fix.
- Write every file through `disk::save`: it runs a `.rs` file through `rustfmt`, so `cargo fmt --all -- --check` holds on generated code, it leaves an unchanged file untouched, so a second run changes nothing, and under `check` it records the path and writes nothing.
- A hand-written file the bridge only edits in place (the pins of the root `Cargo.toml`) goes through `disk::edit`, which `.gitattributes` does not list.
- `cli.rs` writes only `bindings/cli/src/doors.gen.rs`: the sorted `DOORS` table (name, signature, docs, why uncallable) and one door fn per callable name; the parsers and the commands are hand-written in `bindings/cli/src/lib.rs` and `main.rs`.
- The CLI marks a name `# uncallable` when an argument holds a type without `Deserialize` or the return one without `Serialize`, at any depth (`Vec<Pair>`), or when it takes a `&mut` other than an `Rng`, which it opens from a seed.
- Every type name `mrly list` prints means one JSON shape: a dimension-generic cell prints `Cell2d|Cell3d`, and `Cell` is only the core cell.
- The wasm units are the top-level modules, one per unit crate; each is a subpath `@mrlyprod/mrly/<unit>` with a browser entry `<unit>.js` (`await init()` fetches the wasm) and a Node entry `<unit>.node.js` (reads it with `readFileSync` and calls `initSync` on import).
- The package root `.` is the generated `index.js`: the six units as namespaces plus `init()`, which awaits all six; `package.json` is generated whole, its `exports` and `files` from the units.
- A JS free function named by a reserved word takes a trailing `_`, in the wrapper and in the `.d.ts` alike: `core.tensor.new_`, `math.counts.void_`.
- `docs` are the `///` lines for docstrings; a module takes its own file's `//!` lines first and the `///` on its `mod` line only when the file has none; the facade's `pub use` docs are not read.
- Doc links cross as plain text: `[`Tile::new`](crate::recipe::Tile::new)` reads `` `Tile::new` ``, `[`Rule`]` reads `` `Rule` ``; web links stay.
- `defined_at` is `file:line` under the unit's `src`, the unit first: `core/ramp.rs:87` is `crates/mrly-core/src/ramp.rs:87`; `derives` and `serde` attributes are recorded verbatim.
- `cargo test -p bridge`: one test per rule in `src/tests.rs`, a determinism test, and the count test: entries = `pub fn` grep lines - lines inside `macro_rules` + `pub const fn` + 2 per named enum + trait fns x `impl Named` blocks + 1 per `default`.

## ADDING THINGS

- Every recipe ends with `scripts/bridge.sh`, then `scripts/bridge.sh check`, `cargo test -p bridge`, and `scripts/wasm.sh` before the JS tests; the generated files it rewrites are listed in `.gitattributes`.
- A `pub fn`: write it with `///` docs in `crates/mrly-<unit>/src`; nothing else is hand-written. The run rewrites `crates/bridge/manifest.json`, `bindings/cli/src/doors.gen.rs`, `bindings/py/src/doors.rs`, the `__init__.py` and `__init__.pyi` of its module under `bindings/py/python/mrly/`, `bindings/js/units/<unit>/src/lib.rs`, `bindings/js/<unit>.js` and `bindings/js/<unit>.d.ts`. If the run refuses it as skippable, cross it or add one line to `skip.txt`.
- A type: derive `Clone, Serialize, Deserialize` for `plain` data, add a self method for a `class`, or keep it fieldless for an `enum`; the same files change as for a `pub fn`.
- A module: `pub mod x;` with `//!` docs in the unit crate; the run adds `bindings/py/python/mrly/<unit>/x/` and the namespace in `<unit>.js` and `<unit>.d.ts`.
- A unit crate: `CONTRIBUTING.md` has the hand-written steps; the run then writes `bindings/js/units/<name>/`, `bindings/js/<name>.js`, `<name>.node.js`, `<name>.d.ts`, the `exports` and `files` of `bindings/js/package.json`, `index.js`, `index.node.js`, `index.d.ts` and `bindings/py/python/mrly/<name>/`.
- A hand type: add a row to `HAND` in `model.rs` (its name, Rust paths, JS stem, word, TS type and data form, Python wrappers and stub type); write `hand::<stem>_from_js` and `<stem>_to_js` in `bindings/js/hand/src/lib.rs` and its wrapper in `bindings/py/src/hand.rs`, listed in `HAND_NAMES` in `py.rs`; a type that is not plain serde in the CLI also needs its arms in `cli.rs` (`special`, `read`, `give`); add a test per rule to `tests.rs`.
