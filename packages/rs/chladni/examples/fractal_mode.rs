// A plate cut in a fractal shape: design code 7 (an L of three quarters) repeated to level 4 makes a Sierpinski triangle plate of 81 cells on a 16 by 16 grid.
// Mode number 5 (counting from 0) is drawn by sign: '+' where the plate moves up, '-' where it moves down, ' ' where there is no plate (the raster holds NaN there).
// Its nodal domains are the connected '+' and '-' regions, counted by figures().
// cargo run -p mrly-chladni --example fractal_mode
use mrly_chladni::nodal::figures;

fn main() {
    let count = 6;
    let modes = figures(7, 2, 4, count).unwrap();
    let side = modes.side;
    let mode = count - 1;
    println!(
        "eigenvalue {:.5}  nodal domains {}",
        modes.values[mode], modes.domains[mode]
    );
    let field = &modes.fields[mode * side * side..(mode + 1) * side * side];
    for row in field.chunks(side) {
        let line: String = row
            .iter()
            .map(|&v| match v {
                v if v.is_nan() => "  ",
                v if v > 0.0 => "++",
                _ => "--",
            })
            .collect();
        println!("{}", line.trim_end());
    }
}
