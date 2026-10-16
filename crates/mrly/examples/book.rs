use mrly::core::json;

fn main() {
    let map: serde_json::Map<String, serde_json::Value> = mrly::font::map()
        .into_iter()
        .map(|(c, rows)| {
            let path: Vec<[usize; 2]> = mrly::font::path(c)
                .into_iter()
                .map(|(r, col)| [r, col])
                .collect();
            (c.to_string(), json!({ "rows": rows, "path": path }))
        })
        .collect();
    println!("{}", serde_json::Value::Object(map));
}
