// Rule 90 from a single cell draws Pascal's triangle mod 2: the Sierpinski triangle.
// 16 steps, # for 1 and . for 0.
// cargo run -p mrly-wolfram --example sierpinski
use mrly_wolfram::single_seed;

fn main() {
    let steps = 16;
    let grid = single_seed(90, steps).unwrap();
    let width = 2 * steps + 1;
    for row in 0..=steps {
        let line: String = (0..width)
            .map(|col| {
                if grid.at(row * width + col) == 1 {
                    '#'
                } else {
                    '.'
                }
            })
            .collect();
        println!("{line}");
    }
}
