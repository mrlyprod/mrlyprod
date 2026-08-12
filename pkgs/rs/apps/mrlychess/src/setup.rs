use mrlycore::{json, Json};

/// One square of the board and whatever piece stands on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    /// The piece standing here, 1 to 6 from pawn to king, or 0 for an empty square.
    pub kind: u8,
    /// The side the piece plays for, 0 for white and 1 for black.
    pub team: u8,
    /// Whether the piece has moved yet, which castling and the double pawn step ask about.
    pub moved: bool,
}

/// The empty square a board is filled with before the pieces land.
pub const HOLE: Square = Square {
    kind: 0,
    team: 0,
    moved: false,
};

fn piece_of(ch: char) -> Option<(u8, u8)> {
    let team = if ch.is_ascii_uppercase() { 0 } else { 1 };
    let kind = match ch.to_ascii_uppercase() {
        'P' => 1,
        'N' => 2,
        'B' => 3,
        'R' => 4,
        'Q' => 5,
        'K' => 6,
        _ => return None,
    };
    Some((kind, team))
}

fn rank_width(rank: &str) -> usize {
    rank.chars()
        .map(|ch| ch.to_digit(10).map(|d| d as usize).unwrap_or(1))
        .sum()
}

/// Deals a board from a rank-by-rank layout string, beside the width and height it implies.
pub fn deal(layout: &str) -> (Vec<Square>, usize, usize) {
    let ranks: Vec<&str> = layout.split('/').collect();
    let h = ranks.len().max(1);
    let w = rank_width(ranks.first().copied().unwrap_or("")).max(1);
    let mut board = vec![HOLE; w * h];
    for (y, rank) in ranks.iter().enumerate() {
        let mut x = 0;
        for ch in rank.chars() {
            if let Some(d) = ch.to_digit(10) {
                x += d as usize;
            } else if let Some((kind, team)) = piece_of(ch) {
                if x < w && y < h {
                    board[y * w + x] = Square {
                        kind,
                        team,
                        moved: false,
                    };
                }
                x += 1;
            }
        }
    }
    (board, w, h)
}

/// The dials a game is dealt and drawn with.
pub struct Set {
    /// The rank-by-rank layout the board is dealt from, up to 26 files wide.
    pub layout: String,
    /// The size of one square's tile, 5 to 16.
    pub tile: i64,
    /// Whether the six kinds wear each other's marks as a disguise.
    pub obfuscate: bool,
    /// How many plies pass between fresh color rolls, or 0 to keep the dealt colors.
    pub reskin: i64,
    /// The surface the board is drawn on, grid or canvas.
    pub surface: String,
    /// The look the pieces take, digits or emojis.
    pub skin: String,
}

impl Set {
    /// Builds the standard chess opening on a grid in the digits skin.
    pub fn new() -> Set {
        Set {
            layout: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
            tile: 5,
            obfuscate: false,
            reskin: 0,
            surface: "grid".to_string(),
            skin: "digits".to_string(),
        }
    }
    /// Applies one dial by name, returning the value taken or a note on why it was refused.
    pub fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "surface" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if name != "grid" && name != "canvas" {
                    return Err("surface must be grid or canvas");
                }
                self.surface = name.to_string();
                Ok(json!(name))
            }
            "skin" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if name == "tiles" {
                    return Err("chess has no tiles skin");
                }
                if name != "digits" && name != "emojis" {
                    return Err("skin must be digits or emojis");
                }
                self.skin = name.to_string();
                Ok(json!(name))
            }
            "layout" => {
                let layout = value.as_str().ok_or("value must be a string")?;
                let (_, w, _) = deal(layout);
                if w > 26 {
                    return Err("layout too wide");
                }
                self.layout = layout.to_string();
                Ok(json!(layout))
            }
            "tile" | "reskin" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                let (min, max) = match key {
                    "tile" => (5, 16),
                    _ => (0, 50),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "tile" => self.tile = n,
                    _ => self.reskin = n,
                }
                Ok(json!(n))
            }
            "obfuscate" => {
                let on = value.as_bool().ok_or("value must be a bool")?;
                self.obfuscate = on;
                Ok(json!(on))
            }
            _ => Err("no such key"),
        }
    }
    /// Writes the dials out as JSON.
    pub fn to_json(&self) -> Json {
        json!({
            "layout": &self.layout,
            "tile": self.tile,
            "obfuscate": self.obfuscate,
            "reskin": self.reskin,
            "surface": &self.surface,
            "skin": &self.skin,
        })
    }
    /// Reads the dials back from JSON, keeping the default for anything missing or refused.
    pub fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                let _ = set.apply(key, val);
            }
        }
        set
    }
}
