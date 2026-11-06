use mrly_color::Color;

pub(crate) fn hsl(hue: usize, saturation: usize, lightness: usize) -> Color {
    let (h, s, l) = (
        hue as f64 % 360.0,
        saturation as f64 / 100.0,
        lightness as f64 / 100.0,
    );
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h / 60.0) as usize {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let byte = |v: f64| ((v + m) * 255.0).round() as u8;
    Color::rgb(byte(r), byte(g), byte(b))
}
