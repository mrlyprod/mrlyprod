use super::Menu;
use mrlycore::json;
use mrlycore::ui;
use mrlyos::kernel::Iden;

pub(super) fn tree(menu: &Menu, _iden: &Iden) -> Option<ui::Node> {
    let found = menu.found();
    let mut nodes = Vec::new();
    let mut field = ui::Node::field(
        &menu.query,
        "search",
        ui::Call::new("menu.search", json!({})),
        "q",
    )
    .live();
    if let Some(first) = found.first().and_then(|m| m["route"].as_str()) {
        field = field.enter(ui::Call::new("nav.open", json!({ "app": first })));
    }
    nodes.push(field);
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
            nodes.push(ui::Node::wrap(
                seen.iter()
                    .map(|c| ui::Node::button(c, ui::Call::new("menu.search", json!({ "q": c }))))
                    .collect(),
            ));
        }
    }
    let rows: Vec<ui::Node> = found
        .iter()
        .map(|m| {
            let route = m["route"].as_str().unwrap_or("");
            let title = m["title"].as_str().unwrap_or(route);
            let open = ui::Call::new("nav.open", json!({ "app": route }));
            if menu.mode == "list" {
                ui::Node::label(title, m["category"].as_str().unwrap_or(""), Some(open))
            } else {
                ui::Node::button(title, open)
            }
        })
        .collect();
    nodes.push(if menu.mode == "list" {
        ui::Node::column(rows)
    } else {
        ui::Node::grid(3, rows)
    });
    Some(ui::Node::column(nodes))
}
