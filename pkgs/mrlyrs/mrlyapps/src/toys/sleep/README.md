# sleep

A screensaver to fall asleep to. One soft square of color drifts across a dark field, and every time it touches an edge it bounces away in a new color drawn from the palette. The field is 32x24 cells and the square moves half a cell per step by default, slow enough to watch without watching. Where it starts, where it heads and every color it turns are dealt from a seed.

## Dials

- *cols* and *rows* set the field, 8 to 64 across and 8 to 48 down; *size* is the square's side, 2 to 12 cells.
- *speed* is cells per step, from 0.1 to 2.0.
- *palette* holds five colors out of the box; each bounce draws one at random. Empty it and the square invents its own.
- Colors take names or hex, the whole list at once or one slot at a time (*palette.0*).
- *scale* gives each cell its pixels, 2 to 16.
