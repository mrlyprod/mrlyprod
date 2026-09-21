use super::vec::Vec3;

/// A builder packing triangles and lines into one flat float buffer.
#[derive(Default)]
pub struct Pack {
    tris: Vec<f32>,
    lines: Vec<f32>,
}

impl Pack {
    /// Builds an empty pack.
    pub fn new() -> Pack {
        Pack::default()
    }
    /// Packs one triangle, writing position and normal per vertex.
    pub fn face(&mut self, verts: [Vec3; 3], normal: Vec3) {
        for v in verts {
            self.tris
                .extend([v.x, v.y, v.z, normal.x, normal.y, normal.z]);
        }
    }
    /// Packs a quad as two triangles sharing the normal.
    pub fn quad(&mut self, verts: [Vec3; 4], normal: Vec3) {
        self.face([verts[0], verts[1], verts[2]], normal);
        self.face([verts[0], verts[2], verts[3]], normal);
    }
    /// Packs one line, writing position, spin flag and color per endpoint.
    pub fn line(&mut self, a: Vec3, b: Vec3, spins: bool, color: [u8; 4]) {
        for v in [a, b] {
            self.lines
                .extend([v.x, v.y, v.z, if spins { 1.0 } else { 0.0 }]);
            self.lines.extend(color.map(|c| c as f32 / 255.0));
        }
    }
    /// Closes the pack into one buffer, the two section lengths first.
    pub fn buffer(self) -> Vec<f32> {
        let mut out = vec![self.tris.len() as f32, self.lines.len() as f32];
        out.extend(self.tris);
        out.extend(self.lines);
        out
    }
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
}
