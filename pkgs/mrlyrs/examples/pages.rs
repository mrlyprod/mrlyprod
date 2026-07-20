use serde_json::json;
use std::fs;

fn main() {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "cdn/pages".to_string());
    let mut slugs: Vec<String> = fs::read_dir(&dir)
        .unwrap()
        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
        .filter_map(|name| name.strip_suffix(".md").map(str::to_string))
        .collect();
    slugs.sort();
    let pages: Vec<_> = slugs
        .iter()
        .map(|slug| {
            let md = fs::read_to_string(format!("{dir}/{slug}.md")).unwrap();
            json!({
                "slug": slug,
                "title": title(&md),
                "html": mrly::core::md::html(&md),
            })
        })
        .collect();
    println!("{}", serde_json::to_string(&pages).unwrap());
}

fn title(md: &str) -> String {
    md.lines()
        .find_map(|line| line.strip_prefix("# "))
        .unwrap_or("MrlyProd")
        .to_string()
}
