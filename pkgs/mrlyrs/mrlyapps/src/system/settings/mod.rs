use mrlyos::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use serde_json::{json, Value as Json};

use mrlycore::colors::NAMES as COLORS;

pub const MODES: [&str; 2] = ["grid", "list"];

pub const FONTS: [&str; 5] = ["mono", "sans", "serif", "display", "mrly"];

pub const EMOJIS: [&str; 2] = ["system", "noto"];

pub const RENDERS: [&str; 2] = ["cpu", "gpu"];

pub const MATERIALS: [&str; 2] = ["solid", "glass"];

pub const WALLPAPERS: [&str; 2] = ["color", "pattern"];

pub struct Settings {
    launchpad: String,
    darkmode: bool,
    color: String,
    fill: String,
    font: String,
    emoji: String,
    scale: f64,
    radius: f64,
    pace: f64,
    sound: bool,
    haptics: bool,
    note: String,
    wave: String,
    duration: f64,
    background: String,
    width: f64,
    render: String,
    material: String,
    wallpaper: String,
    seed: f64,
    detail: f64,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings::new()
    }
}

impl Settings {
    const KEYS: [&'static str; 21] = [
        "launchpad",
        "darkmode",
        "color",
        "fill",
        "font",
        "emoji",
        "scale",
        "radius",
        "pace",
        "sound",
        "haptics",
        "note",
        "wave",
        "duration",
        "background",
        "width",
        "render",
        "material",
        "wallpaper",
        "seed",
        "detail",
    ];
    pub fn new() -> Settings {
        Settings {
            launchpad: "grid".to_string(),
            darkmode: false,
            color: "blue".to_string(),
            fill: "random".to_string(),
            font: "mono".to_string(),
            emoji: "system".to_string(),
            scale: 5.0,
            radius: 2.0,
            pace: 150.0,
            sound: true,
            haptics: true,
            note: "random".to_string(),
            wave: "sine".to_string(),
            duration: 150.0,
            background: "white".to_string(),
            width: 500.0,
            render: "cpu".to_string(),
            material: "solid".to_string(),
            wallpaper: "color".to_string(),
            seed: 0.0,
            detail: 96.0,
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "darkmode" | "sound" | "haptics" => {
                let on = value.as_bool().ok_or("value must be a bool")?;
                match key {
                    "darkmode" => self.darkmode = on,
                    "sound" => self.sound = on,
                    _ => self.haptics = on,
                }
                Ok(json!(on))
            }
            "launchpad" | "color" | "fill" | "font" | "emoji" | "note" | "wave" | "background"
            | "render" | "material" | "wallpaper" => {
                let pick = value.as_str().ok_or("value must be a string")?;
                let allowed = match key {
                    "launchpad" => MODES.contains(&pick),
                    "color" => COLORS.contains(&pick),
                    "fill" => pick == "random" || COLORS.contains(&pick),
                    "font" => FONTS.contains(&pick),
                    "emoji" => EMOJIS.contains(&pick),
                    "note" => pick == "random" || mrlymusic::theory::NAMES.contains(&pick),
                    "background" => COLORS.contains(&pick),
                    "render" => RENDERS.contains(&pick),
                    "material" => MATERIALS.contains(&pick),
                    "wallpaper" => WALLPAPERS.contains(&pick),
                    _ => mrlymusic::wave::NAMES.contains(&pick),
                };
                if !allowed {
                    return Err("no such option");
                }
                match key {
                    "launchpad" => self.launchpad = pick.to_string(),
                    "color" => self.color = pick.to_string(),
                    "fill" => self.fill = pick.to_string(),
                    "font" => self.font = pick.to_string(),
                    "emoji" => self.emoji = pick.to_string(),
                    "note" => self.note = pick.to_string(),
                    "background" => self.background = pick.to_string(),
                    "render" => self.render = pick.to_string(),
                    "material" => self.material = pick.to_string(),
                    "wallpaper" => self.wallpaper = pick.to_string(),
                    _ => self.wave = pick.to_string(),
                }
                Ok(json!(pick))
            }
            "scale" | "radius" | "pace" | "duration" | "width" | "seed" | "detail" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                let (min, max) = match key {
                    "scale" => (3.0, 6.0),
                    "radius" => (0.0, 4.0),
                    "duration" => (50.0, 1000.0),
                    "width" => (500.0, 1500.0),
                    "seed" => (0.0, 999.0),
                    "detail" => (32.0, 160.0),
                    _ => (0.0, 400.0),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "scale" => self.scale = n,
                    "radius" => self.radius = n,
                    "duration" => self.duration = n,
                    "width" => self.width = n,
                    "seed" => self.seed = n,
                    "detail" => self.detail = n,
                    _ => self.pace = n,
                }
                Ok(json!(n))
            }
            _ => Err("no such key"),
        }
    }
    fn get(&self, key: &str) -> Json {
        match key {
            "launchpad" => json!(self.launchpad),
            "darkmode" => json!(self.darkmode),
            "color" => json!(self.color),
            "fill" => json!(self.fill),
            "font" => json!(self.font),
            "emoji" => json!(self.emoji),
            "scale" => json!(self.scale),
            "radius" => json!(self.radius),
            "pace" => json!(self.pace),
            "sound" => json!(self.sound),
            "haptics" => json!(self.haptics),
            "note" => json!(self.note),
            "wave" => json!(self.wave),
            "duration" => json!(self.duration),
            "background" => json!(self.background),
            "width" => json!(self.width),
            "render" => json!(self.render),
            "material" => json!(self.material),
            "wallpaper" => json!(self.wallpaper),
            "seed" => json!(self.seed),
            "detail" => json!(self.detail),
            _ => Json::Null,
        }
    }
}

