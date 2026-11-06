// A mosaic game: each mask is a square that keeps some corners, and each kept corner sends to the next mask (a slot).
// The matrix counts how many corners lead from mask to mask; its largest eigenvalue (rho) is the growth per level, and log_base(rho) is the fractal dimension.
// One mask that keeps 3 of 4 corners is the Sierpinski gasket, and 8 of 9 corners is the carpet. A two-mask game gives rho = 1 + sqrt(2).
// cargo run -p mrly-chaos --example mosaic
use mrly_chaos::{Mosaic, mosaic_game};

fn main() {
    let carpet: Vec<(u8, u8)> = (0..9u8).filter(|&d| d != 4).map(|d| (d, 0)).collect();
    let games = [
        ("gasket", 2, vec![vec![(0, 0), (1, 0), (2, 0)]]),
        ("carpet", 3, vec![carpet]),
        (
            "two masks",
            2,
            vec![vec![(0, 0), (1, 1), (2, 1)], vec![(3, 0), (0, 1)]],
        ),
    ];
    for (name, base, masks) in games {
        let mosaic = Mosaic {
            base,
            dimension: 2,
            masks,
        };
        let game = mosaic_game(&mosaic).unwrap();
        println!("{name}");
        println!("  matrix {:?}", game.matrix);
        println!("  rho {:.6}  dimension {:.6}", game.rho, game.dimension);
    }
    println!(
        "check: log2 3 = {:.6}, log3 8 = {:.6}, 1 + sqrt 2 = {:.6}",
        3f64.log2(),
        8f64.log(3.0),
        1.0 + 2f64.sqrt()
    );
}
