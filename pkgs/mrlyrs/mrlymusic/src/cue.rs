use super::theory::{freq, ROOT};
use serde_json::{json, Value as Json};

pub fn payload(name: &str) -> Json {
    let (offset, ms, gain) = match name {
        "good" => (31, 140, 0.3),
        "bad" => (13, 160, 0.3),
        "win" => (36, 320, 0.3),
        "lose" => (5, 380, 0.3),
        _ => (24, 90, 0.25),
    };
    json!({ "op": "note", "freq": freq(ROOT + offset), "ms": ms, "gain": gain })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cues_are_notes_without_wave() {
        for name in ["blip", "good", "bad", "win", "lose"] {
            let cue = payload(name);
            assert_eq!(cue["op"], "note");
            assert!(cue["freq"].as_f64().unwrap() > 0.0);
            assert!(cue["ms"].as_i64().unwrap() >= 90);
            assert!(cue["gain"].as_f64().unwrap() > 0.0);
            assert!(cue.get("wave").is_none());
        }
    }
    #[test]
    fn cues_land_their_offsets() {
        assert_eq!(payload("blip")["freq"], json!(freq(ROOT + 24)));
        assert_eq!(payload("good")["freq"], json!(freq(ROOT + 31)));
        assert_eq!(payload("bad")["freq"], json!(freq(ROOT + 13)));
        assert_eq!(payload("win")["freq"], json!(freq(ROOT + 36)));
        assert_eq!(payload("lose")["freq"], json!(freq(ROOT + 5)));
        assert_eq!(payload("mystery"), payload("blip"));
    }
}
