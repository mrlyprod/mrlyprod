mod collection;
mod source;

use collection::Collection;
use mrly_error::{Error, Result, value_error};
use mrly_json::{Map, json};
use mrly_rand::Rng;
use source::Source;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BATCH: usize = 20_000;
const REDRAWS: usize = 64;

const USAGE: &str =
    "mrly-press                       the collections and what the press makes of them
mrly-press NAME                  every size its collection.toml lists
mrly-press NAME SIZE...          sizes such as 1k, 2k, 4k or 200
mrly-press NAME full [FROM TO]   every entry of a source that enumerates, readings only
--threads N                      threads to press with (default: every core)";

fn io(error: std::io::Error) -> Error {
    Error::Value(error.to_string())
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("the press lives three folders under the repo")
        .to_path_buf()
}

fn out(root: &Path) -> PathBuf {
    root.join("data").join("packages/rs/press")
}

fn mix(seed: u64, index: usize, attempt: usize) -> u64 {
    let mut z = seed
        ^ (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (attempt as u64).wrapping_mul(0xd1b5_4a32_d192_ed03);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn size(label: &str) -> Result<usize> {
    let (digits, scale) = match label.strip_suffix('k') {
        Some(digits) => (digits, 1000),
        None => (label, 1),
    };
    match digits.parse::<usize>() {
        Ok(n) if n > 0 => Ok(n * scale),
        _ => value_error(format!("size {label:?} wants a count such as 200 or 1k.")),
    }
}

fn par<T: Send>(
    range: Range<usize>,
    threads: usize,
    work: impl Fn(usize) -> Result<T> + Sync,
) -> Result<Vec<T>> {
    let next = AtomicUsize::new(range.start);
    let mut done: Vec<(usize, Result<T>)> = std::thread::scope(|scope| {
        let hands: Vec<_> = (0..threads)
            .map(|_| {
                scope.spawn(|| {
                    let mut own = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        if i >= range.end {
                            return own;
                        }
                        own.push((i, work(i)));
                    }
                })
            })
            .collect();
        hands
            .into_iter()
            .flat_map(|hand| hand.join().expect("a press thread panicked"))
            .collect()
    });
    done.sort_by_key(|(i, _)| *i);
    done.into_iter().map(|(_, made)| made).collect()
}

fn fresh(dir: &Path) -> Result<()> {
    if dir.exists() {
        fs::remove_dir_all(dir).map_err(io)?;
    }
    fs::create_dir_all(dir).map_err(io)
}

fn manifest(dir: &Path, collection: &Collection, name: &str, count: usize) -> Result<()> {
    let value = json!({
        "name": name,
        "collection": collection.name,
        "title": collection.text("title"),
        "description": collection.text("description"),
        "version": VERSION,
        "seed": collection.number("seed"),
        "count": count,
    });
    fs::write(dir.join("manifest.json"), mrly_json::pretty(&value)? + "\n").map_err(io)
}

fn line(index: usize, key: &str, meta: Map, files: Vec<String>) -> Result<String> {
    let mut row = Map::new();
    row.insert("index".into(), json!(index));
    row.insert("key".into(), json!(key));
    row.extend(meta);
    if !files.is_empty() {
        row.insert("files".into(), json!(files));
    }
    Ok(mrly_json::write(&row)? + "\n")
}

fn keys(source: &dyn Source, seed: u64, count: usize, threads: usize) -> Result<Vec<String>> {
    let drawn = par(0..count, threads, |i| {
        source.draw(&mut Rng::new(mix(seed, i, 0)))
    })?;
    let mut seen = HashSet::new();
    let mut out = Vec::with_capacity(count);
    for (i, mut key) in drawn.into_iter().enumerate() {
        let mut attempt = 0;
        while seen.contains(&key) {
            attempt += 1;
            if attempt > REDRAWS {
                return value_error(format!("entry {i} drew {REDRAWS} keys already taken."));
            }
            key = source.draw(&mut Rng::new(mix(seed, i, attempt)))?;
        }
        seen.insert(key.clone());
        out.push(key);
    }
    Ok(out)
}

fn press(
    root: &Path,
    collection: &Collection,
    source: &dyn Source,
    label: &str,
    threads: usize,
) -> Result<()> {
    let start = Instant::now();
    let count = size(label)?;
    let seed = collection.number("seed").unwrap_or(0);
    let name = format!("mrly-{}-{label}", collection.name);
    let dir = out(root).join(&collection.name).join(&name);
    fresh(&dir.join("files"))?;
    let keys = keys(source, seed, count, threads)?;
    let lines = par(0..count, threads, |i| {
        let pressed = source.press(&keys[i], true)?;
        let mut files = Vec::new();
        for (kind, bytes) in pressed.files {
            let file = format!("files/{i:05}.{kind}");
            fs::write(dir.join(&file), bytes).map_err(io)?;
            files.push(file);
        }
        line(i, &keys[i], pressed.meta, files)
    })?;
    fs::write(dir.join("entries.jsonl"), lines.concat()).map_err(io)?;
    manifest(&dir, collection, &name, count)?;
    println!(
        "{name}: {count} entries in {:.1}s, {}",
        start.elapsed().as_secs_f64(),
        dir.display()
    );
    Ok(())
}

fn full(
    root: &Path,
    collection: &Collection,
    source: &dyn Source,
    span: Option<(usize, usize)>,
    threads: usize,
) -> Result<()> {
    let start = Instant::now();
    let Some(total) = source.count() else {
        return value_error(format!(
            "{} only draws; it has no full set.",
            collection.name
        ));
    };
    let (from, to) = span.unwrap_or((0, total));
    let to = to.min(total);
    if from >= to {
        return value_error(format!(
            "the span {from}..{to} is empty; the set holds {total}."
        ));
    }
    let name = format!("mrly-{}-full", collection.name);
    let dir = out(root).join(&collection.name).join(&name);
    fs::create_dir_all(&dir).map_err(io)?;
    manifest(&dir, collection, &name, total)?;
    let path = dir.join(format!("entries-{from}-{to}.jsonl"));
    let mut file = fs::File::create(&path).map_err(io)?;
    for at in (from..to).step_by(BATCH) {
        let lines = par(at..(at + BATCH).min(to), threads, |i| {
            let key = source.key(i)?;
            line(i, &key, source.press(&key, false)?.meta, Vec::new())
        })?;
        file.write_all(lines.concat().as_bytes()).map_err(io)?;
        println!(
            "{name}: {} of {} in {:.0}s",
            (at + BATCH).min(to) - from,
            to - from,
            start.elapsed().as_secs_f64()
        );
    }
    println!("{name}: {}", path.display());
    Ok(())
}

fn list(root: &Path) -> Result<()> {
    for name in collection::names(root)? {
        let collection = Collection::read(root, &name)?;
        let sizes = collection.list("sizes").join(" ");
        let made = if source::known(&name) {
            "pressed"
        } else {
            "no source yet"
        };
        println!(
            "{name:<10} {:<11} {sizes:<10} {made}",
            collection.text("kind").unwrap_or("")
        );
    }
    Ok(())
}

fn run(words: Vec<String>) -> Result<()> {
    let mut threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let mut rest = Vec::new();
    let mut words = words.into_iter();
    while let Some(word) = words.next() {
        if word == "--threads" {
            threads = match words.next().and_then(|n| n.parse().ok()) {
                Some(n) if n > 0 => n,
                _ => return value_error("--threads wants a count above zero."),
            };
        } else if word == "--help" || word == "-h" {
            println!("{USAGE}");
            return Ok(());
        } else {
            rest.push(word);
        }
    }
    let root = root();
    let Some((name, sizes)) = rest.split_first() else {
        return list(&root);
    };
    let collection = Collection::read(&root, name)?;
    if collection.text("version") != Some(VERSION) {
        return value_error(format!(
            "{name} is pinned to version {:?}; the press is {VERSION}.",
            collection.text("version").unwrap_or("none")
        ));
    }
    let source = source::source(&collection)?;
    if sizes.first().map(String::as_str) == Some("full") {
        let span = match &sizes[1..] {
            [] => None,
            [from, to] => match (from.parse(), to.parse()) {
                (Ok(from), Ok(to)) => Some((from, to)),
                _ => return value_error("full wants FROM and TO as numbers."),
            },
            _ => return value_error("full takes no span or FROM TO."),
        };
        return full(&root, &collection, source.as_ref(), span, threads);
    }
    let sizes = if sizes.is_empty() {
        collection.list("sizes")
    } else {
        sizes.to_vec()
    };
    for label in &sizes {
        press(&root, &collection, source.as_ref(), label, threads)?;
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
