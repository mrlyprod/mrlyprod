use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;
use mrlymath::dim::graph::{core_graph, tunnel_graph};
use mrlymath::life::{churn, entropy};
use mrlymath::two::{census, Cell2d};
use mrlynum::fft::{peak_ring, radial_profile};
use mrlynum::graph::census::components;
use std::f64::consts::PI;

/// The dihedral subgroup fixing a frame, by order and name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symmetry {
    /// The number of square symmetries fixing the frame, 1 to 8.
    pub order: usize,
    /// The subgroup's name: d4, c4, d2, d2d, c2, m, md or 1.
    pub name: &'static str,
}

/// The readings of one frame.
#[derive(Clone, Debug, PartialEq)]
pub struct Reading {
    /// The frame's side.
    pub side: usize,
    /// The number of filled cells.
    pub fill: usize,
    /// The 4-adjacent components of the filled cells on the plane.
    pub components: usize,
    /// The 4-adjacent components of the empty cells on the plane, the background included.
    pub holes: usize,
    /// The Euler characteristic of the filled cells.
    pub euler: i64,
    /// The least-squares slope of the dyadic box counts against the scale.
    pub box_slope: f64,
    /// The dihedral subgroup fixing the frame.
    pub symmetry: Symmetry,
    /// The ring of the strongest non-zero frequency read over the full Fourier square.
    pub ring: usize,
    /// The share of the non-zero power sitting on that ring.
    pub share: f64,
    /// The peak ring the ring cut reads, the corners dropped.
    pub ring_cut: usize,
    /// The frame's binary entropy in millibits.
    pub entropy: i64,
    /// The fraction of cells changed since the previous frame, zero without one.
    pub churn: f64,
    /// Every proper divisor of the side the block test cuts at.
    pub cuts: Vec<usize>,
    /// The smallest cut whose two factors pack into codes, as (d, outer, inner).
    pub factors: Option<(usize, u128, u128)>,
}

/// Reads one frame, the churn taken against the previous frame when given.
pub fn read(frame: &Cell2d, previous: Option<&Cell2d>) -> Result<Reading> {
    let grid = frame.types();
    let side = grid.shape[0];
    let fill = grid.sum() as usize;
    let pieces = if fill == 0 {
        0
    } else {
        components(&core_graph(frame)?)
    };
    let holes = if fill == side * side {
        0
    } else {
        components(&tunnel_graph(frame)?)
    };
    let euler = census::euler(frame)?;
    let (ring, share, ring_cut) = spectrum_peaks(grid);
    let churn = match previous {
        Some(prev) => churn(&[prev.clone(), frame.clone()]),
        None => 0.0,
    };
    Ok(Reading {
        side,
        fill,
        components: pieces,
        holes,
        euler,
        box_slope: box_slope(grid),
        symmetry: symmetry(grid),
        ring,
        share,
        ring_cut,
        entropy: entropy(frame),
        churn,
        cuts: cuts(grid),
        factors: factors(grid),
    })
}

/// Splits a square frame at a divisor d of its side: every non-zero d-block must be one tile, then the frame is outer (x) inner.
pub fn block_split(frame: &Tensor, d: usize) -> Option<(Tensor, Tensor)> {
    let side = frame.shape[0];
    if d == 0 || !side.is_multiple_of(d) {
        return None;
    }
    let n = side / d;
    let mut outer = Tensor::new(vec![d, d]);
    let mut inner: Option<Tensor> = None;
    for i in 0..d {
        for j in 0..d {
            let mut block = Tensor::new(vec![n, n]);
            let mut live = false;
            for p in 0..n {
                for q in 0..n {
                    if frame.get(&[i * n + p, j * n + q]) != 0 {
                        block.set(&[p, q], 1);
                        live = true;
                    }
                }
            }
            if !live {
                continue;
            }
            outer.set(&[i, j], 1);
            match &inner {
                None => inner = Some(block),
                Some(first) => {
                    if first.bytes() != block.bytes() {
                        return None;
                    }
                }
            }
        }
    }
    inner.map(|block| (outer, block))
}

/// Returns every proper divisor of the side at which the frame splits.
pub fn cuts(frame: &Tensor) -> Vec<usize> {
    let side = frame.shape[0];
    (2..side)
        .filter(|&d| side.is_multiple_of(d) && block_split(frame, d).is_some())
        .collect()
}

/// Packs a 0/1 tile into its row-major code, None past 128 cells.
pub fn pack(tile: &Tensor) -> Option<u128> {
    if tile.size() > 128 {
        return None;
    }
    let mut code: u128 = 0;
    for (i, &b) in tile.bytes().iter().enumerate() {
        if b != 0 {
            code |= 1 << i;
        }
    }
    Some(code)
}

