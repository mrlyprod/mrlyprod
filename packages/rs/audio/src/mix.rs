pub fn mix(tracks: &[Vec<f32>]) -> Vec<f32> {
    let longest = tracks.iter().map(Vec::len).max().unwrap_or(0);
    let mut out = vec![0.0f32; longest];
    for track in tracks {
        for (s, t) in out.iter_mut().zip(track) {
            *s += t;
        }
    }
    out
}

pub fn peak(samples: &[f32]) -> f32 {
    samples.iter().fold(0.0f32, |m, s| m.max(s.abs()))
}

pub fn normalize(samples: &[f32], peak: f32) -> Vec<f32> {
    let top = self::peak(samples);
    let k = if top > 0.0 { peak / top } else { 0.0 };
    samples.iter().map(|s| s * k).collect()
}
