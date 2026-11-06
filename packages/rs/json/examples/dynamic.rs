// Build, parse and edit a JSON value without a Rust type.
// cargo run -p mrly-json --example dynamic
use mrly_json::{Json, Map, json, parse, pretty, write};

fn main() {
    // json! builds a value in place. Key order is kept as written.
    let mut value = json!({"name": "ada", "tags": ["a", "b"], "age": 36});
    println!("built:   {}", write(&value).unwrap());

    // parse reads text into the same dynamic value.
    let parsed: Json = parse(r#"{"name":"ada","tags":["a","b"],"age":36}"#).unwrap();
    println!("equal:   {}", parsed == value);

    // An object is a Map you can change.
    let object: &mut Map = value.as_object_mut().unwrap();
    object.insert("age".into(), json!(37));
    object.remove("tags");
    println!("edited:  {}", write(&value).unwrap());
    println!("{}", pretty(&value).unwrap());
}
