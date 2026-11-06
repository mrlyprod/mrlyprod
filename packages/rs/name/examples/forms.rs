// One value written as JSON, id, URL, file name and prose, then read back.
// A Named type carries a fixed "kind" first, so every form can be told apart.
// cargo run -p mrly-name --example forms
use mrly_error::{Result, value_error};
use mrly_name::Named;
use serde::{Deserialize, Serialize};

mod circle {
    mrly_name::kind!("circle");
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Circle {
    kind: circle::Kind,
    radius: u32,
    filled: bool,
    color: String,
    center: Vec<u32>,
}

impl Named for Circle {
    const KIND: &'static str = "circle";
    // Fields that hold lists, so a one-item list is not read back as a bare number.
    const LISTS: &'static [&'static str] = &["center"];
    // Fields printed as a bare word in prose: "circle blue", not "circle color blue".
    const BARE: &'static [&'static str] = &["color"];
    fn checked(self) -> Result<Self> {
        if self.radius == 0 {
            return value_error("a circle wants a radius above zero.");
        }
        Ok(self)
    }
}

fn main() {
    let circle = Circle {
        kind: circle::Kind,
        radius: 5,
        filled: true,
        color: "blue".into(),
        center: vec![2, 3],
    };
    println!("json   {}", circle.to_json());
    println!("id     {}", circle.to_id());
    println!("url    {}", circle.to_url().unwrap());
    println!("file   {}", circle.to_file().unwrap());
    println!("prose  {}", circle.to_prose().unwrap());

    // Every form reads back to the same value.
    let url = Circle::from_url(&circle.to_url().unwrap()).unwrap();
    let file = Circle::from_file(&circle.to_file().unwrap()).unwrap();
    let json = Circle::from_json(&circle.to_json()).unwrap();
    println!(
        "same   {}",
        url == circle && file == circle && json == circle
    );

    // checked() runs on every read.
    let bad = Circle::from_json(
        r#"{"kind":"circle","radius":0,"filled":false,"color":"red","center":[]}"#,
    );
    println!("check  {}", bad.unwrap_err());
    // The wrong kind is refused.
    println!(
        "kind   {}",
        Circle::from_url("/square?radius=3").unwrap_err()
    );
}
