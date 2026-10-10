use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;
use mrlyrs::num::series::fibonacci;

const NAME: &str = "wiki-stern-diatomic-sequence";
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
    let bars: usize = rows.iter().map(|row| row.len()).sum();
    let lit: usize = rows
        .iter()
        .zip(&peaks)
        .map(|(row, &peak)| row.iter().filter(|&&value| value == peak).count())
        .sum();
    assert_eq!(bars, 263);
    assert_eq!(lit, 15);
    save(NAME, &json!({"rows": rows}))
}
