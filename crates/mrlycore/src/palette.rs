use crate::colors::Color;

// PALETTE

/// The palette black, #000000.
pub const BLACK: Color = Color::rgb(0, 0, 0);
/// The palette white, #ffffff.
pub const WHITE: Color = Color::rgb(255, 255, 255);
/// The palette red, #ff3d40.
pub const RED: Color = Color::rgb(255, 61, 64);
/// The palette red-light, #ff9d95.
pub const RED_LIGHT: Color = Color::rgb(255, 157, 149);
/// The palette red-dark, #a80016.
pub const RED_DARK: Color = Color::rgb(168, 0, 22);
/// The palette orange, #ff8f2c.
pub const ORANGE: Color = Color::rgb(255, 143, 44);
/// The palette orange-light, #ffc093.
pub const ORANGE_LIGHT: Color = Color::rgb(255, 192, 147);
/// The palette orange-dark, #a25400.
pub const ORANGE_DARK: Color = Color::rgb(162, 84, 0);
/// The palette yellow, #ffd100.
pub const YELLOW: Color = Color::rgb(255, 209, 0);
/// The palette yellow-light, #ffe591.
pub const YELLOW_LIGHT: Color = Color::rgb(255, 229, 145);
/// The palette yellow-dark, #9e8100.
pub const YELLOW_DARK: Color = Color::rgb(158, 129, 0);
/// The palette green, #32cc58.
pub const GREEN: Color = Color::rgb(50, 204, 88);
/// The palette green-light, #5eee79.
pub const GREEN_LIGHT: Color = Color::rgb(94, 238, 121);
/// The palette green-dark, #007f2c.
pub const GREEN_DARK: Color = Color::rgb(0, 127, 44);
/// The palette mint, #00d1bb.
pub const MINT: Color = Color::rgb(0, 209, 187);
/// The palette mint-light, #48efd8.
pub const MINT_LIGHT: Color = Color::rgb(72, 239, 216);
/// The palette mint-dark, #008173.
pub const MINT_DARK: Color = Color::rgb(0, 129, 115);
/// The palette teal, #00cad8.
pub const TEAL: Color = Color::rgb(0, 202, 216);
/// The palette teal-light, #48e9f7.
pub const TEAL_LIGHT: Color = Color::rgb(72, 233, 247);
/// The palette teal-dark, #007c85.
pub const TEAL_DARK: Color = Color::rgb(0, 124, 133);
/// The palette cyan, #1ec9f3.
pub const CYAN: Color = Color::rgb(30, 201, 243);
/// The palette cyan-light, #86e2ff.
pub const CYAN_LIGHT: Color = Color::rgb(134, 226, 255);
/// The palette cyan-dark, #007c98.
pub const CYAN_DARK: Color = Color::rgb(0, 124, 152);
/// The palette blue, #008cff.
pub const BLUE: Color = Color::rgb(0, 140, 255);
/// The palette blue-light, #84bdff.
pub const BLUE_LIGHT: Color = Color::rgb(132, 189, 255);
/// The palette blue-dark, #00559f.
pub const BLUE_DARK: Color = Color::rgb(0, 85, 159);
/// The palette indigo, #6768fa.
pub const INDIGO: Color = Color::rgb(103, 104, 250);
/// The palette indigo-light, #9ea9ff.
pub const INDIGO_LIGHT: Color = Color::rgb(158, 169, 255);
/// The palette indigo-dark, #3c2abc.
pub const INDIGO_DARK: Color = Color::rgb(60, 42, 188);
/// The palette purple, #d332e9.
pub const PURPLE: Color = Color::rgb(211, 50, 233);
/// The palette purple-light, #f08aff.
pub const PURPLE_LIGHT: Color = Color::rgb(240, 138, 255);
/// The palette purple-dark, #870097.
pub const PURPLE_DARK: Color = Color::rgb(135, 0, 151);
/// The palette pink, #ff325a.
pub const PINK: Color = Color::rgb(255, 50, 90);
/// The palette pink-light, #ff9a9f.
pub const PINK_LIGHT: Color = Color::rgb(255, 154, 159);
/// The palette pink-dark, #a50030.
pub const PINK_DARK: Color = Color::rgb(165, 0, 48);
/// The palette brown, #b18462.
pub const BROWN: Color = Color::rgb(177, 132, 98);
/// The palette brown-light, #dfaf8c.
pub const BROWN_LIGHT: Color = Color::rgb(223, 175, 140);
/// The palette brown-dark, #754c2b.
pub const BROWN_DARK: Color = Color::rgb(117, 76, 43);
/// The palette gray, #8e8e93.
pub const GRAY: Color = Color::rgb(142, 142, 147);
/// The palette gray-light, #bababf.
pub const GRAY_LIGHT: Color = Color::rgb(186, 186, 191);
/// The palette gray-dark, #56565a.
pub const GRAY_DARK: Color = Color::rgb(86, 86, 90);

/// The names of the fifteen palette colors, in palette order.
pub const NAMES: [&str; 15] = [
    "black", "white", "red", "orange", "yellow", "green", "mint", "teal", "cyan", "blue", "indigo",
    "purple", "pink", "brown", "gray",
];

/// The fifteen named colors, in name order.
pub const PALETTE: [Color; 15] = [
    BLACK, WHITE, RED, ORANGE, YELLOW, GREEN, MINT, TEAL, CYAN, BLUE, INDIGO, PURPLE, PINK, BROWN,
    GRAY,
];

