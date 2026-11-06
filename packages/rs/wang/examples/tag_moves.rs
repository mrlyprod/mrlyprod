// The eight moves on tags (four turns, four flips), checked against real triangle geometry.
// For each tag and move, the moved triangles must be exactly the triangles of the moved tag.
// cargo run -p mrly-wang --example tag_moves
use mrly_wang::tag::{GROUP, Move, sides};
use mrly_wang::triangle;
use std::collections::BTreeSet;

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

fn main() {
    for op in GROUP {
        let agree = (0..16u8)
            .filter(|&tag| {
                let image: BTreeSet<[(i64, i64); 3]> = shape(tag)
                    .iter()
                    .map(|points| {
                        let mut out = points.map(|p| moved(op, p));
                        out.sort_unstable();
                        out
                    })
                    .collect();
                image == shape(op.tag(tag))
            })
            .count();
        println!("{op:?}: {agree}/16 tags match the drawn triangles");
    }

    let cycle = (0..16u8).all(|t| Move::Turn(1).tag(t) == (t << 1 | t >> 3) & 15);
    println!("a quarter turn is a left rotate of the 4 NESW bits: {cycle}");
}
