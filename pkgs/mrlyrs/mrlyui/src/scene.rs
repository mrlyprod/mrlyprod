use crate::frame::{field, Frame};
use crate::raster::{fill, line, Seg, Tri};
use mrlymath::space::{Camera, Vec3};

pub struct Face {
    pub verts: [Vec3; 3],
    pub normal: Vec3,
    pub color: [u8; 4],
}

pub struct Edge {
    pub ends: [Vec3; 2],
    pub color: [u8; 4],
}

#[derive(Default)]
pub struct Scene {
    pub faces: Vec<Face>,
    pub edges: Vec<Edge>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            faces: Vec::new(),
            edges: Vec::new(),
        }
    }
    pub fn face(&mut self, verts: [Vec3; 3], normal: Vec3, color: [u8; 4]) {
        self.faces.push(Face {
            verts,
            normal,
            color,
        });
    }
    pub fn quad(&mut self, verts: [Vec3; 4], normal: Vec3, color: [u8; 4]) {
        self.face([verts[0], verts[1], verts[2]], normal, color);
        self.face([verts[0], verts[2], verts[3]], normal, color);
    }
    pub fn edge(&mut self, a: Vec3, b: Vec3, color: [u8; 4]) {
        self.edges.push(Edge {
            ends: [a, b],
            color,
        });
    }
    pub fn paint(&self, cam: &Camera, size: usize, background: [u8; 4]) -> Frame {
        let eye = cam.view();
        let mut solid = Vec::new();
        let mut glass = Vec::new();
        for f in &self.faces {
            let n_eye = eye.apply(f.normal);
            let us = f.verts.map(|v| eye.apply(v));
            if f.color[3] == 255 && !cam.facing(n_eye, us[0]) {
                continue;
            }
            let Some(ps) = us
                .iter()
                .map(|&u| cam.project(u, size as f32))
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            let tri = Tri {
                x: [ps[0][0], ps[1][0], ps[2][0]],
                y: [ps[0][1], ps[1][1], ps[2][1]],
                z: [ps[0][2], ps[1][2], ps[2][2]],
                color: f.color,
            };
            if f.color[3] == 255 {
                solid.push(tri);
            } else {
                glass.push(((ps[0][2] + ps[1][2] + ps[2][2]) / 3.0, tri));
            }
        }
        glass.sort_by(|a, b| b.0.total_cmp(&a.0));
        let mut buf = vec![background; size * size];
        let mut zbuf = vec![f32::MAX; size * size];
        for t in &solid {
            fill(t, size, size, &mut buf, &mut zbuf, true);
        }
        for (_, t) in &glass {
            fill(t, size, size, &mut buf, &mut zbuf, false);
        }
        for e in &self.edges {
            let a = eye.apply(e.ends[0]);
            let b = eye.apply(e.ends[1]);
            let (Some(pa), Some(pb)) = (cam.project(a, size as f32), cam.project(b, size as f32))
            else {
                continue;
            };
            let seg = Seg {
                x: [pa[0], pb[0]],
                y: [pa[1], pb[1]],
                z: [pa[2], pb[2]],
                color: e.color,
            };
            line(&seg, size, size, &mut buf, &mut zbuf);
        }
        field(size, size, buf, background)
    }
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

