# Libraries

A pick is choosing a value for a host: a color, a glyph, a tile. There are
no pickers. Components render inline; nothing floats but toast and the
`ask` confirm.

## The pattern

The vocabulary apps each own a `library`: a persisted list of kept values,
seeded with defaults for cold start.

- **colors** seeds all 15 kernel names. `colors.keep {}`, `colors.drop
  {name}`.
- **emoji** seeds a few catalog emoji. `emoji.keep/drop {value}`, cap 24.
- **font** seeds a few chars. `font.keep {char?}` (no arg keeps the
  current char), `font.drop {char}`, cap 24.
- **tile** seeds the 2D classics. `tile.save {name?}`, `tile.name {id,
  name}`, `tile.drop {id}`, cap 12. Entries carry the same `{v:1, tile,
  paint}` value hosts consume, plus a rendered thumb frame.

Each app's view ends in a library card; libraries ride the app's normal
`state()`/`save()`/`load()`, so they persist and replay like any state.

## Hosts

`components/library.tsx` renders a library inline wherever a host asks for
a value: swatch grid for colors, glyph buttons for emoji/font, canvas
thumbs for tiles. It reads real app state through `peek` (`peeks.ts`,
installed at boot); a tap fires the host's own `${host}.set {key, value}`,
the only journaled call. Sleep and matrix palette slots cycle through the
colors library in place. Timer needs no vocabulary: inline h/m steppers
fire `timer.set duration` directly. Typed glyph fields stay; the kernel
validates and bad input fails the verb.

## Agent parity

Libraries are kernel state behind journaled verbs: an agent peeks the same
library a finger sees, edits it with the same verbs, and its pick is the
same host `.set` a tap fires.
