# Archive

- This folder is the public archive of the research tree as of mrlyprod commit `37db942b702797eafe3f57837a606d3122681ff7`.
- The site, Cargo and Bun ignore it: no route, no workspace member, no test reads it.
- It is rebuilt into the site one topic at a time; until then it is read, not edited.
- `demos/` was `site/demos/`, with the demo-only files of `site/lib/` in `demos/lib/`.
- `figures/` holds the `research-*`, `paper-*`, `wiki-*`, `demo-*` and `moire-*` figures of `figures/` and its `census/` crate.
- Files keep their content as of that commit, so their paths and imports name that layout.

# MrlyMath

The mathematics of the mrly tree: a parity rule on the corners of a cube, substituted into itself by the Kronecker product, and everything that falls out of those two moves. One subject, one tree, and this page is its law; the tree is the site at [mrly.net/research](https://mrly.net/research/), built from these files and nothing else.

## THE LAW

- The tree is the site: `P.md` here is the page `/research/P/`, a folder's `README.md` opens that folder's index page, and `lab/` has no page, only its code.
- Type folders, not topic folders: a topic is one slug that repeats across `notes/`, `claims/`, `papers/`, `lab/` and `../figures/`, never a box of its own.
- `wiki/<slug>.md` is a concept page: known mathematics, one idea a page, worked from small cases, opening on a front matter block of `title`, `lead` and `prerequisites` (wiki slugs) and carrying an `## In the tree` section; it never states a claim of this tree.
- `notes/<slug>.md` is a note: timeless, present tense, no dates and no story, opening on a front matter block of `title`, `lead`, `figure` and `slug`; the index of the notes is built from that block and nothing is registered anywhere.
- `claims/<slug>.md` is a claims file: one dated line per claim, `- YYYY-MM-DD [Tag] the claim. Witness: its generator`, or for a Proved row the note section that carries the proof, append only; each file is one page with its tag filter, and `claims/README.md` holds the top 10, each row a claim quoted verbatim from the file it links, above the list of every file.
- Every claim carries exactly one tag, defined here and never restated on a page: **Proved** means a proof is given or restated on the page; **Verified** means recomputed from scratch by a crate test or a lab study, or read at source in the literature; **Conjecture** means checked on a finite domain with no derivation; **Refuted** means killed, with the witness beside it.
- A claim is never deleted because someone else published it first: a dated cross-reference line is added, and only a counterexample moves a claim.
- Every printed number names its generator: a crate function in `../crates`, a study in `lab/rs/` or `lab/py/`, a sequence row in `sequences.md`, or an [OEIS](https://oeis.org) entry; a number with no generator keeps its claim at Conjecture.
- A theorem from the literature is cited at its source, resolved to one URL in `REFS.md`; a claim of this tree never rests on a citation alone.
- `sequences.md` is one generated ledger, written by `cargo run -p ledger`: a sequence is a row only when a claim cites it or the OEIS holds it, its id the canonical JSON name, its anchor the id's hash; never a page per sequence, and the enumerated millions are data, not pages.
- `papers/<slug>.md` is a paper: markdown, a cover in front matter (`title`, `lead`, `date`, `figure`), print CSS for the paged document, no LaTeX; the shelf of first editions is deprecated and never edited.
- `../figures/` holds one file a figure, `<name>.ts`, drawn through the pen of `mrlyjs/view`; `bun ../figures/press.ts` presses it to `<name>-dark.webp`, `<name>-light.webp` and `<name>.png` under `data/`, keyed by its sources, and no image enters git; the desk's figures console uploads the keys and pins them in `../site/figures.lock`, which is committed with the figure, and `bun run check` in `../site` is red until the lock matches the press; a figure is drawn by code, never by hand, and a page names a figure by its name alone.
- `lab/rs/<study>/` is a Rust crate in the root workspace and `lab/py/<study>/` a Python study run with `uv`; a study README says what it computes, how to run it and which claim lines it witnesses, and a study is deleted the day a crate function or a demo computes its numbers.
- The demos in `../site/demos` run the same crates through wasm; a note links the demo that shows it, and every demo is linked from some page.
- A link between pages is a relative `.md` path, as `[cuts](notes/cuts.md)` and `[parity](wiki/parity.md)` are from this page, and the site makes the URL; the only site URL in content is a demo's, `/demos/<name>/`.
- Math is written in backticks so it renders everywhere; lines never wrap; no comment in code beyond a section delimiter, no em-dash anywhere.
- Author line MrlyProd on every route: Carlo directs, Claude writes and computes, and `/method/` says so.
- One check: `bun run check` in `../site` passes on this tree or fails on a bad line, and nothing else gates it.

## LICENCE

Text and figures [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); code [MIT](https://opensource.org/license/mit). The sequences themselves belong to the OEIS and its contributors.
