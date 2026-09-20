# AWS

- `net.ts` is the `mrlynet-site` Lambda: a deploy from the desk wakes it, it builds main and pushes what changed.
- The event is `{ source, repo, sha }`; `source` is one of `push`, `schedule`, `automator`, `manual`, and any other word reads as none.
- `repo` says what moved: `mrlyprod/mrlyprod` is this repo, the slug in `SHELF_REPO` is a research-shelf change and rebuilds against the current main, any other slug is ignored.
- `SHELF_REPO` is an `owner/repo` in the Lambda env; empty means no shelf, and then nothing is fetched or mounted at `MRLY_SHELF`.
- A sha without a repo is never trusted; with no trusted sha it falls back to the ETag poll, which is what the hourly safety-net schedule sends.
- A wake for a sha already built is a no-op, unless a line of `build/net/head` is still empty and the build never finished.
- `build/net/head` in `mrlydev` is four lines: source sha, source etag, shelf sha, shelf etag. Every GitHub and codeload fetch retries three times, 1s, 3s, 9s.
- The build mounts `site/node_modules` from the `/opt/node` layer when `/opt/node/bun.lock.sha256` matches the checkout's `bun.lock`, else installs with `--frozen-lockfile`; then it runs `scripts/pkg.ts` and `bun run push` there.
- `NODE_LAYER_DIR` overrides `/opt/node` on the desk; the layer is built from the lockfile by the console outside this repo.
- `net.test.ts` covers the payload parser and the modules decision: `bun test` from `aws/` (a path filter would also match the `/git/` copies under `site/dist`).
- `s3.ts` is the S3 client it shares with `site/scripts/pkg.ts` and `push.ts`: credentials from the environment, no SDK.
- The Lambda never runs cargo: `scripts/wasm.sh` only builds `pkg/`.
- Bundled with `bun build aws/net.ts --target=bun` into one `handler.js`; the infrastructure console lives outside this repo.
