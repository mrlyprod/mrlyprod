// A cube as a Wavefront OBJ model: corner points (v) and faces (f) that name them, counting from 1.
// Six squares have 24 corners, but a cube has 8: `soup` welds the shared ones.
// cargo run -p mrly-obj --example cube
use mrly_obj::{encode, soup};

fn main() -> Result<(), String> {
    // Each face is a square: one axis held at 0 or 1, the other two walking round.
    let mut corners = Vec::new();
    for axis in 0..3 {
        for side in [0.0, 1.0] {
            let mut square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
            // Corners run anticlockwise seen from outside, so the near face is walked backwards.
            if side == 0.0 {
                square.reverse();
            }
            for (a, b) in square {
                let mut point = [0.0; 3];
                point[axis] = side;
                point[(axis + 1) % 3] = a;
                point[(axis + 2) % 3] = b;
                corners.push(point);
            }
        }
    }
    let (vertices, faces) = soup(&corners, 4)?;
    println!(
        "{} loose corners, {} welded vertices, {} faces",
        corners.len(),
        vertices.len(),
        faces.len()
    );
    print!("{}", encode(&vertices, &faces)?);
    Ok(())
}
