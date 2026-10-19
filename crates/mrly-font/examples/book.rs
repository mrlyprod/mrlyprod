//! Prints the font book: every glyph's rows and pen path as one JSON object, the table the site draws its letters from.

use serde_json::{Map, Value, json};

fn main() {
    let map: Map<String, Value> = mrly_font::map()
        .into_iter()
        .map(|(c, rows)| {
            let path: Vec<[usize; 2]> = mrly_font::path(c)
                .into_iter()
                .map(|(r, col)| [r, col])
                .collect();
            (c.to_string(), json!({ "rows": rows, "path": path }))
        })
        .collect();
    println!("{}", Value::Object(map));
}
