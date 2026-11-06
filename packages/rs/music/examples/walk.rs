// A walk that arrives on a chord: every frame steps through the pool until it lands on the target.
// It makes a short run into the chord a piece opens or closes on.
// cargo run -p mrly-music --example walk
use mrly_audio::name;
use mrly_music::{ROOT, Scale, pool, walk};

fn main() -> Result<(), String> {
    let notes = pool(Scale::Major, ROOT, 3);
    let target = [ROOT + 12, ROOT + 16, ROOT + 19];
    for step in [1, -1] {
        println!("step {step:+}, landing on G3 B3 D4");
        for frame in walk(&notes, &target, 6, step)? {
            let names: Vec<String> = frame.into_iter().map(name).collect();
            println!("  {}", names.join(" "));
        }
    }
    // The pool wraps: a walk that runs off the top comes back in at the bottom.
    let names: Vec<String> = walk(&notes, &[ROOT + 2], 3, 1)?
        .into_iter()
        .map(|f| name(f[0]))
        .collect();
    println!("up onto A2: {}", names.join(" "));
    println!("{}", walk(&notes, &[ROOT + 1], 4, 1).unwrap_err());
    Ok(())
}
