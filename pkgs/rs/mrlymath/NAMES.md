# NAMES

One canonical string per mathematical thing.
Implemented by `mrlymath::name` behind the `Named` trait.

## GRAMMAR

- Every name reads `mrly_<kind>_<body>`.
- Lowercase only; chars `[a-z0-9_]`; filename, URL and dataset safe.
- The body is `_`-separated fields in a fixed per-kind order.
- A field is one of:
  - a tag run: tag letters each followed by a value (`d2`, `c7n3r1`),
  - a bare word (`fractal`),
  - a bare number (the bang code),
  - a flag letter, present iff true (`i`).
- Numbers are plain decimal; a leading zero never appears.
- Slots (repeated layers) print as one field each, tags concatenated.
- Defaults elide by fixed per-kind rules, so one value has one spelling.
- `from_str` accepts the canonical spelling only; `to_str` always emits it.

## LAWS

- `from_str(to_str(x)) == canonical(x)` for every value x.
- `to_str(from_str(s)) == s` for every canonical string s.
- Aliases that draw one picture share one name (see tile).

## KIND BANG

- `mrly_bang_d<dim>[_q<base>]_<code>`.
- Field order: d, q, code.
- `q` elides when the base is 2; `q2` never appears.
- The code must fit the corner space: `code < 2^(base^dim)`.
- Carrier: `name::Bang { code, dimension, base }`.
- Examples: `mrly_bang_d2_7`, `mrly_bang_d3_23`, `mrly_bang_d2_q3_511`.
- Replaces `factory::name` (`mrly_d2_b2_7`) and `Design::name` (`mrly_07`).

## KIND RULE

- `mrly_rule_b<counts>_s<counts>[_w]`.
- Counts are single digits 0-8, concatenated strictly ascending.
- `w` is present iff the boundary wraps.
- Carrier: `name::Rule { birth, survive, boundary }`, bridging `life::Config`.
- Conway is `mrly_rule_b3_s23`.

## KIND SAGA

- `mrly_saga_<op>[_<op>...]`, implemented for `saga::Saga`.
- Each field is one op token, with `x<reps>` trailing when reps exceed 1.
- Reps sit in 2..=99; `x1` never appears; the empty saga prints `id`.
- Op tokens: `rot<k>`, `refh`, `refv`, `tsp`, `auto`,
  `cropc<x>r<y>w<w>h<h>`, `padn<count>c<color>`, `tilew<across>h<down>`,
  `scalek<k>`, `map<ten digits>`, `mov[l|r]<dx>[u|d]<dy>f<fill>`,
  `paintc<x>r<y>p<color>`, `floodc<x>r<y>p<color>`,
  `stepp<pen>b<counts>s<counts>[w]`, `stampw<w>d<digits>` (the constant
  floor's literal, never searched).
- Example: `mrly_saga_rot1_padn1c0_stepp1b3s23x5`.

## KIND TILE

- `mrly_tile_<group>_<fields>`, implemented for `mrlycore::tile::Tile`.
- The name speaks the plane: dimension 2 and base two are implied.
- Sources are always codes; classics normalize to theirs:
  Carpet 7, Net 14, Htree 3, Vtree 5, Void 9.
- Width, height, factor-when-derived and base never print; the size law
  (`Tile::resize`) rebuilds them on parse, and `Tile::check` must pass.
- Rotations print mod 4; `r0` elides; `l1` elides; absent flags elide.
- Codes must sit in the plane: `c` values 0-15.
- Naming a tile that fails `Tile::check`, or one holding a 3d-only
  classic, is out of contract and may panic.

Per group, after the group word:

- general: `c<code>_n<number>[_r<rot>][_i]`,
  where `i` = anti[0] XOR invert (the pair collapses to one bit).
- fractal: `c<code>_n<number>[_l<level>][_r<rot>][_i]`, same `i` fold.
- magic: one slot field per layer `c<code>n<number>[r<rot>][a]`,
  then `[_i]`; `a` = anti of that slot, `i` = invert.
- special: `c<code>_f<factor>_n<number>[_r<rot>][_x][_i]`,
  `x` = flip, `i` = invert; anti is dead here and folds to false.
- mosaic: `f<factor>_n<number>`, then three slot fields
  `c<code>[r<rot>][a]`, then `[_i]`.

Examples:

- `mrly_tile_fractal_c7_n3_l2` - the starter carpet.
- `mrly_tile_general_c3_n5_r1_i`
- `mrly_tile_magic_c7n3_c14n5r2a_i`
- `mrly_tile_special_c5_f3_n5_x`
- `mrly_tile_mosaic_f3_n3_c7_c14r1_c5a_i`
