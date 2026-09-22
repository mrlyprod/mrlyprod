#[path = "../tests/common/rows.rs"]
mod rows;

use serde_json::Value;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    std::fs::create_dir_all(&root).unwrap();
    let mut total = 0;
    for module in rows::MODULES {
        let table = rows::rows(module);
        total += table.len();
        let mut text = serde_json::to_string_pretty(&Value::Array(table.clone())).unwrap();
        text.push('\n');
        std::fs::write(root.join(format!("{module}.json")), text).unwrap();
        println!("{module}.json {} rows", table.len());
    }
    println!("{total} rows");
}
