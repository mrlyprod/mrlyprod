// With repeat on, a voice plays the same bar every time a chord comes back.
// Off, every bar is drawn fresh. Here the blues returns to C six times.
// cargo run -p mrly-music --example repeat
use mrly_music::{BLUES, Movement, Piece, Voice, compose};

fn main() -> Result<(), String> {
    let voice = Voice::new(vec![55, 57, 59, 60, 62, 64, 66], vec![Movement::Random]);
    for repeat in [false, true] {
        let mut piece = Piece::new(BLUES, vec![voice.clone()]);
        piece.repeat = repeat;
        let part = &compose(&piece, 4 * BLUES.len(), 5)?[0];
        let bars: Vec<&[Vec<i64>]> = part.chunks(4).collect();
        let on_c: Vec<usize> = BLUES
            .char_indices()
            .filter(|&(_, c)| c == 'C')
            .map(|(i, _)| i)
            .collect();
        let same = on_c.iter().all(|&i| bars[i] == bars[on_c[0]]);
        println!("repeat {repeat:<5}: the six C bars are all the same: {same}");
        for &i in &on_c[..3] {
            println!("  bar {i:>2} {:?}", bars[i]);
        }
    }
    Ok(())
}
