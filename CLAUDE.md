# Hey Claude,

- Light, stable rules only.
- Anything that evolves (design, architecture, API) lives in `docs/`, not here.

## Files
- `TREE.md` for the layout (generated with `uv run python utils/tree.py`)
- `COMMANDS.md` for commands.
- `docs/` for design; `docs/COMMENTS.md` explains tricky code.

## General
- Backwards compatibility is not a concern. Code freely.
- While working, `cargo check` is enough.
- Rebuild wasm after any change to the mrlyrs/mrlyjs/mrlynet pipeline: `wasm-pack build pkgs/mrlyjs --target web`.

## Writing Style
- Less is more!
- Keep all written files lean, clear, and durable.
- This repo is public. Never reference files that live outside it.

## Coding Style
- NEVER write a comment, and delete any you find. The one exception is a CAPITALISED section delimiter.
- If code is truly important or confusing, explain it in `docs/COMMENTS.md`, never inline.
- One empty line between sections, never two.
- No em-dash in code, use a hyphen instead.

## Environment
- Use `.venv/` (the uv/tooling default).
- Run Python through `uv run`; system pythons are often too old for our tools.
- Batch per-file work into one process; shell loops that spawn a process per file stall under the sandbox.

## Git
- Never hand-write a commit.
- Ship with `uv run python utils/ship.py`.
- Or single commit: `uv run python utils/git.py push`.
- One push per finished unit of work.

## Dependencies
- Prefer writing from scratch.
- If a dependency is unavoidable, hide it so it can be swapped later.
- If unsure, stop and discuss.

## Output
- Save generated files to `data/`. Never delete generated files.
- Secrets only ever in `.env`.
