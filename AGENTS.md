# Hey Codex

Light, stable rules only. Anything that evolves (design, architecture, API) lives in `docs/`, not here.

## Files
- `TREE.md` for the layout (generated with `uv run python utils/tree.py`)
- `COMMANDS.md` for commands.
- `docs/` for design; `docs/COMMENTS.md` explains tricky code.

## Coding Style
- NEVER write a comment. The one exception is a CAPITALISED section delimiter.
- If code is truly important or confusing, explain it in `docs/COMMENTS.md`, never inline.
- One empty line between sections, never two.
- No em-dash in code, use a hyphen instead.
- Do not add inline comments. Preserve existing comments unless the task calls for changing them.

## Writing
- Less is more!
- Keep all written files lean, clear, and durable.
- This repo is public. Never reference files that live outside it.

## Web
- Rebuild wasm after any change to the mrlyrs/mrlyjs/mrlynet pipeline.
- Backwards compatibility is not a concern. Code freely.

## Environment
- Use `.venv/` (the uv/tooling default).
- Run Python through `uv run`; system pythons are often too old for our tools.
- Batch per-file work into one process; shell loops that spawn a process per file stall under the sandbox.

## Git
- Never hand-write a commit.
- Ship with `uv run python utils/ship.py`.
- Or `uv run python utils/git.py push`.
- One push per finished unit of work.
- While working, `cargo check` is enough.

## Dependencies
- Prefer writing from scratch.
- If a dependency is unavoidable, hide it so it can be swapped later.
- If unsure, stop and discuss.

## Output
- Save generated files to `data/`. Never delete generated files.
- Secrets only ever in `.env`.
