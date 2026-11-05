// The 16 mrly-four tags: what each one is, and checks of every claim made about them.
//
// A cell is a square cut by its two diagonals into four triangles pointing inwards.
// Each triangle is named by the edge it sits on: N, E, S, W, clockwise.
// A tag holds one bit per triangle, written NESW, so 1100 reads "N and E" and is hex c.
use mrly_four::tag::{GROUP, Move, class, orbit, sides};
use mrly_four::{Cell4d, E, HALVES, N, S, W, blocks, from_strings, triangle};
use mrly_math::bang::Code;
use mrly_math::bang::universe;
use mrly_math::name::bang::of as bang_word;
use std::collections::BTreeSet;

// The array stores the same bits as a little 2x2 grid: N E on top, W S below.
// That grid is also the four residue corners of a 2d base-2 bang code, whose bit index
// is row*2 + col. So every tag has one bang code: same sets, different bit order.
fn to_bang(tag: u8) -> u64 {
    let [n, e, s, w] = sides(tag).map(u64::from);
    n | e << 1 | w << 2 | s << 3
}

// Which triangles does this tag paint, as real points in the doubled grid?
fn shape(tag: u8) -> BTreeSet<[(i64, i64); 3]> {
    (0..4)
        .filter(|&side| sides(tag)[side] == 1)
        .map(|side| {
            let mut points = triangle(0, 0, side);
            points.sort_unstable();
            points
        })
        .collect()
}

// The same eight moves, applied to points around the cell centre (1, 1).
fn moved(op: Move, (x, y): (i64, i64)) -> (i64, i64) {
    let (u, v) = (x - 1, y - 1);
    let (a, b) = match op {
        Move::Turn(k) => match k % 4 {
            0 => (u, v),
            1 => (v, -u),
            2 => (-u, -v),
            _ => (-v, u),
        },
        Move::FlipX => (-u, v),
        Move::FlipY => (u, -v),
        Move::FlipMain => (v, u),
        Move::FlipAnti => (-v, -u),
    };
    (a + 1, b + 1)
}

fn main() -> Result<(), String> {
    println!("hex nesw  grid   class      orbit  turn1 bang word");
    for tag in 0..16u8 {
        let [n, e, s, w] = sides(tag);
        let code = Code::from(to_bang(tag));
        println!(
            "{tag:x}   {n}{e}{s}{w}  {n}{e}/{w}{s}  {:<10} {:<6} {:x}     {} {}",
            class(tag),
            orbit(tag).len(),
            Move::Turn(1).tag(tag),
            to_bang(tag),
            bang_word(code, 2, 2).unwrap_or_else(|| "-".to_string()),
        );
    }

    // Check every tag under every move against real triangle geometry.
    let mut agree = 0;
    for tag in 0..16u8 {
        for op in GROUP {
            let image: BTreeSet<[(i64, i64); 3]> = shape(tag)
                .iter()
                .map(|points| {
                    let mut out = points.map(|p| moved(op, p));
                    out.sort_unstable();
                    out
                })
                .collect();
            agree += usize::from(image == shape(op.tag(tag)));
        }
    }
    println!("\ntags x moves matching the drawn triangles: {agree}/128");

    // Bits rotate as a plain 4-bit cycle.
    let cycle = (0..16u8).all(|t| Move::Turn(1).tag(t) == (t << 1 | t >> 3) & 15);
    println!("a quarter turn counter-clockwise is a left rotate of NESW: {cycle}");
    println!(
        "FlipX moves {} bits, FlipY {}, FlipMain {}, FlipAnti {}",
        (Move::FlipX.tag(N | S | E) ^ (N | S | E)).count_ones(),
        (Move::FlipY.tag(N | E | W) ^ (N | E | W)).count_ones(),
        (Move::FlipMain.tag(N | E) ^ (N | E)).count_ones(),
        (Move::FlipAnti.tag(N | W) ^ (N | W)).count_ones(),
    );

    // The six classes of tags are the same sets as the bang orbits in 2d.
    let classes: BTreeSet<Vec<u64>> = (0..16u8)
        .map(|t| {
            orbit(t)
                .into_iter()
                .map(to_bang)
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect()
        })
        .collect();
    let orbits: BTreeSet<Vec<u64>> = (0..16u64)
        .map(|c| {
            let codes = universe::orbit(Code::from(c), 2);
            codes.into_iter().map(|code| code.get() as u64).collect()
        })
        .collect();
    println!(
        "{} tag classes, same sets as the 2d bang orbits: {}",
        classes.len(),
        classes == orbits
    );

    // Is the 2x2 block a picture? Compare plain array moves with real moves.
    let chiral = from_strings(&["c60", "39f"])?;
    let raw_turn = array_turn(&chiral);
    println!(
        "\nblock array turned as a plain grid == real turn: {}",
        raw_turn == chiral.rotate(1)
    );
    let raw_flip = array_flip(&chiral);
    println!(
        "block array flipped as a plain grid == real flip: {}",
        raw_flip == chiral.transform(Move::FlipX)
    );
    println!("half tiles in the block are dominoes, not half squares:");
    for half in HALVES {
        let cell = from_strings(&[&format!("{half:x}")])?;
        println!("  {half:x} {:?}", blocks(&cell));
    }
    Ok(())
}

fn array_turn(cell: &Cell4d) -> Cell4d {
    let [rows, cols] = cell.shape();
    let mut out = Cell4d::new(cell.height, cell.width);
    for r in 0..rows {
        for c in 0..cols {
            out.types[(cols - 1 - c) * rows + r] = cell.types[r * cols + c];
        }
    }
    out
}

fn array_flip(cell: &Cell4d) -> Cell4d {
    let [rows, cols] = cell.shape();
    let mut out = Cell4d::new(cell.width, cell.height);
    for r in 0..rows {
        for c in 0..cols {
            out.types[r * cols + (cols - 1 - c)] = cell.types[r * cols + c];
        }
    }
    out
}
