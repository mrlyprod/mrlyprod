# NAMES

Words, not letters. One word per thing, the same word everywhere. Letters live only inside a formula, declared by the sentence that uses them. Implemented by `mrlyrs::math::name` behind the `Named` trait.

## THE WORDS

| word | meaning |
|---|---|
| `dim` | number of axes: 1, 2, 3 |
| `base` | digits per axis; 2 unless said |
| `code` | the design as a number |
| `level` | substitution depth |
| `side` | cells per axis of a render, `base^level` |
| `lattice` | `square` or `hex`; square unless said |
| `twist` | one unit index per filled digit; absent when nothing turns |
| `magic` | the list of codes of a mixed word |
| `mosaic` | mask code plus palette codes |
| `special` | the mask code of a special tile |
| `factor` | the side of a special or mosaic mask |
| `turn` | quarter turns of a slot |
| `flip` | whether a special tile flips its mask |
| `invert` | whether the finished tile inverts |
| `fill`, `void`, `cells` | counts of a render |
| `faces`, `nodes`, `edges`, `parts`, `holes`, `euler`, `contacts` | the other counts, as the crates name them |
| `area`, `volume`, `tube` | measures |
| `measure` | the reading taken off a design: `fills`, `voids`, `surface`, ... |
| `axis` | the index a reading runs along: `level` or `side` |
| `dimension` | `log(fill) / log(base)`, per axis; a formula, never a parameter |
| `birth`, `survive`, `wrap`, `mask` | life rules |

- Fractal dimension is derived, so it never needs a symbol: write `log(fill) / log(base)` or the number.
- The design's digit set gets no global letter. In a proof: "let `F` be the digits of the code", once, then `F` for that proof.
- Indices `i, j, k` and a running integer `n` stay local, as in every paper.
- The digit transform is `hat F(t) = sum_(d in F) e(d t)`, its level form `hat F_level(t) = prod_(l < level) hat F(base^l t)`; a page that writes it under another letter says so at the first use.

## THE NAME IS A JSON OBJECT

- The canonical form of every named thing is one JSON object: `kind` first, then the words above as keys, in a fixed order per kind, defaults elided, no whitespace. Equality of things is equality of strings.
- Every other form is a view with a function. `to_json()` is canonical; `to_url()`, `to_file()` and `to_mrly()` are cut from it; `to_id()` hashes it. The decodable views have a `from_`.
- Adding a key never breaks an old name: absent means default.
- Registries, catalogs, censuses and the ledger are JSONL: one object per line.
- Rust: one struct per kind under serde, `default` and `skip_serializing_if` for elision, field order as canonical order.

## LAWS

- `from_json(to_json(x)) == checked(x)` for every value `x`.
- `to_json(from_json(s)) == s` for every canonical string `s`.
- `from_json` reads any key order, whitespace and spelt default and folds it to the canonical value.
- Aliases that draw one picture share one name.

## THE VIEWS

| view | Koch |
|---|---|
| `to_json()` | `{"kind":"bang","dim":2,"lattice":"hex","base":3,"code":39,"twist":[0,1,5,0]}` |
| `to_url()` | `/bang?dim=2&lattice=hex&base=3&code=39&twist=0,1,5,0` |
| `to_file()` | `bang_dim=2_lattice=hex_base=3_code=39_twist=[0,1,5,0]` |
| `to_mrly()` | `bang dim 2, hex, base 3, code 39, twist [0 1 5 0]` |
| `to_id()` | the first 8 hex digits of the sha256 of the canonical JSON |

- `to_url()`: the kind is the path, the keys the query string, lists comma-joined, flags `true`. `from_url()` reads it back; a key the kind lists reads a lone value as a one-item list.
- `to_file()`: the kind, then `key=value` joined by `_`, lists in brackets. The alphabet `[a-z0-9_=,\[\]]` is safe on every OS. `from_file()` cuts at an underscore followed by `key=`, so a word value may carry underscores.
- `to_mrly()`: the kind, then `key value` pairs joined by commas, lists in brackets with spaces, a true flag as its bare key, a lattice as its bare word. Pages use this form.
- `to_id()`: one-way; it identifies but never decodes. A registry line holds the object.
- A ledger sequence is its own kind, never a dotted design name: `sequence_dim=3_code=23_measure=surface_axis=level`.