/// Returns the smallest cut whose outer and inner factors both pack, as (d, outer code, inner code).
pub fn factors(frame: &Tensor) -> Option<(usize, u128, u128)> {
    for d in cuts(frame) {
        if let Some((outer, inner)) = block_split(frame, d) {
            if let (Some(a), Some(b)) = (pack(&outer), pack(&inner)) {
                return Some((d, a, b));
            }
        }
    }
    None
}

/// Returns the first torus shift and cut at which the shifted frame factors, as (dr, dc, d, outer, inner).
pub fn shifted_factors(grid: &Tensor) -> Option<(usize, usize, usize, u128, u128)> {
    let n = grid.shape[0];
    if grid.sum() == 0 {
        return None;
    }
    let bytes = grid.bytes();
    let mut shifted = Tensor::new(vec![n, n]);
    for dr in 0..n {
        for dc in 0..n {
            for r in 0..n {
                for c in 0..n {
                    let v = bytes[((r + dr) % n) * n + (c + dc) % n];
                    shifted.bytes_mut()[r * n + c] = v;
                }
            }
            if let Some((d, a, b)) = factors(&shifted) {
                return Some((dr, dc, d, a, b));
            }
        }
    }
    None
}

fn images(grid: &Tensor) -> Vec<Tensor> {
    let mut out = Vec::with_capacity(8);
    for k in 0..4 {
        let turned = grid.rot90(k, (0, 1));
        out.push(turned.flip(1));
        out.push(turned);
    }
    out
}

/// Returns the dihedral subgroup fixing the frame.
pub fn symmetry(grid: &Tensor) -> Symmetry {
    let same = |t: &Tensor| t.bytes() == grid.bytes();
    let r90 = same(&grid.rot90(1, (0, 1)));
    let r180 = same(&grid.rot90(2, (0, 1)));
    let fh = same(&grid.flip(1));
    let fv = same(&grid.flip(0));
    let t = same(&grid.transpose(0, 1));
    let at = same(&grid.rot90(2, (0, 1)).transpose(0, 1));
    let order = 1 + [r90, r180, r90, fh, fv, t, at]
        .iter()
        .filter(|&&b| b)
        .count();
    let name = match (order, r90, fh, t, r180) {
        (8, ..) => "d4",
        (4, true, ..) => "c4",
        (4, _, true, ..) => "d2",
        (4, ..) => "d2d",
        (2, _, _, _, true) => "c2",
        (2, ..) if fh || fv => "m",
        (2, ..) => "md",
        _ => "1",
    };
    Symmetry { order, name }
}

/// Returns the least-squares slope of ln(box count) against ln(side over box) over dyadic boxes, zero on an empty frame.
pub fn box_slope(grid: &Tensor) -> f64 {
    let side = grid.shape[0];
    if grid.sum() == 0 || side < 2 {
        return 0.0;
    }
    let mut points = Vec::new();
    let mut b = 1;
    while b < side {
        let m = side.div_ceil(b);
        let mut boxes = vec![false; m * m];
        for r in 0..side {
            for c in 0..side {
                if grid.get(&[r, c]) != 0 {
                    boxes[(r / b) * m + c / b] = true;
                }
            }
        }
        let count = boxes.iter().filter(|&&x| x).count();
        points.push(((side as f64 / b as f64).ln(), (count as f64).ln()));
        b *= 2;
    }
    let n = points.len() as f64;
    let (sx, sy): (f64, f64) = points
        .iter()
        .fold((0.0, 0.0), |(a, b), (x, y)| (a + x, b + y));
    let (sxx, sxy): (f64, f64) = points
        .iter()
        .fold((0.0, 0.0), |(a, b), (x, y)| (a + x * x, b + x * y));
    let denominator = n * sxx - sx * sx;
    if denominator.abs() < 1e-12 {
        return 0.0;
    }
    (n * sxy - sx * sy) / denominator
}

/// Transforms a real n-square field exactly, rows then columns, returning the real and imaginary parts.
pub fn dft2(field: &[f64], n: usize) -> (Vec<f64>, Vec<f64>) {
    let cos: Vec<f64> = (0..n)
        .map(|k| (2.0 * PI * k as f64 / n as f64).cos())
        .collect();
    let sin: Vec<f64> = (0..n)
        .map(|k| (2.0 * PI * k as f64 / n as f64).sin())
        .collect();
    let mut re = vec![0.0; n * n];
    let mut im = vec![0.0; n * n];
    for r in 0..n {
        for kc in 0..n {
            let (mut a, mut b) = (0.0, 0.0);
            for c in 0..n {
                let v = field[r * n + c];
                if v != 0.0 {
                    let phase = (kc * c) % n;
                    a += v * cos[phase];
                    b -= v * sin[phase];
                }
            }
            re[r * n + kc] = a;
            im[r * n + kc] = b;
        }
    }
    let mut out_re = vec![0.0; n * n];
    let mut out_im = vec![0.0; n * n];
    for kc in 0..n {
        for kr in 0..n {
            let (mut a, mut b) = (0.0, 0.0);
            for r in 0..n {
                let phase = (kr * r) % n;
                let (x, y) = (re[r * n + kc], im[r * n + kc]);
                a += x * cos[phase] + y * sin[phase];
                b += y * cos[phase] - x * sin[phase];
            }
            out_re[kr * n + kc] = a;
            out_im[kr * n + kc] = b;
        }
    }
    (out_re, out_im)
}