impl App for Settings {
    fn route(&self) -> &str {
        "settings"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("settings").emoji("⚙️").category("system")
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![Verb::new(
            "settings.set",
            json!({ "key": "string", "value": "any" }),
        )]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "settings.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                match self.apply(&key, call.arg("value")) {
                    Ok(value) => Outcome::ok(json!({ "key": key, "value": value })),
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn share(&self) -> Option<Json> {
        Some(self.save())
    }
    fn save(&self) -> Json {
        let mut out = json!({});
        for key in Settings::KEYS {
            out[key] = self.get(key);
        }
        out
    }
    fn load(&mut self, state: &Json) {
        for key in Settings::KEYS {
            let _ = self.apply(key, &state[key]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_stylesheet() {
        let s = Settings::new();
        assert_eq!(s.get("launchpad"), json!("grid"));
        assert_eq!(s.get("darkmode"), json!(false));
        assert_eq!(s.get("color"), json!("blue"));
        assert_eq!(s.get("fill"), json!("random"));
        assert_eq!(s.get("font"), json!("mono"));
        assert_eq!(s.get("emoji"), json!("system"));
        assert_eq!(s.get("scale"), json!(5.0));
        assert_eq!(s.get("radius"), json!(2.0));
        assert_eq!(s.get("pace"), json!(150.0));
        assert_eq!(s.get("sound"), json!(true));
        assert_eq!(s.get("haptics"), json!(true));
        assert_eq!(s.get("note"), json!("random"));
        assert_eq!(s.get("wave"), json!("sine"));
        assert_eq!(s.get("duration"), json!(150.0));
        assert_eq!(s.get("background"), json!("white"));
        assert_eq!(s.get("width"), json!(500.0));
        assert_eq!(s.get("render"), json!("cpu"));
        assert_eq!(s.get("material"), json!("solid"));
        assert_eq!(s.get("wallpaper"), json!("color"));
        assert_eq!(s.get("seed"), json!(0.0));
        assert_eq!(s.get("detail"), json!(96.0));
    }
    #[test]
    fn share_carries_the_saved_state() {
        let shared = Settings::new().share().unwrap();
        assert_eq!(shared["font"], json!("mono"));
        assert_eq!(shared["launchpad"], json!("grid"));
    }
    #[test]
    fn actions_offer_the_natural_verb() {
        let iden = Iden::new("aria");
        let verbs = Settings::new().actions(&iden);
        assert_eq!(verbs.len(), 1);
        assert_eq!(verbs[0].name, "settings.set");
    }
    #[test]
    fn act_applies_key_and_value() {
        let iden = Iden::new("aria");
        let mut s = Settings::new();
        let out = s.act(
            &iden,
            &Call::new("settings.set", json!({ "key": "color", "value": "pink" })),
        );
        assert!(out.ok);
        assert_eq!(out.data, json!({ "key": "color", "value": "pink" }));
        assert_eq!(s.get("color"), json!("pink"));
        let out = s.act(
            &iden,
            &Call::new("settings.set", json!({ "key": "color", "value": 7 })),
        );
        assert!(!out.ok);
        let out = s.act(
            &iden,
            &Call::new("settings.set", json!({ "key": "volume", "value": 1 })),
        );
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no such key"));
    }
    #[test]
    fn act_applies_the_launchpad() {
        let iden = Iden::new("aria");
        let mut s = Settings::new();
        let out = s.act(
            &iden,
            &Call::new(
                "settings.set",
                json!({ "key": "launchpad", "value": "list" }),
            ),
        );
        assert!(out.ok);
        assert_eq!(out.data, json!({ "key": "launchpad", "value": "list" }));
        assert_eq!(s.get("launchpad"), json!("list"));
        let out = s.act(
            &iden,
            &Call::new(
                "settings.set",
                json!({ "key": "launchpad", "value": "carousel" }),
            ),
        );
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("no such option"));
        assert_eq!(s.get("launchpad"), json!("list"));
    }
    #[test]
    fn unknown_verb_fails() {
        let iden = Iden::new("aria");
        let mut s = Settings::new();
        let out = s.act(&iden, &Call::new("settings.reset", json!({})));
        assert!(!out.ok);
        assert_eq!(out.note.as_deref(), Some("unknown verb"));
    }
    #[test]
    fn apply_holds_the_bounds() {
        let mut s = Settings::new();
        assert_eq!(
            s.apply("launchpad", &json!("carousel")),
            Err("no such option")
        );
        assert_eq!(
            s.apply("launchpad", &json!(5)),
            Err("value must be a string")
        );
        assert_eq!(s.apply("launchpad", &json!("list")), Ok(json!("list")));
        assert_eq!(s.apply("color", &json!("beige")), Err("no such option"));
        assert_eq!(s.apply("fill", &json!("beige")), Err("no such option"));
        assert_eq!(s.apply("font", &json!("wide")), Err("no such option"));
        assert_eq!(s.apply("emoji", &json!("twemoji")), Err("no such option"));
        assert_eq!(s.apply("fill", &json!("brown")), Ok(json!("brown")));
        assert_eq!(s.apply("font", &json!("serif")), Ok(json!("serif")));
        assert_eq!(s.apply("font", &json!("mrly")), Ok(json!("mrly")));
        assert_eq!(s.apply("emoji", &json!("noto")), Ok(json!("noto")));
        assert_eq!(s.apply("scale", &json!(2)), Err("out of range"));
        assert_eq!(s.apply("scale", &json!(7)), Err("out of range"));
        assert_eq!(s.apply("radius", &json!(5)), Err("out of range"));
        assert_eq!(s.apply("pace", &json!(500)), Err("out of range"));
        assert_eq!(
            s.apply("darkmode", &json!("yes")),
            Err("value must be a bool")
        );
        assert_eq!(s.apply("scale", &json!(3)), Ok(json!(3.0)));
        assert_eq!(s.apply("pace", &json!(0)), Ok(json!(0.0)));
        assert_eq!(s.apply("note", &json!("H")), Err("no such option"));
        assert_eq!(s.apply("wave", &json!("noise")), Err("no such option"));
        assert_eq!(s.apply("duration", &json!(20)), Err("out of range"));
        assert_eq!(s.apply("duration", &json!(2000)), Err("out of range"));
        assert_eq!(s.apply("sound", &json!(1)), Err("value must be a bool"));
        assert_eq!(s.apply("note", &json!("C#")), Ok(json!("C#")));
        assert_eq!(s.apply("note", &json!("random")), Ok(json!("random")));
        assert_eq!(s.apply("wave", &json!("square")), Ok(json!("square")));
        assert_eq!(s.apply("duration", &json!(300)), Ok(json!(300.0)));
        assert_eq!(s.apply("haptics", &json!(false)), Ok(json!(false)));
        assert_eq!(
            s.apply("background", &json!("beige")),
            Err("no such option")
        );
        assert_eq!(s.apply("background", &json!("black")), Ok(json!("black")));
        assert_eq!(s.apply("width", &json!(300)), Err("out of range"));
        assert_eq!(s.apply("width", &json!(2000)), Err("out of range"));
        assert_eq!(s.apply("width", &json!(750)), Ok(json!(750.0)));
        assert_eq!(s.apply("render", &json!("webgl")), Err("no such option"));
        assert_eq!(s.apply("render", &json!(1)), Err("value must be a string"));
        assert_eq!(s.apply("render", &json!("gpu")), Ok(json!("gpu")));
        assert_eq!(
            s.apply("material", &json!("frosted")),
            Err("no such option")
        );
        assert_eq!(
            s.apply("material", &json!(1)),
            Err("value must be a string")
        );
        assert_eq!(s.apply("material", &json!("glass")), Ok(json!("glass")));
        assert_eq!(s.apply("wallpaper", &json!("image")), Err("no such option"));
        assert_eq!(
            s.apply("wallpaper", &json!(true)),
            Err("value must be a string")
        );
        assert_eq!(
            s.apply("wallpaper", &json!("pattern")),
            Ok(json!("pattern"))
        );
        assert_eq!(s.apply("seed", &json!(-1)), Err("out of range"));
        assert_eq!(s.apply("seed", &json!(1000)), Err("out of range"));
        assert_eq!(s.apply("seed", &json!("7")), Err("value must be a number"));
        assert_eq!(s.apply("seed", &json!(7)), Ok(json!(7.0)));
        assert_eq!(s.apply("detail", &json!(31)), Err("out of range"));
        assert_eq!(s.apply("detail", &json!(161)), Err("out of range"));
        assert_eq!(
            s.apply("detail", &json!("high")),
            Err("value must be a number")
        );
        assert_eq!(s.apply("detail", &json!(128)), Ok(json!(128.0)));
    }
    #[test]
    fn accent_accepts_every_palette_name() {
        let mut s = Settings::new();
        for name in mrlycore::colors::NAMES {
            assert_eq!(
                s.apply("color", &json!(name)),
                Ok(json!(name)),
                "color {name}"
            );
        }
    }
    #[test]
    fn save_load_roundtrips() {
        let iden = Iden::new("aria");
        let mut a = Settings::new();
        a.apply("launchpad", &json!("list")).unwrap();
        a.apply("darkmode", &json!(true)).unwrap();
        a.apply("color", &json!("mint")).unwrap();
        a.apply("scale", &json!(4.0)).unwrap();
        a.apply("material", &json!("glass")).unwrap();
        a.apply("wallpaper", &json!("pattern")).unwrap();
        a.apply("seed", &json!(42.0)).unwrap();
        let mut b = Settings::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden), a.state(&iden));
        assert_eq!(b.save(), a.save());
    }
    #[test]
    fn load_keeps_defaults_on_garbage() {
        let mut s = Settings::new();
        s.load(&json!({ "color": "beige", "scale": 99, "font": "wide" }));
        assert_eq!(s.get("color"), json!("blue"));
        assert_eq!(s.get("scale"), json!(5.0));
        assert_eq!(s.get("font"), json!("mono"));
        s.load(&json!({ "color": "mint", "radius": 88 }));
        assert_eq!(s.get("color"), json!("mint"));
        assert_eq!(s.get("radius"), json!(2.0));
        s.load(&json!({ "background": "beige", "width": 2000 }));
        assert_eq!(s.get("background"), json!("white"));
        assert_eq!(s.get("width"), json!(500.0));
        s.load(&json!({ "launchpad": "carousel" }));
        assert_eq!(s.get("launchpad"), json!("grid"));
        s.load(&json!({ "launchpad": 5 }));
        assert_eq!(s.get("launchpad"), json!("grid"));
        s.load(&json!({ "launchpad": "list" }));
        assert_eq!(s.get("launchpad"), json!("list"));
        s.load(&json!({ "render": "webgl" }));
        assert_eq!(s.get("render"), json!("cpu"));
        s.load(&json!({ "render": "gpu" }));
        assert_eq!(s.get("render"), json!("gpu"));
        s.load(&json!({ "material": "frosted" }));
        assert_eq!(s.get("material"), json!("solid"));
        s.load(&json!({ "material": "glass" }));
        assert_eq!(s.get("material"), json!("glass"));
        s.load(&json!({ "wallpaper": "image", "seed": 5000 }));
        assert_eq!(s.get("wallpaper"), json!("color"));
        assert_eq!(s.get("seed"), json!(0.0));
        s.load(&json!({ "wallpaper": "pattern", "seed": 42 }));
        assert_eq!(s.get("wallpaper"), json!("pattern"));
        assert_eq!(s.get("seed"), json!(42.0));
    }
}
