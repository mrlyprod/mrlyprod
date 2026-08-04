use super::app::{App, Call, Outcome};
use super::iden::Iden;
use super::os::Os;
use mrlycore::image::Image;
use mrlycore::{json, Json};

pub fn iden() -> Iden {
    Iden::new("aria")
}

pub fn send<A: App>(app: &mut A, verb: &str, args: Json) -> Outcome {
    app.call(&iden(), &Call::new(verb, args))
}

pub fn seeded<A: App>(mut app: A, verb: &str, seed: u64) -> A {
    let _ = app.call(&iden(), &Call::new(verb, json!({ "seed": seed })));
    app
}

impl Os {
    pub fn capture(&self, app: &str) -> Json {
        self.shoot(app).unwrap_or(Json::Null)
    }
    pub fn snapshot(&self, app: &str) -> Result<Vec<u8>, &'static str> {
        let frame = self.shoot(app).ok_or("no such app")?;
        if frame.is_null() {
            return Err("nothing to shoot here");
        }
        let image = Image::from_json(&frame).map_err(|_| "bad frame")?;
        let scale = (512 / image.width.max(image.height).max(1)).max(1);
        image.png(scale).map_err(|_| "could not render frame")
    }
}
