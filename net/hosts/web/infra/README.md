# INFRA

- The code of mrly.net that runs in AWS or talks to it: the edge function, the push, the builder Lambda, the stats Lambda, the S3 client, the content types.
- `bun test net/hosts/web/infra` from the repo root runs every test, each beside the file it covers; `uv run --no-project python net/hosts/web/infra/test_stats.py` runs the stats test.

## FILES

- `edge.js` is the CloudFront function source: one plain file, `decide(uri)` and `handler(event)`, the `/*MOVED*/{}` mark the redirect table replaces and the `/*LOCK*/` mark the guard fills on a locked site.
- `edge.ts` renders the function (`code(moved, locked)`) and evaluates the open form with the config's `redirects.json` pasted in, for dev and shots.
- `guard.js`, `worker.ts` and `locked.ts` are the lock, below.
- `mime.ts` is the one extension table: `kind(path)` answers a content type for push, shots and dev; `rawKind(path, text)` answers what `/raw/` serves: plain text for text and SVG, a passive type (image, audio, video, font, pdf, wasm, archive) as is, anything else as octet-stream.
- `s3.ts` is the S3 client `push.ts`, `builder.ts` and `wasm.ts` share: names and credentials from the environment, no SDK.
- `wasm.ts` fills an empty `<config>/wasm/` from the S3 copy `<config>/wasm.lock` pins (bucket prefix `pkg/<hash>/`, bucket env key `push.wasm` in `site.json`), prints its hash and exits 1 when it does not match the pin; a config with no `wasm.lock` fetches nothing and prints `none`; `pin()` and `wasmHash()` serve `bun run content`, which only reports.

## LOCK

- A site with `lock` in its `site.json` (`{ "realm": "example.org" }`) is private; a site without it is open, as before. No password lives in any config.
- Two halves: the lock page asks, the edge refuses. A check in the page alone hides nothing: every chunk, `/raw/` file and `git.json` stays one request away, so the edge says no to every file without the cookie.
- `guard.js` is the guard both edges run, pure: `gate(ask, kit)` answers pass, a rewrite or a response; `needs(uri)` names the secrets a path reads.
- `edge.js` takes `guard.js` at its `/*LOCK*/` mark behind `import cf from 'cloudfront'` and `require('crypto')`, reads `key`, `salt` and `hash` from the function's KeyValueStore and maps the answer to CloudFront; an open site gets an empty mark and never reads a store.
- `worker.ts` runs the same guard in a Cloudflare Worker with WebCrypto: `guarded(request, env, next)`, the three values as the Worker secrets `LOCK_KEY`, `LOCK_SALT` and `LOCK_HASH`; a cookie from either edge opens the other.
- Open without the cookie: `/_lock/` (the lock page, its script, sheet, fonts and mark), `/robots.txt` (rewritten to `/_lock/robots.txt`, `Disallow: /`), `/_unlock` and `/_lock/out`.
- Every other path, routes, `/raw/`, `llms.txt`, sitemaps and chunks alike, rewrites to `/_lock/index.html`, which names the realm and nothing else.
- The cookie: `lock=<expiry>.<hmac-sha256(key, expiry)>`, `Path=/; Secure; HttpOnly; SameSite=Lax`, 30 days, compared in constant time; an expired, forged or missing one gets the lock page.
- The door: `GET /_unlock` with the password URI-encoded in the `x-lock` header, never in the URL, since a CloudFront Function cannot read a body; `sha256(salt + password)` against the stored hash answers 204 with the cookie, or 401; both are `no-store`.
- `/_lock/out` expires the cookie and 302s home.
- A store that cannot be read fails closed: no cookie passes and the door answers 401.
- `locked.ts` builds `_lock/`: the shell's `unlock.tsx`, booting the site's design's lock part with its password field, in its own bundle with no `site:routes`; its plugin refuses any site module but the design, so the page shows the realm, the wordmark and the look, nothing else.
- `push` adds `_lock/` when `site.json` has `lock`; `bun net/hosts/web/infra/locked.ts` builds it into `dist/` for shots, `MRLY_DESIGN` picking the design.

## WEAKNESSES

- No rate limit: a Function cannot count tries across requests, so only a long password (five random words) stops guessing; an AWS WAF rate rule is the next step if the logs show tries.
- The stored hash is one salted sha256, fast to brute-force if the store ever leaks; a Function offers no bcrypt or scrypt.
- The password crosses the wire in a header over TLS: our code never logs it and CloudFront's standard logs leave custom headers out, but a proxy or an extension that logs headers sees it.
- The cookie is a bearer token: whoever copies it is in until it expires or the key rotates; standard logs can record the `Cookie` field, so a locked site's logs, if any, must leave it out.
- One password for everyone: no users and no revocation per person; rotating the key logs everyone out.
- A locked path answers 200 with the lock page, not 401, since a rewrite keeps S3's status; `X-Robots-Tag: noindex` and `Disallow: /` keep crawlers off.
- A locked site's 404 page is the lock page, so the shell's `404.html` never reaches a stranger; a visitor with the cookie who asks for a missing file sees it too.
- The lock hides a site, never its repo, its design code, its domain or its wordmark.

## PUSH

- `push.ts` builds the site and ships it by manifest diff; the `push` block in `site.json` names the prefix, the guarded paths, the bucket env keys and the manifest store.
- A file the build hashed is immutable for a year; every other file is max-age 0 and s-maxage 60; `push` refuses an immutable file with no hash in its name.
- Upload order: hashed files, then the rest, then the shell (`index.html`, `404.html`), each group done before the next; a removed file is kept a week, then deleted.
- `--dry` prints the tiers and writes nothing; `--force` uploads every file; `DRY=1` holds the manifest on local disk instead of S3.

## BUILDER

- `builder.ts` is the builder Lambda: the event, the head, the GitHub ancestor check and ETag poll, the `/opt/node` modules layer, then `bun run push`, over the config `../../../../aws/net.ts` passes in.
- A `manual` event with `force: true` skips the seen and unchanged answers and pushes with `--force`.

## STATS

- `stats.py` is the stats Lambda: CloudFront, Lambda and bucket numbers into one JSON key, read by `/stats/`.
- It reads `STATS_BUCKET`, `STATS_DISTRIBUTION` (empty means no cdn block), `STATS_FUNCTIONS`, `STATS_KEY` and the `label=prefix` lists `STATS_SIZES`, `STATS_COUNTS` and `STATS_FOLDERS`, each empty when unset.
