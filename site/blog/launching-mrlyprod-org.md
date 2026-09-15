---
title: Launching the site
date: 2026-09-02
lead: One address for the demos, the papers and the research notes.
---

This site collects work that was scattered across a repository into one place you can read in a browser. Nothing here is new mathematics; it is the same mathematics, published where it can be found.

MrlyMath is the study of designs on the corners of a cube grown by the Kronecker product. A code says which corners are filled, the product grows the design level by level, and the counts, slices, spectra and integer sequences that follow are what the work is about.

There are three things to read. The demos are 28 browser pages that draw a design and the numbers around it; every number in them comes out of the Rust crates compiled to wasm, so the browser only paints. The papers are 12 write-ups, each one a folder with a PDF, its LaTeX source, a short README and the scripts that check every computational claim. The research pages are the working notes behind both: one page per idea, with the figures the studies produced.

Everything is open source at [github.com/mrlyprod/mrlyprod](https://github.com/mrlyprod/mrlyprod) under MIT, and the papers live in a second public repository with their text and figures under CC BY 4.0. A paper's claims are meant to be rerun rather than trusted: `python3 scripts/verify.py` in a paper's folder reproduces its computational facts, and `cargo test --workspace` runs the crates underneath.

What comes next is more of the same. More papers are in draft, and the demos grow whenever a crate learns something worth drawing. A shop is planned for the physical side of the mark - the objects the designs turn into once they leave the screen. No dates are promised for either; when something lands it will be posted here.
