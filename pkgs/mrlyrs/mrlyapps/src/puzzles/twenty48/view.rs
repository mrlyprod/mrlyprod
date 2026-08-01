use super::Twenty48;
use mrlycore::json;
use mrlycore::ui;
use mrlyos::kernel::Iden;

pub(super) fn tree(game: &Twenty48, _iden: &Iden) -> Option<ui::Node> {
    let set = |key: &str| ui::Call::new("twenty48.set", json!({ "key": key }));
    let slide = |dir: &str| ui::Call::new("twenty48.slide", json!({ "dir": dir }));
    let mut nodes = vec![ui::Node::image(game.render().fact())];
    if game.over {
        nodes.push(ui::Node::group(vec![
            ui::Node::text("game over", ui::Role::Title),
            ui::Node::text(&format!("score {}", game.score), ui::Role::Note),
            ui::Node::button("play again", ui::Call::new("twenty48.reset", json!({}))),
        ]));
    } else {
        nodes.push(ui::Node::grid(
            4,
            vec![
                ui::Node::button("<", slide("left")),
                ui::Node::button("^", slide("up")),
                ui::Node::button("v", slide("down")),
                ui::Node::button(">", slide("right")),
            ],
        ));
        nodes.push(ui::Node::text(
            &format!("score {} - steps {}", game.score, game.steps),
            ui::Role::Note,
        ));
    }
    nodes.push(ui::Node::text("rules", ui::Role::Label));
    nodes.push(ui::Node::range(
        "grid",
        game.set.grid,
        2,
        8,
        1,
        set("grid"),
        "value",
    ));
    nodes.push(ui::Node::text("look", ui::Role::Label));
    nodes.push(ui::Node::choice(
        "skin",
        &game.set.skin,
        vec!["tiles".to_string(), "digits".to_string()],
        ui::Pick::Segments,
        set("skin"),
        "value",
    ));
    nodes.push(ui::Node::choice(
        "design",
        &game.set.design,
        super::DESIGNS.iter().map(|d| d.to_string()).collect(),
        ui::Pick::Cycle,
        set("design"),
        "value",
    ));
    Some(ui::Node::column(nodes))
}
