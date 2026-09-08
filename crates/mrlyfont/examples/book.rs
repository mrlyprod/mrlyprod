use mrlycore::json;

fn main() {
    let map: serde_json::Map<String, serde_json::Value> = mrlyfont::map()
        .into_iter()
        .map(|(c, rows)| {
            let path: Vec<[usize; 2]> = mrlyfont::path(c)
                .into_iter()
                .map(|(r, col)| [r, col])
                .collect();
            (c.to_string(), json!({ "rows": rows, "path": path }))
        })
        .collect();
    println!("{}", serde_json::Value::Object(map));
}
