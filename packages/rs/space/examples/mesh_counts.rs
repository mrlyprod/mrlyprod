// What a Mesh holds: corners, triangles, and the edges between them.
// Squares and pentagons are split into triangles, which adds as many edges as faces, so every closed
// solid still has corners - edges + faces = 2 (Euler).
// A cell's mesh has two triangles for every unit square of its surface, as census counts it.
// cargo run -p mrly-space --example mesh_counts
use mrly_space::{SOLIDS, mesh, solid};
use mrly_three::{carpet, census};
use std::collections::BTreeSet;

fn main() {
    println!("solid   corners  edges  triangles  euler");
    for name in SOLIDS {
        let m = solid(name).unwrap();
        let edges: BTreeSet<(usize, usize)> = m
            .faces
            .iter()
            .flat_map(|&[a, b, c]| [(a, b), (b, c), (c, a)])
            .map(|(a, b)| (a.min(b), a.max(b)))
            .collect();
        let (v, e, f) = (m.verts.len(), edges.len(), m.faces.len());
        let euler = v as i64 - e as i64 + f as i64;
        println!("{name:<7} {v:>7} {e:>6} {f:>10} {euler:>6}");
    }
    println!();
    println!("sponge  surface  triangles");
    for level in 1..=3 {
        let sponge = carpet(3, level).unwrap();
        let surface = census(&sponge).unwrap().surface;
        let triangles = mesh(&sponge).faces.len();
        println!("{level:>6} {surface:>8} {triangles:>10}");
    }
}
