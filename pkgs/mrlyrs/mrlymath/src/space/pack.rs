use super::vec::Vec3;

pub const MESH_WGSL: &str = include_str!("mesh.wgsl");

pub struct Edge {
    pub ends: [Vec3; 2],
    pub color: [u8; 4],
}

#[derive(Default)]
pub struct Pack {
    tris: Vec<f32>,
    lines: Vec<f32>,
}

impl Pack {
    pub fn new() -> Pack {
        Pack::default()
    }
    pub fn face(&mut self, verts: [Vec3; 3], normal: Vec3) {
        for v in verts {
            self.tris
                .extend([v.x, v.y, v.z, normal.x, normal.y, normal.z]);
        }
    }
    pub fn quad(&mut self, verts: [Vec3; 4], normal: Vec3) {
        self.face([verts[0], verts[1], verts[2]], normal);
        self.face([verts[0], verts[2], verts[3]], normal);
    }
    pub fn line(&mut self, a: Vec3, b: Vec3, spins: bool, color: [u8; 4]) {
        for v in [a, b] {
            self.lines
                .extend([v.x, v.y, v.z, if spins { 1.0 } else { 0.0 }]);
            self.lines.extend(color.map(|c| c as f32 / 255.0));
        }
    }
    pub fn buffer(self) -> Vec<f32> {
        let mut out = vec![self.tris.len() as f32, self.lines.len() as f32];
        out.extend(self.tris);
        out.extend(self.lines);
        out
    }
}

pub fn axis_edges(ink: [u8; 4]) -> Vec<Edge> {
    let o = Vec3::new(0.0, -1.0, 0.0);
    let mut out = vec![
        Edge {
            ends: [o, Vec3::new(1.4, -1.0, 0.0)],
            color: [220, 70, 70, 255],
        },
        Edge {
            ends: [o, Vec3::new(0.0, 0.4, 0.0)],
            color: [80, 190, 90, 255],
        },
        Edge {
            ends: [o, Vec3::new(0.0, -1.0, 1.4)],
            color: [80, 120, 230, 255],
        },
    ];
    let faint = [ink[0], ink[1], ink[2], 64];
    for k in 0..=6 {
        let c = -1.2 + 0.4 * k as f32;
        out.push(Edge {
            ends: [Vec3::new(c, -1.0, -1.2), Vec3::new(c, -1.0, 1.2)],
            color: faint,
        });
        out.push(Edge {
            ends: [Vec3::new(-1.2, -1.0, c), Vec3::new(1.2, -1.0, c)],
            color: faint,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pack_lays_out_the_wire_format() {
        let mut pack = Pack::new();
        pack.face(
            [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
            Vec3::new(0.0, 0.0, 1.0),
        );
        pack.line(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            true,
            [255, 0, 0, 128],
        );
        let buf = pack.buffer();
        assert_eq!(buf[0], 18.0);
        assert_eq!(buf[1], 16.0);
        assert_eq!(&buf[2..8], &[0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        assert_eq!(&buf[8..14], &[1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        assert_eq!(&buf[14..20], &[0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        let glass = 128.0 / 255.0;
        assert_eq!(&buf[20..28], &[0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, glass]);
        assert_eq!(&buf[28..36], &[0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, glass]);
    }
    #[test]
    fn the_pack_splits_quads_and_keeps_furniture_still() {
        let mut pack = Pack::new();
        pack.quad(
            [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
            Vec3::new(0.0, 0.0, 1.0),
        );
        pack.line(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            false,
            [0, 0, 0, 255],
        );
        let buf = pack.buffer();
        assert_eq!(buf[0], 36.0);
        assert_eq!(buf[2 + 36 + 3], 0.0);
        assert_eq!(buf[2 + 36 + 8 + 3], 0.0);
    }
    #[test]
    fn the_axes_stand_on_the_floor() {
        let edges = axis_edges([255, 255, 255, 255]);
        assert_eq!(edges.len(), 17);
        assert_eq!(edges[0].ends[0], Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(edges[16].color[3], 64);
    }
    #[test]
    fn the_shader_ships_with_the_wire_format() {
        assert!(MESH_WGSL.contains("fn vs_main"));
        assert!(MESH_WGSL.contains("fn vs_line"));
        assert!(MESH_WGSL.contains("var<uniform>"));
    }
}
