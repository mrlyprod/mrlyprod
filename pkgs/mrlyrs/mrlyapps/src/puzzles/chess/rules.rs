use super::setup::{Square, HOLE};
use super::Chess;

const ORTHO: [(i8, i8); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
const DIAG: [(i8, i8); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
const ROYAL: [(i8, i8); 8] = [
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];
const KNIGHT: [(i8, i8); 8] = [
    (1, -2),
    (2, -1),
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
];

const EMPTY: &[(i8, i8)] = &[];

type Steps = &'static [(i8, i8)];

fn rules(kind: u8) -> (Steps, Steps) {
    match kind {
        2 => (&KNIGHT, EMPTY),
        3 => (EMPTY, &DIAG),
        4 => (EMPTY, &ORTHO),
        5 => (EMPTY, &ROYAL),
        6 => (&ROYAL, EMPTY),
        _ => (EMPTY, EMPTY),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Normal,
    Check,
    Checkmate,
    Stalemate,
}

impl Chess {
    fn provisional(&self, board: &[Square], ep: Option<usize>, px: usize, py: usize) -> Vec<usize> {
        let sq = board[self.cell(px, py)];
        let foe = 1 - sq.team;
        let mut moves = Vec::new();
        if sq.kind == 1 {
            let dir: i32 = if sq.team == 0 { -1 } else { 1 };
            let ny = py as i32 + dir;
            if self.bound(px as i32, ny) && board[self.cell(px, ny as usize)].kind == 0 {
                moves.push(self.cell(px, ny as usize));
                let ny2 = py as i32 + dir * 2;
                if !sq.moved
                    && self.bound(px as i32, ny2)
                    && board[self.cell(px, ny2 as usize)].kind == 0
                {
                    moves.push(self.cell(px, ny2 as usize));
                }
            }
            for dx in [-1i32, 1] {
                let nx = px as i32 + dx;
                let nyy = py as i32 + dir;
                if !self.bound(nx, nyy) {
                    continue;
                }
                let c = self.cell(nx as usize, nyy as usize);
                let t = board[c];
                if t.kind != 0 && t.team == foe {
                    moves.push(c);
                }
                if ep == Some(c) {
                    moves.push(c);
                }
            }
            return moves;
        }
        let (leaps, rides) = rules(sq.kind);
        for &(dx, dy) in leaps {
            let nx = px as i32 + dx as i32;
            let ny = py as i32 + dy as i32;
            if self.bound(nx, ny) {
                let t = board[self.cell(nx as usize, ny as usize)];
                if t.kind == 0 || t.team == foe {
                    moves.push(self.cell(nx as usize, ny as usize));
                }
            }
        }
        for &(dx, dy) in rides {
            let mut nx = px as i32 + dx as i32;
            let mut ny = py as i32 + dy as i32;
            while self.bound(nx, ny) {
                let c = self.cell(nx as usize, ny as usize);
                let t = board[c];
                if t.kind == 0 {
                    moves.push(c);
                } else {
                    if t.team == foe {
                        moves.push(c);
                    }
                    break;
                }
                nx += dx as i32;
                ny += dy as i32;
            }
        }
        moves
    }
    pub fn find_king(&self, board: &[Square], team: u8) -> Option<(usize, usize)> {
        for y in 0..self.h {
            for x in 0..self.w {
                let p = board[self.cell(x, y)];
                if p.kind == 6 && p.team == team {
                    return Some((x, y));
                }
            }
        }
        None
    }
    fn attacked(&self, board: &[Square], tx: usize, ty: usize, by: u8) -> bool {
        let target = self.cell(tx, ty);
        for y in 0..self.h {
            for x in 0..self.w {
                let p = board[self.cell(x, y)];
                if p.kind == 0 || p.team != by {
                    continue;
                }
                if p.kind == 1 {
                    let dir: i32 = if p.team == 0 { -1 } else { 1 };
                    if y as i32 + dir == ty as i32
                        && (x as i32 - 1 == tx as i32 || x as i32 + 1 == tx as i32)
                    {
                        return true;
                    }
                } else if self.provisional(board, None, x, y).contains(&target) {
                    return true;
                }
            }
        }
        false
    }
    pub fn king_safe(&self, board: &[Square], team: u8) -> bool {
        match self.find_king(board, team) {
            Some((kx, ky)) => !self.attacked(board, kx, ky, 1 - team),
            None => false,
        }
    }
    pub fn valid(&self, board: &[Square], ep: Option<usize>, px: usize, py: usize) -> Vec<usize> {
        let sq = board[self.cell(px, py)];
        let team = sq.team;
        let foe = 1 - team;
        let from = self.cell(px, py);
        let mut valid = Vec::new();
        for to in self.provisional(board, ep, px, py) {
            let mut next = board.to_vec();
            let (mx, _) = self.coords(to);
            let mut moved = sq;
            moved.moved = true;
            next[to] = moved;
            next[from] = HOLE;
            if sq.kind == 1 && ep == Some(to) {
                next[self.cell(mx, py)] = HOLE;
            }
            if self.king_safe(&next, team) {
                valid.push(to);
            }
        }
        if sq.kind == 6 && !sq.moved && !self.attacked(board, px, py, foe) {
            let y = py;
            let rx = self.w - 1;
            if px + 2 < self.w {
                let kr = board[self.cell(rx, y)];
                if kr.kind == 4 && !kr.moved && kr.team == team {
                    let clear = (px + 1..rx).all(|x| board[self.cell(x, y)].kind == 0);
                    if clear
                        && !self.attacked(board, px + 1, y, foe)
                        && !self.attacked(board, px + 2, y, foe)
                    {
                        valid.push(self.cell(px + 2, y));
                    }
                }
            }
            if px >= 2 {
                let qr = board[self.cell(0, y)];
                if qr.kind == 4 && !qr.moved && qr.team == team {
                    let clear = (1..px).all(|x| board[self.cell(x, y)].kind == 0);
                    if clear
                        && !self.attacked(board, px - 1, y, foe)
                        && !self.attacked(board, px - 2, y, foe)
                    {
                        valid.push(self.cell(px - 2, y));
                    }
                }
            }
        }
        valid
    }
    pub fn execute(&mut self, from: usize, to: usize, promote: u8) {
        let (fx, fy) = self.coords(from);
        let (tx, ty) = self.coords(to);
        let piece = self.board[from];
        let mut new_ep = None;
        if piece.kind == 1 && self.ep == Some(to) {
            let gone = self.cell(tx, fy);
            self.board[gone] = HOLE;
        }
        if piece.kind == 6 && (tx as i32 - fx as i32).abs() == 2 {
            let (src, dst) = if tx > fx {
                (self.cell(self.w - 1, fy), self.cell(fx + 1, fy))
            } else {
                (self.cell(0, fy), self.cell(fx - 1, fy))
            };
            let mut rook = self.board[src];
            rook.moved = true;
            self.board[dst] = rook;
            self.board[src] = HOLE;
        }
        if piece.kind == 1 && (ty as i32 - fy as i32).abs() == 2 {
            new_ep = Some(self.cell(fx, (fy + ty) / 2));
        }
        let mut moved = piece;
        moved.moved = true;
        self.board[to] = moved;
        self.board[from] = HOLE;
        if piece.kind == 1 && (ty == 0 || ty == self.h - 1) {
            self.board[to].kind = promote;
        }
        self.ep = new_ep;
    }
    pub fn promoting(&self, from: usize, to: usize) -> bool {
        let (_, ty) = self.coords(to);
        self.board[from].kind == 1 && (ty == 0 || ty == self.h - 1)
    }
    pub fn status(&self, board: &[Square], ep: Option<usize>, team: u8) -> Status {
        let in_check = !self.king_safe(board, team) && self.find_king(board, team).is_some();
        for y in 0..self.h {
            for x in 0..self.w {
                let p = board[self.cell(x, y)];
                if p.kind != 0 && p.team == team && !self.valid(board, ep, x, y).is_empty() {
                    return if in_check {
                        Status::Check
                    } else {
                        Status::Normal
                    };
                }
            }
        }
        if in_check {
            Status::Checkmate
        } else {
            Status::Stalemate
        }
    }
}