/// The thirteen hues, in name order.
pub const HUES: [Color; 13] = [
    RED, ORANGE, YELLOW, GREEN, MINT, TEAL, CYAN, BLUE, INDIGO, PURPLE, PINK, BROWN, GRAY,
];

/// The hues one shade lighter, in name order.
pub const HUES_LIGHT: [Color; 13] = [
    RED_LIGHT,
    ORANGE_LIGHT,
    YELLOW_LIGHT,
    GREEN_LIGHT,
    MINT_LIGHT,
    TEAL_LIGHT,
    CYAN_LIGHT,
    BLUE_LIGHT,
    INDIGO_LIGHT,
    PURPLE_LIGHT,
    PINK_LIGHT,
    BROWN_LIGHT,
    GRAY_LIGHT,
];

/// The hues one shade darker, in name order.
pub const HUES_DARK: [Color; 13] = [
    RED_DARK,
    ORANGE_DARK,
    YELLOW_DARK,
    GREEN_DARK,
    MINT_DARK,
    TEAL_DARK,
    CYAN_DARK,
    BLUE_DARK,
    INDIGO_DARK,
    PURPLE_DARK,
    PINK_DARK,
    BROWN_DARK,
    GRAY_DARK,
];

// THEME

/// One theme: the surfaces and the thirteen inks resolved for a dark or a light ground.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Theme {
    /// The ground every figure is painted on.
    pub ground: Color,
    /// The page background, one step off the ground.
    pub bg: Color,
    /// The raised panel.
    pub panel: Color,
    /// The sunken well.
    pub deep: Color,
    /// The hairline between things.
    pub line: Color,
    /// The foreground, the strongest tone.
    pub fg: Color,
    /// The dimmed foreground, for anything secondary.
    pub dim: Color,
    /// The interactive accent.
    pub accent: Color,
    /// The tone written on the accent.
    pub on_accent: Color,
    /// The red ink on this ground.
    pub red: Color,
    /// The orange ink on this ground.
    pub orange: Color,
    /// The yellow ink on this ground.
    pub yellow: Color,
    /// The green ink on this ground.
    pub green: Color,
    /// The mint ink on this ground.
    pub mint: Color,
    /// The teal ink on this ground.
    pub teal: Color,
    /// The cyan ink on this ground.
    pub cyan: Color,
    /// The blue ink on this ground.
    pub blue: Color,
    /// The indigo ink on this ground.
    pub indigo: Color,
    /// The purple ink on this ground.
    pub purple: Color,
    /// The pink ink on this ground.
    pub pink: Color,
    /// The brown ink on this ground.
    pub brown: Color,
    /// The gray ink on this ground.
    pub gray: Color,
}

impl Theme {
    /// The thirteen inks in name order.
    pub fn hues(&self) -> [Color; 13] {
        [
            self.red,
            self.orange,
            self.yellow,
            self.green,
            self.mint,
            self.teal,
            self.cyan,
            self.blue,
            self.indigo,
            self.purple,
            self.pink,
            self.brown,
            self.gray,
        ]
    }
    /// The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo.
    pub fn inks(&self) -> [Color; 6] {
        [
            self.blue,
            self.orange,
            self.yellow,
            self.green,
            self.pink,
            self.indigo,
        ]
    }
}

/// The dark theme.
pub const DARK: Theme = Theme {
    ground: Color::rgb(0, 0, 0),
    bg: Color::rgb(7, 7, 7),
    panel: Color::rgb(17, 17, 18),
    deep: Color::rgb(0, 0, 0),
    line: Color::rgb(31, 31, 32),
    fg: Color::rgb(255, 255, 255),
    dim: Color::rgb(142, 142, 147),
    accent: Color::rgb(158, 169, 255),
    on_accent: Color::rgb(0, 0, 0),
    red: Color::rgb(255, 61, 64),
    orange: Color::rgb(255, 143, 44),
    yellow: Color::rgb(255, 209, 0),
    green: Color::rgb(50, 204, 88),
    mint: Color::rgb(0, 209, 187),
    teal: Color::rgb(0, 202, 216),
    cyan: Color::rgb(30, 201, 243),
    blue: Color::rgb(0, 140, 255),
    indigo: Color::rgb(103, 104, 250),
    purple: Color::rgb(211, 50, 233),
    pink: Color::rgb(255, 50, 90),
    brown: Color::rgb(177, 132, 98),
    gray: Color::rgb(142, 142, 147),
};

/// The light theme.
pub const LIGHT: Theme = Theme {
    ground: Color::rgb(255, 255, 255),
    bg: Color::rgb(248, 248, 249),
    panel: Color::rgb(241, 241, 242),
    deep: Color::rgb(232, 232, 233),
    line: Color::rgb(221, 221, 223),
    fg: Color::rgb(0, 0, 0),
    dim: Color::rgb(86, 86, 90),
    accent: Color::rgb(60, 42, 188),
    on_accent: Color::rgb(255, 255, 255),
    red: Color::rgb(255, 61, 64),
    orange: Color::rgb(255, 143, 44),
    yellow: Color::rgb(255, 209, 0),
    green: Color::rgb(50, 204, 88),
    mint: Color::rgb(0, 209, 187),
    teal: Color::rgb(0, 202, 216),
    cyan: Color::rgb(30, 201, 243),
    blue: Color::rgb(0, 140, 255),
    indigo: Color::rgb(103, 104, 250),
    purple: Color::rgb(211, 50, 233),
    pink: Color::rgb(255, 50, 90),
    brown: Color::rgb(177, 132, 98),
    gray: Color::rgb(142, 142, 147),
};
