# COMMANDS

- Run from the `mrlyprod/` root: `uv run python aws/<script>.py <verb>`.
- Ids come from a local `.env`; the live values are the built-in defaults.
- Credentials come from the shell, never from `.env`.
- Read-only verbs run bare; a mutating verb prints its plan and needs `--yes`.

## SITE

- `site.py check` - cert, ns, bucket, distribution, wiring, dns: one GO or HOLD each.
- `site.py bucket` - block public access, drop website config, seal the bucket policy.
- `site.py function` - publish the `mrlynet-router` viewer-request function.
- `site.py headers` - publish the `mrlynet-security` response headers policy.
- `site.py distribution` - update the mrly.net distribution to the desired config.
- DNS is never written: mrly.net already points at the distribution and carries the mail records.

## DEPLOY

- `bun run push` in `sites/net` - build against the remote manifest, upload what changed, delete what went.
- The remote manifest is `s3://$MRLYDEV_BUCKET/build/net/manifest.json`; it is the truth, never the local `.cache`.
- Hashed assets carry `max-age=31536000, immutable`; every other object carries `max-age=0`.
- CloudFront runs CachingOptimized with a 1s minimum TTL, so a page refreshes in seconds and nothing is invalidated.
- Uploads go through `Bun.S3Client` in parallel batches of 32: no aws CLI, no SDK, no dependency.
- `deploy.py build` - `bun run build` inside `sites/net`.
- `deploy.py plan` - `bun run push --dry`: the same diff, nothing written.
- `deploy.py status` - remote head sha, remote manifest route count, and when it was written.

## WASM

- `bash scripts/wasm.sh` - wasm-pack build, the two font examples, then hash and upload `sites/net/pkg`.
- The hash is a sha256 over sorted relative paths and file bytes; it lands in `sites/net/pkg.lock`, which is committed.
- Upload goes to `s3://$MRLYPROD_BUCKET/pkg/<hash>/` with `.wasm` and `.js` content types; an existing prefix is left alone.
- `sites/net/scripts/pkg.ts` exports `ensurePkg()`: local `pkg/` when present, else fetch by `pkg.lock`.
- It also exports `pkgHash()`, the same sha256 as the shell, so the two always agree.
- The Lambda never runs cargo; it downloads `pkg/` by `pkg.lock` before building demos.

## CLEAN

- `clean.py distributions` - drop the four dead distributions and six dead OACs.
- `clean.py records` - drop only the web, cdn, git and bot aliases in the mrly.net zone.
- `clean.py buckets` - empty all 13 buckets; every bucket itself stays.
- `clean.py lambda` - schedules, function, layers, stack and role.
- `clean.py user` - strip and drop the `mrlybot` IAM user; `carlo` is never touched.
- `clean.py all` - the five above, in that order.

## ORDER

- `site.py check`
- `site.py bucket --yes`
- `site.py function --yes`
- `site.py headers --yes`
- `site.py distribution --yes`
- `site.py bucket --yes` again, so the policy names the distribution
- `clean.py all --yes`
- `deploy.py push --yes`

## NOTES

- The bucket policy names the distribution ARN.
- So rerun `site.py bucket --yes` once `distribution` has run.
- Nothing here writes to the mrly.net zone; `clean.py records` deletes four dead aliases in it and nothing else.
- `site.py distribution` refuses until the function and headers exist.
- A fresh distribution takes minutes to reach Deployed; `check` reports it.

## LAYERS

- `layers.py publish` - download Bun 1.3.14 aarch64, zip `bun`, `bootstrap`, `runtime.ts`, publish the private `bun` layer.
- `layers.py status` - the published `bun` layer versions in us-east-2, newest first.

## LAMBDA

- One function, `mrlynet`: a builder, not a server. It runs on push, not on visit.
- `iam.py role` - create or update `mrlyrole` and its inline `mrlypolicy`.
- `fn.py bundle` - `bun build aws/net.ts --target=bun` into `data/fn/handler.js`, zipped as `data/fn/mrlynet.zip`. No cloud call.
- `fn.py deploy` - create or update code and config, async retries 0, reserved concurrency 1.
- `fn.py invoke` - one synchronous run, then its status and the tail of its log.
- `fn.py logs` - the last hour of `/aws/lambda/mrlynet`.
- `schedules.py create` - the `rate(10 minutes)` EventBridge schedule, enabled.
- `schedules.py disable` / `schedules.py enable` - the switch; the rest of the schedule is untouched.
- `schedules.py status` - state, rate, target, role, retry policy.

### ORDER

- `bash scripts/wasm.sh` once per pkg change, so `pkg.lock` points at a live prefix
- `uv run python aws/layers.py publish --yes` once, then paste the arn into `BUN_LAYER` in `common.py`
- `uv run python aws/iam.py role --yes`
- `uv run python aws/fn.py bundle`
- `uv run python aws/fn.py deploy --yes`
- `uv run python aws/schedules.py create --yes`
- `uv run python aws/fn.py invoke --yes`
- `uv run python aws/fn.py logs`

### SHAPE

- Runtime `provided.al2` on arm64 with the `bun` layer; handler `handler.fetch`, so the zip holds one `handler.js`.
- Timeout 900s, memory 3008 MB, ephemeral storage 2048 MB; the bun binary is `/opt/bun`, `process.execPath` in the handler.
- The role trusts `lambda` and `scheduler`: logs on the function, read and write s3 on `mrlynet` and `mrlydev`, read on `mrlyprod`.
- The bundle carries the handler only. The site scripts run from the fetched source, so a commit that changes them needs no redeploy.
- `aws/net.ts` reads `s3://mrlydev/build/net/head` (sha then etag), asks GitHub with `If-None-Match`, and on 304 logs `unchanged <sha>` and stops.
- Otherwise it untars the commit into `/tmp/src`, untars the shelf into `/tmp/shelf` as `MRLY_SHELF`, then installs, fetches `pkg/`, and runs `bun run push`.
- Every step logs one line with its elapsed ms: `commit`, `source`, `shelf`, `install`, `pkg`, the `push:` counts, `head`, `done`.
- `clean.py lambda` drops `mrlyrole`; the `mrlynet` function and schedule are named after `MRLYGAME_FUNCTION` there, so drop them by hand until that verb widens.
