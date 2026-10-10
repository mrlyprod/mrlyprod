# AWS

- `net.ts` is the `mrlynet-site` Lambda: a deploy from the desk wakes it, it builds main and pushes what changed.
- It is thin: one config object over `site/kit/lambda.ts`, the builder core; the core holds the event, the head, GitHub, the modules layer and the build, `net.ts` holds only what is mrly.
- The config is the source slug, the head key, the source dir, the user agent, the bucket env key, the site folder, the four source words, and a `prepare` step that runs `scripts/pkg.ts` before the push.
- The event is `{ source, repo, sha, force }`; `force: true` on a `manual` event skips the seen and unchanged answers and runs `bun run push --force`, which repaints every route and reseals every output; any other source ignores it.
- `source` is one of `push`, `schedule`, `automator`, `manual`, and any other word reads as none.
- `repo` says what moved: `mrlyprod/mrlyprod` is this repo, any other slug is ignored.
- A sha is trusted only with its repo beside it and only when GitHub's compare calls it an ancestor of that repo's main; otherwise it falls back to the ETag poll, which is what the hourly safety-net schedule sends.
- A wake for a sha already built is a no-op, unless a line of `build/net/head` is still empty and the build never finished.
- `build/net/head` in `mrlydev` is two lines: source sha, source etag. Every GitHub and codeload fetch retries three times, 1s, 3s, 9s.
- The build copies `site/node_modules` from the `/opt/node` layer, never a symlink, because Bun mixes each module's path from the site root into its chunk names; it does so when `/opt/node/bun.lock.sha256` matches the checkout's `bun.lock`, else installs with `--frozen-lockfile`; then it runs `scripts/pkg.ts`, which fetches the wasm files in parallel into `pkg.next/` and renames it to `pkg/` once all have landed, and `bun run push` there.
- `NODE_LAYER_DIR` overrides `/opt/node` on the desk; the layer is built from the lockfile by the console outside this repo.
- `site/kit/lambda.test.ts` covers the core, so `aws/` carries no test of its own; `bun test ./kit` from `site/` runs it.
- `site/kit/s3.ts` is the S3 client it shares with `site/scripts/pkg.ts` and `push.ts`: bucket names and credentials from the environment, no SDK.
- The Lambda never runs cargo: `scripts/wasm.sh` only builds `pkg/`.
- Bundled with `bun build aws/net.ts --target=bun` into one `handler.js`; the infrastructure console lives outside this repo.
