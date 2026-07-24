use super::Os;
use crate::kernel::app::{Call, Outcome};
use crate::kernel::envelope::Notice;
use mrlycore::colors::Color;
use serde_json::{json, Value as Json};

fn shot_png(frame: &Json) -> Result<Vec<u8>, &'static str> {
    let width = frame["width"].as_u64().ok_or("bad frame")? as usize;
    let height = frame["height"].as_u64().ok_or("bad frame")? as usize;
    let palette: Vec<[u8; 4]> = frame["palette"]
        .as_array()
        .ok_or("bad frame")?
        .iter()
        .map(|v| v.as_str().and_then(|hex| Color::from_hex(hex).ok()))
        .collect::<Option<Vec<Color>>>()
        .ok_or("bad frame")?
        .iter()
        .map(|c| [c.r, c.g, c.b, c.a])
        .collect();
    let rows = frame["rows"].as_array().ok_or("bad frame")?;
    let mut colors = Vec::with_capacity(width * height);
    for row in rows {
        for id in row.as_array().ok_or("bad frame")? {
            let idx = id.as_u64().ok_or("bad frame")? as usize;
            colors.push(*palette.get(idx).ok_or("bad frame")?);
        }
    }
    let scale = (512 / width.max(height).max(1)).max(1);
    mrlycore::png(&colors, width, height, scale).map_err(|_| "could not render frame")
}

impl Os {
    pub fn snapshot(&self, app: &str) -> Result<Vec<u8>, &'static str> {
        let i = self.find(app).ok_or("no such app")?;
        let frame = self.apps[i].capture(&self.iden);
        if frame.is_null() {
            return Err("nothing to shoot here");
        }
        shot_png(&frame)
    }
    pub fn shot(&mut self) -> Outcome {
        let Some(app) = self.focused().map(|r| r.app.clone()) else {
            return Outcome::fail("no current app");
        };
        if app == "photos" {
            return Outcome::fail("nothing to shoot here");
        }
        let Some(i) = self.find(&app) else {
            return Outcome::fail("no current app");
        };
        let iden = self.iden.clone();
        let frame = self.apps[i].capture(&iden);
        if frame.is_null() {
            return Outcome::fail("nothing to shoot here");
        }
        let bytes = match shot_png(&frame) {
            Ok(bytes) => bytes,
            Err(note) => return Outcome::fail(note),
        };
        let data = mrlycore::base64(&bytes);
        if let Some(pi) = self.find("photos") {
            let kept = self.apps[pi].act(
                &iden,
                &Call::new("photos.keep", json!({ "data": data, "mime": "image/png" })),
            );
            if kept.ok {
                self.notices
                    .push(Notice::new("saved", "screenshot → photos", self.now));
            }
        }
        Outcome::ok(json!({ "shot": app }))
    }
}
