use super::Snake;
use mrlyos::kernel::Iden;
use mrlyui::kit;

pub(super) fn tree(snake: &Snake, _iden: &Iden) -> Option<kit::Node> {
    let set = |key: &str| kit::set("snake", key);
    let mut nodes = vec![kit::board(snake.render().fact())];
    if snake.over {
        nodes.push(kit::over(
            "game over",
            &format!("score {}", snake.score),
            "play again",
            kit::call("snake.reset"),
        ));
    } else {
        nodes.push(kit::dpad(
            ["\u{2190}", "\u{2191}", "\u{2193}", "\u{2192}"],
            "snake.turn",
        ));
        nodes.push(kit::meter(&format!(
            "score {} - steps {}",
            snake.score, snake.steps
        )));
    }
    nodes.push(kit::heading("rules"));
    nodes.push(kit::range("grid", snake.set.grid, 5, 64, 1, set("grid")));
    nodes.push(kit::range(
        "apples",
        snake.set.apples,
        1,
        16,
        1,
        set("apples"),
    ));
    nodes.push(kit::toggle("wrap", snake.set.wrap, set("wrap")));
    nodes.push(kit::toggle(
        "self collision",
        snake.set.self_collision,
        set("self_collision"),
    ));
    nodes.push(kit::heading("speed"));
    nodes.push(kit::range("speed", snake.set.speed, 1, 8, 1, set("speed")));
    Some(kit::page(nodes))
}
