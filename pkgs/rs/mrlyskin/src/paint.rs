use mrlycore::colors::hex;
use mrlycore::image::Image;
use mrlycore::tensor::Tensor;
use mrlycore::{json, Json};
use mrlymath::two::{designs, Cell2d};

/// A face that travels beside the pixels instead of baking into them.
#[derive(Clone, Debug, PartialEq)]
pub struct Glyph {
    /// The character or emoji to draw.
    pub ch: String,
    /// The color to draw it in, or None to keep its own colors.
    pub tint: Option<[u8; 4]>,
}

/// One baked tile per role, plus the faces that ride alongside.
#[derive(Clone, Debug)]
pub struct Atlas {
    /// The tile side in pixels.
    pub size: usize,
    /// The baked tiles, indexed by role.
    pub tiles: Vec<Cell2d>,
    /// The unbaked faces, indexed by role.
    pub faces: Vec<Option<Glyph>>,
}

impl Atlas {
    /// Builds an atlas of tiles with an empty face per tile.
    pub fn new(size: usize, tiles: Vec<Cell2d>) -> Atlas {
        let faces = vec![None; tiles.len()];
        Atlas { size, tiles, faces }
    }
    /// Sets the glyph riding the given role, ignoring roles out of range.
    pub fn face(&mut self, id: usize, ch: impl Into<String>, tint: Option<[u8; 4]>) {
        if id < self.faces.len() {
            self.faces[id] = Some(Glyph {
                ch: ch.into(),
                tint,
            });
        }
    }
}

/// Builds a square tile of one color.
pub fn solid_tile(k: usize, color: [u8; 4]) -> Cell2d {
    let mut cell = Cell2d::new(Tensor::full(vec![k, k], 1));
    cell.cell.colors = Some(vec![color; k * k]);
    cell
}

/// Builds a square tile weaving the named motif in the foreground color over the background, solid foreground when the name is unknown.
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

/// Bakes the label's font raster centered into the tile in ink, doing nothing when it will not fit.
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

/// An atlas laid over a grid of role ids.
#[derive(Clone, Debug)]
pub struct Raster {
    /// The color behind transparent tiles.
    pub background: [u8; 4],
    /// The grid of role ids.
    pub ids: Tensor,
    /// The atlas that dresses the ids.
    pub set: Atlas,
}

impl Raster {
    /// Builds a raster of ids dressed by an atlas over a background.
    pub fn new(ids: Tensor, set: Atlas, background: [u8; 4]) -> Raster {
        Raster {
            background,
            ids,
            set,
        }
    }
    /// Returns the raster's width in pixels.
    pub fn width(&self) -> usize {
        self.ids.shape[1] * self.set.size
    }
    /// Returns the raster's height in pixels.
    pub fn height(&self) -> usize {
        self.ids.shape[0] * self.set.size
    }
    /// Composites every tile over the background into one cell of pixels.
    pub fn composite(&self) -> Cell2d {
        let (w, h) = (self.width(), self.height());
        let mut buf = vec![self.background; w * h];
        let k = self.set.size;
        for r in 0..self.ids.shape[0] {
            for c in 0..self.ids.shape[1] {
                let id = self.ids.get(&[r, c]) as usize;
                if id >= self.set.tiles.len() {
                    continue;
                }
                let tile = &self.set.tiles[id];
                for ty in 0..k {
                    for tx in 0..k {
                        let src = tile.cell.color_at(ty * k + tx);
                        over(&mut buf[(r * k + ty) * w + c * k + tx], src);
                    }
                }
            }
        }
        let mut cell = Cell2d::new(Tensor::new(vec![h, w]));
        cell.cell.colors = Some(buf);
        cell
    }
    /// Returns the composited pixels as an image.
    pub fn image(&self) -> Image {
        let pixels = self.composite().cell.colors.unwrap_or_default();
        Image::from_pixels(self.width(), self.height(), &pixels)
    }
    /// Reports the raster as an image fact, with glyph facts riding along when any face does.
    pub fn fact(&self) -> Json {
        let mut out = self.image().to_json();
        let glyphs = self.glyphs();
        if !glyphs.is_empty() {
            out["glyphs"] = json!(glyphs);
        }
        out
    }
    fn glyphs(&self) -> Vec<Json> {
        let mut out = Vec::new();
        if self.set.faces.iter().all(Option::is_none) {
            return out;
        }
        let k = self.set.size;
        for r in 0..self.ids.shape[0] {
            for c in 0..self.ids.shape[1] {
                let id = self.ids.get(&[r, c]) as usize;
                let Some(Some(glyph)) = self.set.faces.get(id) else {
                    continue;
                };
                let mut g = json!({ "x": c * k, "y": r * k, "k": k, "ch": glyph.ch });
                if let Some(tint) = glyph.tint {
                    g["tint"] = json!(hex(tint));
                }
                out.push(g);
            }
        }
        out
    }
}

