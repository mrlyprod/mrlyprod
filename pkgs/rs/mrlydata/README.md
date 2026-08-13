# mrlydata

The dataset press. Any crate that owns something generative declares a well: a named, seeded source of JSON rows behind one tiny trait. Apps need no declaration at all: the press drives every playable app in the registry with the seeded goose and records the episodes as trail wells, one per route. The press gathers every well, pours each one to a dataset folder on disk, and stamps a manifest so a release can be rebuilt bit for bit.

Trail rows come one per accepted call rather than one per episode: flat rows stream, sample and shuffle without unpacking, and the app, seed and step fields still reassemble any episode exactly. Enumerated wells deal their rows in a seed-shuffled order, so a short pour is a fair sample and a full pour still covers the whole set.

## Using

- `cargo run -p mrlydata -- emit <dir> [seed]` pours every dataset (seed 7 by default).
- `cargo run -p mrlydata -- preview <name>` prints three rows of one well.
- `cargo run -p mrlydata -- verify` re-pours samples twice and checks the bytes match.

## Folders

- Each dataset folder holds `train.jsonl` and a `README.md` card.
- A well may tag rows `{"split": "eval"}`; the press routes them to `eval.jsonl`.
- `index.json` names every dataset with row count, byte count and a content sha.
- The sha is the automaton digest of the dataset's jsonl bytes, train then eval.

## Watching

- The manifest's `version` field is the release hook.
- A watcher re-emits by diffing `index.json`: when `version` or any sha moves, pour again and ship the folders.
- No network code lives here; the folders upload as plain files.
