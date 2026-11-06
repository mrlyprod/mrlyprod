// Match the longest name that starts a piece of text.
// cargo run -p mrly-name --example longest
use mrly_name::longest;

fn main() {
    let names: Vec<String> = ["a", "ab", "abc", "b"].map(String::from).to_vec();
    // The result is (index into names, text left after the match).
    for text in ["abcd", "abx", "bb", "xyz", ""] {
        match longest(text, &names) {
            Some((i, rest)) => println!("{text:?} matches {:?}, rest {rest:?}", names[i]),
            None => println!("{text:?} matches nothing"),
        }
    }
}
