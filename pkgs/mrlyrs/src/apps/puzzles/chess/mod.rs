mod persist;
mod render;
mod rules;
mod setup;

#[cfg(test)]
mod tests;

use crate::core::rng::Rng;
use crate::music::cue;
use crate::os::kernel::{App, Call, Effect, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

use self::persist::PROMOTIONS;
use self::render::default_glyphs;
use self::rules::Status;
use self::setup::{deal, Set, Square};

const LETTERS: [&str; 6] = ["p", "n", "b", "r", "q", "k"];

pub struct Chess {
    set: Set,
    rng: Rng,
    seed: u64,
    board: Vec<Square>,
    w: usize,
    h: usize,
    turn: u8,
    ep: Option<usize>,
    over: bool,
    winner: Option<u8>,
    plies: u32,
    selected: Option<usize>,
    targets: Vec<usize>,
    last_move: Option<(usize, usize)>,
    piece_colors: [[u8; 4]; 2],
    board_colors: [[u8; 4]; 2],
    glyphs: [[u8; 25]; 6],
    dark: bool,
}

impl Default for Chess {
    fn default() -> Chess {
        Chess::new()
    }
}

impl Chess {
    pub fn new() -> Chess {
        let set = Set::new();
        let (board, w, h) = deal(&set.layout);
        let mut chess = Chess {
            set,
            rng: Rng::new(0),
            seed: 0,
            board,
            w,
            h,
            turn: 0,
            ep: None,
            over: false,
            winner: None,
            plies: 0,
            selected: None,
            targets: Vec::new(),
            last_move: None,
            piece_colors: [[255, 255, 255, 255], [40, 40, 40, 255]],
            board_colors: [[181, 136, 99, 255], [240, 217, 181, 255]],
            glyphs: default_glyphs(),
            dark: false,
        };
        chess.reset(0);
        chess
    }
    pub fn cell(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }
    pub fn coords(&self, c: usize) -> (usize, usize) {
        (c % self.w, c / self.w)
    }
    pub fn bound(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.w as i32 && y >= 0 && y < self.h as i32
    }
    pub fn n(&self) -> usize {
        self.w * self.h
    }
    pub fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        let (board, w, h) = deal(&self.set.layout);
        self.board = board;
        self.w = w;
        self.h = h;
        self.turn = 0;
        self.ep = None;
        self.over = false;
        self.winner = None;
        self.plies = 0;
        self.selected = None;
        self.targets.clear();
        self.last_move = None;
        self.roll();
    }
    fn square(&self, name: &Json) -> Option<usize> {
        let name = name.as_str()?;
        let mut chars = name.chars();
        let file = chars.next()?;
        if !file.is_ascii_lowercase() {
            return None;
        }
        let x = (file as u8 - b'a') as usize;
        let rank: usize = chars.as_str().parse().ok()?;
        if x >= self.w || rank < 1 || rank > self.h {
            return None;
        }
        Some(self.cell(x, self.h - rank))
    }
    fn alg(&self, c: usize) -> String {
        let (x, y) = self.coords(c);
        format!("{}{}", (b'a' + x as u8) as char, self.h - y)
    }
    fn legal(&self, from: usize, to: usize) -> bool {
        let (fx, fy) = self.coords(from);
        let sq = self.board[from];
        sq.kind != 0
            && sq.team == self.turn
            && self.valid(&self.board, self.ep, fx, fy).contains(&to)
    }
    fn spot(&self, call: &Call) -> Option<usize> {
        if !call.arg("square").is_null() {
            return self.square(call.arg("square"));
        }
        let x = call.arg("x").as_u64()? as usize;
        let y = call.arg("y").as_u64()? as usize;
        if x >= self.w || y >= self.h {
            return None;
        }
        Some(self.cell(x, y))
    }
    fn perform(&mut self, from: usize, to: usize, promote: u8) -> Outcome {
        let capture =
            self.board[to].kind != 0 || (self.board[from].kind == 1 && self.ep == Some(to));
        let mover = self.turn;
        self.execute(from, to, promote);
        self.plies += 1;
        self.selected = None;
        self.targets.clear();
        self.last_move = Some((from, to));
        let reskin = self.set.reskin as u32;
        if reskin > 0 && self.plies.is_multiple_of(reskin) {
            self.roll();
        }
        let opp = 1 - self.turn;
        match self.status(&self.board, self.ep, opp) {
            Status::Checkmate => {
                self.over = true;
                self.winner = Some(mover);
            }
            Status::Stalemate => {
                self.over = true;
                self.winner = None;
            }
            _ => self.turn = opp,
        }
        let name = if self.over {
            if self.winner.is_some() {
                "win"
            } else {
                "lose"
            }
        } else if self.in_check() {
            "bad"
        } else if capture {
            "good"
        } else {
            "blip"
        };
        Outcome::ok(json!({
            "from": self.alg(from),
            "to": self.alg(to),
            "over": self.over,
            "winner": self.winner_fact(),
        }))
        .emit(Effect::new("sound", cue::payload(name)))
    }
    fn moves(&self) -> Vec<Json> {
        let mut out = Vec::new();
        if self.over {
            return out;
        }
        for y in 0..self.h {
            for x in 0..self.w {
                let from = self.cell(x, y);
                let sq = self.board[from];
                if sq.kind != 0 && sq.team == self.turn {
                    for to in self.valid(&self.board, self.ep, x, y) {
                        out.push(json!({ "from": self.alg(from), "to": self.alg(to) }));
                    }
                }
            }
        }
        out
    }
    fn board_fact(&self) -> Json {
        let rows: Vec<Json> = (0..self.h)
            .map(|y| {
                let row: Vec<Json> = (0..self.w)
                    .map(|x| {
                        let sq = self.board[self.cell(x, y)];
                        if sq.kind == 0 {
                            Json::Null
                        } else {
                            let letter = LETTERS[sq.kind as usize - 1];
                            if sq.team == 0 {
                                json!(letter.to_uppercase())
                            } else {
                                json!(letter)
                            }
                        }
                    })
                    .collect();
                json!(row)
            })
            .collect();
        json!(rows)
    }
    fn team_name(team: u8) -> &'static str {
        if team == 0 {
            "white"
        } else {
            "black"
        }
    }
    fn winner_fact(&self) -> Json {
        if !self.over {
            return Json::Null;
        }
        match self.winner {
            Some(team) => json!(Chess::team_name(team)),
            None => json!("draw"),
        }
    }
    fn in_check(&self) -> bool {
        self.find_king(&self.board, self.turn).is_some() && !self.king_safe(&self.board, self.turn)
    }
}

