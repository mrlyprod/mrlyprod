use super::Menu;
use mrlyos::kernel::Iden;
use mrlyui::kit;

pub(super) fn tree(menu: &Menu, _iden: &Iden) -> Option<kit::Node> {
    let found = menu.found();
    let mut nodes = Vec::new();
    let enter = found
        .first()
        .and_then(|m| m["route"].as_str())
        .map(kit::open);
    nodes.push(kit::search(
        &menu.query,
        "search",
        kit::call("menu.search"),
        "q",
        enter,
    ));
    if menu.query.trim().is_empty() {
        let mut seen: Vec<&str> = Vec::new();
        for m in &menu.apps {
            if let Some(c) = m["category"].as_str() {
                if !seen.contains(&c) {
                    seen.push(c);
                }
            }
        }
        if !seen.is_empty() {
            nodes.push(kit::pills(
                seen.iter()
                    .map(|c| kit::button(c, kit::arg("menu.search", "q", c)))
                    .collect(),
            ));
        }
    }
    let rows: Vec<kit::Node> = found
        .iter()
        .map(|m| {
            let route = m["route"].as_str().unwrap_or("");
            let title = m["title"].as_str().unwrap_or(route);
            if menu.mode == "list" {
                kit::item(
                    title,
                    m["category"].as_str().unwrap_or(""),
                    Some(kit::open(route)),
                )
                .badge(m["emoji"].as_str().unwrap_or(""))
            } else {
                kit::tile(m["emoji"].as_str().unwrap_or(""), title, kit::open(route))
            }
        })
        .collect();
    nodes.push(if menu.mode == "list" {
        kit::list(rows)
    } else {
        kit::launch(rows)
    });
    Some(kit::page(nodes))
}
