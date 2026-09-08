# AWS

- `net.ts` is the `mrlynet` Lambda: every 10 minutes it asks GitHub for main, and on a new commit builds the site and pushes what changed.
- `s3.ts` is the S3 client it shares with `sites/net/scripts/pkg.ts` and `push.ts`: credentials from the environment, no SDK.
- The Lambda never runs cargo: `scripts/wasm.sh` uploads `pkg/` once per change and `sites/net/pkg.lock` names the prefix.
- Bundled with `bun build aws/net.ts --target=bun` into one `handler.js`; the infrastructure console lives outside this repo.
