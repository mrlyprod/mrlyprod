# INFRA

- The code of mrly.net that runs in AWS or talks to it: the edge function, the push, the builder Lambda, the stats Lambda, the S3 client, the content types.
- `bun test ./infra` from `site/` runs every test, each beside the file it covers.

## FILES

- `edge.js` is the CloudFront function source: one plain file, `decide(uri)` and `handler(event)`, and the `/*MOVED*/{}` mark the redirect table replaces.
- `edge.ts` evaluates `edge.js` with `../redirects.json` pasted in, for dev and shots.
- `mime.ts` is the one extension table: `kind(path)` answers a content type for push, shots and dev; `rawKind(path, text)` answers what `/raw/` serves: plain text for text and SVG, a passive type (image, audio, video, font, pdf, wasm, archive) as is, anything else as octet-stream.
- `s3.ts` is the S3 client `push.ts`, `builder.ts` and `wasm.ts` share: names and credentials from the environment, no SDK.
- `wasm.ts` fills an empty `../wasm/` from the S3 copy `../wasm.lock` pins (bucket prefix `pkg/<hash>/`) and prints its hash.

## PUSH

- `push.ts` builds the site and ships it by manifest diff; the `push` block in `site.json` names the prefix, the guarded paths, the bucket env keys and the manifest store.
- A file the build hashed is immutable for a year; every other file is max-age 0 and s-maxage 60; `push` refuses an immutable file with no hash in its name.
- Upload order: hashed files, then the rest, then the shell (`index.html`, `404.html`), each group done before the next; a removed file is kept a week, then deleted.
- `--dry` prints the tiers and writes nothing; `--force` uploads every file; `DRY=1` holds the manifest on local disk instead of S3.

## BUILDER

- `builder.ts` is the builder Lambda: the event, the head, the GitHub ancestor check and ETag poll, the `/opt/node` modules layer, then `bun run push`, over the config `../../aws/net.ts` passes in.
- A `manual` event with `force: true` skips the seen and unchanged answers and pushes with `--force`.

## STATS

- `stats.py` is the stats Lambda: CloudFront, Lambda and bucket numbers into one JSON key, read by `/stats/`.
- It reads `STATS_BUCKET`, `STATS_DISTRIBUTION` (empty means no cdn block), `STATS_FUNCTIONS`, `STATS_KEY` and the `label=prefix` lists `STATS_SIZES`, `STATS_COUNTS` and `STATS_FOLDERS`, each empty when unset.
- `test_stats.py` is its test, plain `unittest`.
