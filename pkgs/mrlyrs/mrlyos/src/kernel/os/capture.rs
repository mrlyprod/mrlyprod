use super::Os;
use crate::kernel::app::{Call, Outcome};
use crate::kernel::envelope::Notice;
use mrlycore::image::Image;
use mrlycore::json;

impl Os {
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
                self.apps[pi].call(&iden, &Call::new("photos.keep", json!({ "image": frame })));
            if kept.ok {
                self.notices
                    .push(Notice::new("saved", "screenshot → photos", self.now));
            }
        }
        Outcome::ok(json!({ "shot": app }))
    }
}
