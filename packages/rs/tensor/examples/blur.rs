// blur replaces each cell with the average of the cells the mask selects.
// cargo run -p mrly-tensor --example blur
use mrly_tensor::Tensor;

fn show(label: &str, t: &Tensor) {
    println!("{label}");
    for row in t.bytes().unwrap().chunks(t.shape[1]) {
        let cells: Vec<String> = row.iter().map(|v| format!("{v:>3}")).collect();
        println!("  {}", cells.join(" "));
    }
}

fn main() {
    let mut t = Tensor::new(vec![5, 5]);
    t.set(&[2, 2], 90).unwrap();
    show("one bright cell", &t);

    // A 3x3 mask of ones averages the nine cells around each one: 90 / 9 = 10.
    let all = Tensor::full(vec![3, 3], 1);
    show("blur", &t.blur(&all, false).unwrap());

    // Near an edge the mask reaches outside. Without wrap those cells are skipped,
    // so the corner averages only 4 cells: 90 / 4 = 22.5, rounded to 23.
    // With wrap the outside is the far side, so all 9 cells count.
    let mut corner = Tensor::new(vec![5, 5]);
    corner.set(&[0, 0], 90).unwrap();
    show("corner, no wrap", &corner.blur(&all, false).unwrap());
    show("corner, wrap", &corner.blur(&all, true).unwrap());
}
