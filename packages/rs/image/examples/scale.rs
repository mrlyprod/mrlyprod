// scale turns every pixel into a square block, keeping edges sharp.
// The small heart is the source; the large one is scale(4).
// cargo run -p mrly-image --example scale
use mrly_image::Image;

const HEART: [&str; 7] = [
    ".##.##.", "#######", "#######", "#######", ".#####.", "..###..", "...#...",
];

fn rects(image: &Image, x0: usize, y0: usize) {
    // Merge each run of equal pixels in a row into one rect to keep the file small.
    for y in 0..image.height {
        let row = &image.colors[y * image.width..(y + 1) * image.width];
        let mut x = 0;
        while x < image.width {
            let px = row[x];
            let mut end = x + 1;
            while end < image.width && row[end] == px {
                end += 1;
            }
            if px[3] > 0 {
                println!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"1\" fill=\"rgb({},{},{})\"/>",
                    x0 + x,
                    y0 + y,
                    end - x,
                    px[0],
                    px[1],
                    px[2]
                );
            }
            x = end;
        }
    }
}

fn main() {
    let colors: Vec<[u8; 4]> = HEART
        .iter()
        .flat_map(|row| row.chars())
        .map(|c| {
            if c == '#' {
                [255, 61, 64, 255]
            } else {
                [0, 0, 0, 0]
            }
        })
        .collect();
    let small = Image::new(7, 7, colors).unwrap();
    let big = small.scale(4).unwrap();

    println!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 44 36\" shape-rendering=\"crispEdges\">"
    );
    println!("<rect width=\"44\" height=\"36\" fill=\"#ffffff\"/>");
    rects(&small, 4, 4);
    rects(&big, 14, 4);
    println!("</svg>");
}
