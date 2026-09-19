# MrlyMath

The mathematics of the mrly tree: a parity rule on the corners of a cube, substituted into itself by the Kronecker product, and everything that falls out of those two moves. One subject, one tree, and this page is its law; the index of the notes is built from their front matter at [mrly.net/research](https://mrly.net/research/).

## THE LAW

- Type folders, not topic folders: a topic is one slug that repeats across `notes/`, `claims/`, `papers/`, `figures/` and `lab/`, never a box of its own.
- `notes/<slug>.md` is a note: timeless, present tense, no dates and no story, opening on a front matter block of `title`, `lead`, `figure` and `slug`; the index of the notes is built from that block and nothing is registered anywhere.
- `claims/<slug>.md` is a claims file: one dated line per claim, `- YYYY-MM-DD [Tag] the claim. Witness: its generator`, append only; the built page at `/research/discoveries/` reads every file and filters by tag, topic and date.
- Every claim carries exactly one tag: **Proved**, **Verified**, **Conjecture**, **Refuted**; a claim is never deleted because someone else published it first, a dated cross-reference line is added, and only a counterexample moves a claim.
- Every printed number names its generator: a crate function in `../crates`, a study in `lab/rs/` or `lab/py/`, a sequence row in `sequences.md`, or an [OEIS](https://oeis.org) entry; a number with no generator keeps its claim at Conjecture.
- A theorem from the literature is cited at its source, resolved to one URL in `REFS.md`; a claim of this tree never rests on a citation alone.
- `sequences.md` is one generated ledger, written by `cargo run -p mrlylab --bin ledger`: a sequence is a row only when a claim cites it or the OEIS holds it, its id the canonical JSON name, its anchor the id's hash; never a page per sequence, and the enumerated millions are data, not pages.
- `papers/<slug>.md` is a paper: markdown, a cover in front matter (`title`, `lead`, `date`, `figure`), print CSS for the paged document, no LaTeX; the shelf of first editions is deprecated and never edited.
- `figures/` is one crate: a figure is `src/bin/<name>.rs`, cargo discovers it, `scripts/figures.sh` presses it to `../files/figures/<name>-dark.png` and `<name>-light.png`, and a reel to `.gif`; a figure is drawn by code, never by hand, and a page names a figure by its name alone.
- `lab/rs/<study>/` is a Rust crate in the root workspace and `lab/py/<study>/` a Python study run with `uv`; a study README says what it computes, how to run it and which claim lines it witnesses, and a study is deleted the day a crate function or a demo computes its numbers.
- The demos in `../site/demos` run the same crates through wasm; a note links the demo that shows it, and every demo is linked from some page.
- Math is written in backticks so it renders everywhere; lines never wrap; no comment in code beyond a section delimiter, no em-dash anywhere.
- Author line MrlyProd on every route: Carlo directs, Claude writes and computes, and `/method/` says so.
- One check: `bun run check` in `../site` passes on this tree or fails on a bad line, and nothing else gates it; the push saves `/research/discoveries/` to the Wayback Machine as the witness of priority.

## LICENCE

Text and figures [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); code [MIT](https://opensource.org/license/mit). The sequences themselves belong to the OEIS and its contributors.
