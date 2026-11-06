use crate::Color;

pub const BLACK: Color = Color::rgb(0, 0, 0);
pub const WHITE: Color = Color::rgb(255, 255, 255);
pub const RED: Color = Color::rgb(255, 61, 64);
pub const ORANGE: Color = Color::rgb(255, 143, 44);
pub const YELLOW: Color = Color::rgb(255, 209, 0);
pub const GREEN: Color = Color::rgb(50, 204, 88);
pub const MINT: Color = Color::rgb(0, 209, 187);
pub const TEAL: Color = Color::rgb(0, 202, 216);
pub const CYAN: Color = Color::rgb(30, 201, 243);
pub const BLUE: Color = Color::rgb(0, 140, 255);
pub const INDIGO: Color = Color::rgb(103, 104, 250);
pub const PURPLE: Color = Color::rgb(211, 50, 233);
pub const PINK: Color = Color::rgb(255, 50, 90);
pub const BROWN: Color = Color::rgb(177, 132, 98);
pub const GRAY: Color = Color::rgb(142, 142, 147);

#[rustfmt::skip]
pub const NAMES: [&str; 15] = ["black", "white", "red", "orange", "yellow", "green", "mint", "teal", "cyan", "blue", "indigo", "purple", "pink", "brown", "gray"];

#[rustfmt::skip]
pub const PALETTE: [Color; 15] = [BLACK, WHITE, RED, ORANGE, YELLOW, GREEN, MINT, TEAL, CYAN, BLUE, INDIGO, PURPLE, PINK, BROWN, GRAY];
