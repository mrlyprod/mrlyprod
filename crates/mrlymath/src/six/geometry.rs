use super::models::Cell6d;
use super::{Orientation, Projection, FILL, GRID, LEFT, RIGHT, UP, VOID};
use crate::three::Cell3d;
use crate::two::Cell2d;
use mrlycore::cell::{remap, Cell};
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;

fn backed(front: &Cell2d, back: &Cell2d, tag: u8) -> Cell {
    let (count, spare) = (front.cell.size(), back.cell.size());
    let mut types = Tensor::typed(vec![count + spare], front.types().dtype());
    for i in 0..count {
        types.put(i, front.types().at(i));
    }
    for i in 0..spare {
        types.put(count + i, back.types().at(i));
    }
    Cell {
        types,
        colors: front.cell.colors.as_ref().map(|colors| {
            let mut out = colors.clone();
            out.resize(count + spare, [0u8; 4]);
            out
        }),
        tags: front.cell.tags.as_ref().map(|tags| {
            let mut out = Tensor::filled(vec![count + spare], tag as i64, tags.dtype());
            for i in 0..count {
                out.put(i, tags.at(i));
            }
            out
        }),
    }
}

/// Returns whether the cell's three sides are equal.
pub fn is_cube(cell: &Cell3d) -> bool {
    let s = &cell.types().shape;
    s[0] == s[1] && s[1] == s[2]
}

/// Returns whether the cell's width, height and parity frame a hexagon.
pub fn is_hex(cell: &Cell2d) -> bool {
    let (h, w) = (cell.height(), cell.width());
    if w > h {
        if w.is_multiple_of(2) {
            return false;
        }
        let dx = (3 * (w + 1)) / 4;
        let row_shift = h / 2;
        (dx + row_shift).is_multiple_of(2)
    } else if h > w {
        let dy = (3 * (h + 1)) / 4;
        let row_shift = w / 2;
        (dy + row_shift).is_multiple_of(2)
    } else {
        false
    }
}

/// Returns the orientation a hexagon's width and height imply, or an error when they are equal.
pub fn orientation(width: usize, height: usize) -> Result<Orientation> {
    if width > height {
        return Ok(Orientation::Horizontal);
    }
    if height > width {
        return Ok(Orientation::Vertical);
    }
    value_error("Cell must be a hexagon.")
}

/// Builds a hexagon of the given radius, fill inside and void outside.
///
/// ```
/// use mrlymath::six::{blank, Orientation};
/// let hex = blank(2, Orientation::Horizontal, 1, 0);
/// assert_eq!(hex.types().shape, vec![4, 7]);
/// ```
pub fn blank(radius: usize, orient: Orientation, fill: u8, void: u8) -> Cell2d {
    let n = radius;
    let (height, width) = match orient {
        Orientation::Horizontal => (2 * n, 4 * n - 1),
        Orientation::Vertical => {
            let width = 2 * n;
            let mut height = (7 * n - 1) / 2;
            let row_shift = width / 2;
            while !((3 * (height + 1)) / 4 + row_shift).is_multiple_of(2) {
                height += 1;
            }
            (height, width)
        }
    };
    let mut types = Tensor::full(vec![height, width], fill);
    for r in 0..height {
        let p = match orient {
            Orientation::Horizontal => {
                [0isize, n as isize - 1 - r as isize, r as isize - n as isize]
            }
            Orientation::Vertical => [
                0isize,
                n as isize - 1 - r as isize,
                r as isize - (height - n) as isize,
            ],
        }
        .into_iter()
        .max()
        .unwrap() as usize;
        if p > 0 {
            for c in 0..p {
                types.set(&[r, c], void);
                types.set(&[r, width - 1 - c], void);
            }
        }
    }
    Cell2d::new(types)
}

/// Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along.
pub fn pad(cell: &Cell6d, k: usize, value: u8) -> Result<Cell6d> {
    if k < 1 {
        return Ok(cell.clone());
    }
    let inner = &cell.cell;
    if !is_hex(inner) {
        return value_error("Cell must be a hexagon.");
    }
    let orient = orientation(inner.width(), inner.height())?;
    let n = match orient {
        Orientation::Horizontal => inner.height() / 2,
        Orientation::Vertical => inner.width() / 2,
    };
    let base = blank(n + k, orient, value, GRID);
    let (base_h, base_w) = (base.height(), base.width());
    let (tile_h, tile_w) = (inner.height(), inner.width());
    let y_off = (base_h - tile_h) / 2;
    let x_off = (base_w - tile_w) / 2;
    let mut front = inner.clone();
    for v in front.cell.types.bytes_mut().iter_mut() {
        if *v == GRID {
            *v = value;
        }
    }
    let count = front.cell.size();
    let map: Vec<usize> = (0..base_h * base_w)
        .map(|flat| {
            let (y, x) = (flat / base_w, flat % base_w);
            let inside = y >= y_off && y < y_off + tile_h && x >= x_off && x < x_off + tile_w;
            match inside {
                true => (y - y_off) * tile_w + x - x_off,
                false => count + flat,
            }
        })
        .collect();
    Ok(Cell6d::new(
        Cell2d {
            cell: remap(&backed(&front, &base, value), &map, &[base_h, base_w]),
        },
        cell.projection,
        orient,
        cell.start,
    ))
}

/// Projects a cube into the isometric hexagon of top, left and right faces.
pub fn iso(cell: &Cell3d) -> Result<Cell6d> {
    if !is_cube(cell) {
        return value_error("Cell must be a cube.");
    }
    let grid = cell.types();
    let n = grid.shape[0];
    let width = 2 * n;
    let height = 4 * n - 1;
    let mut types = Tensor::full(vec![height, width], GRID);
    for z in 0..n {
        for y in 0..n {
            for x in 0..n {
                if grid.get(&[x, y, z]) == 0 {
                    continue;
                }
                let gx = x as isize - y as isize + (n as isize - 1);
                let gy = x as isize + y as isize - 2 * z as isize + (2 * n as isize - 2);
                if gx >= 0 && gx < width as isize - 1 && gy >= 0 && gy < height as isize - 2 {
                    let (gx, gy) = (gx as usize, gy as usize);
                    types.set(&[gy, gx], UP);
                    types.set(&[gy, gx + 1], UP);
                    types.set(&[gy + 1, gx], LEFT);
                    types.set(&[gy + 1, gx + 1], RIGHT);
                    types.set(&[gy + 2, gx], LEFT);
                    types.set(&[gy + 2, gx + 1], RIGHT);
                }
            }
        }
    }
    Ok(Cell6d::new(
        Cell2d::new(types),
        Projection::Iso,
        Orientation::Vertical,
        1,
    ))
}

/// Projects a cube's three facing sides into a hexagon of fills and voids.
pub fn pro(cell: &Cell3d) -> Result<Cell6d> {
    if !is_cube(cell) {
        return value_error("Cell must be a cube.");
    }
    let grid = cell.types();
    let n = grid.shape[0];
    let width = 2 * n;
    let height = 4 * n - 1;
    let mut types = Tensor::full(vec![height, width], GRID);
    let place = |x: usize, y: usize, z: usize, face: u8, types: &mut Tensor| {
        let val = if grid.get(&[x, y, z]) == 1 {
            FILL
        } else {
            VOID
        };
        let gx = x as isize - y as isize + (n as isize - 1);
        let gy = x as isize + y as isize - 2 * z as isize + (2 * n as isize - 2);
        if gx >= 0 && gx < width as isize - 1 && gy >= 0 && gy < height as isize - 2 {
            let (gx, gy) = (gx as usize, gy as usize);
            match face {
                0 => {
                    types.set(&[gy + 1, gx], val);
                    types.set(&[gy + 2, gx], val);
                }
                1 => {
                    types.set(&[gy + 1, gx + 1], val);
                    types.set(&[gy + 2, gx + 1], val);
                }
                _ => {
                    types.set(&[gy, gx], val);
                    types.set(&[gy, gx + 1], val);
                }
            }
        }
    };
    let y = n - 1;
    for z in 0..n {
        for x in 0..n {
            place(x, y, z, 0, &mut types);
        }
    }
    let x = n - 1;
    for z in 0..n {
        for y in 0..n {
            place(x, y, z, 1, &mut types);
        }
    }
    let z = n - 1;
    for y in 0..n {
        for x in 0..n {
            place(x, y, z, 2, &mut types);
        }
    }
    Ok(Cell6d::new(
        Cell2d::new(types),
        Projection::Pro,
        Orientation::Vertical,
        1,
    ))
}

