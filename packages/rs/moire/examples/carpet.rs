// Stack layers that use the Sierpinski carpet tile (3 by 3 with the middle left out) at odd scales 1, 3, 5 ... 15.
// Each layer is lit where the carpet is filled. The sum counts the lit layers, so a hole of one scale is usually covered by another (the sum never drops below 4 of 8). ' ' is 4 layers, '@' is 8.
// cargo run -p mrly-moire --example carpet
use mrly_moire::named;

fn main() {
    let preset = named("carpet", 15).unwrap();
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
