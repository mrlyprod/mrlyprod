// The five classic 2D designs as 5x5 tiles. # is filled, . is empty.
// Odd and even coordinates decide each cell: the carpet keeps cells with at most one odd coordinate.
// cargo run -p mrly-math --example classic_designs

use mrly_math::atoms;
use mrly_tensor::Tensor;

fn show(name: &str, tile: &Tensor) {
    println!("{name}");
    let n = tile.shape[0];
    for y in 0..n {
        let row: String = (0..n)
            .map(|x| if tile.at(y * n + x) == 1 { '#' } else { '.' })
            .collect();
        println!("{row}");
    }
    println!();
}

fn main() {
    show("carpet", &atoms::carpet_2d(5));
    show("net", &atoms::net_2d(5));
    show("htree", &atoms::htree_2d(5));
    show("vtree", &atoms::vtree_2d(5));
    show("void", &atoms::void_2d(5));
}
