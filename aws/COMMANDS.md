# COMMANDS

- Run everything from the `mrlyprod/` root.
- Bucket names and distribution ids live one level up in `../.env`, the desk's one .env.
- The AWS session comes from `aws login`, not `.env`; deploying is Carlo's, never the bot's.
- The bot has no aws at all; everything here is Carlo's hands.
- The release train is the desk's: `../utils/release.py` builds into `data/release/`, this folder ships.

## DEPLOY

```sh
uv run python aws/deploy.py net build     # math wasm + the React landing site into data/net/dist/
uv run python aws/deploy.py net push      # build net, sync changed keys to S3, invalidate CloudFront
uv run python aws/deploy.py git build     # the React projection into data/git/dist/
uv run python aws/deploy.py git plan      # print the shell keys git push syncs and the content keys cdn push syncs
uv run python aws/deploy.py git push      # build git, sync the shell to S3, invalidate CloudFront
uv run python aws/deploy.py web build     # wasm release + the React face into data/web/dist/
uv run python aws/deploy.py web push      # sites/web (wasm release + the React face) to web.mrly.net
uv run python aws/deploy.py bot build     # the React notebook into data/bot/dist/
uv run python aws/deploy.py bot push      # build bot, sync the shell to S3, invalidate CloudFront; notes.json untouched
uv run python aws/deploy.py cdn push      # the content store: manifest, tree, site, raw/, tarball, fonts, licenses; cli/ untouched
uv run python aws/deploy.py cli push      # the built cli tarballs from data/release/<version>/, point cli/latest at it
```

## INFRA

- `cloudfront.py` requires the target before the verb: `net|git|web|cdn|bot`.

```sh
uv run python aws/cloudfront.py net check   # report cert, zone, distribution, dns state
uv run python aws/cloudfront.py bot create  # create the target distribution, set the bucket policy
uv run python aws/cloudfront.py bot flip    # point aliases + route53 at the distribution
uv run python aws/cloudfront.py bot harden  # attach the CSP policy
uv run python aws/cloudfront.py net errors  # sync the custom error responses
uv run python aws/cloudfront.py net prune   # drop the cdn origin and cdn/* behavior
uv run python aws/dns.py records            # list the zone (also: zones, set, drop)
uv run python aws/acm.py list               # list certificates (also: request, validation)
uv run python aws/s3.py buckets             # list buckets (also: keys, drop, mkbucket, wipe, rmbucket)
uv run python aws/s3.py audit               # block flags, versioning, encryption, drift between buckets
uv run python aws/budget.py check           # spend vs the $20 tripwire (also: set)
uv run python aws/iam.py check <user>       # billing reach of a user, caller if unnamed (also: grant)
uv run python aws/iam.py role               # create or update the execution role, write ROLE_ARN
```

## LAMBDA

- `fn.py` deploys one folder of `lambdas/` per function.
- `layers.py` builds and publishes what the functions mount at `/opt`.
- `schedules.py` drives EventBridge Scheduler, one schedule per function.
- `fn.py` is the one registry: short key -> deployed name, env keys, layers, arn shape.
- `schedules.py` keys its rates by that same short key: `game`, not `mrlygame`.
- `on`, `off`, `drop` also take a live schedule name for anything off the registry.

```sh
uv run python aws/layers.py build ffmpeg     # static arm64 ffmpeg into data/layers, exec bit set
uv run python aws/layers.py build numpy      # a wheel layer for arm64 py3.13 (also: boto3, hfhub, pillow, requests)
uv run python aws/layers.py build mrlygame <path>  # any local binary as its own bin layer
uv run python aws/layers.py publish ffmpeg   # publish the zip, write FFMPEG_LAYER_ARN to .env
uv run python aws/layers.py list             # published layers and whether .env pins the latest
uv run python aws/layers.py probe            # one throwaway function per layer set, imports reported, then deleted
uv run python aws/fn.py package game         # zip lambdas/mrlygame into data/fn, no aws
uv run python aws/fn.py deploy game          # code, config, retries off, reserved concurrency 1
uv run python aws/fn.py show game            # state, sizes, layers, retries, concurrency, env keys (also: list, drop)
uv run python aws/schedules.py check         # state, rate, target, drift from the desired rate
uv run python aws/schedules.py set           # create or update the desired schedules, new ones land disabled
uv run python aws/schedules.py on game       # enable one (also: off, drop)
```

