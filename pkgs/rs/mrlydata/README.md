# mrlydata

The press, and it runs two jobs off one seeded machinery.

The first job is datasets. Any crate that owns something generative declares a well: a named, seeded source of JSON rows behind one tiny trait. Apps need no declaration at all: the press drives every playable app in the registry with the seeded goose and records the episodes as trail wells, one per route. The press gathers every well, pours each one to a dataset folder on disk, and stamps a manifest so a release can be rebuilt bit for bit.

The second job is artwork. The same seeds press png batches: full artworks with their recipes, bare tiles, and carried sheets whose fourth tile hides a payload in its filled sites. Every batch stamps a manifest of the same shape, so a row alone rebuilds the png it names, byte for byte.

Trail rows come one per accepted call rather than one per episode: flat rows stream, sample and shuffle without unpacking, and the app, seed and step fields still reassemble any episode exactly. Enumerated wells deal their rows in a seed-shuffled order, so a short pour is a fair sample and a full pour still covers the whole set.

## Using

- `cargo run -p mrlydata -- pour <dir> [seed]` pours every dataset (seed 7 by default).
- `cargo run -p mrlydata -- emit data/art [count] [seed]` presses a batch of artworks.
- `cargo run -p mrlydata -- tiles [count] [seed]` presses bare tile pngs into `data/tiles`.
- `cargo run -p mrlydata -- carry <text> [count] [seed]` presses sheets into `data/carry`.
- `cargo run -p mrlydata -- preview <name>` prints three rows of one well.
- `cargo run -p mrlydata -- verify` re-pours samples twice and checks the bytes match.

## Folders

- Each dataset folder holds `train.jsonl` and a `README.md` card.
- A well may tag rows `{"split": "eval"}`; the press routes them to `eval.jsonl`.
- `index.json` names every dataset with row count, byte count and a content sha.
- The sha is the automaton digest of the dataset's jsonl bytes, train then eval.

## Artworks

- One folder per artwork, keyed by its hex key, under a `data/` path.
- Each holds a png per rendering plus `artwork.json`, the full recipe.
- `index.json` names every artwork with its seed, edition and per-file shas.
- Each artwork is generated under its own recorded seed, so the record alone
  rebuilds the same pngs byte for byte.

## Tiles

- One unpainted tile per png, flat in a `data/` folder, named by group and hex key.
- `index.json` names every tile with its seed, group, size, sha and full recipe.
- The recipe alone rebuilds the same png, so a batch is a labelled grid set.

## Sheets

- One carried sheet per png: four tiles laid in the five by five frame.
- The fourth tile is the carrier; the payload rides its filled sites and repeats.
- A tile too small for the payload is redrawn, and a batch that never fits is refused.
- `index.json` names the payload's size and sha, and every sheet's four recipes.

## Watching

- The manifest's `version` field is the release hook.
- A watcher re-emits by diffing `index.json`: when `version` or any sha moves, pour again and ship the folders.
- No network code lives here; the folders upload as plain files.
