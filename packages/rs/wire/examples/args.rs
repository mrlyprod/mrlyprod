// Pack call arguments in the wire format, split them, and read them back.
// This is what a door receives from the host (Python, JavaScript, WASM).
// The other direction, Out, writes into a buffer that only the host reads.
// cargo run -p mrly-wire --example args
use mrly_wire::{ARR, Args, F64, MAP, SMALL, STR, split};

fn text(value: &str) -> Vec<u8> {
    let mut out = vec![STR];
    out.extend((value.len() as u32).to_le_bytes());
    out.extend(value.as_bytes());
    out
}

fn float(value: f64) -> Vec<u8> {
    let mut out = vec![F64];
    out.extend(value.to_le_bytes());
    out
}

fn array(items: &[Vec<u8>]) -> Vec<u8> {
    let mut out = vec![ARR];
    out.extend((items.len() as u32).to_le_bytes());
    items.iter().for_each(|item| out.extend(item));
    out
}

fn map(pairs: &[(&str, Vec<u8>)]) -> Vec<u8> {
    let mut out = vec![MAP];
    out.extend((pairs.len() as u32).to_le_bytes());
    for (key, value) in pairs {
        out.extend(text(key));
        out.extend(value);
    }
    out
}

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    // Integers 0..=127 take one byte: the SMALL flag 0x80 plus the number.
    let parts = [
        vec![SMALL | 42],
        float(2.5),
        text("hi"),
        array(&[vec![SMALL | 1], vec![SMALL | 2], vec![SMALL | 3]]),
        map(&[("x", vec![SMALL | 7]), ("name", text("ada"))]),
    ];

    // A call is one count byte, then each argument as a 4-byte length and its bytes.
    let mut packed = vec![parts.len() as u8];
    for part in &parts {
        packed.extend((part.len() as u32).to_le_bytes());
        packed.extend(part);
    }
    println!("{} bytes packed", packed.len());

    // split cuts the packed bytes back into arguments (at most 24).
    let mut slots: [&[u8]; 24] = [&[]; 24];
    let count = split(&packed, &mut slots);
    let args = Args::new(&slots[..count]);
    println!("{} arguments", args.len());
    println!();

    // Each argument is read by the type you expect. A wrong guess gives None.
    println!(
        "0  {:<30} number {:?}",
        hex(args[0].bytes()),
        args[0].number()
    );
    println!(
        "1  {:<30} number {:?}",
        hex(args[1].bytes()),
        args[1].number()
    );
    println!(
        "2  {:<30} text   {:?}",
        hex(args[2].bytes()),
        args[2].text()
    );
    let list: Vec<f64> = args[3]
        .items()
        .unwrap()
        .iter()
        .filter_map(|a| a.number())
        .collect();
    println!("3  {:<30} items  {:?}", hex(args[3].bytes()), list);
    // A map keeps its keys as text, in the order written.
    let fields = args[4].fields().unwrap();
    println!(
        "4  map of {} fields, {} bytes",
        fields.len(),
        args[4].bytes().len()
    );
    for (key, value) in fields {
        println!("     {key} = {:?} {:?}", value.number(), value.text());
    }
    println!();
    println!("text of a number: {:?}", args[0].text());
    println!("a missing argument is null: {}", args[9].is_null());
}
