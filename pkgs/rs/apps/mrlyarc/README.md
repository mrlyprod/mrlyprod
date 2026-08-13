# arc

A cabinet of grid puzzles. Every task shows a few example pairs - an input
grid and the output some hidden rule makes of it - then hands over fresh test
inputs. The player sizes a working grid, paints it cell by cell with ten pens,
and submits; an exact match is the only score, and a task is over when every
test input is solved.

Two corpora ride inside the crate as packed blobs split into train and eval;
their provenance and licenses live in `corpus/` (see `corpus/SOURCES.md`).
A third set is a live arcade seam that needs a session and stays dark offline.

## Using

- **load** stages a task by index; **pair** and **test** browse it.
- **copy**, **size**, **clear**, **fill**, **paint** shape the working grid.
- **submit** checks the answer cell for cell.
- **solve** hunts a short rule at a chosen depth and paints its answer.
- **fetch** asks the live arcade for a task, in set three only.

## Dials

- `set`: one | two | three, the corpus in play.
- `split`: train | eval.
- `pen`: 0..9, the color paint draws with.
