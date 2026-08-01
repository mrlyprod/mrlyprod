use super::Chess;
use mrlycore::colors::PALETTE;
use mrlycore::tensor::Tensor;
use mrlyui::frame::{solid_tile, Frame, Layer, TileSet};
use mrlyui::skin::Skin;

impl Chess {
    fn palette(&mut self) -> [u8; 4] {
        let c = PALETTE[self.rng.below(PALETTE.len())];
        [c.r, c.g, c.b, 255]
    }
    pub fn roll(&mut self) {
        self.piece_colors[0] = self.palette();
        loop {
            self.piece_colors[1] = self.palette();
            if self.piece_colors[1] != self.piece_colors[0] {
                break;
            }
        }
        self.board_colors[0] = self.palette();
        loop {
            self.board_colors[1] = self.palette();
            if self.board_colors[1] != self.board_colors[0] {
                break;
            }
        }
        self.glyphs = if self.set.obfuscate {
            self.scramble()
        } else {
            mrlyui::skin::chess::glyphs()
        };
    }
    pub fn skin(&self) -> Skin {
        mrlyui::skin::chess::skin(&self.set.skin, &self.glyphs)
    }
    pub fn ids_facts(&self) -> Vec<Vec<u8>> {
        (0..self.h)
            .map(|y| {
                (0..self.w)
                    .map(|x| {
                        let sq = self.board[self.cell(x, y)];
                        if sq.kind == 0 {
                            0
                        } else {
                            sq.kind + sq.team * 6
                        }
                    })
                    .collect()
            })
            .collect()
    }
    fn scramble(&mut self) -> [[u8; 25]; 6] {
        let mut out = [[0u8; 25]; 6];
        for piece in out.iter_mut() {
            for y in 0..5 {
                for x in 0..=2 {
                    let bit = self.rng.below(2) as u8;
                    piece[y * 5 + x] = bit;
                    piece[y * 5 + (4 - x)] = bit;
                }
            }
        }
        out
    }
    fn piece_ids(&self) -> Tensor {
        let mut grid = Tensor::new(vec![self.h, self.w]);
        for (y, row) in self.ids_facts().iter().enumerate() {
            for (x, &id) in row.iter().enumerate() {
                grid.set(&[y, x], id);
            }
        }
        grid
    }
    fn board_ids(&self) -> Tensor {
        let mut grid = Tensor::new(vec![self.h, self.w]);
        for y in 0..self.h {
            for x in 0..self.w {
                grid.set(&[y, x], ((x + y) % 2) as u8);
            }
        }
        grid
    }
    fn glyph(&self, mask: &[u8; 25], fg: [u8; 4]) -> mrlymath::two::Cell2d {
        let k = self.set.tile as usize;
        let clear = [0, 0, 0, 0];
        let mut types = vec![0u8; k * k];
        let mut colors = vec![clear; k * k];
        for ty in 0..k {
            for tx in 0..k {
                let sy = ty * 5 / k;
                let sx = tx * 5 / k;
                if mask[sy * 5 + sx] == 1 {
                    types[ty * k + tx] = 1;
                    colors[ty * k + tx] = fg;
                }
            }
        }
        let mut cell = mrlymath::two::Cell2d::new(Tensor::of(types, vec![k, k]));
        cell.cell.colors = Some(colors);
        cell
    }
    fn pieces_set(&self) -> TileSet {
        let k = self.set.tile as usize;
        let mut tiles = Vec::with_capacity(13);
        tiles.push(solid_tile(k, [0, 0, 0, 0]));
        for kind in 0..6 {
            tiles.push(self.glyph(&self.glyphs[kind], self.piece_colors[0]));
        }
        for kind in 0..6 {
            tiles.push(self.glyph(&self.glyphs[kind], self.piece_colors[1]));
        }
        TileSet::new(k, tiles)
    }
    fn board_set(&self) -> TileSet {
        let k = self.set.tile as usize;
        TileSet::new(
            k,
            vec![
                solid_tile(k, self.board_colors[0]),
                solid_tile(k, self.board_colors[1]),
            ],
        )
    }
    pub fn render(&self) -> Frame {
        let k = self.set.tile as usize;
        let mut frame = Frame::new(self.w * k, self.h * k, mrlyui::frame::board(self.dark));
        frame.push(Layer::Tiles {
            ids: self.board_ids(),
            set: self.board_set(),
        });
        frame.push(Layer::Tiles {
            ids: self.piece_ids(),
            set: self.pieces_set(),
        });
        frame
    }
}
