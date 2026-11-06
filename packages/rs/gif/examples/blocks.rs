// A GIF file is a short header followed by a list of blocks. This walks a tiny animation and names each block.
// Two 4 by 4 frames: a white dot on black that moves one step to the right.
// cargo run -p mrly-gif --example blocks
use mrly_gif::encode;

fn hex(bytes: &[u8]) -> String {
    let shown: Vec<String> = bytes.iter().take(12).map(|b| format!("{b:02x}")).collect();
    let more = if bytes.len() > 12 { " .." } else { "" };
    format!("{}{more}", shown.join(" "))
}

fn line(at: usize, bytes: &[u8], name: &str, says: &str) {
    println!("{at:>3}  {name:<11} {:<38} {says}", hex(bytes));
}

fn word(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

fn main() -> Result<(), String> {
    let mut first = vec![0u8; 16];
    first[5] = 1;
    let mut second = vec![0u8; 16];
    second[6] = 1;
    let file = encode(
        &[first, second],
        4,
        4,
        &[[0, 0, 0, 255], [255, 255, 255, 255]],
        &[50],
    )?;
    println!("{} bytes", file.len());
    println!();

    line(
        0,
        &file[..6],
        "header",
        &String::from_utf8_lossy(&file[..6]),
    );
    let colors = 2 << (file[10] & 7);
    let says = format!("{} by {}, {colors} colors", word(&file, 6), word(&file, 8));
    line(6, &file[6..13], "screen", &says);
    // The global color table: three bytes a color, padded to a power of two.
    let mut at = 13 + 3 * colors;
    let table: Vec<String> = file[13..at]
        .chunks(3)
        .map(|c| format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2]))
        .collect();
    line(13, &file[13..at], "colors", &table.join(" "));
    loop {
        match file[at] {
            0x21 => {
                // An extension: introducer, label, then sub-blocks of up to 255 bytes, ended by a 0.
                let mut end = at + 2;
                while file[end] != 0 {
                    end += 1 + file[end] as usize;
                }
                let body = &file[at..=end];
                let (name, says) = match file[at + 1] {
                    0xFF => ("loop", format!("NETSCAPE2.0, {} = forever", word(body, 16))),
                    0xF9 => (
                        "control",
                        format!(
                            "delay {}/100 s, {}, transparent {}",
                            word(body, 4),
                            // What to do with this frame once the next is due; see the dispose example.
                            ["unset", "keep", "background", "previous"]
                                [(body[3] >> 2 & 3) as usize],
                            if body[3] & 1 == 1 {
                                body[6].to_string()
                            } else {
                                "none".into()
                            }
                        ),
                    ),
                    _ => ("extension", String::new()),
                };
                line(at, body, name, &says);
                at = end + 1;
            }
            0x2C => {
                let says = format!(
                    "{} by {} at ({}, {})",
                    word(&file, at + 5),
                    word(&file, at + 7),
                    word(&file, at + 1),
                    word(&file, at + 3)
                );
                line(at, &file[at..at + 10], "image", &says);
                at += 10;
                line(
                    at,
                    &file[at..at + 1],
                    "code size",
                    &format!("{} bits a color", file[at]),
                );
                at += 1;
                while file[at] != 0 {
                    let end = at + 1 + file[at] as usize;
                    line(
                        at,
                        &file[at..end],
                        "lzw data",
                        &format!("{} bytes", file[at]),
                    );
                    at = end;
                }
                line(at, &file[at..at + 1], "end", "no more data");
                at += 1;
            }
            _ => {
                line(at, &file[at..at + 1], "trailer", "end of file");
                break;
            }
        }
    }
    Ok(())
}
