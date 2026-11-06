// Read JSON into a Rust struct, write it back, and see a type mismatch.
// cargo run -p mrly-json --example typed
use mrly_json::{parse, read, take, write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
    label: String,
}

fn main() {
    let text = r#"{"x": 1.5, "y": -2, "label": "start"}"#;

    // read: text straight to a type.
    let point: Point = read(text).unwrap();
    println!("read:  {point:?}");
    println!("write: {}", write(&point).unwrap());

    // take: a value you already parsed to a type.
    let value = parse(text).unwrap();
    let again: Point = take(&value).unwrap();
    println!("take:  {again:?}");

    // Failures come back as Error::Json with the serde message.
    let wrong = read::<Point>(r#"{"x": "one", "y": 0, "label": "bad"}"#);
    println!("wrong: {}", wrong.unwrap_err());
    let broken = parse("{\"x\": ");
    println!("broken: {}", broken.unwrap_err());
}
