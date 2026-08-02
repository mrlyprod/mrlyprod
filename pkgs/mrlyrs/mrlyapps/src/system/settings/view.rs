use super::{Settings, EMOJIS, FONTS, MATERIALS, MODES, RENDERS, WALLPAPERS};
use mrlyos::kernel::Iden;
use mrlyui::kit;

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

pub(super) fn tree(set: &Settings, _iden: &Iden) -> Option<kit::Node> {
    let key = |name: &str| kit::set("settings", name);
    let nodes = vec![
        kit::card(vec![
            kit::heading("desk"),
            kit::segments("launchpad", &set.launchpad, names(&MODES), key("launchpad")),
            kit::toggle("darkmode", set.darkmode, key("darkmode")),
        ]),
        kit::card(vec![
            kit::heading("background"),
            kit::swatch(&set.background, key("background")),
            kit::select("fill", &set.fill, super::fills(), key("fill")),
        ]),
        kit::card(vec![
            kit::heading("accent"),
            kit::swatch(&set.color, key("color")),
        ]),
        kit::card(vec![
            kit::heading("measure"),
            kit::range("scale", set.scale, 3, 6, 1, key("scale")),
            kit::range("radius", set.radius, 0, 4, 1, key("radius")),
            kit::range("pace", set.pace, 0, 400, 25, key("pace")),
        ]),
        kit::card(vec![
            kit::heading("render"),
            kit::segments("render", &set.render, names(&RENDERS), key("render")),
            kit::range("detail", set.detail, 32, 160, 1, key("detail")),
        ]),
        kit::card(vec![
            kit::heading("type"),
            kit::segments("font", &set.font, names(&FONTS), key("font")),
        ]),
        kit::card(vec![
            kit::heading("sound"),
            kit::toggle("sound", set.sound, key("sound")),
            kit::select("note", &set.note, super::notes(), key("note")),
            kit::segments("wave", &set.wave, super::waves(), key("wave")),
            kit::range("duration", set.duration, 50, 1000, 50, key("duration")),
        ]),
        kit::card(vec![
            kit::heading("pattern"),
            kit::segments(
                "wallpaper",
                &set.wallpaper,
                names(&WALLPAPERS),
                key("wallpaper"),
            ),
            kit::range("seed", set.seed, 0, 999, 1, key("seed")),
        ]),
        kit::card(vec![
            kit::heading("web only"),
            kit::dead("emoji", &set.emoji),
            kit::dead("material", &set.material),
            kit::dead("haptics", bool_word(set.haptics)),
            kit::dead("width", &set.width.to_string()),
            kit::dead("session", "export import reset"),
            kit::dead("install", "browser"),
        ]),
    ];
    let _ = (EMOJIS, MATERIALS);
    Some(kit::page(nodes))
}

fn bool_word(on: bool) -> &'static str {
    match on {
        true => "on",
        false => "off",
    }
}
