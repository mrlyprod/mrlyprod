// Reading OBJ text the way other programs write it.
// Corners may carry texture and normal numbers after slashes, count backwards from the end with
// negative numbers, and long lines may be split with a backslash. Everything but v and f is skipped.
// A face may name a vertex that comes later in the file, but never one that is missing.
// cargo run -p mrly-obj --example read
use mrly_obj::{decode, encode};

const TEXT: &str = "# a square pyramid
o pyramid
v 0 0 0
v 1 0 0
v 1 1 0
v 0 1 0
v 0.5 0.5 1.25
vn 0 0 -1
f 1//1 2//1 3//1 4//1
f 1/1 2/2 -1/3
f 2 3 5
f 3 4 \\
  5
f -2 -5 -1
";

fn main() -> Result<(), String> {
    let (vertices, faces) = decode(TEXT)?;
    println!("{} vertices, faces {:?}", vertices.len(), faces);
    println!("written back:");
    print!("{}", encode(&vertices, &faces)?);
    for bad in ["v 1 2", "v 0 0 0\nf 1 2 3", "v 0 0 nan"] {
        println!("{:?}: {}", bad, decode(bad).unwrap_err());
    }
    Ok(())
}
