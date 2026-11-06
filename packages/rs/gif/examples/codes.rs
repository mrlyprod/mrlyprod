// LZW, the compression inside every GIF frame, read out of a real file one code at a time.
// Each code stands for a string of colors. Each new table entry is the string just read plus the first color of the next.
// cargo run -p mrly-gif --example codes
use mrly_gif::encode;

fn digits(colors: &[u8]) -> String {
    colors.iter().map(|c| c.to_string()).collect()
}

fn main() -> Result<(), String> {
    let frames = [vec![
        0u8, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    ]];
    let row = &frames[0];
    println!("row  {}", digits(row));
    println!();

    // A still picture, one row high. The file opens with 13 bytes of header, 6 of colors and 10 of image header.
    // Then come the minimum code size, a length, and that many bytes of packed codes.
    let file = encode(
        &frames,
        row.len(),
        1,
        &[[0, 0, 0, 255], [255, 255, 255, 255]],
        &[0],
    )?;
    let min = file[29] as usize;
    let packed = &file[31..31 + file[30] as usize];

    // With a minimum code size of 2, codes 0 to 3 are single colors, 4 is clear and 5 is end.
    // Codes start 3 bits wide and grow a bit each time the table reaches a power of two.
    let (clear, end) = (1 << min, (1 << min) + 1);
    let mut table: Vec<Vec<u8>> = (0..=end).map(|c| vec![c as u8]).collect();
    let mut last: Option<Vec<u8>> = None;
    let mut out = Vec::new();
    let (mut at, mut bits) = (0, min + 1);
    println!("code  bits  stands for  new entry");
    loop {
        // The codes are packed into bytes lowest bit first.
        let bit = |k: usize| (packed[k / 8] >> (k % 8) & 1) as usize;
        let code: usize = (0..bits).map(|k| bit(at + k) << k).sum();
        let width = bits;
        at += bits;
        if code == clear || code == end {
            let name = if code == clear { "clear" } else { "end" };
            println!("{code:>4}  {width:>4}  {name}");
            if code == end {
                break;
            }
            continue;
        }
        // A code one past the table is the string just read plus its own first color.
        let string = match (table.get(code), &last) {
            (Some(known), _) => known.clone(),
            (None, Some(prev)) => [prev.as_slice(), &prev[..1]].concat(),
            (None, None) => return Err("a new code came first.".into()),
        };
        let mut entry = String::new();
        if let Some(prev) = &last {
            table.push([prev.as_slice(), &string[..1]].concat());
            entry = format!("{} = {}", table.len() - 1, digits(&table[table.len() - 1]));
            if table.len() == 1 << bits {
                bits += 1;
            }
        }
        let shown = format!("{code:>4}  {width:>4}  {:<10}  {entry}", digits(&string));
        println!("{}", shown.trim_end());
        out.extend_from_slice(&string);
        last = Some(string);
    }

    let bytes: Vec<String> = packed.iter().map(|b| format!("{b:02x}")).collect();
    println!();
    println!(
        "{} pixels in {} bytes: {}",
        row.len(),
        packed.len(),
        bytes.join(" ")
    );
    println!("decoded back: {}", out == *row);
    Ok(())
}
