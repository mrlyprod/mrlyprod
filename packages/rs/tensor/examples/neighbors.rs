// Count the live neighbors of every cell, the first step of Game of Life.
// cargo run -p mrly-tensor --example neighbors
use mrly_tensor::{Dtype, Tensor};

fn show(label: &str, t: &Tensor) {
    println!("{label}");
    for row in t.bytes().unwrap().chunks(t.shape[1]) {
        let cells: Vec<String> = row.iter().map(u8::to_string).collect();
        println!("  {}", cells.join(" "));
    }
}

fn main() {
    // A blinker: three live cells in a row.
    let board = Tensor::of(
        vec![
            0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0, //
            0, 1, 1, 1, 0, //
            0, 0, 0, 0, 0, //
            0, 0, 0, 0, 0,
        ],
        vec![5, 5],
    )
    .unwrap();

    // The mask picks which cells around the center count. Sides must be odd.
    // This one is the eight surrounding cells, not the center.
    let ring = Tensor::of(vec![1, 1, 1, 1, 0, 1, 1, 1, 1], vec![3, 3]).unwrap();

    // target 1 counts live cells; wrap false treats outside as empty.
    let counts = board.neighbors(&ring, 1, false, Dtype::U8).unwrap();
    show("board", &board);
    show("live neighbors", &counts);
    println!(
        "an even mask fails: {}",
        board
            .neighbors(&Tensor::new(vec![2, 2]), 1, false, Dtype::U8)
            .unwrap_err()
    );
}
