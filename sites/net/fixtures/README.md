# Fixtures

- `videos/` is the local stand-in for the film manifest.
- `index.json` names the chunks; each chunk is newest first.
- Dev serves them under `/videos/`, matching the live origin.
- Stills and clips are pressed on demand into `data/net/fixtures/`.
- Pressing needs `ffmpeg`. Without it a cell falls back to a plain plate.
- Nothing here ships: the build never reads this folder.
