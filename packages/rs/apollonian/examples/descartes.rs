// Four circles that touch in pairs obey Descartes' rule, and any one can be swapped for its mirror twin.
// Here: the root quad with curvatures -1, 2, 2, 3, and what each reflection gives.
// cargo run -p mrly-apollonian --example descartes
use mrly_apollonian::{reflect, root, sound, swap};

fn main() {
    let quad = root("-1,2,2,3").unwrap();
    println!("root quad, sound: {}", sound(&quad));
    for c in quad {
        println!("  k={:>2} x={:>2} y={:>2}", c.k, c.x, c.y);
    }
    println!();
    println!("swap one circle for its twin:");
    for at in 0..4 {
        let twin = reflect(&quad, at);
        let next = swap(&quad, at);
        println!(
            "  k={:>2} -> k={:>2}, new quad sound: {}",
            quad[at].k,
            twin.k,
            sound(&next)
        );
    }
}
