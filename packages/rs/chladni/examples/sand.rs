// Sprinkle sand on a vibrating plate and shake. Grains bounce hard where the plate moves a lot and barely where it is still, so they drift to the nodal lines.
// shake() moves each grain a number of steps; `from` and `seed` make the kicks repeatable. Here 19 881 grains start on an even grid and get 40 rounds of 64 steps.
// The picture counts grains per cell (24 by 24 cells): ' ' is empty, '@' is the most crowded. Compare the bright lines with nodal_lines.
// cargo run -p mrly-chladni --example sand
use mrly_chladni::{plate, shake};

fn main() {
    let size = 48;
    let field = plate(3, 5, 1, size).unwrap();
    let per_side = 141;
    let mut grains: Vec<f32> = Vec::with_capacity(per_side * per_side * 2);
    for row in 0..per_side {
        for col in 0..per_side {
            grains.push((col as f32 + 0.5) / per_side as f32);
            grains.push((row as f32 + 0.5) / per_side as f32);
        }
    }
    let rounds = 40;
    for round in 0..rounds {
        grains = shake(&field, size, &grains, round * 64, 64, 7).unwrap();
    }
    let cells = 24;
    let mut count = vec![0u32; cells * cells];
    let mut alive = 0;
    for pair in grains.chunks(2) {
        if pair[0].is_nan() {
            continue;
        }
        alive += 1;
        let col = ((pair[0] * cells as f32) as usize).min(cells - 1);
        let row = ((pair[1] * cells as f32) as usize).min(cells - 1);
        count[row * cells + col] += 1;
    }
    println!("{alive} grains on the plate");
    let top = *count.iter().max().unwrap() as f32;
    let ramp = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    for row in count.chunks(cells) {
        let line: String = row
            .iter()
            .map(|&n| {
                ramp[((n as f32 / top) * 9.0).round() as usize]
                    .to_string()
                    .repeat(2)
            })
            .collect();
        println!("{}", line.trim_end());
    }
}
