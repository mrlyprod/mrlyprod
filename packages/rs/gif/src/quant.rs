use crate::map::Near;

pub(crate) const EMPTY: u32 = u32::MAX;
const ROUNDS: usize = 6;

pub(crate) fn opaque(pixel: [u8; 4]) -> bool {
    pixel[3] >= 128
}

pub(crate) fn key(pixel: [u8; 4]) -> u32 {
    (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | pixel[2] as u32
}

fn rgb(key: u32) -> [u8; 3] {
    [(key >> 16) as u8, (key >> 8) as u8, key as u8]
}

struct Counter {
    keys: Vec<u32>,
    ranks: Vec<u32>,
    counts: Vec<u64>,
    order: Vec<u32>,
    cap: usize,
}

impl Counter {
    fn new(cap: usize) -> Counter {
        let size = (cap * 4).next_power_of_two();
        Counter {
            keys: vec![EMPTY; size],
            ranks: vec![0; size],
            counts: vec![0; size],
            order: Vec::with_capacity(cap),
            cap,
        }
    }
    fn add(&mut self, key: u32) -> Option<usize> {
        let mask = self.keys.len() - 1;
        let mut slot = (key.wrapping_mul(0x9E37_79B1) >> 8) as usize & mask;
        loop {
            if self.keys[slot] == key {
                self.counts[slot] += 1;
                return Some(self.ranks[slot] as usize);
            }
            if self.keys[slot] == EMPTY {
                if self.order.len() == self.cap {
                    return None;
                }
                self.keys[slot] = key;
                self.counts[slot] = 1;
                self.ranks[slot] = self.order.len() as u32;
                self.order.push(slot as u32);
                return Some(self.order.len() - 1);
            }
            slot = (slot + 1) & mask;
        }
    }
}

pub(crate) type Indexed = (Vec<[u8; 3]>, Vec<Vec<u8>>);

pub(crate) fn exact(frames: &[&[[u8; 4]]], clear: bool, room: usize) -> Option<Indexed> {
    let first = clear as usize;
    let mut counter = Counter::new(room);
    let mut out = Vec::with_capacity(frames.len());
    for frame in frames {
        let mut indices = Vec::with_capacity(frame.len());
        let mut last = (EMPTY, 0);
        for &pixel in frame.iter() {
            if !opaque(pixel) {
                indices.push(0);
                continue;
            }
            let k = key(pixel);
            if k != last.0 {
                last = (k, (counter.add(k)? + first) as u8);
            }
            indices.push(last.1);
        }
        out.push(indices);
    }
    let mut palette = vec![[0, 0, 0]; first];
    palette.extend(
        counter
            .order
            .iter()
            .map(|&slot| rgb(counter.keys[slot as usize])),
    );
    Some((palette, out))
}

struct Point {
    color: [f64; 3],
    weight: f64,
}

fn points(frames: &[&[[u8; 4]]]) -> Vec<Point> {
    let mut counter = Counter::new(1 << 16);
    let mut full = true;
    'scan: for frame in frames {
        for &pixel in frame.iter() {
            if opaque(pixel) && counter.add(key(pixel)).is_none() {
                full = false;
                break 'scan;
            }
        }
    }
    if full {
        return counter
            .order
            .iter()
            .map(|&slot| {
                let [r, g, b] = rgb(counter.keys[slot as usize]);
                Point {
                    color: [r as f64, g as f64, b as f64],
                    weight: counter.counts[slot as usize] as f64,
                }
            })
            .collect();
    }
    let mut bins = vec![(0u64, [0u64; 3]); 1 << 15];
    for frame in frames {
        for &[r, g, b, a] in frame.iter() {
            if a < 128 {
                continue;
            }
            let bin = &mut bins[(r as usize >> 3) << 10 | (g as usize >> 3) << 5 | b as usize >> 3];
            bin.0 += 1;
            bin.1[0] += r as u64;
            bin.1[1] += g as u64;
            bin.1[2] += b as u64;
        }
    }
    bins.into_iter()
        .filter(|bin| bin.0 > 0)
        .map(|(count, sums)| Point {
            color: sums.map(|sum| sum as f64 / count as f64),
            weight: count as f64,
        })
        .collect()
}

