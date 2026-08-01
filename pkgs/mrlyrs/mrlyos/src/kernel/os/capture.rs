use super::Os;
use crate::kernel::app::{Call, Outcome};
use crate::kernel::envelope::Notice;
use mrlycore::image::Image;
use mrlycore::{json, Json};

fn shot_png(frame: &Json) -> Result<Vec<u8>, &'static str> {
    let image = Image::from_json(frame).map_err(|_| "bad frame")?;
    let scale = (512 / image.width.max(image.height).max(1)).max(1);
    image.png(scale).map_err(|_| "could not render frame")
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
        if frame.is_null() || Image::from_json(&frame).is_err() {
            return Outcome::fail("nothing to shoot here");
        }
        if let Some(pi) = self.find("photos") {
            let kept =
                self.apps[pi].act(&iden, &Call::new("photos.keep", json!({ "image": frame })));
            if kept.ok {
                self.notices
                    .push(Notice::new("saved", "screenshot → photos", self.now));
            }
        }
        Outcome::ok(json!({ "shot": app }))
    }
}