/// Slices a cube through its center across the main diagonal into a hexagon.
pub fn cut(cell: &Cell3d) -> Result<Cell6d> {
    if !is_cube(cell) {
        return value_error("Cell must be a cube.");
    }
    let scale = 4usize;
    let block = Tensor::full(vec![scale, scale, scale], 1);
    let grid = cell.types().kron(&block);
    let size = grid.shape[0];
    let k = (3 * (size - 1)) / 2;
    let mut rows: Vec<Vec<u8>> = Vec::new();
    for z in (0..size).step_by(2) {
        let target = k - z;
        let min_x = target.saturating_sub(size - 1);
        let max_x = (size - 1).min(target);
        if min_x > max_x {
            continue;
        }
        let mut row = Vec::new();
        for x in min_x..=max_x {
            let y = target - x;
            row.push(grid.get(&[x, y, z]));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Ok(Cell6d::new(
            Cell2d::new(Tensor::new(vec![1, 1])),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        ));
    }
    let width = rows.iter().map(|r| r.len()).max().unwrap();
    let height = rows.len();
    let mut types = Tensor::full(vec![height, width], GRID);
    for (r, row) in rows.iter().enumerate() {
        let offset = (width - row.len()) / 2;
        for (c, &v) in row.iter().enumerate() {
            types.set(&[r, c + offset], if v == 1 { FILL } else { VOID });
        }
    }
    Ok(Cell6d::new(
        Cell2d::new(types),
        Projection::Cut,
        Orientation::Horizontal,
        0,
    ))
}

/// Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included.
pub fn tessellate(cell: &Cell6d, mask: &Tensor) -> Result<Cell2d> {
    let inner = &cell.cell;
    if !is_hex(inner) {
        return value_error("Cell must be a hexagon.");
    }
    let orient = orientation(inner.width(), inner.height())?;
    let (tile_h, tile_w) = (inner.height(), inner.width());
    let (dx, dy, row_shift) = match orient {
        Orientation::Horizontal => ((3 * (tile_w + 1)) / 4, tile_h, tile_h / 2),
        Orientation::Vertical => (tile_w, (3 * (tile_h + 1)) / 4, tile_w / 2),
    };
    let mut positions = Vec::new();
    for r in 0..mask.shape[0] {
        for c in 0..mask.shape[1] {
            if mask.get(&[r, c]) == 0 {
                continue;
            }
            let (mut px, mut py) = (c * dx, r * dy);
            match orient {
                Orientation::Horizontal => {
                    if !c.is_multiple_of(2) {
                        py += row_shift;
                    }
                }
                Orientation::Vertical => {
                    if !r.is_multiple_of(2) {
                        px += row_shift;
                    }
                }
            }
            positions.push((px, py));
        }
    }
    if positions.is_empty() {
        return Ok(Cell2d::new(Tensor::new(vec![1, 1])));
    }
    let min_x = positions.iter().map(|p| p.0).min().unwrap();
    let min_y = positions.iter().map(|p| p.1).min().unwrap();
    let max_x = positions.iter().map(|p| p.0 + tile_w).max().unwrap();
    let max_y = positions.iter().map(|p| p.1 + tile_h).max().unwrap();
    let (final_w, final_h) = (max_x - min_x, max_y - min_y);
    let count = inner.cell.size();
    let mut map = vec![count; final_h * final_w];
    for &(px, py) in &positions {
        let (dest_x, dest_y) = (px - min_x, py - min_y);
        for y in 0..tile_h {
            for x in 0..tile_w {
                if inner.types().get(&[y, x]) != GRID {
                    map[(dest_y + y) * final_w + dest_x + x] = y * tile_w + x;
                }
            }
        }
    }
    let back = Cell2d::new(Tensor::full(vec![1, 1], GRID));
    Ok(Cell2d {
        cell: remap(&backed(inner, &back, 0), &map, &[final_h, final_w]),
    })
}

/// Tessellates a hexagonal cell over a full width-by-height mask.
pub fn tile(cell: &Cell6d, width: usize, height: usize) -> Result<Cell2d> {
    tessellate(cell, &Tensor::full(vec![height, width], 1))
}

/// Crops one interlocking step off each side of a sheet tiled at the given size.
pub fn tile_crop(cell: &Cell2d, size: (usize, usize)) -> Result<Cell2d> {
    let (w, h) = size;
    let orient = orientation(w, h)?;
    let (crop_x, crop_y) = match orient {
        Orientation::Horizontal => ((w - 1) / 4, h / 2),
        Orientation::Vertical => (w / 2, (h - 1) / 4),
    };
    crop(cell, crop_x, crop_y)
}

fn crop(cell: &Cell2d, crop_x: usize, crop_y: usize) -> Result<Cell2d> {
    let (current_h, current_w) = (cell.height(), cell.width());
    if crop_y * 2 >= current_h || crop_x * 2 >= current_w {
        return Ok(Cell2d::new(Tensor::new(vec![1, 1])));
    }
    let (new_h, new_w) = (current_h - 2 * crop_y, current_w - 2 * crop_x);
    let map: Vec<usize> = (0..new_h * new_w)
        .map(|flat| (flat / new_w + crop_y) * current_w + flat % new_w + crop_x)
        .collect();
    Ok(Cell2d {
        cell: remap(&cell.cell, &map, &[new_h, new_w]),
    })
}

/// Builds the disc mask of cells within hex distance radius of the center.
pub fn radial_mask(radius: usize, orient: Orientation) -> Tensor {
    if radius < 1 {
        return Tensor::new(vec![1, 1]);
    }
    let size = 2 * radius - 1;
    let center = radius - 1;
    let mut mask = Tensor::new(vec![size, size]);
    let (c_q, c_r) = match orient {
        Orientation::Horizontal => (
            center as isize,
            center as isize - ((center - (center & 1)) / 2) as isize,
        ),
        Orientation::Vertical => (
            center as isize - ((center - (center & 1)) / 2) as isize,
            center as isize,
        ),
    };
    for r in 0..size {
        for c in 0..size {
            let (q, r_axial) = match orient {
                Orientation::Horizontal => (c as isize, r as isize - ((c - (c & 1)) / 2) as isize),
                Orientation::Vertical => (c as isize - ((r - (r & 1)) / 2) as isize, r as isize),
            };
            let dq = q - c_q;
            let dr = r_axial - c_r;
            if (dq.abs() + dr.abs() + (dq + dr).abs()) / 2 < radius as isize {
                mask.set(&[r, c], 1);
            }
        }
    }
    mask
}

/// Tessellates a hexagonal cell over the disc mask of the given radius.
pub fn radial(cell: &Cell6d, radius: usize) -> Result<Cell2d> {
    let inner = &cell.cell;
    if !is_hex(inner) {
        return value_error("Cell must be a hexagon.");
    }
    let orient = orientation(inner.width(), inner.height())?;
    tessellate(cell, &radial_mask(radius, orient))
}

/// Crops the interlocking overhang off a disc tiled at the given radius and tile size.
pub fn radial_crop(cell: &Cell2d, radius: usize, size: (usize, usize)) -> Result<Cell2d> {
    let (w, h) = size;
    let orient = orientation(w, h)?;
    let rings = radius.saturating_sub(1);
    let (crop_x, crop_y) = match orient {
        Orientation::Horizontal => (h / 2, rings * (h / 2)),
        Orientation::Vertical => (rings * (w / 2), w / 2),
    };
    crop(cell, crop_x, crop_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::three;
    #[test]
    fn blank_frames_both_orientations() {
        let b = blank(2, Orientation::Horizontal, 1, 0);
        assert_eq!(b.types().shape, vec![4, 7]);
        assert_eq!(
            b.types().bytes(),
            vec![
                0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0
            ]
        );
        let v = blank(2, Orientation::Vertical, 1, 0);
        assert_eq!(v.types().shape, vec![7, 4]);
        assert!(is_hex(&b));
        assert!(is_hex(&v));
    }
    #[test]
    fn radial_mask_is_the_hex_disc() {
        let m = radial_mask(2, Orientation::Horizontal);
        assert_eq!(m.bytes(), vec![0, 1, 0, 1, 1, 1, 1, 1, 1]);
    }
    #[test]
    fn radial_crop_trims_the_overhang() {
        let hex = Cell6d::new(
            blank(2, Orientation::Horizontal, FILL, GRID),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        );
        let (w, h) = (hex.width(), hex.height());
        let disc = radial(&hex, 2).unwrap();
        let cropped = radial_crop(&disc, 2, (w, h)).unwrap();
        assert_eq!(cropped.height(), disc.height() - h);
        assert_eq!(cropped.width(), disc.width() - h);
        let tight = radial_crop(&disc, 9, (w, h)).unwrap();
        assert_eq!(tight.types().shape, vec![1, 1]);
    }
    #[test]
    fn radial_crop_shrinks_the_two_axes_apart() {
        let radius = 3;
        let rings = radius - 1;
        for orient in [Orientation::Horizontal, Orientation::Vertical] {
            let hex = Cell6d::new(blank(2, orient, FILL, GRID), Projection::Cut, orient, 0);
            let (w, h) = (hex.width(), hex.height());
            let disc = radial(&hex, radius).unwrap();
            let cropped = radial_crop(&disc, radius, (w, h)).unwrap();
            let (lost_x, lost_y) = match orient {
                Orientation::Horizontal => (h, rings * h),
                Orientation::Vertical => (rings * w, w),
            };
            assert_ne!(lost_x, lost_y, "{orient:?}");
            assert_eq!(cropped.width(), disc.width() - lost_x, "{orient:?}");
            assert_eq!(cropped.height(), disc.height() - lost_y, "{orient:?}");
        }
    }
    #[test]
    fn tessellate_and_crop_carry_colors_and_tags() {
        let painted = crate::six::paint(
            Cell6d::new(
                blank(2, Orientation::Horizontal, FILL, GRID),
                Projection::Cut,
                Orientation::Horizontal,
                0,
            ),
            None,
            None,
        );
        assert!(painted.cell.cell.colors.is_some());
        let sheet = tile(&painted, 2, 2).unwrap();
        let colors = sheet.cell.colors.as_ref().unwrap();
        assert_eq!(colors.len(), sheet.width() * sheet.height());
        let opaque = colors.iter().filter(|c| c[3] > 0).count();
        assert_eq!(
            opaque,
            sheet.types().bytes().iter().filter(|&&v| v != GRID).count()
        );
        let cropped = tile_crop(&sheet, (painted.width(), painted.height())).unwrap();
        assert_eq!(
            cropped.cell.colors.as_ref().unwrap().len(),
            cropped.width() * cropped.height()
        );
    }
    #[test]
    fn pad_carries_colors_across_the_ring() {
        let painted = crate::six::paint(
            Cell6d::new(
                blank(2, Orientation::Horizontal, FILL, VOID),
                Projection::Cut,
                Orientation::Horizontal,
                0,
            ),
            None,
            None,
        );
        let source = painted.cell.cell.colors.clone().unwrap();
        let wider = pad(&painted, 1, GRID).unwrap();
        let grown = wider.cell.cell.colors.as_ref().unwrap();
        let y_off = (wider.height() - painted.height()) / 2;
        let x_off = (wider.width() - painted.width()) / 2;
        for y in 0..painted.height() {
            for x in 0..painted.width() {
                assert_eq!(
                    grown[(y + y_off) * wider.width() + x + x_off],
                    source[y * painted.width() + x]
                );
            }
        }
        assert_eq!(grown[0], [0, 0, 0, 0]);
    }
    #[test]
    fn projections_have_expected_frames() {
        let c = three::carpet(3, 1).unwrap();
        let i = iso(&c).unwrap();
        assert_eq!(i.cell.types().shape, vec![11, 6]);
        assert_eq!(i.start, 1);
        let p = pro(&c).unwrap();
        assert_eq!(p.cell.types().shape, vec![11, 6]);
        let q = cut(&c).unwrap();
        assert_eq!(q.orientation, Orientation::Horizontal);
        assert_eq!(q.start, 0);
        assert!(iso(&three::Cell3d::new(Tensor::new(vec![2, 3, 2]))).is_err());
    }
}
