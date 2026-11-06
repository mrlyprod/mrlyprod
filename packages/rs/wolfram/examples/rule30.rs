// Rule 30 from a single cell: simple rule, simple start, and the right side looks random.
// 48 steps; black cells are drawn as horizontal runs.
// cargo run -p mrly-wolfram --example rule30
use mrly_wolfram::single_seed;

fn main() {
    let steps = 48;
    let grid = single_seed(30, steps).unwrap();
    let width = 2 * steps + 1;
    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {}\">",
        steps + 1
    );
    print!("<path d=\"");
    for row in 0..=steps {
        let mut col = 0;
        while col < width {
            if grid.at(row * width + col) == 0 {
                col += 1;
                continue;
            }
            let start = col;
            while col < width && grid.at(row * width + col) == 1 {
                col += 1;
            }
            print!("M{start} {row}h{0}v1h-{0}z", col - start);
        }
    }
    println!("\"/>");
    println!("</svg>");
}
