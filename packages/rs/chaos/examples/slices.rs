// Slice a fractal with the line x + y = h and ask how big the cut is. slice_game() tracks the carry as a finite game and gives the dimension of the cut.
// The anti-diagonal (h = 1) cuts the carpet in a Cantor set, whose dimension is log3 2 = 0.6309. It cuts the gasket in a whole segment (dimension 1).
// A cut that misses the fractal has no states and dimension -infinity.
// cargo run -p mrly-chaos --example slices
use mrly_chaos::slice_game;

fn main() {
    let carpet: Vec<u8> = (0..9).filter(|&d| d != 4).collect();
    let gasket: Vec<u8> = vec![0, 1, 2];
    println!(
        "{:<7} {:>5} {:>7} {:>10}",
        "shape", "h", "states", "dimension"
    );
    for (name, base, digits, heights) in [
        ("carpet", 3, &carpet, vec![(1, 1), (1, 2), (3, 2), (2, 1)]),
        ("gasket", 2, &gasket, vec![(1, 1), (1, 3), (0, 1), (3, 2)]),
    ] {
        for (p, q) in heights {
            let game = slice_game(base, digits, p, q).unwrap();
            println!(
                "{name:<7} {:>5} {:>7} {:>10.4}",
                format!("{p}/{q}"),
                game.states.len(),
                game.dimension
            );
        }
    }
    println!("log3 2 = {:.4}", 2f64.log(3.0));
}
