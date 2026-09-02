use mrlycore::Map;
use mrlycore::{json, Json};

fn main() {
    let mut out = Map::new();
    for c in mrlyfont::supported() {
        let anim = mrlyfont::animate(&c.to_string(), 1);
        out.insert(
            c.to_string(),
            json!({
                "rows": anim.rows,
                "cols": anim.cols,
                "frames": anim.frames,
            }),
        );
    }
    println!("{}", Json::Object(out));
}
