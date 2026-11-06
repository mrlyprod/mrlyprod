// The same stack of checkerboards on a hexagonal lattice instead of a square one.
// The sum counts how many of the 8 layers are lit at each point, from ' ' (none) to '@' (all 8). Rows of the hex lattice are shifted half a cell.
// cargo run -p mrly-moire --example hive
use mrly_moire::named;

fn main() {
    let preset = named("hive", 15).unwrap();
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
