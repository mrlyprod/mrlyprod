use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;

const NAME: &str = "wiki-prime-counting-function";
const TOP: usize = 200;
const SAMPLES: usize = 1200;

fn trim(v: f64) -> f64 {
    (v * 1e9).round() / 1e9 + 0.0
}

fn main() -> Result<()> {
    let left = 2.0;
    let right = (TOP + 1) as f64;
    let guess: Vec<f64> = (0..=SAMPLES)
        .map(|k| {
            let x = left + (right - left) * k as f64 / SAMPLES as f64;
            trim(x / x.ln())
        })
        .collect();
    save(NAME, &json!({"guess": guess}))
}
