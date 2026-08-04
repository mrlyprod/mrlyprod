use super::vec::{Mat3, Vec3};
use mrlycore::trig;

pub const TURN: i64 = trig::N as i64;
pub const NEAR: f32 = 0.25;
pub const PITCH_MAX: i64 = 56;

pub fn view(yaw: i64, pitch: i64) -> Mat3 {
    Mat3::pitch(pitch) * Mat3::yaw(-yaw)
}

pub fn beam(yaw: i64, pitch: i64) -> Vec3 {
    let p = Mat3::pitch(-pitch).apply(Vec3::new(0.0, 0.0, 1.0));
    Mat3::yaw(yaw).apply(p)
}

pub fn project(u: Vec3, dist: f32, size: f32, focal: f32) -> Option<[f32; 3]> {
    let depth = dist - u.z;
    if depth < NEAR {
        return None;
    }
    let half = size / 2.0;
    Some([
        half + focal * u.x / depth,
        half - focal * u.y / depth,
        depth,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_origin_projects_to_center() {
        let p = project(Vec3::new(0.0, 0.0, 0.0), 3.0, 96.0, 115.2).unwrap();
        assert_eq!(p[0], 48.0);
        assert_eq!(p[1], 48.0);
        assert_eq!(p[2], 3.0);
    }
    #[test]
    fn points_behind_the_eye_clip() {
        assert!(project(Vec3::new(0.0, 0.0, 3.0), 3.0, 96.0, 115.2).is_none());
    }
    #[test]
    fn the_view_undoes_the_orbit() {
        let cam = Mat3::yaw(37) * Mat3::pitch(-21);
        let eye = cam.apply(Vec3::new(0.0, 0.0, 3.0));
        let u = view(37, 21).apply(eye);
        assert!(u.x.abs() < 1e-5 && u.y.abs() < 1e-5 && (u.z - 3.0).abs() < 1e-5);
    }
    #[test]
    fn positive_pitch_looks_from_above() {
        let cam = Mat3::yaw(0) * Mat3::pitch(-21);
        assert!(cam.apply(Vec3::new(0.0, 0.0, 3.0)).y > 0.0);
        assert!(beam(0, 21).y > 0.0);
    }
    #[test]
    fn beams_are_unit_length() {
        for (y, p) in [(0i64, 0i64), (40, 28), (200, -56), (128, 56)] {
            let b = beam(y, p);
            assert!((b.dot(b) - 1.0).abs() < 1e-5);
        }
    }
    #[test]
    fn the_default_beam_faces_the_camera() {
        assert_eq!(beam(0, 0), Vec3::new(0.0, 0.0, 1.0));
    }
}
