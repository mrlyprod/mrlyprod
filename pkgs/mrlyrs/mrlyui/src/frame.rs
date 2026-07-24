use mrlycore::paint::Paint;
use mrlycore::tensor::Tensor;
use mrlycore::tile::{Group, Tile};
use mrlymath::two::tile as tile2d;
use mrlymath::two::{designs, Cell2d};
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct TileSet {
    pub size: usize,
    pub tiles: Vec<Cell2d>,
}

impl TileSet {
    pub fn new(size: usize, tiles: Vec<Cell2d>) -> TileSet {
        TileSet { size, tiles }
    }
}

pub fn board(dark: bool) -> [u8; 4] {
    let c = if dark {
        mrlycore::colors::BOARD_DARK
    } else {
        mrlycore::colors::BOARD_LIGHT
    };
    [c.r, c.g, c.b, c.a]
}

pub fn ink(dark: bool) -> [u8; 4] {
    let c = if dark {
        mrlycore::colors::WHITE
    } else {
        mrlycore::colors::BLACK
    };
    [c.r, c.g, c.b, c.a]
}

pub fn hex(c: [u8; 4]) -> String {
    if c[3] == 255 {
        format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
    } else {
        format!("#{:02x}{:02x}{:02x}{:02x}", c[0], c[1], c[2], c[3])
    }
}

pub fn hex_of(hex: &str) -> [u8; 4] {
    let code = hex.trim_start_matches('#');
    let byte = |i: usize| u8::from_str_radix(&code[i..i + 2], 16).unwrap_or(0);
    match code.len() {
        6 => [byte(0), byte(2), byte(4), 255],
        8 => [byte(0), byte(2), byte(4), byte(6)],
        _ => [0, 0, 0, 255],
    }
}

pub fn solid_tile(k: usize, color: [u8; 4]) -> Cell2d {
    solid_rect(k, k, color)
}

pub fn solid_rect(w: usize, h: usize, color: [u8; 4]) -> Cell2d {
    let mut cell = Cell2d::new(Tensor::full(vec![h, w], 1));
    cell.cell.colors = Some(vec![color; w * h]);
    cell
}

pub fn motif_tile(name: &str, k: usize, fg: [u8; 4], bg: [u8; 4]) -> Cell2d {
    let mask = motif(name, k);
    let colors: Vec<[u8; 4]> = mask
        .bytes()
        .iter()
        .map(|&v| if v == 1 { fg } else { bg })
        .collect();
    let mut cell = Cell2d::new(mask);
    cell.cell.colors = Some(colors);
    cell
}

pub fn bake(tile: &mut Cell2d, label: &str, k: usize, ink: [u8; 4]) {
    let mask = mrlyfont::raster(label);
    let mask_h = mask.len();
    let mask_w = mask.first().map(Vec::len).unwrap_or(0);
    if mask_h + 2 > k || mask_w + 2 > k {
        return;
    }
    let oy = (k - mask_h) / 2;
    let ox = (k - mask_w) / 2;
    if let Some(colors) = tile.cell.colors.as_mut() {
        for (y, row) in mask.iter().enumerate() {
            for (x, &v) in row.iter().enumerate() {
                if v == 1 {
                    colors[(oy + y) * k + (ox + x)] = ink;
                }
            }
        }
    }
}

fn shaped(tile: &Tile) -> bool {
    let slots = tile.sources.len();
    let wanted = match tile.group {
        Group::Mosaic => 3,
        Group::Magic => 2,
        _ => 1,
    };
    slots >= wanted
        && tile.numbers.len() == slots
        && tile.levels.len() == slots
        && tile.rotations.len() == slots
        && tile.numbers.iter().all(|&n| n >= 1)
}

pub fn sample_types(cell: &Cell2d, k: usize) -> Tensor {
    let (w, h) = (cell.width(), cell.height());
    let mut out = Tensor::new(vec![k, k]);
    for y in 0..k {
        for x in 0..k {
            out.set(&[y, x], cell.types().get(&[y * h / k, x * w / k]));
        }
    }
    out
}

pub fn probe(tile: &Tile) -> bool {
    shaped(tile)
        && tile2d::build(tile)
            .map(|c| c.width() == tile.width && c.height() == tile.height)
            .unwrap_or(false)
}

