use crate::wells::wells;
use mrlycore::data::Well;
use mrlycore::errors::Result;
use mrlycore::io::{make, write};
use mrlycore::json::clip;
use mrlycore::{json, Json};
use mrlymath::crypto::hash::quick_hexdigest;
use std::path::Path;

/// The default seed a pour runs under.
pub const SEED: u64 = 7;
/// The default row budget of every dataset.
pub const ROWS: usize = 128;

const SPLIT: &str = "split";
const EVAL: &str = "eval";

/// Pours every well into the directory with the default row budget, returning the manifest.
pub fn pour(dir: &Path, seed: u64) -> Result<Json> {
    pour_sized(dir, seed, ROWS)
}

/// Pours every well into the directory, at most rows rows each, returning the manifest it wrote.
pub fn pour_sized(dir: &Path, seed: u64, rows: usize) -> Result<Json> {
    make(dir)?;
    let mut datasets = Vec::new();
    for well in wells() {
        datasets.push(pour_well(well.as_ref(), dir, seed, rows)?);
    }
    let manifest = json!({
        "v": 1,
        "version": env!("CARGO_PKG_VERSION"),
        "seed": clip(seed),
        "datasets": datasets,
    });
    write(
        &dir.join("index.json"),
        (manifest.pretty() + "\n").as_bytes(),
    )?;
    Ok(manifest)
}

fn pour_well(well: &dyn Well, dir: &Path, seed: u64, rows: usize) -> Result<Json> {
    let home = dir.join(well.name());
    make(&home)?;
    let mut train = String::new();
    let mut eval = String::new();
    let mut schema: Vec<(String, &'static str)> = Vec::new();
    let mut count = 0;
    for mut row in well.pour(seed, rows) {
        let held = row[SPLIT].as_str() == Some(EVAL);
        if let Some(map) = row.as_object_mut() {
            map.remove(SPLIT);
        }
        gather(&mut schema, &row);
        let bucket = if held { &mut eval } else { &mut train };
        bucket.push_str(&row.to_string());
        bucket.push('\n');
        count += 1;
    }
    write(&home.join("train.jsonl"), train.as_bytes())?;
    if !eval.is_empty() {
        write(&home.join("eval.jsonl"), eval.as_bytes())?;
    }
    write(
        &home.join("README.md"),
        card(well, seed, count, &schema).as_bytes(),
    )?;
    let mut bytes = train.into_bytes();
    bytes.extend_from_slice(eval.as_bytes());
    Ok(json!({
        "name": well.name(),
        "about": well.about(),
        "rows": count,
        "bytes": bytes.len(),
        "sha": quick_hexdigest(&bytes)?,
    }))
}

fn gather(schema: &mut Vec<(String, &'static str)>, row: &Json) {
    let Some(map) = row.as_object() else {
        return;
    };
    for (key, value) in map.iter() {
        if !schema.iter().any(|(name, _)| name == key) {
            schema.push((key.clone(), kind(value)));
        }
    }
}

fn kind(value: &Json) -> &'static str {
    match value {
        Json::Null => "null",
        Json::Bool(_) => "bool",
        Json::Int(_) => "int",
        Json::Str(_) => "string",
        Json::Arr(_) => "array",
        Json::Obj(_) => "object",
    }
}

fn pretty(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn card(well: &dyn Well, seed: u64, rows: usize, schema: &[(String, &'static str)]) -> String {
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str("license: mit\n");
    out.push_str(&format!("pretty_name: {}\n", pretty(well.name())));
    out.push_str("---\n\n");
    out.push_str(&format!("# {}\n\n", well.name()));
    out.push_str(&format!(
        "{} One JSON object per line; the same seed and version rebuild identical bytes.\n\n",
        well.about()
    ));
    out.push_str("## Rows\n\n");
    for (key, kind) in schema {
        out.push_str(&format!("- `{key}` ({kind})\n"));
    }
    out.push_str("\n## Seed\n\n");
    out.push_str(&format!("- seed {seed}\n- rows {rows}\n"));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::json::parse;
    use mrlycore::state::guard;
    use std::fs;

    struct Tagged;

    impl Well for Tagged {
        fn name(&self) -> &str {
            "tagged"
        }
        fn about(&self) -> &str {
            "A test well that holds back every third row for evaluation."
        }
        fn pour(&self, _seed: u64, count: usize) -> Vec<Json> {
            (0..count)
                .map(|n| {
                    if n % 3 == 2 {
                        json!({ "n": n, "split": "eval" })
                    } else {
                        json!({ "n": n })
                    }
                })
                .collect()
        }
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("mrlydata_{name}_{}", std::process::id()))
    }

    #[test]
    fn the_press_stamps_a_verifiable_manifest() {
        let _g = guard();
        let dir = scratch("press");
        let manifest = pour_sized(&dir, 7, 2).unwrap();
        assert_eq!(manifest["v"], 1);
        assert_eq!(
            manifest["version"].as_str(),
            Some(env!("CARGO_PKG_VERSION"))
        );
        assert_eq!(manifest["seed"], 7);
        let written = fs::read_to_string(dir.join("index.json")).unwrap();
        assert_eq!(parse(&written).unwrap(), manifest);
        let datasets = manifest["datasets"].as_array().unwrap();
        let names: Vec<&str> = datasets
            .iter()
            .map(|d| d["name"].as_str().unwrap())
            .collect();
        assert!(names.contains(&"tiles"), "the math wells are missing");
        assert!(names.contains(&"glyphs"), "the font wells are missing");
        assert!(
            names.iter().any(|name| name.starts_with("trail_")),
            "the goose trails are missing"
        );
        for dataset in datasets {
            let name = dataset["name"].as_str().unwrap();
            let text = fs::read_to_string(dir.join(name).join("train.jsonl")).unwrap();
            for line in text.lines() {
                assert!(parse(line).is_ok(), "{name} wrote a broken row");
            }
            assert_eq!(
                dataset["sha"].as_str().unwrap(),
                quick_hexdigest(text.as_bytes()).unwrap(),
                "{name} drifted from its sha"
            );
            assert_eq!(dataset["bytes"].as_i64().unwrap() as usize, text.len());
            let readme = fs::read_to_string(dir.join(name).join("README.md")).unwrap();
            assert!(readme.starts_with("---\nlicense: mit\n"));
            assert!(readme.contains("## Rows"));
        }
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn tagged_rows_route_to_eval() {
        let dir = scratch("eval");
        make(&dir).unwrap();
        pour_well(&Tagged, &dir, 7, 6).unwrap();
        let train = fs::read_to_string(dir.join("tagged").join("train.jsonl")).unwrap();
        let eval = fs::read_to_string(dir.join("tagged").join("eval.jsonl")).unwrap();
        assert_eq!(train.lines().count(), 4);
        assert_eq!(eval.lines().count(), 2);
        assert!(!train.contains("split"));
        assert!(!eval.contains("split"));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn two_pours_press_identical_bytes() {
        let _g = guard();
        let (a, b) = (scratch("presa"), scratch("presb"));
        pour_sized(&a, 7, 2).unwrap();
        pour_sized(&b, 7, 2).unwrap();
        let index_a = fs::read(a.join("index.json")).unwrap();
        let index_b = fs::read(b.join("index.json")).unwrap();
        assert_eq!(index_a, index_b);
        let tiles_a = fs::read(a.join("tiles").join("train.jsonl")).unwrap();
        let tiles_b = fs::read(b.join("tiles").join("train.jsonl")).unwrap();
        assert_eq!(tiles_a, tiles_b);
        fs::remove_dir_all(&a).ok();
        fs::remove_dir_all(&b).ok();
    }
}
