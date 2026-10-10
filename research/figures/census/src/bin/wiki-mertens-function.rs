use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;

const NAME: &str = "wiki-mertens-function";
const TOP: usize = 1000;

fn trim(v: f64) -> f64 {
    (v * 1e9).round() / 1e9 + 0.0
}

fn main() -> Result<()> {
    let bound: Vec<f64> = (1..=TOP).map(|n| trim((n as f64).sqrt())).collect();
    save(NAME, &json!({"bound": bound}))
}
