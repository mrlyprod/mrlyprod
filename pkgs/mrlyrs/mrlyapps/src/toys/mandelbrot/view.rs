use super::Mandelbrot;
use mrlyos::kernel::Iden;
use mrlyui::kit;

pub(super) fn tree(m: &Mandelbrot, _iden: &Iden) -> Option<kit::Node> {
    let set = |key: &str| kit::set("mandelbrot", key);
    Some(kit::page(vec![
        kit::board(m.render().fact()),
        kit::meter(&format!("steps {}", m.steps)),
        kit::heading("motion"),
        kit::scaled("zoom", m.set.zoom, 1000, 1050, 1, 1000, set("zoom")),
        kit::range("cycle", m.set.cycle, 30, 3000, 30, set("cycle")),
        kit::scaled("drift", m.set.drift, 0, 4000, 100, 1000, set("drift")),
        kit::scaled("spin", m.set.spin, 0, 50, 1, 1000, set("spin")),
        kit::heading("paint"),
        kit::scaled("band", m.set.band, 2000, 64000, 1000, 1000, set("band")),
        kit::range("fade", m.set.fade, 0, 240, 8, set("fade")),
        kit::range("depth", m.set.depth, 16, 600, 8, set("depth")),
        kit::heading("primary"),
        kit::color(m.set.primary, set("primary")),
        kit::heading("accent"),
        kit::color(m.set.accent, set("accent")),
    ]))
}
