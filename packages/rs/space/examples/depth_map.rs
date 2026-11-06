// A depth map of the level 2 sponge: the nearer a surface, the denser its character.
// The grey runs from black at the back of the bounding sphere to white at its front, so it is the same
// scale from every angle. Two pixel rows make one text row, because a character is twice as tall as wide.
// cargo run -p mrly-space --example depth_map
use mrly_color::ALPHA;
use mrly_space::{View, depth, mesh};
use mrly_three::carpet;

const RAMP: &[u8] = b" .:-=+*#%@";

fn main() {
    let view = View {
        width: 64,
        height: 64,
        background: ALPHA,
        ..View::default()
    };
    let image = depth(&mesh(&carpet(3, 2).unwrap()), &view).unwrap();
    for y in (0..image.height).step_by(2) {
        let row: String = (0..image.width)
            .map(|x| {
                let [top, bottom] = [y, y + 1].map(|y| image.colors[y * image.width + x]);
                if top[3] as u32 + (bottom[3] as u32) < 255 {
                    return ' ';
                }
                let grey = (top[0] as usize + bottom[0] as usize) / 2;
                RAMP[1 + grey * (RAMP.len() - 1) / 256] as char
            })
            .collect();
        println!("{}", row.trim_end());
    }
}
