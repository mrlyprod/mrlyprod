# Pickers

A pick is choosing a value for a host: a color slot, a glyph, a duration, a
tile. Pickers are pure components floated in a shell overlay. They own no
app.

## Component contract

Each picker (`components/{Color,Glyph,Time,Tile}Picker.tsx`) is a pure
function: vocabulary and current value in as props, one `Node` out, Calls
minted through function props. It emits exactly one journaled Call - the
host's own `${host}.set {key, value}` - via its `onpick` prop. Everything
else it fires (`sheet.turn`, `sheet.close`) stays in the shell.

## The sheet slot

`shell/mount.ts` keeps a client-only `picking` union (color | glyph | time |
tile) modeled on the `asked()` confirm: local state, its own `sheet.*`
namespace, an Overlay Node, no backing app.

- `sheet.open {picker, host, key, ...}` seeds the session and fetches its
  vocabulary.
- `sheet.turn {key, value}` edits the session (glyph category, time
  steppers, tile group/catalog/page) and refetches where needed.
- `sheet.close` clears it.

`sheet.*` never reaches the kernel or the journal. The sheet's Send forwards
the host `.set` untouched, then clears `picking`, so a pick auto-closes the
overlay. The journal sees only `${host}.set {key, value}`.

## Data sources

- **Computed vocabularies** - `palette()`, `glyphs(set)`, `designs(req)`,
  pure wasm face exports read fresh at open/turn time. No app state behind
  them.
- **Live app state** - `peek(handle, app)` stays in the face (demoted,
  unused by the shell) for future state-backed pickers.
- **Journaled verbs** - where browsing is itself an action an agent takes
  (the `colors`, `emoji`, `font`, `tile` apps), it stays a real app with
  real verbs.

## Agent parity

The vocabularies live in Rust behind the face, so an agent sees the same
choices a finger does. An agent's pick is the same `${host}.set` a tap
fires - no picker UI, no `sheet.*`, just the journaled outcome.
