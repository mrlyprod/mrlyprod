// A carpet can carry a message in its filled cells, then give it back.
// embed() writes the bytes into the filled sites; extract() reads them using the original as the key.
// cargo run -p mrly-two --example hidden_message

use mrly_two::{capacity, carpet, embed, extract};

fn main() {
    let carrier = carpet(3, 3).unwrap();
    println!("capacity: {} bytes", capacity(&carrier));
    let carried = embed(&carrier, b"hello carpet").unwrap();
    let bytes = extract(&carrier, &carried).unwrap();
    println!("read back: {}", String::from_utf8(bytes).unwrap());
}
