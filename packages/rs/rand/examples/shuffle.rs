// Pick from a list, shuffle it, and sample indices without repeats.
// cargo run -p mrly-rand --example shuffle
use mrly_rand::Rng;

fn main() {
    let mut rng = Rng::new(7);
    let words = ["red", "green", "blue", "gold", "gray"];

    println!("choice  {}", rng.choice(&words).unwrap());

    let mut deck = [1, 2, 3, 4, 5, 6, 7, 8];
    rng.shuffle(&mut deck);
    println!("shuffle {deck:?}");

    // Indices 0..10, three of them, each at most once.
    println!("sample  {:?}", rng.sample_indices(10, 3));

    // Asking for more than exists returns them all.
    println!("all     {:?}", rng.sample_indices(4, 9));

    // An empty list has nothing to choose.
    let none: [u8; 0] = [];
    println!("empty   {}", rng.choice(&none).unwrap_err());
}
