// Adam7 interlacing sends a picture in seven passes, coarse first, so a viewer can show a rough
// version early. We never write such files, but we read them. Here one is put together by hand.
// cargo run -p mrly-png --example adam7
use mrly_png::{compress, crc32, decode};

// Where each pass starts and how far apart its pixels are: x, y, step x, step y.
const PASSES: [(usize, usize, usize, usize); 7] = [
    (0, 0, 8, 8),
    (4, 0, 8, 8),
    (0, 4, 4, 8),
    (2, 0, 4, 4),
    (0, 2, 2, 4),
    (1, 0, 2, 2),
    (0, 1, 1, 2),
];

fn chunk(out: &mut Vec<u8>, name: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    let start = out.len();
    out.extend_from_slice(name);
    out.extend_from_slice(body);
    let sum = crc32(&out[start..]);
    out.extend_from_slice(&sum.to_be_bytes());
}

fn main() -> Result<(), String> {
    // An 8 x 8 gray picture whose pixel at (x, y) has the value 10 * y + x.
    let mut rows = Vec::new();
    let mut pass_of = [[0; 8]; 8];
    for (pass, &(x0, y0, dx, dy)) in PASSES.iter().enumerate() {
        for y in (y0..8).step_by(dy) {
            // Each row of a pass starts with a filter byte. 0 means the row is stored as it is.
            rows.push(0);
            for x in (x0..8).step_by(dx) {
                rows.push((10 * y + x) as u8);
                pass_of[y][x] = pass + 1;
            }
        }
    }

    let mut file = vec![137, 80, 78, 71, 13, 10, 26, 10];
    // Width 8, height 8, 8 bits, gray, and a final 1 that says: interlaced.
    chunk(&mut file, b"IHDR", &[0, 0, 0, 8, 0, 0, 0, 8, 8, 0, 0, 0, 1]);
    chunk(&mut file, b"IDAT", &compress(&rows, 9));
    chunk(&mut file, b"IEND", &[]);

    let (width, _, colors) = decode(&file)?;
    println!("pass that carries each pixel    decoded gray values");
    for (y, line) in colors.chunks(width).enumerate() {
        let passes: Vec<String> = pass_of[y].iter().map(|pass| pass.to_string()).collect();
        let values: Vec<String> = line
            .iter()
            .map(|color| format!("{:>2}", color[0]))
            .collect();
        println!("{}                 {}", passes.join(" "), values.join(" "));
    }
    Ok(())
}
