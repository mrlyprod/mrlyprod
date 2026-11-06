// Stack checkerboards of odd sizes 1, 3, 5 ... 15 on one square and add them up.
// Each layer is a 2 by 2 parity tile scaled to n cells per side. The sum counts how many layers are lit at each point: ' ' is the fewest (1), '@' is all 8 where every board agrees.
// cargo run -p mrly-moire --example heatmap
use mrly_moire::named;

fn main() {
    let preset = named("heatmap", 15).unwrap();
    let size = 40;
    let field = preset.field(size).unwrap();
    println!(
        "layers {:?}  min {}  max {}  mean {:.3}",
        preset.numbers,
        field.min(),
        field.max(),
        field.mean()
    );
    let ramp = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let span = field.max() - field.min();
    for row in field.data.chunks(size) {
        let line: String = row
            .iter()
            .map(|&v| {
                let t = if span > 0.0 {
                    (v - field.min()) / span
                } else {
                    0.0
                };
                ramp[(t * 9.0).round() as usize].to_string().repeat(2)
            })
            .collect();
        println!("{}", line.trim_end());
    }
}
