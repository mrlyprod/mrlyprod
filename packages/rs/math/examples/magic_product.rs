// A magic pattern layers two designs by the Kronecker product: the second design is drawn inside each filled cell of the first.
// Here a 3x3 carpet is the outer layer and a 3x3 net is the inner layer.
// cargo run -p mrly-math --example magic_product

use mrly_math::factory::{MagicLayer, magic};
use mrly_math::name::Bang;

fn main() {
    let carpet = MagicLayer::new(Bang::new(7, 2, 2), 3);
    let net = MagicLayer::new(Bang::new(14, 2, 2), 3);
    let tensor = magic(&[carpet, net]).unwrap();
    let n = tensor.shape[0];
    for y in 0..n {
        let row: String = (0..n)
            .map(|x| if tensor.at(y * n + x) == 1 { '#' } else { '.' })
            .collect();
        println!("{row}");
    }
}
