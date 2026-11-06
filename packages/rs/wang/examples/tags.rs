// The 16 mrly-wang tags: what each one is, how it turns, and its bang code.
// cargo run -p mrly-wang --example tags
//
// A cell is a square cut by its two diagonals into four triangles pointing inwards.
// Each triangle is named by the edge it sits on: N, E, S, W, clockwise.
// A tag holds one bit per triangle, written NESW, so 1100 reads "N and E" and is hex c.
use mrly_math::code::Code;
use mrly_math::name::bang::of as bang_word;
use mrly_wang::Move;
use mrly_wang::tag::{class, orbit, sides};

// The array stores the same bits as a little 2x2 grid: N E on top, W S below.
// That grid is also the four residue corners of a 2d base-2 bang code, whose bit index
// is row*2 + col. So every tag has one bang code: same sets, different bit order.
fn to_bang(tag: u8) -> u64 {
    let [n, e, s, w] = sides(tag).map(u64::from);
    n | e << 1 | w << 2 | s << 3
}

fn main() {
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
}
