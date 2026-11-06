// How rarely two names get the same picture.
// A picture is its 7 by 7 pattern plus its ink; the pattern alone has 2^28 mirrored forms.
// Every picture also keeps between 12 and 37 of its 49 cells filled, so none is blank or solid.
// cargo run -p mrly-avatar --example variety
use mrly_avatar::picture;
use std::collections::HashSet;

const NAMES: usize = 400;

fn main() {
    let pictures: Vec<_> = (0..NAMES)
        .map(|t| picture(format!("user-{t}").as_bytes()))
        .collect();
    let filled: Vec<usize> = pictures
        .iter()
        .map(|p| p.cells.iter().filter(|&&on| on).count())
        .collect();
    let patterns: HashSet<_> = pictures.iter().map(|p| p.cells.clone()).collect();
    let inks: HashSet<_> = pictures.iter().map(|p| p.ink).collect();
    println!("{NAMES} names");
    println!(
        "filled cells    min {}, max {}",
        filled.iter().min().unwrap(),
        filled.iter().max().unwrap()
    );
    println!("distinct shapes {}", patterns.len());
    println!("distinct inks   {}", inks.len());
    let looks: HashSet<_> = pictures.iter().map(|p| (&p.cells, p.ink)).collect();
    println!("look-alikes     {}", NAMES - looks.len());
}
