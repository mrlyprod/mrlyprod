# mrlygame lambda

Presses one quest in the cloud and publishes it under the `videos/` prefix.

## Contract

- The event carries one optional `seed`; without it a fresh 64 bit seed is drawn.
- `press` emits one quest, assembles `<key>.mp4`, and presses `<name>.jpg`.
- The mp4 lands at `videos/v/<name>.mp4`, the poster at `videos/p/<name>.jpg`.
- `videos/index.json` is `{count, chunk, chunks}`, newest chunk first.
- A chunk is an array of `{name, seed, video, poster, duration, frames}`, newest first.
- Chunks are written before the index, so a reader never names a missing chunk.
- Nothing is ever deleted; a repressed name replaces its row in place.
- The answer is the new row plus the running `count`.

## Runtime

- `MRLYGAME_BUCKET` names the origin bucket and is required.
- `MRLYGAME_BIN` points at the emitter binary; it defaults to `/opt/bin/mrlygame`.
- `ffmpeg` must sit on `PATH`; the layer puts it at `/opt/bin/ffmpeg`.
- Media is uploaded immutable, the index and chunks `no-cache`.
- No credentials live here; the caller's role signs the uploads.

## Binary

- The emitter rides in a layer as a static arm64 linux ELF, around 0.6 MB.
- The crate is pure Rust, so the toolchain's own `rust-lld` cross links it on macOS.

```sh
LLD=$(ls ~/.rustup/toolchains/*/lib/rustlib/aarch64-apple-darwin/bin/rust-lld | head -1)
RUSTFLAGS="-C linker=$LLD -C linker-flavor=ld.lld" cargo build --release \
  -p mrlygame --bin mrlygame --target aarch64-unknown-linux-musl
```

## Using

- `uv run lambdas/mrlygame/handler.py local [seed]` presses one quest on this machine.
- Output lands in `data/mrlygame/quest-<seed>/`, wiped and rebuilt on each run.
- Without `MRLYGAME_BIN` the local run falls back to `cargo run -p mrlygame`.
- `uv run lambdas/mrlygame/video.py <dir> [dry|poster]` works a pressed folder on its own.
