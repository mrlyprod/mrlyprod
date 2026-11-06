// A Sierpinski carpet grown by repeated Kronecker products.
// cargo run -p mrly-tensor --example carpet
use mrly_tensor::Tensor;

fn main() {
    // 1 is a filled cell, 0 is a hole. This 3x3 has a hole in the middle.
    let seed = Tensor::of(vec![1, 1, 1, 1, 0, 1, 1, 1, 1], vec![3, 3]).unwrap();

    // fractal(level) takes the product with the seed level-1 times.
    let carpet = seed.fractal(3);
    println!("shape {:?}, filled {}", carpet.shape, carpet.count(1));
    for row in carpet.bytes().unwrap().chunks(carpet.shape[1]) {
        let line: String = row
            .iter()
            .map(|&c| if c == 1 { '#' } else { '.' })
            .collect();
        println!("{line}");
    }
}
