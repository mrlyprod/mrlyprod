// Stack checkerboards of odd sizes 1, 3, 5... 25, each turned by the golden angle (137.5 degrees) more than the last, and average them.
// Where the boards agree the picture is dark; the rest forms a moire rosette. The disc is the area every board covers.
// cargo run -p mrly-tourbillon --example golden_stack
use mrly_tourbillon::field;

fn main() {
    let size = 48;
    let grid = field(25, size, "golden", 0.0, "odd", "plain", "cells", "mean", 1).unwrap();
    let top = grid
        .iter()
        .cloned()
        .filter(|v| !v.is_nan())
        .fold(0.0, f32::max);
    println!("layers 13, size {size}, brightest cell {top:.3}");
    let ramp = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    for row in grid.chunks(size) {
        let line: String = row
            .iter()
            .map(|&v| {
                if v.is_nan() {
                    "  ".to_string()
                } else {
                    let i = ((v / top) * 9.0).round() as usize;
                    ramp[i].to_string().repeat(2)
                }
            })
            .collect();
        println!("{}", line.trim_end());
    }
}
