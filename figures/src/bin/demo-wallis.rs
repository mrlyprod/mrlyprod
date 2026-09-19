use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Color};
use mrlynum::sieve;

const LEVELS: usize = 3;
const SPAN: f64 = 840.0;

fn tones(word: &[u64], side: u64) -> Vec<(u64, Color)> {
    let inks = [ink::yellow(), ink::orange(), ink::blue()];
    let mut run = side;
    let mut out = Vec::new();
    for (place, &letter) in word.iter().enumerate() {
        run /= letter;
        out.push((run, inks[place]));
    }
    out
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let word = sieve::odd_word(LEVELS);
    let (side, sites) = sieve::raster(&word);
    let holes = sieve::punctures(&word, 2);
    assert_eq!(side, 105);
    assert_eq!(sites.iter().filter(|&&b| b == 1).count(), 9216);
    assert_eq!(holes.len() / 3, 201);
    let tones = tones(&word, side as u64);
    let unit = SPAN / side as f64;
    let edge = (board.width as f64 - SPAN) / 2.0;
    board.rect(edge, edge, SPAN, SPAN, ink::line());
    for hole in holes.chunks(3) {
        let wide = hole[2];
        let tone = tones.iter().find(|(size, _)| *size == wide).unwrap().1;
        board.rect(
            edge + hole[1] as f64 * unit,
            edge + hole[0] as f64 * unit,
            wide as f64 * unit,
            wide as f64 * unit,
            tone,
        );
    }
    save("demo-wallis", &board)?;
    Ok(())
}
