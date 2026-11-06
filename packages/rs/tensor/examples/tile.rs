// tile repeats a tensor along each axis.
// cargo run -p mrly-tensor --example tile
use mrly_tensor::Tensor;

fn show(label: &str, t: &Tensor) {
    println!("{label} {:?}", t.shape);
    for row in t.bytes().unwrap().chunks(t.shape[1]) {
        let cells: Vec<String> = row.iter().map(u8::to_string).collect();
        println!("  {}", cells.join(" "));
    }
}

fn main() {
    let t = Tensor::of(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    show("tile", &t);
    // One repeat count per axis: 2 down, 3 across.
    show("tile 2x3", &t.tile(&[2, 3]).unwrap());
    // Wrong number of counts is a shape error.
    println!("{}", t.tile(&[2]).unwrap_err());
}
