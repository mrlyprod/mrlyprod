use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;

const NAME: &str = "wiki-spectral-radius";
const ARROWS: usize = 8;
const MATRIX: [[f64; 2]; 2] = [[2.0, 1.0], [1.0, 3.0]];
const START: (f64, f64) = (1.0, -0.6);

fn push(v: (f64, f64)) -> (f64, f64) {
    (
        MATRIX[0][0] * v.0 + MATRIX[0][1] * v.1,
        MATRIX[1][0] * v.0 + MATRIX[1][1] * v.1,
    )
}

fn main() -> Result<()> {
    let mut v = START;
    let mut angles = Vec::with_capacity(ARROWS);
    for _ in 0..ARROWS {
        angles.push(v.1.atan2(v.0));
        v = push(v);
    }
    let golden = (1.0 + 5f64.sqrt()) / 2.0;
    let settled = golden.atan2(1.0);
    assert_eq!(angles.len(), ARROWS);
    assert!(angles.windows(2).all(|pair| pair[1] > pair[0]));
    assert!((settled - angles[ARROWS - 1]).abs() < 0.10);
    assert!(angles[0] < 0.0);
    save(NAME, &json!({"angles": angles, "settled": settled}))
}
