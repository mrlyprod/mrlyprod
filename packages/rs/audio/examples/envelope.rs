// An envelope shapes a note's loudness over its length.
// Attack rises from silence, decay falls to the sustain level, release falls back to silence at the end.
// cargo run -p mrly-audio --example envelope
use mrly_audio::{Envelope, samples};

fn show(label: &str, envelope: &Envelope, seconds: f64) {
    let gains = envelope.gains(samples(seconds));
    println!("{label}: {} samples", gains.len());
    for step in 0..=10 {
        let at = (gains.len() - 1) * step / 10;
        let g = gains[at];
        println!(
            "  {at:>6}  {g:.3}  {}",
            "#".repeat((g * 40.0).round() as usize)
        );
    }
}

fn main() {
    // The default only fades in and out over 1/64 s, so frames join without clicks.
    show("fade over 0.1 s", &Envelope::fade(), 0.1);

    // A plucked shape: fast attack, a long decay to 30 %, then a release.
    show("pluck over 0.5 s", &Envelope::new(0.01, 0.2, 0.3, 0.1), 0.5);

    // A note shorter than attack plus release shrinks both to fit.
    show(
        "pluck over 0.05 s",
        &Envelope::new(0.01, 0.2, 0.3, 0.1),
        0.05,
    );
}
