use super::{Settings, MATERIALS, MODES, RENDERS, WALLPAPERS};
use mrlyos::kernel::Iden;
use mrlyui::kit;

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

pub(super) fn tree(set: &Settings, _iden: &Iden) -> Option<kit::Node> {
    let key = |name: &str| kit::set("settings", name);
    let mut nodes = vec![kit::heading("desk")];
    nodes.push(kit::segments(
        "launchpad",
        &set.launchpad,
        names(&MODES),
        key("launchpad"),
    ));
    nodes.push(kit::toggle("darkmode", set.darkmode, key("darkmode")));
    nodes.push(kit::heading("background"));
    nodes.push(kit::swatch(&set.background, key("background")));
    nodes.push(kit::select("fill", &set.fill, super::fills(), key("fill")));
    nodes.push(kit::heading("color"));
    nodes.push(kit::swatch(&set.color, key("color")));

    nodes.push(kit::heading("sheet"));
    nodes.push(kit::range("scale", set.scale, 3, 6, 1, key("scale")));
    nodes.push(kit::range("width", set.width, 500, 1500, 100, key("width")));
    nodes.push(kit::range("pace", set.pace, 0, 400, 25, key("pace")));
    nodes.push(kit::range("detail", set.detail, 32, 160, 16, key("detail")));
    nodes.push(kit::segments(
        "render",
        &set.render,
        names(&RENDERS),
        key("render"),
    ));

    nodes.push(kit::heading("sound"));
    nodes.push(kit::toggle("sound", set.sound, key("sound")));
    nodes.push(kit::select("note", &set.note, super::notes(), key("note")));
    nodes.push(kit::select("wave", &set.wave, super::waves(), key("wave")));
    nodes.push(kit::range(
        "duration",
        set.duration,
        50,
        1000,
        50,
        key("duration"),
    ));

    nodes.push(kit::heading("pattern"));
    nodes.push(kit::segments(
        "wallpaper",
        &set.wallpaper,
        names(&WALLPAPERS),
        key("wallpaper"),
    ));
    nodes.push(kit::range("seed", set.seed, 0, 999, 1, key("seed")));

    nodes.push(kit::heading("web only"));
    nodes.push(kit::dead("font", &set.font));
    nodes.push(kit::dead("emoji", &set.emoji));
    nodes.push(kit::dead("material", &set.material));
    nodes.push(kit::dead("haptics", bool_word(set.haptics)));
    nodes.push(kit::dead("radius", &set.radius.to_string()));
    let _ = MATERIALS;
    Some(kit::page(nodes))
}

fn bool_word(on: bool) -> &'static str {
    match on {
        true => "on",
        false => "off",
    }
}
