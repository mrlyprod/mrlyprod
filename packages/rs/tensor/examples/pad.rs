// pad adds a border of one value around a tensor.
// cargo run -p mrly-tensor --example pad
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
    show("pad", &t);
    // pad(count, value) adds count cells on every side.
    show("pad 1 with 9", &t.pad(1, 9));
}
