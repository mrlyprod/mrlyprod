use crate::config::{Config, FPS, FREEZE_US, HEATMAP_FPS, INTER_FREEZE_US, SIZE};
use crate::frames::{frame_palette, heat_colorizer, span_peak};
use crate::music::{soundtrack, Score};
use crate::quest::{quest, Quest};
use crate::Path;
use mrlycore::errors::Result;
use mrlycore::io::{make, write};
use mrlycore::{audio, codec, json, state, Json};
use mrlymath::life::{frames_with, heatmap_range};
use std::path::Path as Dir;

/// The default seed an emit runs under.
pub const SEED: u64 = 7;

/// Generates a quest from the seed under the default caps and writes it into the directory.
pub fn emit(dir: &Dir, seed: u64) -> Result<Json> {
    emit_with(dir, seed, &Config::default())
}

/// Generates a quest from the seed and writes frames, heatmap, audio and record into the directory.
pub fn emit_with(dir: &Dir, seed: u64, config: &Config) -> Result<Json> {
    state::seed(seed);
    let found = quest(config)?;
    write_frames(dir, &found)?;
    write_heatmap(dir, &found)?;
    let scores: Vec<Score> = found.segments.iter().map(|s| s.score.clone()).collect();
    let samples = soundtrack(&scores, &found.story.chapter_lengths());
    write(
        &dir.join("audio.wav"),
        &codec::wav(&audio::pcm(&samples), audio::RATE),
    )?;
    let record = record(&found, seed)?;
    write(&dir.join("quest.json"), (record.pretty() + "\n").as_bytes())?;
    Ok(record)
}

fn write_frames(dir: &Dir, found: &Quest) -> Result<()> {
    let home = dir.join("frames");
    make(&home)?;
    let lengths = found.story.chapter_lengths();
    let mut cursor = 0;
    let mut number = 0;
    for (i, segment) in found.segments.iter().enumerate() {
        let span = &found.grids[cursor..cursor + lengths[i]];
        let wanted = segment.mask.types().sum() as usize + 1;
        let palette = frame_palette(segment.accent, segment.path == Path::Simple, wanted);
        let pngs = frames_with(
            span,
            &segment.mask,
            found.boundary,
            1,
            found.primary.color(),
            &palette,
        )?;
        for png in &pngs {
            number += 1;
            write(&home.join(format!("{number:04}.png")), png)?;
        }
        cursor += lengths[i];
    }
    Ok(())
}

fn write_heatmap(dir: &Dir, found: &Quest) -> Result<()> {
    let home = dir.join("heatmap");
    make(&home)?;
    let lengths = found.story.chapter_lengths();
    let mut cursor = 0;
    let mut number = 0;
    for (i, segment) in found.segments.iter().enumerate() {
        let end = cursor + lengths[i];
        let peak = span_peak(&found.grids[cursor..end]);
        let colorizer = heat_colorizer(found.primary, segment.accent, peak)?;
        let pngs = heatmap_range(&found.grids, cursor, end, &colorizer, 1)?;
        for png in &pngs {
            number += 1;
            write(&home.join(format!("{number:04}.png")), png)?;
        }
        cursor = end;
    }
    Ok(())
}

fn timeline(lengths: &[usize], frame_us: u64) -> Vec<Json> {
    let total: usize = lengths.iter().sum();
    if total == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(total + lengths.len() + 1);
    out.push(json!({ "frame": 1, "us": FREEZE_US }));
    let mut cursor = 0;
    for (i, &length) in lengths.iter().enumerate() {
        for frame in cursor..cursor + length {
            out.push(json!({ "frame": frame + 1, "us": frame_us }));
        }
        cursor += length;
        if i < lengths.len() - 1 {
            out.push(json!({ "frame": cursor, "us": INTER_FREEZE_US }));
        }
    }
    out.push(json!({ "frame": total, "us": FREEZE_US }));
    out
}

