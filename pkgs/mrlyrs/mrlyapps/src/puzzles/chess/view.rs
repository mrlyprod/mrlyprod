use super::Chess;
use mrlyos::kernel::Iden;
use mrlyui::kit;

const SKINS: [&str; 2] = ["digits", "emojis"];
const SURFACES: [&str; 2] = ["grid", "canvas"];

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

pub(super) fn tree(game: &Chess, _iden: &Iden) -> Option<kit::Node> {
    let set = |key: &str| kit::set("chess", key);
    let live = !game.finished();
    let board = kit::squares(
        game.render().fact(),
        game.files(),
        game.ranks(),
        match live {
            true => Some(kit::call("chess.select")),
            false => None,
        },
    );
    let mut nodes = vec![board];
    if !live {
        nodes.push(kit::over(
            "game over",
            &game.outcome(),
            "play again",
            kit::call("chess.reset"),
        ));
    } else {
        nodes.push(kit::meter(&game.standing()));
    }
    nodes.push(kit::card(vec![
        kit::heading("look"),
        kit::segments("skin", game.skin_name(), names(&SKINS), set("skin")),
        kit::segments(
            "surface",
            game.surface_name(),
            names(&SURFACES),
            set("surface"),
        ),
        kit::range("tile", game.tile(), 5, 16, 1, set("tile")),
    ]));
    nodes.push(kit::card(vec![
        kit::heading("rules"),
        kit::toggle("obfuscate", game.obfuscated(), set("obfuscate")),
        kit::range("reskin", game.reskin(), 0, 50, 1, set("reskin")),
    ]));
    Some(kit::page(nodes))
}
