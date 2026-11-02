# QUICKSTART

- Host your own site on MrlyNet, under your own name, without a fork.
- Needs: `git`, Bun (the version `package.json` pins), and Rust with `wasm-bindgen-cli` for the wasm units (`scripts/wasm.sh` names the exact version when it is missing).

## EIGHT COMMANDS

```
git clone https://github.com/mrlyprod/mrlyprod && cd mrlyprod
bun install
bun run init ../mysite --title "My Site" --domain mysite.com --design glass
MRLY_ROOT=../mysite bun run wasm
MRLY_ROOT=../mysite bun run build
MRLY_ROOT=../mysite bun run dev
MRLY_ROOT=../mysite bun run push --dry
MRLY_ROOT=../mysite bun run push
```

## WHAT EACH DOES

- `init` writes a site config folder: `site.json`, `launchpad.json`, `redirects.json`, `public/mark.svg`, and `pages/` with a home, a menu, an about page in markdown and a page for missing urls; it refuses a folder that is not empty.
- `--design` is one of `glass`, `mrly`, `classic`, `blocks`; `--domain` defaults to `example.com`, `--title` to `Example`.
- A folder outside the repo gets a `node_modules` link to the repo's, so its pages import the host by package (`@mrlyprod/shell/...`, `@mrlyprod/sdk/...`).
- `wasm` builds the Rust units and copies the ones the site uses into `../mysite/wasm/`; the build stops while one is missing.
- `build` writes the whole site into `../mysite/dist/` (`MRLY_DIST` overrides) and prints one line: the design, the rows, the files, the wasm units.
- `dev` serves it at `http://127.0.0.1:3000` (`PORT` overrides), with hot reload; a new page needs a restart.
- `push --dry` prints what would upload and writes nothing; `push` ships `dist/` to S3 by manifest diff.

## MAKE IT YOURS

- `site.json`: the name, the domain (`root`), the tint, the design, the contact, the socials, the localStorage `prefix`.
- A new page is a new folder in `pages/`: `app.json` (`title`, `line`), `index.md` or `index.tsx`, `icon.ts` and `icon.svg`; then list it in `launchpad.json`.
- `net/README.md` has the full contract: CONFIG, THE UNIT, ROUTES, BUILD.
- A design of your own sits in a folder `site.json` `designs` names; `designs/README.md` has the contract.

## DEPLOY

- AWS is the one target that ships today: an S3 bucket behind a CloudFront distribution, set up outside this repo.
- The CloudFront viewer-request function is `net/hosts/web/infra/edge.js`, with the site's `redirects.json` pasted over its `/*MOVED*/{}` mark; it sends every path with no dot in its last segment to `/index.html`.
- `push` reads `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION` and the bucket env keys the `push` block of `site.json` names (`SITE_BUCKET` after `init`); `DRY=1` keeps the manifest on local disk.
- Any static host works too: serve `dist/`, answer an unknown path with `404.html`, and send every path with no dot in its last segment to `/index.html`, as `edge.js` decides; a Cloudflare Worker with static assets is one way, and no Worker target ships in this repo yet.
