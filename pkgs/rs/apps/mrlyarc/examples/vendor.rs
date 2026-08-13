use mrlycore::Json;
use std::fs;
use std::path::Path;

const SIDE: usize = 30;

fn fail(note: &str) -> ! {
    eprintln!("! {note}");
    std::process::exit(1);
}

fn grid(value: &Json, out: &mut Vec<u8>) {
    let Some(rows) = value.as_array() else {
        fail("a grid is not an array");
    };
    let h = rows.len();
    let w = rows.first().and_then(|r| r.as_array()).map_or(0, Vec::len);
    if h == 0 || h > SIDE || w == 0 || w > SIDE {
        fail("a grid leaves 1..30");
    }
    out.push(w as u8);
    out.push(h as u8);
    let mut cells = Vec::with_capacity(w * h);
    for row in rows {
        let Some(row) = row.as_array() else {
            fail("a row is not an array");
        };
        if row.len() != w {
            fail("a grid is ragged");
        }
        for cell in row {
            match cell.as_u64() {
                Some(v) if v <= 9 => cells.push(v as u8),
                _ => fail("a cell leaves 0..9"),
            }
        }
    }
    for pair in cells.chunks(2) {
        out.push(pair[0] << 4 | pair.get(1).copied().unwrap_or(0));
    }
}

fn task(id: &str, value: &Json, out: &mut Vec<u8>) {
    if id.len() != 8 || !id.bytes().all(|b| b.is_ascii_hexdigit()) {
        fail("a task id is not 8 hex chars");
    }
    out.extend_from_slice(id.as_bytes());
    for part in ["train", "test"] {
        let Some(pairs) = value[part].as_array() else {
            fail("a task misses train or test");
        };
        if pairs.is_empty() || pairs.len() > 255 {
            fail("a task has no pairs");
        }
        out.push(pairs.len() as u8);
    }
    for part in ["train", "test"] {
        for pair in value[part].as_array().unwrap() {
            grid(&pair["input"], out);
            grid(&pair["output"], out);
        }
    }
}

fn split(clone: &Path, name: &str, out: &mut Vec<u8>) -> usize {
    let dir = clone.join("data").join(name);
    let mut paths: Vec<_> = match fs::read_dir(&dir) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "json"))
            .collect(),
        Err(_) => fail(&format!("cannot read {}", dir.display())),
    };
    paths.sort();
    for path in &paths {
        let Ok(text) = fs::read_to_string(path) else {
            fail("cannot read a task file");
        };
        let Ok(value) = mrlycore::json::parse(&text) else {
            fail("a task file is not json");
        };
        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        task(id, &value, out);
    }
    paths.len()
}

fn pack(clone: &Path, name: &str) {
    let mut raw = Vec::new();
    raw.extend_from_slice(b"ARCP");
    raw.push(1);
    raw.extend_from_slice(&[0; 4]);
    let train = split(clone, "training", &mut raw);
    let eval = split(clone, "evaluation", &mut raw);
    raw[5..7].copy_from_slice(&(train as u16).to_le_bytes());
    raw[7..9].copy_from_slice(&(eval as u16).to_le_bytes());
    let packed = mrlycore::deflate(&raw);
    let home = format!("{}/corpus", env!("CARGO_MANIFEST_DIR"));
    fs::create_dir_all(&home).unwrap();
    let path = format!("{home}/{name}.bin");
    fs::write(&path, &packed).unwrap();
    println!(
        "{path}: {train} train + {eval} eval, {} raw -> {} packed",
        raw.len(),
        packed.len()
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, one, two] = args.as_slice() else {
        fail("usage: vendor <clone-one> <clone-two>");
    };
    pack(Path::new(one), "one");
    pack(Path::new(two), "two");
}
