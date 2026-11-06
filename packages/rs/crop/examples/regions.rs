// Every shape sorts the cells of a square grid into three regions: In (whole cell inside), Cut (the edge passes through it) and Out.
// All arithmetic is exact fractions, so a cell is never judged by a rounded float. The radius here is 3/8 of the grid side.
// '#' is In, '+' is Cut, ' ' is Out.
// cargo run -p mrly-crop --example regions
use mrly_crop::{Frac, Region, named, regions};
use mrly_tensor::Tensor;

fn main() {
    let side = 24;
    let shape = named("ball", 2, Frac::new(3, 8).unwrap()).unwrap();
    let map: Tensor = regions(&shape, &[side, side]).unwrap();
    let mut count = [0usize; 3];
    for flat in 0..map.size() {
        count[map.at(flat) as usize] += 1;
    }
    println!(
        "out {}  cut {}  in {}",
        count[Region::Out as usize],
        count[Region::Cut as usize],
        count[Region::In as usize]
    );
    for row in 0..side {
        let line: String = (0..side)
            .map(|col| match map.at(row * side + col) {
                2 => "##",
                1 => "++",
                _ => "  ",
            })
            .collect();
        println!("{}", line.trim_end());
    }
}
