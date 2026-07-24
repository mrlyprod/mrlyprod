use super::models::Cell6d;
use super::{Orientation, Projection, FILL, GRID, LEFT, RIGHT, UP, VOID};
use crate::three::Cell3d;
use crate::two::Cell2d;
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;

pub fn is_cube(cell: &Cell3d) -> bool {
    let s = &cell.types().shape;
    s[0] == s[1] && s[1] == s[2]
}

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

pub fn orientation(width: usize, height: usize) -> Result<Orientation> {
    if width > height {
        return Ok(Orientation::Horizontal);
    }
    if height > width {
        return Ok(Orientation::Vertical);
    }
    value_error("Cell must be a hexagon.")
}

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
    let mut types = base.types().clone();
    let y_off = (base.height() - inner.height()) / 2;
    let x_off = (base.width() - inner.width()) / 2;
    for y in 0..inner.height() {
        for x in 0..inner.width() {
            let mut v = inner.types().get(&[y, x]);
            if v == GRID {
                v = value;
            }
            types.set(&[y + y_off, x + x_off], v);
        }
    }
    Ok(Cell6d::new(
        Cell2d::new(types),
        cell.projection,
        orient,
        cell.start,
    ))
}

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
    let mut types = Tensor::full(vec![final_h, final_w], GRID);
    for &(px, py) in &positions {
        let (dest_x, dest_y) = (px - min_x, py - min_y);
        for y in 0..tile_h {
            for x in 0..tile_w {
                let v = inner.types().get(&[y, x]);
                if v != GRID {
                    types.set(&[dest_y + y, dest_x + x], v);
                }
            }
        }
    }
    Ok(Cell2d::new(types))
}

pub fn tile(cell: &Cell6d, width: usize, height: usize) -> Result<Cell2d> {
    tessellate(cell, &Tensor::full(vec![height, width], 1))
}

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
    let mut types = Tensor::new(vec![new_h, new_w]);
    for y in 0..new_h {
        for x in 0..new_w {
            types.set(&[y, x], cell.types().get(&[y + crop_y, x + crop_x]));
        }
    }
    Ok(Cell2d::new(types))
}

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

pub fn radial(cell: &Cell6d, radius: usize) -> Result<Cell2d> {
    let inner = &cell.cell;
    if !is_hex(inner) {
        return value_error("Cell must be a hexagon.");
    }
    let orient = orientation(inner.width(), inner.height())?;
    tessellate(cell, &radial_mask(radius, orient))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::three;
    #[test]
    fn blank_matches_python() {
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
    fn radial_mask_matches_python() {
        let m = radial_mask(2, Orientation::Horizontal);
        assert_eq!(m.bytes(), vec![0, 1, 0, 1, 1, 1, 1, 1, 1]);
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