pub fn axes(scene: &mut Scene, ink: [u8; 4]) {
    for e in axis_edges(ink) {
        scene.edge(e.ends[0], e.ends[1], e.color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: [u8; 4] = [255, 0, 0, 255];
    const BG: [u8; 4] = [0, 0, 0, 255];

    fn cam() -> Camera {
        Camera {
            yaw: 0,
            pitch: 0,
            dist: 3.0,
            pan: [0.0, 0.0],
            ortho: false,
        }
    }
    fn pixels(frame: &Frame) -> Vec<[u8; 4]> {
        frame.composite().cell.colors.unwrap()
    }
    fn toward() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }
    fn square(scene: &mut Scene, z: f32, color: [u8; 4]) {
        scene.quad(
            [
                Vec3::new(-0.8, -0.8, z),
                Vec3::new(0.8, -0.8, z),
                Vec3::new(0.8, 0.8, z),
                Vec3::new(-0.8, 0.8, z),
            ],
            toward(),
            color,
        );
    }

    #[test]
    fn solid_faces_paint_like_the_raster() {
        let mut scene = Scene::new();
        square(&mut scene, 0.0, RED);
        let px = pixels(&scene.paint(&cam(), 16, BG));
        assert_eq!(px[8 * 16 + 8], RED);
        assert_eq!(px[0], BG);
    }
    #[test]
    fn backfaces_cull_only_when_solid() {
        let mut scene = Scene::new();
        scene.quad(
            [
                Vec3::new(-0.8, -0.8, 0.0),
                Vec3::new(0.8, -0.8, 0.0),
                Vec3::new(0.8, 0.8, 0.0),
                Vec3::new(-0.8, 0.8, 0.0),
            ],
            Vec3::new(0.0, 0.0, -1.0),
            RED,
        );
        let px = pixels(&scene.paint(&cam(), 16, BG));
        assert!(px.iter().all(|&c| c == BG));
        scene.faces.clear();
        scene.quad(
            [
                Vec3::new(-0.8, -0.8, 0.0),
                Vec3::new(0.8, -0.8, 0.0),
                Vec3::new(0.8, 0.8, 0.0),
                Vec3::new(-0.8, 0.8, 0.0),
            ],
            Vec3::new(0.0, 0.0, -1.0),
            [255, 0, 0, 128],
        );
        let px = pixels(&scene.paint(&cam(), 16, BG));
        assert!(px[8 * 16 + 8][0] > 0);
    }
    #[test]
    fn glass_blends_far_to_near() {
        let mut scene = Scene::new();
        square(&mut scene, 0.4, [255, 0, 0, 128]);
        square(&mut scene, -0.4, [0, 0, 255, 128]);
        let center = pixels(&scene.paint(&cam(), 16, BG))[8 * 16 + 8];
        let mut swapped = Scene::new();
        square(&mut swapped, -0.4, [0, 0, 255, 128]);
        square(&mut swapped, 0.4, [255, 0, 0, 128]);
        assert_eq!(pixels(&swapped.paint(&cam(), 16, BG))[8 * 16 + 8], center);
        assert!(center[0] > 100);
        assert!(center[2] > 20);
    }
    #[test]
    fn glass_hides_behind_solid() {
        let mut scene = Scene::new();
        square(&mut scene, 0.4, RED);
        square(&mut scene, -0.4, [0, 0, 255, 128]);
        assert_eq!(pixels(&scene.paint(&cam(), 16, BG))[8 * 16 + 8], RED);
    }
    #[test]
    fn edges_draw_over_their_faces() {
        let mut scene = Scene::new();
        square(&mut scene, 0.0, RED);
        let ink = [0, 255, 0, 255];
        scene.edge(Vec3::new(-0.8, 0.0, 0.0), Vec3::new(0.8, 0.0, 0.0), ink);
        let px = pixels(&scene.paint(&cam(), 16, BG));
        assert_eq!(px[8 * 16 + 8], ink);
    }
    #[test]
    fn axes_land_lines() {
        let mut scene = Scene::new();
        axes(&mut scene, [255, 255, 255, 255]);
        let px = pixels(&scene.paint(&cam(), 32, BG));
        assert!(px.iter().filter(|&&c| c != BG).count() > 20);
    }
    #[test]
    fn the_axes_share_one_segment_source() {
        let ink = [1, 2, 3, 255];
        let mut scene = Scene::new();
        axes(&mut scene, ink);
        let edges = axis_edges(ink);
        assert_eq!(scene.edges.len(), edges.len());
        assert_eq!(scene.edges[0].ends, edges[0].ends);
        assert_eq!(scene.edges[16].color, edges[16].color);
    }
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