- Every function: arm64, python3.13, `handler.handler`, JSON logs.
- Retries are off and concurrency is 1; a runaway costs one worker, not a fleet.
- New schedules are born DISABLED. Enabling is a separate, deliberate verb.
- `on` and `off` read the live schedule first: an update replaces the whole thing.

### PACKAGING

- One folder per function under `lambdas/`: `handler.py`, helpers, `README.md`.
- The bundle is every file in that folder; `handler.py` is required, dotfiles are skipped.
- `mrlygame` ships `handler.py` plus its sibling `video.py`, nothing else.
- Binaries ride in layers, never in the bundle: `/opt/bin/mrlygame`, `/opt/bin/ffmpeg`.
- `MRLYGAME_BIN` overrides the binary path; unset means `/opt/bin/mrlygame`.
- A bin layer needs a linux arm64 build; a mac binary will not run up there.
- `build <layer>` shells out to pip for arm64 wheels; no aws, no credentials.
- Layer ARNs are version-pinned in `.env`, one key per layer: `<NAME>_LAYER_ARN`.
- `save_env` never overwrites, so paste a new layer ARN over the old line yourself.
- Direct zip upload caps at 50 MB; a fatter layer needs the S3 route.
- Each function lists the `.env` keys it needs in `fn.py`: `game` wants `MRLYGAME_BUCKET`.

### CROSS BUILD

- The mrlygame emitter is pure Rust: no C, no build.rs, nothing outside the workspace.
- So `rust-lld`, already in the rustup toolchain, links arm64 linux straight from this mac.
- No docker, no zig, no cargo-lambda; a bare `cargo build --target` dies on Apple's `ld`.

```sh
LLD=$(ls ~/.rustup/toolchains/*/lib/rustlib/aarch64-apple-darwin/bin/rust-lld | head -1)
RUSTFLAGS="-C linker=$LLD -C linker-flavor=ld.lld" cargo build --release \
  -p mrlygame --bin mrlygame \
  --target aarch64-unknown-linux-musl
```

- Out comes a 0.6 MB static ELF under `target/aarch64-unknown-linux-musl/release/`.
- `../utils/release.py` builds musl the same bare way, so these flags fix the cli train too.

### SHIPPING GAME

- Order matters, each step feeds the next through `.env`.
- `MRLYGAME_BUCKET` first, then `iam.py role`, or the role has no s3 grant on it.
- Then cross build, `layers.py build mrlygame <path>`, `publish mrlygame`, `publish ffmpeg`.
- Then `fn.py deploy game`, then `schedules.py set game`, which lands disabled.
- Republishing a layer needs the new ARN pasted over the old `.env` line by hand.

### ROLE

- The role is one: `iam.py role` trusts lambda and scheduler together.
- `ROLE_NAME` in `.env` names it; `iam.py role` writes `ROLE_ARN` back.
- Inline `mrlypolicy`: logs, s3 get/put/delete/list on the `.env` buckets, invoke on `mrly*`.
- `AWSLambdaBasicExecutionRole` is attached on top of the inline policy.
- Re-run `iam.py role` after adding a bucket; the policy is rebuilt from `.env`.
- No credentials live in a function; the role signs the uploads.
- `fn.py`, `layers.py probe`, and `schedules.py` all read `ROLE_ARN`.

## BILLING REACH

- Attaching policies is scriptable; two steps are console only, root, once.
- Account settings -> IAM user and role access to billing information -> Activate IAM Access.
- Billing and Cost Management -> Cost Explorer -> enable once.
- The toggle gates the console pages, not the `ce`, `budgets`, `cur` APIs.
- So `iam.py check` probing green does not prove the console opens.
