// rot90 turns a tensor a quarter turn on two axes.
// cargo run -p mrly-tensor --example rotate
use mrly_tensor::Tensor;

fn show(label: &str, t: &Tensor) {
    println!("{label} {:?}", t.shape);
    for row in t.bytes().unwrap().chunks(t.shape[1]) {
        let cells: Vec<String> = row.iter().map(u8::to_string).collect();
        println!("  {}", cells.join(" "));
    }
}

fn main() {
    // Not square and not symmetric, so every turn looks different.
    let t = Tensor::of(vec![1, 2, 3, 4, 5, 6], vec![2, 3]).unwrap();
    show("start", &t);
    // k counts quarter turns; axes (0, 1) is the plane of the page.
    for k in 1..=3 {
        show(&format!("k={k}  "), &t.rot90(k, (0, 1)).unwrap());
    }
    // Four turns come home.
    println!("k=4 equals start: {}", t.rot90(4, (0, 1)).unwrap() == t);
}
