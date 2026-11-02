# MRLY.NET

- mrly.net's site config: MrlyNet alone, the shell and the system apps; the one folder the platform reads to build mrly.net, and the default of `MRLY_ROOT`.
- Every brand name, domain, tint and social of mrly.net lives here, so the brand lane (`../test/brand.test.ts`) skips this folder; its own test there keeps MrlyProd to the copyright line (`company`), the `llms` line and the home of the code (`git`, the GitHub social).
- `bun run build` from the repo root builds it into `dist/`; `../README.md` has the build.

## FILES

- `site.json`: identity (`title`, `wordmark` (the word the lock and the welcome show), `root`, `since`, `prefix` (`html[data-prefix]`, the localStorage keys), `mark`, `marks` (`png`, `touch`, `og`), `tint`, `company`, `tagline`, `contact`, `socials`), the routes (`roots`, `menu`, `desk`, `design`, `tree`; `designs`, `research` and `widgets`, absent here), the screens (`screen`, `[width, height, dpr]`, what shots take; `min`, `[width, height]`, the smallest screen served), the opt-in `tiles`, `{ id, title }` of the shell's Tiles saver, and the tool blocks (`shots`, its `routes`, `push`, `git`, `llms`, `fonts`, `icons`); `push` names the env keys of its buckets.
- `roots` is `../system` alone; `git.root` is the repo, `../..`.
- `desk` is the shell's Tiles saver: no system app holds a saver.
- `launchpad.json`: the Launchpad sections and the default dock, the system apps only; the Launchpad's Export JSON writes it byte for byte.
- `world.json`: the world every tab boots, with `mrlynet boot` and `sim`: the system kernel apps (`people`, `chat`, `wallet` with its `causes`, `wish`, `vote`, `gov`, `care`, `ads`, and the rest), the test users `test-1` to `test-8`, and `rules`, each a pick drawn at boot: `payments` and `gifts` on or off, `kinds` human, agent, bot or a mix, `grant` 100 to 100,000 (`../system/README.md`, CIVIC); `../README.md` has it.
- `redirects.json`: the moved urls, each `{ "to", "since" }`, answered by the edge with a 301; empty here.
- No `wasm.lock`: no wasm unit is live, so the builder fetches none (`../hosts/web/infra/README.md`).
- `public/`: copied to the site root: `og.png`, `favicon.svg`, `favicon.png`, `favicon.ico`, `apple-touch-icon.png`; committed images, not drawn at build; `site.json` `mark` and `marks` name them; there is no web manifest, so browsers offer no install.
- `wasm/` and `dist/` are build output, gitignored.