fn moments(points: &[Point]) -> (f64, [f64; 3], f64) {
    let mut weight = 0.0;
    let mut sum = [0.0; 3];
    let mut square = 0.0;
    for point in points {
        weight += point.weight;
        for (total, c) in sum.iter_mut().zip(point.color) {
            *total += point.weight * c;
            square += point.weight * c * c;
        }
    }
    (weight, sum, square)
}

fn spread(points: &[Point]) -> f64 {
    let (weight, sum, square) = moments(points);
    if weight == 0.0 {
        return 0.0;
    }
    square - sum.iter().map(|s| s * s).sum::<f64>() / weight
}

fn mean(points: &[Point]) -> [f64; 3] {
    let (weight, sum, _) = moments(points);
    sum.map(|s| s / weight.max(f64::MIN_POSITIVE))
}

fn widest(points: &[Point]) -> usize {
    let (weight, sum, _) = moments(points);
    let mut best = (0, -1.0);
    for (axis, total) in sum.iter().enumerate() {
        let centre = total / weight;
        let spread: f64 = points
            .iter()
            .map(|p| p.weight * (p.color[axis] - centre).powi(2))
            .sum();
        if spread > best.1 {
            best = (axis, spread);
        }
    }
    best.0
}

fn split(points: &[Point]) -> usize {
    let (total, sums, _) = moments(points);
    let mut weight = 0.0;
    let mut sum = [0.0; 3];
    let mut best = (1, f64::MIN);
    for (k, point) in points[..points.len() - 1].iter().enumerate() {
        weight += point.weight;
        for (total, c) in sum.iter_mut().zip(point.color) {
            *total += point.weight * c;
        }
        let rest = total - weight;
        let left: f64 = sum.iter().map(|s| s * s).sum::<f64>() / weight;
        let right: f64 =
            (0..3).map(|a| (sums[a] - sum[a]).powi(2)).sum::<f64>() / rest.max(f64::MIN_POSITIVE);
        if left + right > best.1 {
            best = (k + 1, left + right);
        }
    }
    best.0
}

fn cut(points: &mut [Point], count: usize) -> Vec<[f64; 3]> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut boxes = vec![(0, points.len(), spread(points))];
    while boxes.len() < count {
        let Some(pick) = (0..boxes.len())
            .filter(|&i| boxes[i].1 - boxes[i].0 > 1 && boxes[i].2 > 0.0)
            .max_by(|&a, &b| boxes[a].2.total_cmp(&boxes[b].2).then(b.cmp(&a)))
        else {
            break;
        };
        let (lo, hi, _) = boxes[pick];
        let part = &mut points[lo..hi];
        let axis = widest(part);
        part.sort_by(|a, b| a.color[axis].total_cmp(&b.color[axis]));
        let k = split(part);
        boxes[pick] = (lo, lo + k, spread(&part[..k]));
        boxes.push((lo + k, hi, spread(&part[k..])));
    }
    boxes
        .iter()
        .map(|&(lo, hi, _)| mean(&points[lo..hi]))
        .collect()
}

fn round(color: [f64; 3]) -> [u8; 3] {
    color.map(|c| c.round().clamp(0.0, 255.0) as u8)
}

fn refine(points: &[Point], centres: &mut [[f64; 3]]) {
    for _ in 0..ROUNDS {
        let palette: Vec<[u8; 3]> = centres.iter().map(|&c| round(c)).collect();
        let near = Near::new(&palette);
        let mut sums = vec![(0.0, [0.0; 3]); centres.len()];
        for point in points {
            let at = near.find(round(point.color).map(|c| c as i32)) as usize;
            sums[at].0 += point.weight;
            for (total, c) in sums[at].1.iter_mut().zip(point.color) {
                *total += point.weight * c;
            }
        }
        for (centre, (weight, sum)) in centres.iter_mut().zip(sums) {
            if weight > 0.0 {
                *centre = sum.map(|s| s / weight);
            }
        }
    }
}

pub(crate) fn median(frames: &[&[[u8; 4]]], count: usize) -> Vec<[u8; 3]> {
    let mut points = points(frames);
    let mut centres = cut(&mut points, count);
    refine(&points, &mut centres);
    sort(centres.into_iter().map(round).collect())
}

fn sort(mut palette: Vec<[u8; 3]>) -> Vec<[u8; 3]> {
    palette.sort_by_key(|&[r, g, b]| (299 * r as u32 + 587 * g as u32 + 114 * b as u32, [r, g, b]));
    palette.dedup();
    palette
}
