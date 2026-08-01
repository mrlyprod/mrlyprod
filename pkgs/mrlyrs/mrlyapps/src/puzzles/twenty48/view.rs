use super::Twenty48;
use mrlyos::kernel::Iden;
use mrlyui::kit;

pub(super) fn tree(game: &Twenty48, _iden: &Iden) -> Option<kit::Node> {
    let set = |key: &str| kit::set("twenty48", key);
    let mut nodes = vec![kit::board(game.render().fact())];
    if game.over {
        nodes.push(kit::over(
            "game over",
            &format!("score {}", game.score),
            "play again",
            kit::call("twenty48.reset"),
        ));
    } else {
        nodes.push(kit::dpad(["<", "^", "v", ">"], "twenty48.slide"));
        nodes.push(kit::meter(&format!(
            "score {} - steps {}",
            game.score, game.steps
        )));
    }
    nodes.push(kit::heading("rules"));
    nodes.push(kit::range("grid", game.set.grid, 2, 8, 1, set("grid")));
    nodes.push(kit::heading("look"));
    nodes.push(kit::segments(
        "skin",
        &game.set.skin,
        vec!["tiles".to_string(), "digits".to_string()],
        set("skin"),
    ));
    nodes.push(kit::cycle(
        "design",
        &game.set.design,
        super::DESIGNS.iter().map(|d| d.to_string()).collect(),
        set("design"),
    ));
    Some(kit::page(nodes))
}
