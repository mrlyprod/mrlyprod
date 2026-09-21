use crate::core::colors::Color;

/// The palette black, #000000.
pub const BLACK: Color = Color::rgb(0, 0, 0);
/// The palette white, #ffffff.
pub const WHITE: Color = Color::rgb(255, 255, 255);
/// The palette red, #ff3d40.
pub const RED: Color = Color::rgb(255, 61, 64);
/// The palette orange, #ff8f2c.
pub const ORANGE: Color = Color::rgb(255, 143, 44);
/// The palette yellow, #ffd100.
pub const YELLOW: Color = Color::rgb(255, 209, 0);
/// The palette green, #32cc58.
pub const GREEN: Color = Color::rgb(50, 204, 88);
/// The palette mint, #00d1bb.
pub const MINT: Color = Color::rgb(0, 209, 187);
/// The palette teal, #00cad8.
pub const TEAL: Color = Color::rgb(0, 202, 216);
/// The palette cyan, #1ec9f3.
pub const CYAN: Color = Color::rgb(30, 201, 243);
/// The palette blue, #008cff.
pub const BLUE: Color = Color::rgb(0, 140, 255);
/// The palette indigo, #6768fa.
pub const INDIGO: Color = Color::rgb(103, 104, 250);
/// The palette purple, #d332e9.
pub const PURPLE: Color = Color::rgb(211, 50, 233);
/// The palette pink, #ff325a.
pub const PINK: Color = Color::rgb(255, 50, 90);
/// The palette brown, #b18462.
pub const BROWN: Color = Color::rgb(177, 132, 98);
/// The palette gray, #8e8e93.
pub const GRAY: Color = Color::rgb(142, 142, 147);

/// The fifteen names, in palette order.
#[rustfmt::skip]
pub const NAMES: [&str; 15] = ["black", "white", "red", "orange", "yellow", "green", "mint", "teal", "cyan", "blue", "indigo", "purple", "pink", "brown", "gray"];

/// The fifteen colors, in name order.
#[rustfmt::skip]
pub const PALETTE: [Color; 15] = [BLACK, WHITE, RED, ORANGE, YELLOW, GREEN, MINT, TEAL, CYAN, BLUE, INDIGO, PURPLE, PINK, BROWN, GRAY];
