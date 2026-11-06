// The partials of a timbre: which multiples of the pitch sound, and how loud.
// Every partial is a sine. The shape picks the recipe: which multiples, how loud, and whether each adds or subtracts.
// cargo run -p mrly-audio --example partials
use mrly_audio::{Timbre, Wave, hertz};

fn main() {
    // A square wave is odd multiples at 1/n, a triangle odd multiples at 1/n^2 with alternating signs,
    // a sawtooth every multiple at 1/n with alternating signs. A sine is the pitch alone.
    // Weights carry a constant so that the sum draws the wave itself: 4/pi, 8/pi^2 and 2/pi.
    for shape in Wave::all() {
        let timbre = Timbre::new(shape, 5);
        let parts: Vec<String> = timbre
            .partials(440.0)
            .iter()
            .map(|(multiple, weight)| format!("{multiple}x {weight:+.3}"))
            .collect();
        println!("{:<9} {}", shape.name(), parts.join("  "));
    }

    // A partial above half the sample rate (the Nyquist limit) cannot be played, so it is dropped.
    // High notes keep fewer of their 16 sawtooth partials. Sine partials never fold back below the limit.
    let rich = Timbre::new(Wave::Sawtooth, 16);
    for note in [45, 81, 93, 105, 117] {
        println!(
            "midi {note:>3} at {:>8.1} Hz keeps {:>2} of 16",
            hertz(note),
            rich.partials(hertz(note)).len()
        );
    }
}
