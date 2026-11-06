use crate::entry::{self, Entry, slug};
use crate::grid::{Grid, parse, print};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub const SIZES: [(usize, &str); 3] = [(1, "1x"), (2, "2x"), (4, "4x")];
const KINDS: [&str; 2] = ["txt", "ts"];

pub type Sized = (usize, &'static str, Grid);

pub fn save(path: PathBuf, bytes: impl AsRef<[u8]>, problems: &mut Vec<String>) {
    if let Err(error) = fs::write(&path, bytes) {
        problems.push(format!("{}: {error}", path.display()));
    }
}

fn names(dir: &Path) -> Vec<String> {
    let mut out: Vec<String> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|one| one.file_name().into_string().ok())
        .filter(|name| !name.starts_with('.'))
        .collect();
    out.sort();
    out
}

fn known(name: &str) -> bool {
    name == "entry.toml"
        || name.split_once('.').is_some_and(|(size, kind)| {
            SIZES.iter().any(|(_, label)| *label == size) && KINDS.contains(&kind)
        })
}

fn about(dir: &Path, name: &str, problems: &mut Vec<String>) -> Option<Entry> {
    let text = match fs::read_to_string(dir.join("entry.toml")) {
        Ok(text) => text,
        Err(error) => {
            problems.push(format!("entry.toml: {error}"));
            return None;
        }
    };
    match entry::read(&text) {
        Ok(entry) => {
            if slug(&entry.title) != name {
                problems.push(format!(
                    "the folder should be named {}.",
                    slug(&entry.title)
                ));
            }
            Some(entry)
        }
        Err(error) => {
            problems.push(format!("entry.toml: {error}"));
            None
        }
    }
}

fn grids(dir: &Path, problems: &mut Vec<String>) -> Vec<Sized> {
    let mut out = Vec::new();
    for (factor, label) in SIZES {
        let Ok(text) = fs::read_to_string(dir.join(format!("{label}.txt"))) else {
            problems.push(format!("{label}.txt is missing."));
            continue;
        };
        let grid = match parse(&text) {
            Ok(grid) => grid,
            Err(error) => {
                problems.push(format!("{label}.txt: {error}"));
                continue;
            }
        };
        if print(&grid) != text {
            problems.push(format!(
                "{label}.txt is not in its plain form: check line endings, case, blank lines and the final newline."
            ));
        }
        let unused = grid.unused();
        if !unused.is_empty() {
            let keys: String = unused.into_iter().collect();
            problems.push(format!("{label}.txt lists colors it never uses: {keys}."));
        }
        out.push((factor, label, grid));
    }
    out
}

fn one(
    dir: &Path,
    out: &Path,
    name: &str,
    build: &impl Fn(&Path, Option<&Entry>, &[Sized], &mut Vec<String>) -> String,
) -> (String, Vec<String>) {
    let mut problems = Vec::new();
    let _ = fs::remove_dir_all(out);
    if let Err(error) = fs::create_dir_all(out) {
        return (String::new(), vec![format!("{}: {error}", out.display())]);
    }
    for stray in names(dir).into_iter().filter(|name| !known(name)) {
        problems.push(format!("{stray} does not belong in an entry."));
    }
    let entry = about(dir, name, &mut problems);
    let found = grids(dir, &mut problems);
    for (_, label, _) in &found {
        let script = dir.join(format!("{label}.ts"));
        if script.exists()
            && let Err(error) = fs::copy(&script, out.join(format!("{label}.ts")))
        {
            problems.push(format!("{label}.ts: {error}"));
        }
    }
    let summary = build(out, entry.as_ref(), &found, &mut problems);
    (summary, problems)
}

pub fn run(
    collection: &str,
    build: impl Fn(&Path, Option<&Entry>, &[Sized], &mut Vec<String>) -> String,
) -> ExitCode {
    let Some(repo) = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(3) else {
        eprintln!("the crate lies outside its repo.");
        return ExitCode::FAILURE;
    };
    let here = repo.join("collections").join(collection);
    let data_dir = repo.join("data").join("collections").join(collection);
    let only: Vec<String> = std::env::args().skip(1).collect();
    let all: Vec<String> = names(&here)
        .into_iter()
        .filter(|name| here.join(name).is_dir())
        .collect();
    let mut failed = false;
    for stale in names(&data_dir)
        .iter()
        .filter(|name| only.is_empty() && !all.contains(name))
    {
        let _ = fs::remove_dir_all(data_dir.join(stale));
    }
    for name in only.iter().filter(|name| !all.contains(name)) {
        println!("FAIL {name}\n     no such entry.");
        failed = true;
    }
    for name in all
        .iter()
        .filter(|name| only.is_empty() || only.contains(name))
    {
        let (summary, problems) = one(&here.join(name), &data_dir.join(name), name, &build);
        let mark = if problems.is_empty() { "ok  " } else { "FAIL" };
        println!("{mark} {name:<28} {summary}");
        for problem in &problems {
            println!("     {problem}");
        }
        failed |= !problems.is_empty();
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
