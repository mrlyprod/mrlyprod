use mrly_error::{Result, value_error};
use mrly_three::{Cell3d, Vec3, quads};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mesh {
    pub verts: Vec<Vec3>,
    pub faces: Vec<[usize; 3]>,
    pub normals: Vec<Vec3>,
    pub colors: Option<Vec<[u8; 4]>>,
}

impl Mesh {
    pub(crate) fn check(&self) -> Result<()> {
        if self.normals.len() != self.faces.len() {
            return value_error("a mesh wants one normal per face.");
        }
        if self
            .colors
            .as_ref()
            .is_some_and(|c| c.len() != self.faces.len())
        {
            return value_error("a mesh wants one color per face.");
        }
        if let Some(i) = self
            .faces
            .iter()
            .position(|f| f.iter().any(|&v| v >= self.verts.len()))
        {
            return value_error(format!("face {i} points past the vertices."));
        }
        let finite = |v: &Vec3| v.x.is_finite() && v.y.is_finite() && v.z.is_finite();
        if !self.verts.iter().all(finite) || !self.normals.iter().all(finite) {
            return value_error("a mesh wants finite vertices and normals.");
        }
        Ok(())
    }
}

pub fn mesh(cell: &Cell3d) -> Mesh {
    let grid = cell.types();
    let half = grid.shape.iter().copied().max().unwrap_or(0) as f64 / 2.0;
    let voxel = |c: f32, n: f32| (c as f64 * half + half - n as f64 / 2.0 - 0.5).round() as usize;
    let painted = cell.cell.colors.as_ref().filter(|c| c.len() == grid.size());
    let mut out = Mesh {
        verts: Vec::new(),
        faces: Vec::new(),
        normals: Vec::new(),
        colors: painted.map(|_| Vec::new()),
    };
    for quad in quads(cell) {
        let at = out.verts.len();
        out.verts.extend(quad.verts);
        out.faces
            .extend([[at, at + 1, at + 2], [at, at + 2, at + 3]]);
        out.normals.extend([quad.normal; 2]);
        if let (Some(colors), Some(painted)) = (&mut out.colors, painted) {
            let ([a, _, b, _], n) = (quad.verts, quad.normal);
            let c = (a + b).scale(0.5);
            let flat = grid.index(&[voxel(c.x, n.x), voxel(c.y, n.y), voxel(c.z, n.z)]);
            colors.extend([painted[flat]; 2]);
        }
    }
    out
}