fn record(found: &Quest, seed: u64) -> Result<Json> {
    let lengths = found.story.chapter_lengths();
    Ok(json!({
        "v": 1,
        "key": found.key.clone(),
        "name": found.name(),
        "seed": seed.to_string(),
        "story": found.story.to_json()?,
        "manifest": {
            "fps": FPS,
            "heatmap_fps": HEATMAP_FPS,
            "canvas": found.grids.first().map_or(0, |g| g.width()),
            "size": SIZE,
            "audio": "audio.wav",
            "segments": lengths.clone(),
            "freeze_us": FREEZE_US,
            "inter_freeze_us": INTER_FREEZE_US,
            "frames": timeline(&lengths, 1_000_000 / FPS as u64),
            "heatmap": timeline(&lengths, 1_000_000 / HEATMAP_FPS as u64),
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::state::guard;
    use std::fs;

    const PINNED: u64 = 1;

    fn tiny() -> Config {
        Config {
            max_generations: 16,
            max_segments: 4,
            max_canvas: 15,
            min_tile: 3,
            max_tile: 5,
            min_mask: 3,
            max_mask: 5,
            attempts: 64,
        }
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("mrlygame_{name}_{}", std::process::id()))
    }

    #[test]
    fn two_emits_press_identical_bytes() {
        let _g = guard();
        let (a, b) = (scratch("emita"), scratch("emitb"));
        make(&a).unwrap();
        make(&b).unwrap();
        emit_with(&a, PINNED, &tiny()).unwrap();
        emit_with(&b, PINNED, &tiny()).unwrap();
        for name in ["quest.json", "audio.wav"] {
            let x = fs::read(a.join(name)).unwrap();
            let y = fs::read(b.join(name)).unwrap();
            assert_eq!(x, y, "{name} drifted between emits");
        }
        for folder in ["frames", "heatmap"] {
            let x = fs::read(a.join(folder).join("0001.png")).unwrap();
            let y = fs::read(b.join(folder).join("0001.png")).unwrap();
            assert_eq!(x, y, "{folder} drifted between emits");
        }
        fs::remove_dir_all(&a).ok();
        fs::remove_dir_all(&b).ok();
    }

    #[test]
    fn the_emitted_folder_holds_the_whole_product() {
        let _g = guard();
        let dir = scratch("press");
        make(&dir).unwrap();
        let record = emit_with(&dir, PINNED, &tiny()).unwrap();
        let count = record["manifest"]["segments"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n.as_u64().unwrap() as usize)
            .sum::<usize>();
        assert!(count > 0);
        for folder in ["frames", "heatmap"] {
            for frame in 1..=count {
                let path = dir.join(folder).join(format!("{frame:04}.png"));
                let bytes = fs::read(&path).unwrap();
                assert_eq!(&bytes[1..4], b"PNG");
            }
            assert!(!dir
                .join(folder)
                .join(format!("{:04}.png", count + 1))
                .exists());
        }
        let wav = fs::read(dir.join("audio.wav")).unwrap();
        assert_eq!(&wav[0..4], b"RIFF");
        assert!(wav.len() > 44 + audio::RATE);
        assert!(wav[44..].iter().any(|&b| b != 0));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn the_record_carries_the_assembly_manifest() {
        let _g = guard();
        let dir = scratch("record");
        make(&dir).unwrap();
        let record = emit_with(&dir, PINNED, &tiny()).unwrap();
        assert_eq!(record["v"], json!(1));
        let key = record["key"].as_str().unwrap().to_string();
        assert_eq!(record["name"].as_str(), Some(&*format!("mrly_quest_{key}")));
        assert_eq!(
            record["seed"].as_str().and_then(|s| s.parse::<u64>().ok()),
            Some(PINNED)
        );
        assert!(record["story"]["chapters"].as_array().is_some());
        let manifest = &record["manifest"];
        assert_eq!(manifest["fps"], json!(FPS));
        assert_eq!(manifest["heatmap_fps"], json!(HEATMAP_FPS));
        assert_eq!(manifest["size"], json!(SIZE));
        assert_eq!(manifest["audio"], json!("audio.wav"));
        assert_eq!(manifest["freeze_us"], json!(FREEZE_US));
        assert_eq!(manifest["inter_freeze_us"], json!(INTER_FREEZE_US));
        assert!(manifest["canvas"].as_u64().unwrap() > 0);
        let lengths: Vec<usize> = manifest["segments"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n.as_u64().unwrap() as usize)
            .collect();
        let total: usize = lengths.iter().sum();
        for (name, fps) in [("frames", FPS), ("heatmap", HEATMAP_FPS)] {
            let entries = manifest[name].as_array().unwrap();
            assert_eq!(entries.len(), total + lengths.len() + 1);
            assert_eq!(entries[0]["us"].as_u64(), Some(FREEZE_US));
            assert_eq!(entries[1]["us"].as_u64(), Some(1_000_000 / fps as u64));
            assert_eq!(
                entries.last().unwrap()["frame"].as_u64(),
                Some(total as u64)
            );
        }
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_top_bit_seed_survives_the_record_and_replays() {
        let _g = guard();
        let seed = u64::MAX - 41;
        let (a, b) = (scratch("bigseeda"), scratch("bigseedb"));
        make(&a).unwrap();
        make(&b).unwrap();
        let record = emit_with(&a, seed, &tiny()).unwrap();
        assert_eq!(record["seed"].as_str(), Some(seed.to_string().as_str()));
        let stored: u64 = record["seed"].as_str().unwrap().parse().unwrap();
        emit_with(&b, stored, &tiny()).unwrap();
        let x = fs::read(a.join("quest.json")).unwrap();
        let y = fs::read(b.join("quest.json")).unwrap();
        assert_eq!(x, y, "the recorded seed replayed a different quest");
        fs::remove_dir_all(&a).ok();
        fs::remove_dir_all(&b).ok();
    }

    #[test]
    fn timelines_hold_the_freeze_policy() {
        let frames = timeline(&[2, 3], 125_000);
        assert_eq!(frames.len(), 2 + 3 + 3);
        assert_eq!(frames[0], json!({ "frame": 1, "us": FREEZE_US }));
        assert_eq!(frames[3], json!({ "frame": 2, "us": INTER_FREEZE_US }));
        assert_eq!(
            frames.last().unwrap(),
            &json!({ "frame": 5, "us": FREEZE_US })
        );
        assert!(timeline(&[], 125_000).is_empty());
    }
}