fn centred(values: &[f64], n: usize) -> Vec<f64> {
    let half = n / 2;
    let mut out = vec![0.0; n * n];
    for r in 0..n {
        for c in 0..n {
            out[((r + half) % n) * n + (c + half) % n] = values[r * n + c];
        }
    }
    out
}

/// Returns the centred power square of a frame on its own torus.
pub fn power_square(grid: &Tensor) -> Vec<f64> {
    let n = grid.shape[0];
    let field: Vec<f64> = grid.bytes().iter().map(|&b| b as f64).collect();
    let (re, im) = dft2(&field, n);
    let power: Vec<f64> = re.iter().zip(&im).map(|(a, b)| a * a + b * b).collect();
    centred(&power, n)
}

fn spectrum_peaks(grid: &Tensor) -> (usize, f64, usize) {
    let n = grid.shape[0];
    let power = power_square(grid);
    let half = n / 2;
    let ring_of = |r: usize, c: usize| {
        let dr = r as f64 - half as f64;
        let dc = c as f64 - half as f64;
        dr.hypot(dc).round() as usize
    };
    let floor = 1e-9 * power[half * n + half];
    let mut best = 0.0;
    let mut ring = 0;
    let mut total = 0.0;
    for r in 0..n {
        for c in 0..n {
            if r == half && c == half {
                continue;
            }
            let p = power[r * n + c];
            if p <= floor {
                continue;
            }
            total += p;
            if p > best {
                best = p;
                ring = ring_of(r, c);
            }
        }
    }
    if ring == 0 || total <= 0.0 {
        return (0, 0.0, 0);
    }
    let mut on_ring = 0.0;
    for r in 0..n {
        for c in 0..n {
            if !(r == half && c == half) && ring_of(r, c) == ring && power[r * n + c] > floor {
                on_ring += power[r * n + c];
            }
        }
    }
    let cut = peak_ring(&radial_profile(&power, n));
    (ring, on_ring / total, cut)
}

/// Returns the first ring where the ring mean of a mask's signed transform on the canvas torus turns negative, zero when it never does.
pub fn first_negative_lobe(mask: &Tensor, canvas: usize) -> usize {
    let side = mask.shape[0];
    let centre = (side - 1) / 2;
    let mut field = vec![0.0; canvas * canvas];
    for r in 0..side {
        for c in 0..side {
            if mask.get(&[r, c]) != 0 {
                let rr = (r + canvas - centre % canvas) % canvas;
                let cc = (c + canvas - centre % canvas) % canvas;
                field[rr * canvas + cc] += 1.0;
            }
        }
    }
    let (re, _) = dft2(&field, canvas);
    let profile = radial_profile(&centred(&re, canvas), canvas);
    profile
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, &v)| v < 0.0)
        .map(|(k, _)| k)
        .unwrap_or(0)
}

/// Returns the frame's canonical bytes under the dihedral group and torus translation.
pub fn canonical(grid: &Tensor) -> Vec<u8> {
    let n = grid.shape[0];
    if grid.sum() == 0 {
        return grid.bytes().to_vec();
    }
    let mut best: Option<Vec<u8>> = None;
    let mut shifted = vec![0u8; n * n];
    for image in images(grid) {
        let bytes = image.bytes();
        for r0 in 0..n {
            for c0 in 0..n {
                if bytes[r0 * n + c0] == 0 {
                    continue;
                }
                for r in 0..n {
                    let rr = (r + n - r0) % n;
                    for c in 0..n {
                        shifted[rr * n + (c + n - c0) % n] = bytes[r * n + c];
                    }
                }
                if best.as_ref().is_none_or(|b| shifted < *b) {
                    best = Some(shifted.clone());
                }
            }
        }
    }
    best.unwrap_or_default()
}

