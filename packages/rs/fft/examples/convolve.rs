// Blur a field by convolving it with a mask, using the FFT.
// cargo run -p mrly-fft --example convolve
use mrly_fft::{convolve, embed_kernel};

fn main() {
    let size = 8;
    // One bright cell in the top-left corner.
    let mut field = vec![0.0; size * size];
    field[0] = 1.0;

    // A 3x3 mask of ones, placed into a size x size kernel with its center at the origin.
    let kernel = embed_kernel(&[1; 9], 3, size).unwrap();
    let out = convolve(&field, &kernel, size).unwrap();

    // Adding 0.0 turns a printed -0 into 0.
    // The blob wraps around the edges: FFT convolution is circular.
    for row in out.chunks(size) {
        let cells: Vec<String> = row
            .iter()
            .map(|v| format!("{:.0}", v.round() + 0.0))
            .collect();
        println!("{}", cells.join(" "));
    }
    println!("total {:.0}", out.iter().sum::<f64>().round());

    // The mask side must be odd.
    println!("{}", embed_kernel(&[1; 4], 2, size).unwrap_err());
}
