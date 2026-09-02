use mrlycore::json;

const WORDMARK: &str = "MRLYPROD";
const PAD: usize = 1;

fn main() {
    let write = mrlyfont::animate(WORDMARK, PAD);
    let merged = mrlyfont::merge(WORDMARK, PAD);
    let anim = mrlyfont::cycle(&write, &merged, mrlyfont::HOLD);
    println!(
        "{}",
        json!({
            "rows": anim.rows,
            "cols": anim.cols,
            "fps": anim.fps,
            "frames": anim.frames,
        })
    );
}
