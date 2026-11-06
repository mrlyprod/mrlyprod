// A color as hex, CSS, name and alpha, and what bad input does.
// cargo run -p mrly-color --example formats
use mrly_color::{BLUE, Color, RED, named};

fn main() {
    println!("hex      {}", BLUE.to_hex());
    println!("css      {}", BLUE.css());

    // Alpha below 255 adds two more hex digits, and rgba() in CSS.
    let faint = RED.alpha(128);
    println!("alpha    {} {}", faint.to_hex(), faint.css());

    // from_hex takes #RRGGBB or #RRGGBBAA, with or without the #.
    println!("parse    {:?}", Color::from_hex("#008cff").unwrap());
    println!("parse    {:?}", Color::from_hex("ff3d4080").unwrap());

    // named looks up the fifteen palette names.
    println!("named    {}", named("indigo").unwrap().to_hex());
    println!("invert   {}", BLUE.invert().to_hex());

    println!("bad hex  {}", Color::from_hex("#12345").unwrap_err());
    println!("bad name {}", named("teal-ish").unwrap_err());
}
