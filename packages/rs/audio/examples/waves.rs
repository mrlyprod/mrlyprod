// The four wave shapes, sampled at eight points of one turn.
// A phase counts turns: 0.25 is a quarter of the way through one cycle.
// cargo run -p mrly-audio --example waves
use mrly_audio::Wave;

fn main() {
    print!("phase    ");
    for i in 0..=8 {
        print!(" {:>6.3}", i as f64 / 8.0);
    }
    println!();
    for wave in Wave::all() {
        print!("{:<9}", wave.name());
        for i in 0..=8 {
            print!(" {:>6.3}", wave.sample(i as f64 / 8.0));
        }
        println!();
    }

    // Summing a wave's 300 sine partials draws the wave itself.
    // The gap is taken at eight phases clear of the square's and sawtooth's jumps, where any finite sum overshoots.
    for wave in Wave::all() {
        let recipe = wave.recipe(300);
        let gap = (0..8)
            .map(|i| (i as f64 + 0.5) / 8.0)
            .fold(0.0, |worst, phase| {
                let sum: f64 = recipe
                    .iter()
                    .map(|&(m, w)| w * Wave::Sine.sample(m * phase))
                    .sum();
                f64::max(worst, (sum - wave.sample(phase)).abs())
            });
        println!(
            "{:<9} 300 partials sum to within {gap:.4} of sample",
            wave.name()
        );
    }

    // The sine is a polynomial, so it gives the same bits on every machine.
    // Here it is checked against the platform's own sine at 100 000 phases.
    let worst = (0..100_000)
        .map(|i| {
            let phase = i as f64 / 100_000.0;
            (Wave::Sine.sample(phase) - (std::f64::consts::TAU * phase).sin()).abs()
        })
        .fold(0.0, f64::max);
    println!(
        "sine against f64::sin: worst error under 1e-9: {}",
        worst < 1e-9
    );
}