pub fn tile_cell(tile: &Tile, k: usize, fg: [u8; 4], bg: [u8; 4]) -> Cell2d {
    if !shaped(tile) {
        return solid_tile(k, fg);
    }
    match tile2d::build(tile) {
        Ok(cell) if cell.width() > 0 && cell.height() > 0 => {
            let mask = sample_types(&cell, k);
            let colors: Vec<[u8; 4]> = mask
                .bytes()
                .iter()
                .map(|&v| if v != 0 { fg } else { bg })
                .collect();
            let mut out = Cell2d::new(mask);
            out.cell.colors = Some(colors);
            out
        }
        _ => solid_tile(k, fg),
    }
}

pub fn work_cell(tile: &Tile, paint: &Paint, k: usize, bg: [u8; 4]) -> Cell2d {
    let ink = paint.primary.color();
    let solid = [ink.r, ink.g, ink.b, ink.a];
    if !shaped(tile) {
        return solid_tile(k, solid);
    }
    let Ok(mut cell) = tile2d::build(tile) else {
        return solid_tile(k, solid);
    };
    if cell.width() == 0 || cell.height() == 0 {
        return solid_tile(k, solid);
    }
    if mrlycore::paint::coat(&mut cell.cell, paint, None).is_err() {
        return solid_tile(k, solid);
    }
    let (w, h) = (cell.width(), cell.height());
    let mask = sample_types(&cell, k);
    let mut colors = Vec::with_capacity(k * k);
    for y in 0..k {
        for x in 0..k {
            let picked = cell.cell.color_at((y * h / k) * w + x * w / k);
            colors.push(if picked[3] == 0 { bg } else { picked });
        }
    }
    let mut out = Cell2d::new(mask);
    out.cell.colors = Some(colors);
    out
}

fn motif(name: &str, k: usize) -> Tensor {
    let solid = || Tensor::full(vec![k, k], 1);
    let built = match name {
        "carpet" => designs::carpet(k, 1),
        "net" => designs::net(k, 1),
        "vtree" => designs::vtree(k, 1),
        "htree" => designs::htree(k, 1),
        _ => return solid(),
    };
    match built {
        Ok(c) if c.width() == k && c.height() == k => c.types().clone(),
        _ => solid(),
    }
}

#[derive(Clone, Debug)]
pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub cell: Cell2d,
}

impl Sprite {
    pub fn new(x: f64, y: f64, cell: Cell2d) -> Sprite {
        Sprite { x, y, cell }
    }
}

#[derive(Clone, Debug)]
pub enum Layer {
    Tiles { ids: Tensor, set: TileSet },
    Sprites(Vec<Sprite>),
    Field(Cell2d),
}

#[derive(Clone, Debug, Default)]
pub struct Hud {
    pub prompt: String,
    pub options: Vec<String>,
}

