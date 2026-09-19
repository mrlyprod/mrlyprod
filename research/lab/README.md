# Lab

- The code that regenerates every number in this tree: one study per folder, sterile because it is public.
- `rs/<study>/` is a Rust crate in the root workspace, run with `bash scripts/cargo.sh cargo run --release -p <study>` from `mrlyprod/`; `py/<study>/` is Python, run with `uv run python research/lab/py/<study>/<file>.py` from the repo root.
- A bare `cargo check` or `cargo test` never touches a study; `-p <study>` names one, `--workspace` names all.
- Rust studies depend on the public crates by path and rent only num-bigint, num-rational and faer.
- No comments, no logs, no dates, no story, no data over 100KB; a README names what the study computes, how to run it, and the page lines it witnesses.
- A study is deleted the day its numbers have a crate function, a demo, or a shelf script.
