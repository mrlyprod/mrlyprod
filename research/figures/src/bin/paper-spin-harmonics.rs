use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};

const SIDE: usize = 3;
const CODE: u32 = 45;

type Cell = (usize, usize);
type Chord = (Cell, Cell, i64);

fn cells() -> Vec<Cell> {
    (0..SIDE * SIDE)
        .filter(|j| CODE >> j & 1 == 1)
        .map(|j| (j / SIDE, j % SIDE))
        .collect()
}

fn chords(cells: &[Cell]) -> Vec<Chord> {
    let mut out = Vec::new();
    for (a, p) in cells.iter().enumerate() {
        for q in &cells[a + 1..] {
            let dr = p.0 as i64 - q.0 as i64;
            let dc = p.1 as i64 - q.1 as i64;
            out.push((*p, *q, dr * dr + dc * dc));
        }
    }
    out
}

fn main() -> Result<()> {
    let cells = cells();
    let chords = chords(&cells);
    let long: Vec<_> = chords.iter().filter(|c| c.2 == 4).collect();
    assert_eq!(cells.len(), 4, "code 45 fills four cells");
    assert_eq!(chords.len(), 6, "four centres carry six chords");
    assert_eq!(long.len(), 2, "two chords have squared length 4/9");

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let step = frame.w / SIDE as f64;
    let lattice = Grid::new(frame, SIDE, SIDE, 0.10);
    let centre = |cell: Cell| {
        (
            frame.x + (cell.1 as f64 + 0.5) * step,
            frame.y + (cell.0 as f64 + 0.5) * step,
        )
    };

    let plate = ink::fade(ink::dim(), 0.16);
    for row in 0..SIDE {
        for col in 0..SIDE {
            lattice.fill(&mut board, col, row, plate);
        }
    }
    for cell in &cells {
        lattice.fill(&mut board, cell.1, cell.0, ink::blue());
    }

    let thin = step * 0.017;
    let thick = step * 0.038;
    let weight = |d2: i64| if d2 == 4 { thick } else { thin };
    let casing = |d2: i64| weight(d2) + step * if d2 == 4 { 0.016 } else { 0.008 };
    for chord in &chords {
        board.segment(
            centre(chord.0),
            centre(chord.1),
            casing(chord.2),
            ink::ground(),
        );
    }
    for chord in chords.iter().filter(|c| c.2 != 4) {
        board.segment(centre(chord.0), centre(chord.1), thin, ink::dim());
    }
    for chord in &long {
        board.segment(centre(chord.0), centre(chord.1), thick, ink::pink());
    }

    save("paper-spin-harmonics", &board)?;
    Ok(())
}
