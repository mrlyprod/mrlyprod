use mrlycore::json;

fn main() {
    let book: Vec<(String, Vec<String>)> = mrlyfont::map()
        .into_iter()
        .map(|(c, rows)| (c.to_string(), rows))
        .collect();
    let map: serde_json::Map<String, serde_json::Value> =
        book.into_iter().map(|(c, rows)| (c, json!(rows))).collect();
    println!("{}", serde_json::Value::Object(map));
}
