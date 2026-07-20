use serde_json::{json, Value as Json};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    pub kind: u8,
    pub team: u8,
    pub moved: bool,
}

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

pub struct Set {
    pub layout: String,
    pub tile: i64,
    pub obfuscate: bool,
    pub reskin: i64,
    pub surface: String,
    pub skin: String,
}

impl Set {
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
    pub fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "surface" => {
                let name = value.as_str().ok_or("value must be a string")?;
                if name != "grid" && name != "canvas" {
                    return Err("surface must be grid or canvas");
                }
                if name == "canvas" && self.skin == "emojis" {
                    return Err("emojis need the grid surface");
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
                if name == "emojis" && self.surface != "grid" {
                    return Err("emojis need the grid surface");
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
    pub fn to_json(&self) -> Json {
        json!({
            "layout": self.layout,
            "tile": self.tile,
            "obfuscate": self.obfuscate,
            "reskin": self.reskin,
            "surface": self.surface,
            "skin": self.skin,
        })
    }
    pub fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                let _ = set.apply(key, val);
            }
        }
        if set.skin == "emojis" && set.surface != "grid" {
            set.skin = "digits".to_string();
        }
        set
    }
}
