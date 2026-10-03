use figures::{ink, save, Board, Color, Frame, Grid};
use mrlyrs::core::colors::shades;
use mrlyrs::core::error::Result;

const BASE: usize = 12;
const DIGITS: [usize; 4] = [0, 1, 8, 9];
const LEVELS: [(usize, usize, [usize; 4]); 3] = [
    (1, 16, [0, 2, 4, 6]),
    (2, 64, [0, 2, 16, 18]),
    (3, 256, [0, 2, 64, 66]),
];

fn level(n: usize) -> Vec<usize> {
    let mut points = vec![0];
    for i in 0..n {
        let step = BASE.pow(i as u32);
        points = points
            .iter()
            .flat_map(|p| DIGITS.iter().map(move |d| p + d * step))
            .collect();
    }
    points.sort_unstable();
    points.dedup();
    points
}

fn tiles_residues() -> bool {
    (0..BASE).any(|a| {
        (a + 1..BASE).any(|b| {
            (b + 1..BASE).any(|c| {
                let mut seen = [false; BASE];
                DIGITS.iter().all(|d| {
                    [a, b, c].iter().all(|e| {
                        let r = (d + e) % BASE;
                        !std::mem::replace(&mut seen[r], true)
                    })
                })
            })
        })
    })
}

fn cover(points: &[usize], period: usize, shifts: &[usize; 4]) -> Vec<usize> {
    let mut owner = vec![usize::MAX; period];
    for (k, shift) in shifts.iter().enumerate() {
        for p in points {
            let r = (p + shift) % period;
            assert_eq!(owner[r], usize::MAX);
            owner[r] = k;
        }
    }
    assert!(owner.iter().all(|&k| k < 4));
    owner
}

fn main() -> Result<()> {
    assert!(!tiles_residues());
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let unit = frame.w / 30.0;
    let [tint, blue, shadow] = shades(ink::blue());
    let tones: [Color; 4] = [ink::yellow(), tint, blue, shadow];
    let mut corner = 0.0;
    for (n, period, shifts) in LEVELS {
        let points = level(n);
        assert_eq!(points.len(), 4usize.pow(n as u32));
        let owner = cover(&points, period, &shifts);
        let side = 1 << (n + 1);
        assert_eq!(side * side, period);
        let size = side as f64 * unit;
        let panel = Frame::new(frame.x + corner, frame.y + corner, size, size);
        let grid = Grid::new(panel, side, side, 0.14);
        for (r, k) in owner.iter().enumerate() {
            grid.fill(&mut board, r % side, r / side, tones[*k]);
        }
        corner += size + unit;
    }
    save("research-tilings", &board)?;
    Ok(())
}
