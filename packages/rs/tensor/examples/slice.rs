// slice takes one layer out of a 3D tensor.
// cargo run -p mrly-tensor --example slice
use mrly_tensor::Tensor;

fn main() {
    // Two layers of 2x3, numbered 0 to 11 in memory order.
    let data: Vec<u8> = (0..12).collect();
    let cube = Tensor::of(data, vec![2, 2, 3]).unwrap();
    println!("cube shape {:?}", cube.shape);

    // Cut axis 0 at index 1: the second layer.
    let layer = cube.slice(0, 1).unwrap();
    println!(
        "axis 0, index 1 -> {:?} {:?}",
        layer.shape,
        layer.bytes().unwrap()
    );
    // Cut axis 2 at index 0: the first column of every row.
    let column = cube.slice(2, 0).unwrap();
    println!(
        "axis 2, index 0 -> {:?} {:?}",
        column.shape,
        column.bytes().unwrap()
    );

    // get reads one cell by its position on each axis.
    println!("cell [1, 0, 2] = {}", cube.get(&[1, 0, 2]).unwrap());
    println!("{}", cube.slice(0, 2).unwrap_err());
}
