# Lab

- The code that regenerates every number in this tree: one study per folder, sterile because it is public.
- A study is a README.md plus code: Python run with `uv run python research/lab/<study>/<file>.py` from the repo root, or a Rust crate in this workspace run with `cargo run --release --manifest-path research/lab/Cargo.toml -p <study>`.
- Rust studies depend on the public crates by path and rent only num-bigint, num-rational and faer.
- No comments, no logs, no dates, no story, no data over 100KB; a README names what the study computes, how to run it, and the page lines it witnesses.
- A study is deleted the day its numbers have a crate function, a demo, or a shelf script.