## KIND BANG

- Keys: `dim`, `lattice`, `base`, `code`, `twist`.
- `lattice` elides at `square`, `base` at 2, `twist` when absent or all zero.
- The code fits the digit space: `code < 2^(base^dim)`, and `base^dim` stays below 128.
- A twist holds one unit per filled digit, each below 4 on the square lattice and 6 on the hex.
- Carrier: `name::Bang { dim, lattice, base, code, twist }`.
- Examples: `{"kind":"bang","dim":2,"code":7}` the carpet, `{"kind":"bang","dim":3,"code":23}` the sponge, `{"kind":"bang","dim":2,"base":3,"code":511}`.

## KIND RULE

- Keys: `birth`, `survive`, `wrap`.
- A side is a list of counts, sorted and unique, or a sequence word: the sequence as `life::Source` spells it, then `_zeros` if zeros stay, then `_ones` if ones stay.
- A listed count may be any size: `{"kind":"rule","birth":[12,13],"survive":"fibonacci","wrap":true}`.
- `wrap` elides at false.
- Carrier: `name::Rule { birth, survive, wrap }`; `Rule::of` reads one out of a `life::Config` and `Rule::config` builds one over a mask.
- Conway is `{"kind":"rule","birth":[3],"survive":[2,3]}`.

## KIND SEQUENCE

- Keys: `dim`, `base`, `code`, `measure`, `axis`.
- `base` elides at 2, and the design half obeys the bang code law.
- `measure` is one of `fills`, `voids`, `surface`, `peak`, `heights`, `vertices`, `edges`, `faces`, `euler`, `triangles`, `holes`, `pieces`; `axis` is `level` or `side`.
- Carrier: `name::Sequence { dim, base, code, measure, axis }`; `ledger::Key::named` builds one and `Key::id` hashes it to the row's anchor.
- The carpet's odd-side fills are `{"kind":"sequence","dim":2,"code":7,"measure":"fills","axis":"side"}`, file `sequence_dim=2_code=7_measure=fills_axis=side`, id `8a9e4ce8`.

## KIND TILE

- The key that carries the codes says the group: `code` flat or, with `level`, fractal; `magic` a list of letters; `special` one mask code; `mosaic` three codes.
- Keys in order: `code` | `special` | `magic` | `mosaic`, `factor`, `side`, `level`, `turn`, `flip`, `invert`.
- `side` and `turn` are one number for one slot and one number per slot for a magic tile; a mosaic shares one side.
- `level` elides at 1, `turn` when nothing turns, `flip` and `invert` at false.
- Classics fold to their codes: Carpet 7, Net 14, Htree 3, Vtree 5, Void 9, Point 8, Dust 1, Hline 12, Vline 10, Star 6. Codes sit in the plane, 0 to 15.
- Width, height and base never print and the size law rebuilds them.
- Carrier: `name::Tile`; `Tile::of` folds a `mrlyrs::gen::recipe::Tile` and `Tile::recipe` builds one back, resized and checked.
- Examples: `{"kind":"tile","code":7,"side":3,"level":2}` the starter carpet, `{"kind":"tile","code":3,"side":5,"turn":1,"invert":true}`, `{"kind":"tile","magic":[7,14],"side":[3,5],"turn":[0,2],"invert":true}`, `{"kind":"tile","special":5,"factor":3,"side":5,"flip":true}`, `{"kind":"tile","mosaic":[7,14,5],"factor":3,"side":3,"turn":[0,1,0],"invert":true}`.

## KIND WORD

- Keys: `dim`, `magic`, `side`, `base`.
- `magic` lists the letter codes first letter outermost, `side` the side each renders at, `base` the base of each letter; `base` elides when every letter is base 2.
- Two letters at least; every letter fits its own digit space and no letter is code 0.
- Carrier: `name::Word { dim, magic, side, base }`.
- Example: `{"kind":"word","dim":2,"magic":[7,14,9],"side":[3,7,5]}`.
