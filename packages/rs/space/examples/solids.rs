// The five Platonic solids: tetrahedron, cube, octahedron, dodecahedron and icosahedron.
// Each face is one flat shade, set by how squarely it faces the light.
// cargo run -p mrly-space --example solids
use mrly_image::{Image, svg};
use mrly_space::{SOLIDS, View, draw, solid};

fn main() {
    let view = View {
        width: 32,
        height: 32,
        yaw: 20.0,
        pitch: 25.0,
        samples: 2,
        ..View::default()
    };
    let images: Vec<Image> = SOLIDS
        .iter()
        .map(|name| draw(&solid(name).unwrap(), &view).unwrap())
        .collect();
    println!("{}", svg(&images, 4, 4));
}