fn over(dst: &mut [u8; 4], src: [u8; 4]) {
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

    #[test]
    fn tiles_composite_to_pixels() {
        let red = [255, 0, 0, 255];
        let blue = [0, 0, 255, 255];
        let set = Atlas::new(2, vec![solid_tile(2, red), solid_tile(2, blue)]);
        let ids = Tensor::of(vec![0, 1, 1, 0], vec![2, 2]);
        let raster = Raster::new(ids, set, [0, 0, 0, 255]);
        assert_eq!(raster.width(), 4);
        assert_eq!(raster.height(), 4);
        let colors = raster.composite().cell.colors.unwrap();
        assert_eq!(colors[0], red);
        assert_eq!(colors[2], blue);
    }
    #[test]
    fn transparent_shows_background() {
        let set = Atlas::new(1, vec![solid_tile(1, [0, 0, 0, 0])]);
        let ids = Tensor::new(vec![1, 1]);
        let raster = Raster::new(ids, set, [9, 9, 9, 255]);
        let colors = raster.composite().cell.colors.unwrap();
        assert_eq!(colors[0], [9, 9, 9, 255]);
    }
    #[test]
    fn image_indexes_a_palette() {
        let red = [255, 0, 0, 255];
        let black = [0, 0, 0, 255];
        let set = Atlas::new(1, vec![solid_tile(1, black), solid_tile(1, red)]);
        let ids = Tensor::of(vec![0, 1, 1, 0], vec![2, 2]);
        let raster = Raster::new(ids, set, black);
        let image = raster.image();
        assert_eq!(
            image.palette,
            vec![
                mrlycore::Color::rgb(0, 0, 0),
                mrlycore::Color::rgb(255, 0, 0)
            ]
        );
        assert_eq!(image.rows, vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(raster.fact(), image.to_json());
    }
    #[test]
    fn faces_ride_the_fact_as_glyphs() {
        let clear = [0, 0, 0, 0];
        let mut set = Atlas::new(2, vec![solid_tile(2, clear), solid_tile(2, clear)]);
        set.face(1, "💣", None);
        let ids = Tensor::of(vec![0, 1], vec![1, 2]);
        let raster = Raster::new(ids, set, [0, 0, 0, 255]);
        let fact = raster.fact();
        assert_eq!(
            fact["glyphs"],
            json!([{ "x": 2, "y": 0, "k": 2, "ch": "💣" }])
        );
    }
    #[test]
    fn stray_ids_leave_the_background() {
        let set = Atlas::new(1, vec![solid_tile(1, [1, 2, 3, 255])]);
        let ids = Tensor::of(vec![9], vec![1, 1]);
        let raster = Raster::new(ids, set, [9, 9, 9, 255]);
        assert_eq!(raster.composite().cell.colors.unwrap()[0], [9, 9, 9, 255]);
    }
}
