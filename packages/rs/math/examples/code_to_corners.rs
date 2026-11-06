// A code is a bit mask over the corners of a cell. Bit i set means corner i is filled.
// Code 11 on a 2x2 grid has bits 0, 1 and 3 set.
// cargo run -p mrly-math --example code_to_corners

use mrly_math::Code;
use mrly_math::factory::{code_to_corners, corners_to_code, total_codes};

fn main() {
    let code = Code::from(11u64);
    let corners = code_to_corners(code, 2, 2).unwrap();
    println!("code {code} in binary {:04b}", code.get());
    println!("filled corners: {corners:?}");
    println!("back to code: {}", corners_to_code(&corners, 2, 2));
    println!("codes in 2D base 2: {}", total_codes(2, 2).unwrap());
    println!("codes in 2D base 3: {}", total_codes(2, 3).unwrap());
    println!("codes in 3D base 2: {}", total_codes(3, 2).unwrap());
}
