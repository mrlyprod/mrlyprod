pub(crate) struct Flat {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pixels: Vec<[u8; 4]>,
    depth: Vec<f64>,
}

fn edge(a: [f64; 3], b: [f64; 3], p: (f64, f64)) -> f64 {
    (b[0] - a[0]) * (p.1 - a[1]) - (b[1] - a[1]) * (p.0 - a[0])
}

fn byte(v: f64) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn clipped(a: [f64; 3], b: [f64; 3], width: f64, height: f64) -> Option<([f64; 3], [f64; 3])> {
    let run = [b[0] - a[0], b[1] - a[1]];
    if !run[0].is_finite() || !run[1].is_finite() || !a[0].is_finite() || !a[1].is_finite() {
        return None;
    }
    let (mut near, mut far) = (0.0f64, 1.0f64);
    for (slope, room) in [
        (-run[0], a[0]),
        (run[0], width - a[0]),
        (-run[1], a[1]),
        (run[1], height - a[1]),
    ] {
        if slope == 0.0 {
            if room < 0.0 {
                return None;
            }
            continue;
        }
        let cut = room / slope;
        match slope < 0.0 {
            true if cut > far => return None,
            true => near = near.max(cut),
            false if cut < near => return None,
            false => far = far.min(cut),
        }
    }
    let at = |t: f64| {
        [
            a[0] + run[0] * t,
            a[1] + run[1] * t,
            a[2] + (b[2] - a[2]) * t,
        ]
    };
    Some((at(near), at(far)))
}

impl Flat {
    pub(crate) fn new(width: usize, height: usize, board: [u8; 4]) -> Flat {
        Flat {
            width,
            height,
            pixels: vec![board; width * height],
            depth: vec![1.0; width * height],
        }
    }

    fn paint(&mut self, i: usize, color: [f64; 3], alpha: f64) {
        let old = self.pixels[i];
        let keep = 1.0 - alpha;
        let over = |src: f64, dst: u8| byte(src * alpha + dst as f64 / 255.0 * keep);
        self.pixels[i] = [
            over(color[0], old[0]),
            over(color[1], old[1]),
            over(color[2], old[2]),
            255,
        ];
    }

    pub(crate) fn tri(
        &mut self,
        points: [[f64; 3]; 3],
        color: [f64; 3],
        alpha: f64,
        test: bool,
        write: bool,
    ) {
        if alpha <= 0.0 {
            return;
        }
        let area = edge(points[0], points[1], (points[2][0], points[2][1]));
        if area == 0.0 {
            return;
        }
        let xs = [points[0][0], points[1][0], points[2][0]];
        let ys = [points[0][1], points[1][1], points[2][1]];
        let low = |v: &[f64; 3]| v.iter().cloned().fold(f64::MAX, f64::min).floor().max(0.0);
        let high = |v: &[f64; 3]| v.iter().cloned().fold(f64::MIN, f64::max).ceil().max(0.0);
        let x0 = low(&xs) as usize;
        let x1 = (high(&xs) as usize).min(self.width);
        let y0 = low(&ys) as usize;
        let y1 = (high(&ys) as usize).min(self.height);
        for py in y0..y1 {
            for px in x0..x1 {
                let p = (px as f64 + 0.5, py as f64 + 0.5);
                let e0 = edge(points[0], points[1], p);
                let e1 = edge(points[1], points[2], p);
                let e2 = edge(points[2], points[0], p);
                let inside =
                    (e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0) || (e0 <= 0.0 && e1 <= 0.0 && e2 <= 0.0);
                if !inside {
                    continue;
                }
                let i = py * self.width + px;
                let z = (e1 * points[0][2] + e2 * points[1][2] + e0 * points[2][2]) / area;
                if test && z >= self.depth[i] {
                    continue;
                }
                if write {
                    self.depth[i] = z;
                }
                self.paint(i, color, alpha);
            }
        }
    }

    pub(crate) fn line(
        &mut self,
        a: [f64; 3],
        b: [f64; 3],
        color: [f64; 3],
        alpha: f64,
        test: bool,
        write: bool,
    ) {
        if alpha <= 0.0 {
            return;
        }
        let Some((a, b)) = clipped(a, b, self.width as f64, self.height as f64) else {
            return;
        };
        let run = (b[0] - a[0]).abs().max((b[1] - a[1]).abs());
        let steps = run.ceil().max(1.0);
        let count = steps as usize;
        let mut last = usize::MAX;
        for s in 0..=count {
            let t = s as f64 / steps;
            let x = a[0] + (b[0] - a[0]) * t;
            let y = a[1] + (b[1] - a[1]) * t;
            let z = a[2] + (b[2] - a[2]) * t;
            if x < 0.0 || y < 0.0 {
                continue;
            }
            let (px, py) = (x.floor() as usize, y.floor() as usize);
            if px >= self.width || py >= self.height {
                continue;
            }
            let i = py * self.width + px;
            if i == last {
                continue;
            }
            last = i;
            if test && z >= self.depth[i] {
                continue;
            }
            if write {
                self.depth[i] = z;
            }
            self.paint(i, color, alpha);
        }
    }
}