impl Hud {
    fn empty(&self) -> bool {
        self.prompt.is_empty() && self.options.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub background: [u8; 4],
    pub layers: Vec<Layer>,
    pub hud: Hud,
}

impl Frame {
    pub fn new(width: usize, height: usize, background: [u8; 4]) -> Frame {
        Frame {
            width,
            height,
            background,
            layers: Vec::new(),
            hud: Hud::default(),
        }
    }
    pub fn push(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
    pub fn say(&mut self, prompt: impl Into<String>, options: Vec<String>) {
        self.hud = Hud {
            prompt: prompt.into(),
            options,
        };
    }
    pub fn composite(&self) -> Cell2d {
        let (w, h) = (self.width, self.height);
        let mut buf = vec![self.background; w * h];
        for layer in &self.layers {
            match layer {
                Layer::Field(cell) => {
                    if cell.width() == w && cell.height() == h {
                        for (i, px) in buf.iter_mut().enumerate() {
                            over(px, cell.cell.color_at(i));
                        }
                    }
                }
                Layer::Tiles { ids, set } => {
                    let k = set.size;
                    let rows = ids.shape[0];
                    let cols = ids.shape[1];
                    for r in 0..rows {
                        for c in 0..cols {
                            let id = ids.get(&[r, c]) as usize;
                            if id >= set.tiles.len() {
                                continue;
                            }
                            let tile = &set.tiles[id];
                            for ty in 0..k {
                                for tx in 0..k {
                                    let src = tile.cell.color_at(ty * k + tx);
                                    let x = c * k + tx;
                                    let y = r * k + ty;
                                    over(&mut buf[y * w + x], src);
                                }
                            }
                        }
                    }
                }
                Layer::Sprites(sprites) => {
                    for s in sprites {
                        let sw = s.cell.width();
                        let sh = s.cell.height();
                        let px = s.x.floor() as i64;
                        let py = s.y.floor() as i64;
                        for ty in 0..sh {
                            for tx in 0..sw {
                                let x = px + tx as i64;
                                let y = py + ty as i64;
                                if x < 0 || y < 0 || x >= w as i64 || y >= h as i64 {
                                    continue;
                                }
                                let src = s.cell.cell.color_at(ty * sw + tx);
                                over(&mut buf[y as usize * w + x as usize], src);
                            }
                        }
                    }
                }
            }
        }
        let mut cell = Cell2d::new(Tensor::new(vec![h, w]));
        cell.cell.colors = Some(buf);
        cell
    }
    pub fn fact(&self) -> Value {
        let (rows, palette) = self.raster();
        json!({
            "width": self.width,
            "height": self.height,
            "rows": rows,
            "palette": palette.iter().map(|c| hex(*c)).collect::<Vec<_>>(),
        })
    }
    pub fn raster(&self) -> (Vec<Vec<usize>>, Vec<[u8; 4]>) {
        let pixels = self.composite().cell.colors.unwrap_or_default();
        let mut palette: Vec<[u8; 4]> = Vec::new();
        let mut rows = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for x in 0..self.width {
                let color = pixels[y * self.width + x];
                let id = match palette.iter().position(|&c| c == color) {
                    Some(id) => id,
                    None => {
                        palette.push(color);
                        palette.len() - 1
                    }
                };
                row.push(id);
            }
            rows.push(row);
        }
        (rows, palette)
    }
    pub fn to_json(&self) -> Value {
        let layers: Vec<Value> = self
            .layers
            .iter()
            .map(|layer| match layer {
                Layer::Tiles { ids, set } => {
                    let rows = ids.shape[0];
                    let cols = ids.shape[1];
                    let grid: Vec<Vec<u8>> = (0..rows)
                        .map(|r| (0..cols).map(|c| ids.get(&[r, c])).collect())
                        .collect();
                    let colors: Vec<[u8; 4]> = set.tiles.iter().map(representative).collect();
                    json!({
                        "type": "tiles",
                        "rows": rows,
                        "cols": cols,
                        "size": set.size,
                        "ids": grid,
                        "colors": colors,
                    })
                }
                Layer::Sprites(sprites) => {
                    let items: Vec<Value> = sprites
                        .iter()
                        .map(|s| {
                            json!({
                                "x": s.x,
                                "y": s.y,
                                "w": s.cell.width(),
                                "h": s.cell.height(),
                                "color": representative(&s.cell),
                            })
                        })
                        .collect();
                    json!({ "type": "sprites", "items": items })
                }
                Layer::Field(cell) => json!({
                    "type": "field",
                    "rows": cell.height(),
                    "cols": cell.width(),
                }),
            })
            .collect();
        let mut out = json!({
            "width": self.width,
            "height": self.height,
            "background": self.background,
            "layers": layers,
        });
        if !self.hud.empty() {
            out["hud"] = json!({
                "prompt": self.hud.prompt,
                "options": self.hud.options,
            });
        }
        out
    }
}

pub fn sprite_fact(cell: &Cell2d) -> Value {
    let (w, h) = (cell.width(), cell.height());
    let mut palette: Vec<[u8; 4]> = Vec::new();
    let mut rows = Vec::with_capacity(h);
    for y in 0..h {
        let mut row = Vec::with_capacity(w);
        for x in 0..w {
            let color = cell.cell.color_at(y * w + x);
            let id = match palette.iter().position(|&c| c == color) {
                Some(id) => id,
                None => {
                    palette.push(color);
                    palette.len() - 1
                }
            };
            row.push(id);
        }
        rows.push(row);
    }
    json!({
        "width": w,
        "height": h,
        "rows": rows,
        "palette": palette.iter().map(|c| hex(*c)).collect::<Vec<_>>(),
    })
}

pub fn glyph_fact(text: &str) -> Value {
    let rows = mrlyfont::raster(text);
    json!({
        "text": text,
        "width": rows.first().map(Vec::len).unwrap_or(0),
        "height": rows.len(),
        "rows": rows,
    })
}

pub fn empty_fact(width: usize, height: usize) -> Value {
    json!({ "width": width, "height": height, "rows": [], "palette": [] })
}

pub fn field(w: usize, h: usize, colors: Vec<[u8; 4]>, background: [u8; 4]) -> Frame {
    let mut cell = Cell2d::new(Tensor::new(vec![h, w]));
    cell.cell.colors = Some(colors);
    let mut frame = Frame::new(w, h, background);
    frame.push(Layer::Field(cell));
    frame
}

pub fn mix(from: [u8; 4], to: [u8; 4], t: f64) -> [u8; 4] {
    let lerp = |a: u8, b: u8| (a as f64 + (b as f64 - a as f64) * t) as u8;
    [
        lerp(from[0], to[0]),
        lerp(from[1], to[1]),
        lerp(from[2], to[2]),
        lerp(from[3], to[3]),
    ]
}

fn representative(tile: &Cell2d) -> [u8; 4] {
    for i in 0..tile.cell.size() {
        let c = tile.cell.color_at(i);
        if c[3] != 0 {
            return c;
        }
    }
    [0, 0, 0, 0]
}

pub(crate) fn over(dst: &mut [u8; 4], src: [u8; 4]) {
    let sa = src[3] as u32;
    if sa == 0 {
        return;
    }
    if sa == 255 {
        *dst = src;
        return;
    }
    let da = dst[3] as u32;
    let inv = 255 - sa;
    let out_a = sa + da * inv / 255;
    for i in 0..3 {
        let s = src[i] as u32;
        let d = dst[i] as u32;
        dst[i] = ((s * sa + d * da * inv / 255) / out_a.max(1)) as u8;
    }
    dst[3] = out_a as u8;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::tile::{Design, Source};
    fn general_carpet(n: usize) -> Tile {
        let mut tile = Tile::new(Group::General).size(n, n);
        tile.sources = vec![Source::Classic(Design::Carpet)];
        tile.numbers = vec![n];
        tile.levels = vec![1];
        tile.rotations = vec![0];
        tile.anti = vec![false];
        tile.factor = n;
        tile
    }
    #[test]
    fn tile_cell_matches_the_motif() {
        let fg = [255, 0, 0, 255];
        let bg = [0, 0, 0, 0];
        let cell = tile_cell(&general_carpet(3), 3, fg, bg);
        let motif = motif_tile("carpet", 3, fg, bg);
        assert_eq!(cell.types(), motif.types());
        assert_eq!(cell.cell.colors, motif.cell.colors);
    }
    #[test]
    fn tile_cell_resamples_to_k() {
        let fg = [255, 255, 255, 255];
        let bg = [0, 0, 0, 0];
        let mut tile = general_carpet(3);
        tile.group = Group::Fractal;
        tile.levels = vec![2];
        tile.width = 9;
        tile.height = 9;
        let cell = tile_cell(&tile, 4, fg, bg);
        assert_eq!(cell.width(), 4);
        assert_eq!(cell.height(), 4);
        assert_eq!(cell.cell.colors.as_ref().unwrap().len(), 16);
        assert_eq!(cell.cell.color_at(0), fg);
    }
    #[test]
    fn tile_cell_falls_back_to_solid() {
        let fg = [9, 9, 9, 255];
        let bg = [0, 0, 0, 0];
        let mut unbuildable = general_carpet(3);
        unbuildable.sources = vec![Source::Classic(Design::Xtree)];
        assert_eq!(
            tile_cell(&unbuildable, 3, fg, bg).cell.colors,
            solid_tile(3, fg).cell.colors
        );
        let mut misshaped = general_carpet(3);
        misshaped.sources.clear();
        assert_eq!(
            tile_cell(&misshaped, 3, fg, bg).cell.colors,
            solid_tile(3, fg).cell.colors
        );
    }
    #[test]
    fn work_cell_is_deterministic() {
        use mrlycore::paint::{Edition, Ink, Paint, Scheme, Target};
        let bg = [0, 0, 0, 0];
        let mut paint = Paint::new(Edition::Random);
        paint.scheme = Scheme::Multicolor;
        paint.target = Target::Fill;
        paint.primary = Ink::Black;
        paint.secondary = vec![Ink::Red, Ink::Teal, Ink::Blue];
        let tile = general_carpet(5);
        let a = work_cell(&tile, &paint, 4, bg);
        let b = work_cell(&tile, &paint, 4, bg);
        assert_eq!(a, b);
        assert_eq!(a.cell.colors.as_ref().unwrap().len(), 16);
        let mut broken = general_carpet(3);
        broken.sources = vec![Source::Classic(Design::Xtree)];
        let ink = paint.primary.color();
        assert_eq!(
            work_cell(&broken, &paint, 3, bg).cell.colors,
            solid_tile(3, [ink.r, ink.g, ink.b, ink.a]).cell.colors
        );
    }
    #[test]
    fn tiles_composite_to_pixels() {
        let red = [255, 0, 0, 255];
        let blue = [0, 0, 255, 255];
        let set = TileSet::new(2, vec![solid_tile(2, red), solid_tile(2, blue)]);
        let ids = Tensor::of(vec![0, 1, 1, 0], vec![2, 2]);
        let mut frame = Frame::new(4, 4, [0, 0, 0, 255]);
        frame.push(Layer::Tiles { ids, set });
        let cell = frame.composite();
        assert_eq!(cell.width(), 4);
        assert_eq!(cell.height(), 4);
        let colors = cell.cell.colors.unwrap();
        assert_eq!(colors[0], red);
        assert_eq!(colors[2], blue);
    }
    #[test]
    fn sprite_blits_at_float_position() {
        let red = [255, 0, 0, 255];
        let mut frame = Frame::new(4, 4, [0, 0, 0, 255]);
        frame.push(Layer::Sprites(vec![Sprite::new(
            1.7,
            0.2,
            solid_rect(2, 1, red),
        )]));
        let colors = frame.composite().cell.colors.unwrap();
        assert_eq!(colors[1], red);
        assert_eq!(colors[2], red);
        assert_eq!(colors[0], [0, 0, 0, 255]);
    }
    #[test]
    fn sprite_clips_off_screen() {
        let red = [255, 0, 0, 255];
        let mut frame = Frame::new(2, 2, [0, 0, 0, 255]);
        frame.push(Layer::Sprites(vec![Sprite::new(
            -1.0,
            -1.0,
            solid_rect(2, 2, red),
        )]));
        let colors = frame.composite().cell.colors.unwrap();
        assert_eq!(colors[0], red);
        assert_eq!(colors[3], [0, 0, 0, 255]);
    }
    #[test]
    fn hud_omitted_when_empty_then_emitted() {
        let mut frame = Frame::new(1, 1, [0, 0, 0, 255]);
        assert!(frame.to_json().get("hud").is_none());
        frame.say("circle 3", vec!["circle 3".into(), "net 2".into()]);
        let hud = &frame.to_json()["hud"];
        assert_eq!(hud["prompt"], "circle 3");
        assert_eq!(hud["options"][1], "net 2");
    }
    #[test]
    fn raster_indexes_a_palette() {
        let red = [255, 0, 0, 255];
        let black = [0, 0, 0, 255];
        let set = TileSet::new(1, vec![solid_tile(1, black), solid_tile(1, red)]);
        let ids = Tensor::of(vec![0, 1, 1, 0], vec![2, 2]);
        let mut frame = Frame::new(2, 2, black);
        frame.push(Layer::Tiles { ids, set });
        let (rows, palette) = frame.raster();
        assert_eq!(palette, vec![black, red]);
        assert_eq!(rows, vec![vec![0, 1], vec![1, 0]]);
    }
    #[test]
    fn transparent_shows_background() {
        let set = TileSet::new(1, vec![solid_tile(1, [0, 0, 0, 0])]);
        let ids = Tensor::new(vec![1, 1]);
        let mut frame = Frame::new(1, 1, [9, 9, 9, 255]);
        frame.push(Layer::Tiles { ids, set });
        let colors = frame.composite().cell.colors.unwrap();
        assert_eq!(colors[0], [9, 9, 9, 255]);
    }
    #[test]
    fn sprite_fact_matches_frame_fact_encoding() {
        let red = [255, 0, 0, 255];
        let black = [0, 0, 0, 255];
        let set = TileSet::new(1, vec![solid_tile(1, black), solid_tile(1, red)]);
        let ids = Tensor::of(vec![0, 1, 1, 0], vec![2, 2]);
        let mut frame = Frame::new(2, 2, black);
        frame.push(Layer::Tiles { ids, set });
        let fact = sprite_fact(&frame.composite());
        assert_eq!(fact, frame.fact());
    }
    #[test]
    fn glyph_fact_wraps_the_raster() {
        let fact = glyph_fact("42");
        let rows = mrlyfont::raster("42");
        assert_eq!(fact["text"], "42");
        assert_eq!(fact["height"], json!(5));
        assert_eq!(fact["width"], json!(rows[0].len()));
    }
    #[test]
    fn glyph_fact_of_empty_text_is_empty() {
        let fact = glyph_fact("");
        assert_eq!(fact["width"], json!(0));
        assert_eq!(fact["height"], json!(0));
    }
}
