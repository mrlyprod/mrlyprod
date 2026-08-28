mod constants;
mod frame;
mod lattice;
mod pairs;
mod rays;
mod star;
mod sums;

use lattice::{Family, Rule};

fn header(title: &str) {
    println!();
    println!("== {title} ==");
}

fn main() {
    let carpet = Rule::new(Family::Carpet);
    let void = Rule::new(Family::Void);
    header("cut ink laws and the void centre");
    star::ink_laws(55);
    header("layer pair correlations, exact on the full hexagon");
    pairs::small_pairs(&carpet);
    pairs::doubling(&carpet);
    pairs::persistence();
    header("quarter line and crosshairs, carpet");
    frame::quarter(&carpet);
    header("void star");
    frame::void_star(&void);
    header("twisted averages and the coarse crosshair law");
    rays::twisted();
    rays::crosshairs();
    header("ghost star decay");
    star::ghost();
    frame::fading(&carpet);
    star::fading_lattice(&carpet);
    header("constants");
    constants::run();
}
