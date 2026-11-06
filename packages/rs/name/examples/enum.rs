// named_enum! makes an enum with a word for each variant.
// cargo run -p mrly-name --example enum
use mrly_name::named_enum;

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Fruit {
        Apple => "apple",
        Fig => "fig",
        Plum => "plum",
    }
}

fn main() {
    // all() lists every variant in order, as a fixed-size array.
    for fruit in Fruit::all() {
        println!("{:?} is spelled {}", fruit, fruit.name());
    }
    // Display uses the word, and parse reads it back.
    println!("display {}", Fruit::Fig);
    println!("parse   {:?}", "plum".parse::<Fruit>().unwrap());
    // An unknown word is a value error naming the enum.
    println!("unknown {}", "kiwi".parse::<Fruit>().unwrap_err());
}
