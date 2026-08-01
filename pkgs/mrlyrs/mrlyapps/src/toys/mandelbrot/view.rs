use super::Mandelbrot;
use mrlycore::json;
use mrlycore::ui;
use mrlyos::kernel::Iden;
use mrlyui::frame;

pub(super) fn tree(m: &Mandelbrot, _iden: &Iden) -> Option<ui::Node> {
    let set = |key: &str| ui::Call::new("mandelbrot.set", json!({ "key": key }));
    Some(ui::Node::column(vec![
        ui::Node::image(m.render().fact()),
        ui::Node::text(&format!("steps {}", m.steps), ui::Role::Note),
        ui::Node::text("motion", ui::Role::Label),
        ui::Node::range("zoom", m.set.zoom, 1000, 1050, 1, set("zoom"), "value").scaled(1000),
        ui::Node::range("cycle", m.set.cycle, 30, 3000, 30, set("cycle"), "value"),
        ui::Node::range("drift", m.set.drift, 0, 4000, 100, set("drift"), "value").scaled(1000),
        ui::Node::range("spin", m.set.spin, 0, 50, 1, set("spin"), "value").scaled(1000),
        ui::Node::text("paint", ui::Role::Label),
        ui::Node::range("band", m.set.band, 2000, 64000, 1000, set("band"), "value").scaled(1000),
        ui::Node::range("fade", m.set.fade, 0, 240, 8, set("fade"), "value"),
        ui::Node::range("depth", m.set.depth, 16, 600, 8, set("depth"), "value"),
        ui::Node::text("primary", ui::Role::Label),
        ui::Node::field(
            &frame::hex(m.set.primary),
            "#rrggbb",
            set("primary"),
            "value",
        ),
        ui::Node::text("accent", ui::Role::Label),
        ui::Node::field(&frame::hex(m.set.accent), "#rrggbb", set("accent"), "value"),
    ]))
}
