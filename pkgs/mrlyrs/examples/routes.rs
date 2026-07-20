use serde_json::json;

fn main() {
    let routes: Vec<_> = mrly::net::registry::catalogue()
        .iter()
        .map(|app| {
            let m = app.manifest();
            json!({
                "route": m.route,
                "title": m.title,
                "emoji": m.emoji,
                "hidden": m.hidden,
            })
        })
        .collect();
    println!("{}", serde_json::to_string(&routes).unwrap());
}