impl App for Chess {
    fn route(&self) -> &str {
        "chess"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("chess").emoji("♟️").category("puzzles")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        json!({
            "score": 0,
            "steps": self.plies,
            "over": self.over,
            "seed": self.seed,
            "settings": self.set.to_json(),
            "turn": Chess::team_name(self.turn),
            "check": self.in_check(),
            "winner": self.winner_fact(),
            "board": self.board_fact(),
            "moves": self.moves(),
            "selected": match self.selected {
                Some(c) => json!(self.alg(c)),
                None => Json::Null,
            },
            "targets": self.targets.iter().map(|&c| json!(self.alg(c))).collect::<Vec<_>>(),
            "last_move": match self.last_move {
                Some((f, t)) => json!({ "from": self.alg(f), "to": self.alg(t) }),
                None => Json::Null,
            },
            "frame": self.render().fact(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        let mut out = Vec::new();
        if !self.over {
            out.push(Verb::new("chess.select", json!({ "square": "square" })));
            out.push(Verb::new(
                "chess.move",
                json!({
                    "from": "square",
                    "to": "square",
                    "promote": "queen | rook | bishop | knight",
                }),
            ));
        }
        out.push(Verb::new("chess.reset", json!({ "seed": "int" })));
        out.push(Verb::new(
            "chess.set",
            json!({ "key": "string", "value": "any" }),
        ));
        out
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "chess.select" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let Some(square) = self.spot(call) else {
                    return Outcome::fail("no such square");
                };
                if let Some(from) = self.selected {
                    if self.targets.contains(&square) {
                        return self.perform(from, square, 5);
                    }
                }
                let sq = self.board[square];
                if sq.kind != 0 && sq.team == self.turn {
                    let (x, y) = self.coords(square);
                    self.selected = Some(square);
                    self.targets = self.valid(&self.board, self.ep, x, y);
                    let targets: Vec<Json> =
                        self.targets.iter().map(|&c| json!(self.alg(c))).collect();
                    return Outcome::ok(json!({
                        "selected": self.alg(square),
                        "targets": targets,
                    }))
                    .emit(Effect::new("sound", cue::payload("blip")));
                }
                self.selected = None;
                self.targets.clear();
                Outcome::ok(json!({ "selected": Json::Null, "targets": [] }))
            }
            "chess.move" => {
                if self.over {
                    return Outcome::fail("round over, reset to continue");
                }
                let Some(from) = self.square(call.arg("from")) else {
                    return Outcome::fail("no such square");
                };
                let Some(to) = self.square(call.arg("to")) else {
                    return Outcome::fail("no such square");
                };
                if !self.legal(from, to) {
                    return Outcome::fail("illegal move")
                        .emit(Effect::new("sound", cue::payload("bad")));
                }
                let promote = match call.arg("promote") {
                    Json::Null => 5,
                    given => {
                        let Some(kind) = given
                            .as_str()
                            .and_then(|p| PROMOTIONS.iter().find(|(name, _)| *name == p))
                            .map(|(_, kind)| *kind)
                        else {
                            return Outcome::fail("promote must be queen, rook, bishop, or knight");
                        };
                        if !self.promoting(from, to) {
                            return Outcome::fail("nothing to promote");
                        }
                        kind
                    }
                };
                self.perform(from, to, promote)
            }
            "chess.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "chess.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        let seed = self.seed;
                        self.reset(seed);
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn save(&self) -> Json {
        self.snapshot()
    }
    fn load(&mut self, state: &Json) {
        self.restore(state);
    }
}
