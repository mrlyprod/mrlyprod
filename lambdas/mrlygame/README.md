# mrlygame lambda

Plumbing for pressing quests in the cloud. Wired to nothing yet.

## Contract

- The event carries one optional `seed`; the default is 7.
- `press` emits one quest, assembles `<key>.mp4`, and reads the key back.
- `handler` presses under `/tmp` and uploads the mp4 and `quest.json`.
- The answer names the key, the seed and both uploaded object paths.

## Runtime

- `MRLYGAME_BUCKET` names the upload bucket and is required.
- `MRLYGAME_BIN` points at the emitter binary; it defaults to `/opt/bin/mrlygame`.
- `ffmpeg` must sit on `PATH`.
- No credentials live here; the caller's role signs the uploads.

## Using

- `uv run lambdas/mrlygame/handler.py local [seed]` presses one quest on this machine.
- Output lands in `data/mrlygame/quest-<seed>/`, wiped and rebuilt on each run.
- Without `MRLYGAME_BIN` the local run falls back to `cargo run -p mrlygame`.
- `uv run lambdas/mrlygame/video.py <dir> [dry]` assembles a pressed folder on its own.
