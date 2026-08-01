use super::Snake;
use mrlycore::json;
use mrlycore::ui;
use mrlyos::kernel::Iden;

pub(super) fn tree(snake: &Snake, _iden: &Iden) -> Option<ui::Node> {
    let set = |key: &str| ui::Call::new("snake.set", json!({ "key": key }));
    let turn = |dir: &str| ui::Call::new("snake.turn", json!({ "dir": dir }));
    let mut nodes = vec![ui::Node::image(snake.render().fact())];
    if snake.over {
        nodes.push(ui::Node::group(vec![
            ui::Node::text("game over", ui::Role::Title),
            ui::Node::text(&format!("score {}", snake.score), ui::Role::Note),
            ui::Node::button("play again", ui::Call::new("snake.reset", json!({}))),
        ]));
    } else {
        nodes.push(ui::Node::grid(
            4,
            vec![
                ui::Node::button("<", turn("left")),
                ui::Node::button("^", turn("up")),
                ui::Node::button("v", turn("down")),
                ui::Node::button(">", turn("right")),
            ],
        ));
        nodes.push(ui::Node::text(
            &format!("score {} - steps {}", snake.score, snake.steps),
            ui::Role::Note,
        ));
    }
    nodes.push(ui::Node::text("rules", ui::Role::Label));
    nodes.push(ui::Node::range(
        "grid",
        snake.set.grid,
        5,
        64,
        1,
        set("grid"),
        "value",
    ));
    nodes.push(ui::Node::range(
        "apples",
        snake.set.apples,
        1,
        16,
        1,
        set("apples"),
        "value",
    ));
    nodes.push(ui::Node::toggle(
        "wrap",
        snake.set.wrap,
        set("wrap"),
        "value",
    ));
    nodes.push(ui::Node::toggle(
        "self collision",
        snake.set.self_collision,
        set("self_collision"),
        "value",
    ));
    nodes.push(ui::Node::text("speed", ui::Role::Label));
    nodes.push(ui::Node::range(
        "speed",
        snake.set.speed,
        1,
        8,
        1,
        set("speed"),
        "value",
    ));
    Some(ui::Node::column(nodes))
}
