# AWS

- `net.ts` is the `mrlynet-site` Lambda: a deploy from the desk wakes it, it builds main and pushes what changed.
- It is thin: one config object over `site/kit/lambda.ts`, the builder core both sites share; the core holds the event, the head, GitHub, the modules layer and the build, `net.ts` holds only what is mrly.
- The config is the source slug, the head key, the source dir, the user agent, the bucket env key, the site folder, the `SHELF_REPO` env key, the four source words, and a `prepare` step that runs `scripts/pkg.ts` before the push.
- The event is `{ source, repo, sha }`; `source` is one of `push`, `schedule`, `automator`, `manual`, and any other word reads as none.
- `repo` says what moved: `mrlyprod/mrlyprod` is this repo, the slug in `SHELF_REPO` is a research-shelf change and rebuilds against the current main, any other slug is ignored.
- `SHELF_REPO` is an `owner/repo` in the Lambda env; empty means no shelf, and then nothing is fetched or mounted at `MRLY_SHELF`.
- A sha is trusted only with its repo beside it and only when GitHub's compare calls it an ancestor of that repo's main; otherwise it falls back to the ETag poll, which is what the hourly safety-net schedule sends.
- A wake for a sha already built is a no-op, unless a line of `build/net/head` is still empty and the build never finished.
- `build/net/head` in `mrlydev` is four lines: source sha, source etag, shelf sha, shelf etag. Every GitHub and codeload fetch retries three times, 1s, 3s, 9s.
- The build mounts `site/node_modules` from the `/opt/node` layer when `/opt/node/bun.lock.sha256` matches the checkout's `bun.lock`, else installs with `--frozen-lockfile`; then it runs `scripts/pkg.ts` and `bun run push` there.
- `NODE_LAYER_DIR` overrides `/opt/node` on the desk; the layer is built from the lockfile by the console outside this repo.
- `site/kit/lambda.test.ts` covers the core over both shapes of config, so `aws/` carries no test of its own; `bun test ./kit` from `site/` runs it.
- `site/kit/s3.ts` is the S3 client it shares with `site/scripts/pkg.ts` and `push.ts`: bucket names and credentials from the environment, no SDK.
- The Lambda never runs cargo: `scripts/wasm.sh` only builds `pkg/`.
- Bundled with `bun build aws/net.ts --target=bun` into one `handler.js`; the infrastructure console lives outside this repo.
