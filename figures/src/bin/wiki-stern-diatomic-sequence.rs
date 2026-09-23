use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::num::series::fibonacci;

const LEVEL: usize = 8;

fn stern(top: usize) -> Vec<usize> {
    let mut s = vec![0usize; top + 1];
    s[1] = 1;
    for n in 2..=top {
        s[n] = if n % 2 == 0 {
            s[n / 2]
        } else {
            s[n / 2] + s[n / 2 + 1]
        };
    }
    s
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let s = stern(1 << LEVEL);
    let rows: Vec<&[usize]> = (0..LEVEL).map(|k| &s[1 << k..=2 << k]).collect();
    let peaks: Vec<usize> = rows
        .iter()
        .map(|row| *row.iter().max().unwrap_or(&0))
        .collect();
    assert_eq!(peaks, fibonacci(34)[1..].to_vec());
    for (k, row) in rows.iter().enumerate() {
        assert!(row.iter().eq(row.iter().rev()));
        assert_eq!(row.iter().sum::<usize>(), 3usize.pow(k as u32) + 1);
        if let Some(next) = rows.get(k + 1) {
            for (i, pair) in row.windows(2).enumerate() {
                assert_eq!(next[2 * i], pair[0]);
                assert_eq!(next[2 * i + 1], pair[0] + pair[1]);
            }
        }
    }
    let finest = frame.w / (1 << (LEVEL - 1)) as f64;
    let width = finest * 0.66;
    let span = frame.w - width;
    let band = frame.h / LEVEL as f64;
    let tall = band * 0.8;
    let mut bars = 0usize;
    let mut lit = 0usize;
    for (k, row) in rows.iter().enumerate() {
        let foot = frame.y + band * (k + 1) as f64 - (band - tall) / 2.0;
        for (i, &value) in row.iter().enumerate() {
            let hit = value == peaks[k];
            let height = tall * value as f64 / peaks[k] as f64;
            let x = frame.x + span * i as f64 / (1 << k) as f64;
            board.rect(
                x,
                foot - height,
                width,
                height,
                if hit { ink::yellow() } else { ink::blue() },
            );
            bars += 1;
            lit += hit as usize;
        }
    }
    assert_eq!(bars, 263);
    assert_eq!(lit, 15);
    save("wiki-stern-diatomic-sequence", &board)?;
    Ok(())
}
