// The Thue-Morse plane at side 32: cell (i, j) is the parity of the 1 bits in i xor j.
// Black cells are drawn as horizontal runs.
// cargo run -p mrly-thue --example plane
use mrly_thue::{Lift, lift};

fn main() {
    let side = 32;
    let grid = lift(Lift::Xor, side);
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {side} {side}\">");
    print!("<path d=\"");
    for (y, row) in grid.chunks(side).enumerate() {
        let mut x = 0;
        while x < side {
            if row[x] == 0 {
                x += 1;
                continue;
            }
            let start = x;
            while x < side && row[x] == 1 {
                x += 1;
            }
            print!("M{start} {y}h{}v1h-{}z", x - start, x - start);
        }
    }
    println!("\"/>");
    println!("</svg>");
}
