use super::setup::{Set, Square};
use super::Chess;
use serde_json::{json, Value as Json};

pub const PROMOTIONS: [(&str, u8); 4] = [("queen", 5), ("rook", 4), ("bishop", 3), ("knight", 2)];

impl Chess {
    fn colors_fact(colors: &[[u8; 4]; 2]) -> Json {
        json!(colors.iter().map(|c| json!(c.to_vec())).collect::<Vec<_>>())
    }
    fn colors_load(value: &Json, into: &mut [[u8; 4]; 2]) {
        let Some(pair) = value.as_array() else {
            return;
        };
        if pair.len() != 2 {
            return;
        }
        for (slot, given) in into.iter_mut().zip(pair) {
            let Some(parts) = given.as_array() else {
                return;
            };
            if parts.len() != 4 || !parts.iter().all(|p| p.as_u64().is_some_and(|n| n < 256)) {
                return;
            }
            for (i, part) in parts.iter().enumerate() {
                slot[i] = part.as_u64().unwrap_or(0) as u8;
            }
        }
    }
    pub fn snapshot(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "plies": self.plies,
            "turn": self.turn,
            "ep": self.ep,
            "over": self.over,
            "winner": self.winner,
            "last_move": match self.last_move {
                Some((f, t)) => json!([f, t]),
                None => Json::Null,
            },
            "board": self
                .board
                .iter()
                .map(|sq| json!([sq.kind, sq.team, if sq.moved { 1 } else { 0 }]))
                .collect::<Vec<_>>(),
            "piece_colors": Chess::colors_fact(&self.piece_colors),
            "board_colors": Chess::colors_fact(&self.board_colors),
            "glyphs": self
                .glyphs
                .iter()
                .map(|g| json!(g.to_vec()))
                .collect::<Vec<_>>(),
        })
    }
    pub fn restore(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        self.reset(state["seed"].as_u64().unwrap_or(0));
        let squares = state["board"].as_array().and_then(|given| {
            if given.len() != self.n() {
                return None;
            }
            let mut board = Vec::with_capacity(given.len());
            for entry in given {
                let kind = entry[0].as_u64()?;
                let team = entry[1].as_u64()?;
                let moved = entry[2].as_u64()?;
                if kind > 6 || team > 1 || moved > 1 {
                    return None;
                }
                board.push(Square {
                    kind: kind as u8,
                    team: team as u8,
                    moved: moved == 1,
                });
            }
            Some(board)
        });
        if let Some(board) = squares {
            self.board = board;
            self.turn = state["turn"]
                .as_u64()
                .map(|t| (t as u8).min(1))
                .unwrap_or(0);
            self.ep = state["ep"]
                .as_u64()
                .map(|c| c as usize)
                .filter(|&c| c < self.n());
            self.over = state["over"].as_bool().unwrap_or(false);
            self.winner = state["winner"].as_u64().map(|t| (t as u8).min(1));
            self.plies = state["plies"].as_u64().unwrap_or(0) as u32;
            self.last_move = state["last_move"].as_array().and_then(|pair| {
                let f = pair.first()?.as_u64()? as usize;
                let t = pair.get(1)?.as_u64()? as usize;
                if f < self.n() && t < self.n() {
                    Some((f, t))
                } else {
                    None
                }
            });
            Chess::colors_load(&state["piece_colors"], &mut self.piece_colors);
            Chess::colors_load(&state["board_colors"], &mut self.board_colors);
            if let Some(given) = state["glyphs"].as_array() {
                if given.len() == 6 {
                    for (slot, g) in self.glyphs.iter_mut().zip(given) {
                        let Some(bits) = g.as_array() else {
                            continue;
                        };
                        if bits.len() != 25 {
                            continue;
                        }
                        for (i, bit) in bits.iter().enumerate() {
                            slot[i] = bit.as_u64().map(|b| (b as u8).min(1)).unwrap_or(0);
                        }
                    }
                }
            }
            if let Some(pos) = state["pos"].as_u64() {
                self.rng.seek(pos as u128);
            }
        }
    }
}