/// Returns the torus shift carrying frame a onto frame b, when one exists.
pub fn translate_of(a: &Tensor, b: &Tensor) -> Option<(usize, usize)> {
    let n = a.shape[0];
    if a.shape != b.shape || a.sum() != b.sum() {
        return None;
    }
    let (x, y) = (a.bytes(), b.bytes());
    let first = x.iter().position(|&v| v != 0)?;
    let (r0, c0) = (first / n, first % n);
    for r1 in 0..n {
        for c1 in 0..n {
            if y[r1 * n + c1] == 0 {
                continue;
            }
            let (dr, dc) = ((r1 + n - r0) % n, (c1 + n - c0) % n);
            let fits = (0..n * n).all(|i| {
                let (r, c) = (i / n, i % n);
                x[i] == y[((r + dr) % n) * n + (c + dc) % n]
            });
            if fits {
                return Some((dr, dc));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlymath::two::carpet;
    fn blinker() -> Cell2d {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1);
        t.set(&[2, 2], 1);
        t.set(&[3, 2], 1);
        Cell2d::new(t)
    }
    #[test]
    fn the_blinker_reads_one_bar_of_d2_with_no_cut() {
        let reading = read(&blinker(), None).unwrap();
        assert_eq!(
            (
                reading.fill,
                reading.components,
                reading.holes,
                reading.euler
            ),
            (3, 1, 1, 1)
        );
        assert_eq!(
            reading.symmetry,
            Symmetry {
                order: 4,
                name: "d2"
            }
        );
        assert!(reading.cuts.is_empty());
        assert_eq!(reading.factors, None);
        assert!(reading.ring > 0 && reading.share > 0.0);
        assert_eq!(reading.entropy, entropy(&blinker()));
        let turned = Cell2d::new(blinker().types().rot90(1, (0, 1)));
        assert_eq!(canonical(blinker().types()), canonical(turned.types()));
        assert_eq!(read(&turned, Some(&blinker())).unwrap().churn, 4.0 / 25.0);
    }
    #[test]
    fn the_full_block_is_d4_with_a_flat_spectrum_and_cuts_at_two() {
        let block = Cell2d::new(Tensor::full(vec![4, 4], 1));
        let reading = read(&block, None).unwrap();
        assert_eq!(
            (
                reading.fill,
                reading.components,
                reading.holes,
                reading.euler
            ),
            (16, 1, 0, 1)
        );
        assert_eq!(
            reading.symmetry,
            Symmetry {
                order: 8,
                name: "d4"
            }
        );
        assert_eq!((reading.ring, reading.share, reading.ring_cut), (0, 0.0, 0));
        assert_eq!(reading.entropy, 0);
        assert_eq!(reading.cuts, vec![2]);
        assert_eq!(reading.factors, Some((2, 0b1111, 0b1111)));
        assert!((reading.box_slope - 2.0).abs() < 1e-9);
    }
    #[test]
    fn the_level_two_carpet_factors_at_three_into_two_carpet_tiles() {
        let frame = carpet(3, 2).unwrap();
        let reading = read(&frame, None).unwrap();
        assert_eq!(
            (
                reading.fill,
                reading.components,
                reading.holes,
                reading.euler
            ),
            (64, 1, 9, -8)
        );
        assert_eq!(reading.symmetry.order, 8);
        assert_eq!(reading.cuts, vec![3]);
        assert_eq!(reading.factors, Some((3, 495, 495)));
        let (outer, inner) = block_split(frame.types(), 3).unwrap();
        assert_eq!(outer.kron(&inner).bytes(), frame.types().bytes());
        assert_eq!(
            block_split(&frame.types().rot90(1, (0, 1)), 3).map(|(a, _)| pack(&a)),
            Some(Some(495))
        );
    }
    #[test]
    fn a_shifted_frame_is_found_and_read_back() {
        let a = blinker();
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[4, 0], 1);
        t.set(&[0, 0], 1);
        t.set(&[1, 0], 1);
        let b = Cell2d::new(t);
        assert_eq!(translate_of(a.types(), b.types()), Some((3, 3)));
        assert_eq!(translate_of(a.types(), a.types()), Some((0, 0)));
        assert_eq!(canonical(a.types()), canonical(b.types()));
        let carpet = mrlymath::two::carpet(3, 2).unwrap();
        let moved = Cell2d::new(Tensor::of(canonical(carpet.types()), vec![9, 9]));
        assert_eq!(factors(moved.types()), None);
        let (_, _, d, outer, inner) = shifted_factors(moved.types()).unwrap();
        assert_eq!((d, inner), (3, 495));
        let tile = |code: u128| {
            Tensor::of(
                (0..9).map(|i| ((code >> i) & 1) as u8).collect(),
                vec![3, 3],
            )
        };
        assert!(translate_of(&tile(495), &tile(outer)).is_some());
        assert_eq!(
            first_negative_lobe(&mrlymath::life::moore().types().clone(), 27),
            9
        );
    }
}
