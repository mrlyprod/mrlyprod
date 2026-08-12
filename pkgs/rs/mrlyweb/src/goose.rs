use mrlycore::json::Map;
use mrlycore::rng::Rng;
use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os, Verb};

const TRIES: usize = 8;
const PATIENCE: u32 = 3;
const STROKE: i64 = 4;
const WORDS: [&str; 8] = [
    "goose", "honk", "gray", "lake", "reed", "wing", "nest", "sky",
];
const FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

/// The seeded random player that exercises any installed app through its advertised verbs.
pub struct Goose {
    rng: Rng,
    stalls: u32,
}

impl Goose {
    /// Builds a goose whose whole walk is fixed by the seed.
    pub fn new(seed: u64) -> Goose {
        Goose {
            rng: Rng::new(seed),
            stalls: 0,
        }
    }
    /// Answers whether every arg of a verb carries a hint the goose can fill, reading the text alone.
    /// ```
    /// use mrlycore::json;
    /// use mrlyos::kernel::Verb;
    /// use mrlyweb::Goose;
    /// assert!(Goose::plays(&Verb::new("toy.pick", json!({ "n": "int 0..9" }))));
    /// assert!(!Goose::plays(&Verb::new("toy.set", json!({ "key": "string", "value": "any" }))));
    /// let menu = json!({ "key": { "volume": "int 0..3" }, "value": "of key" });
    /// assert!(Goose::plays(&Verb::new("toy.tune", menu)));
    /// ```
    pub fn plays(verb: &Verb) -> bool {
        let Some(fields) = verb.args.as_object() else {
            return false;
        };
        let picked = Self::menu(fields);
        if let Some((target, _)) = &picked {
            let Some(hint) = fields.get(target) else {
                return false;
            };
            if !hint.is_object() || !Self::fillable(hint) {
                return false;
            }
        }
        fields.iter().all(|(key, hint)| match &picked {
            Some((target, of)) if key == of || key == target => true,
            _ => Self::fillable(hint),
        })
    }
    /// Plays one accepted call against the os, resetting when the game jams, or None while patience holds.
    /// ```
    /// let (mut a, mut b) = (mrlyweb::registry::boot("full"), mrlyweb::registry::boot("full"));
    /// let first = mrlyweb::Goose::new(7).step(&mut a);
    /// assert_eq!(first, mrlyweb::Goose::new(7).step(&mut b));
    /// ```
    pub fn step(&mut self, os: &mut Os) -> Option<Call> {
        let verbs = os.envelope(Some(&json!({}))).view?.actions;
        let pool: Vec<&Verb> = verbs
            .iter()
            .filter(|v| !v.name.ends_with(".reset"))
            .filter(|v| Self::plays(v))
            .collect();
        if pool.is_empty() {
            return self.reset(os, &verbs);
        }
        for _ in 0..TRIES {
            let verb = pool[self.rng.below(pool.len())];
            let Some(args) = Self::fill(&mut self.rng, &verb.args) else {
                continue;
            };
            let call = Call::new(&verb.name, args);
            if os.call(call.clone()).ok {
                self.stalls = 0;
                return Some(call);
            }
        }
        self.stalls += 1;
        if self.stalls >= PATIENCE {
            self.stalls = 0;
            return self.reset(os, &verbs);
        }
        None
    }
    fn reset(&mut self, os: &mut Os, verbs: &[Verb]) -> Option<Call> {
        let verb = verbs.iter().find(|v| v.name.ends_with(".reset"))?;
        let args = Self::fill(&mut self.rng, &verb.args)?;
        let call = Call::new(&verb.name, args);
        os.call(call.clone());
        Some(call)
    }
    fn menu(fields: &Map) -> Option<(String, String)> {
        fields.iter().find_map(|(name, hint)| {
            let target = hint.as_str()?.strip_prefix("of ")?.trim();
            Some((target.to_string(), name.clone()))
        })
    }
    fn fillable(hint: &Json) -> bool {
        match hint {
            Json::Str(token) => Self::reads(token),
            Json::Obj(fields) => !fields.is_empty() && fields.values().all(Self::fillable),
            _ => false,
        }
    }
    fn reads(token: &str) -> bool {
        if token.contains('|') {
            return true;
        }
        if let Some(ranges) = token.strip_prefix("points ") {
            return Self::pair(ranges).is_some();
        }
        if let Some(range) = token.strip_prefix("int ") {
            return Self::span(range).is_some();
        }
        matches!(
            token,
            "int" | "number" | "u8" | "u64" | "bool" | "square" | "string" | "text"
        )
    }
    fn span(text: &str) -> Option<(i64, i64)> {
        let (lo, hi) = text.split_once("..")?;
        let lo: i64 = lo.trim().parse().ok()?;
        let hi: i64 = hi.trim().parse().ok()?;
        (lo <= hi).then_some((lo, hi))
    }
    fn pair(text: &str) -> Option<((i64, i64), (i64, i64))> {
        let (x, y) = text.trim().split_once(' ')?;
        Some((Self::span(x)?, Self::span(y.trim())?))
    }
    fn fill(rng: &mut Rng, args: &Json) -> Option<Json> {
        let fields = args.as_object()?;
        let picked = Self::menu(fields);
        let mut out = Map::new();
        let mut chosen = None;
        for (key, hint) in fields {
            if let Some((target, of)) = &picked {
                if key == of {
                    continue;
                }
                if key == target {
                    let inner = hint.as_object()?;
                    let names: Vec<&String> = inner.keys().collect();
                    if names.is_empty() {
                        return None;
                    }
                    let name = (*rng.choice(&names)).clone();
                    chosen = Some((of.clone(), inner.get(&name)?.clone()));
                    out.insert(key.clone(), json!(name));
                    continue;
                }
            }
            out.insert(key.clone(), Self::grow(rng, key, hint)?);
        }
        if picked.is_some() {
            let (of, hint) = chosen?;
            let value = Self::grow(rng, &of, &hint)?;
            out.insert(of, value);
        }
        Some(Json::Obj(out))
    }
    fn grow(rng: &mut Rng, key: &str, hint: &Json) -> Option<Json> {
        match hint {
            Json::Str(token) => Self::roll(rng, key, token),
            Json::Obj(fields) if !fields.is_empty() => {
                let mut out = Map::new();
                for (name, hint) in fields {
                    out.insert(name.clone(), Self::grow(rng, name, hint)?);
                }
                Some(Json::Obj(out))
            }
            _ => None,
        }
    }
    fn roll(rng: &mut Rng, key: &str, hint: &str) -> Option<Json> {
        if hint.contains('|') {
            let options: Vec<&str> = hint.split('|').map(str::trim).collect();
            let pick = *rng.choice(&options);
            if options.iter().all(|option| Self::whole(option)) {
                return Some(json!(pick.parse::<i64>().ok()?));
            }
            return Some(json!(pick));
        }
        if let Some(ranges) = hint.strip_prefix("points ") {
            let ((x0, x1), (y0, y1)) = Self::pair(ranges)?;
            let mut out = Vec::new();
            for _ in 0..rng.range(1, STROKE) {
                out.push(json!([rng.range(x0, x1), rng.range(y0, y1)]));
            }
            return Some(Json::Arr(out));
        }
        if let Some(range) = hint.strip_prefix("int ") {
            let (lo, hi) = Self::span(range)?;
            return Some(json!(rng.range(lo, hi)));
        }
        match hint {
            "int" | "number" | "u8" | "u64" if key == "seed" => {
                Some(json!(rng.range(0, 999_999_999)))
            }
            "int" | "number" | "u8" | "u64" => Some(json!(rng.range(0, 15))),
            "bool" => Some(json!(rng.boolean())),
            "square" => Some(json!(format!("{}{}", rng.choice(&FILES), rng.range(1, 8)))),
            "string" | "text" => Some(json!(*rng.choice(&WORDS))),
            _ => None,
        }
    }
    fn whole(text: &str) -> bool {
        let digits = text.strip_prefix('-').unwrap_or(text);
        !digits.is_empty()
            && digits.bytes().all(|byte| byte.is_ascii_digit())
            && text.parse::<i64>().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::{App, Iden, Outcome};

    struct Toy {
        secret: i64,
        over: bool,
    }

    impl App for Toy {
        fn route(&self) -> &str {
            "toy"
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            let mut out = Vec::new();
            if !self.over {
                out.push(Verb::new("toy.pick", json!({ "n": "int 0..9" })));
                out.push(Verb::new("toy.mode", json!({ "m": "calm | wild" })));
                out.push(Verb::new(
                    "toy.set",
                    json!({
                        "key": { "volume": "int 0..3", "loud": "bool" },
                        "value": "of key",
                    }),
                ));
            }
            out.push(Verb::new("toy.reset", json!({ "seed": "int" })));
            out
        }
        fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
            match call.verb.as_str() {
                "toy.pick" => {
                    if self.over {
                        return Outcome::fail("round over, reset to continue");
                    }
                    if call.arg("n").as_i64() == Some(self.secret) {
                        self.over = true;
                        Outcome::ok(json!({ "hit": true }))
                    } else {
                        Outcome::fail("miss")
                    }
                }
                "toy.mode" => Outcome::ok(json!({})),
                "toy.set" => match (call.arg("key").as_str(), call.arg("value")) {
                    (Some("volume"), value)
                        if value.as_i64().is_some_and(|n| (0..=3).contains(&n)) =>
                    {
                        Outcome::ok(json!({}))
                    }
                    (Some("loud"), Json::Bool(_)) => Outcome::ok(json!({})),
                    _ => Outcome::fail("no such setting"),
                },
                "toy.reset" => {
                    self.over = false;
                    Outcome::ok(json!({}))
                }
                _ => Outcome::fail("unknown"),
            }
        }
    }

    struct Wall;

    impl App for Wall {
        fn route(&self) -> &str {
            "wall"
        }
        fn actions(&self, _iden: &Iden) -> Vec<Verb> {
            vec![
                Verb::new("wall.hit", json!({ "n": "int" })),
                Verb::new("wall.dream", json!({ "shape": "any" })),
                Verb::new("wall.reset", json!({})),
            ]
        }
        fn call(&mut self, _iden: &Iden, call: &Call) -> Outcome {
            match call.verb.as_str() {
                "wall.reset" => Outcome::ok(json!({})),
                _ => Outcome::fail("bounced"),
            }
        }
    }

    fn boot() -> Os {
        Os::new(Iden::new("aria")).install(Box::new(Toy {
            secret: 3,
            over: false,
        }))
    }

    fn transcript(seed: u64, steps: usize) -> Vec<Option<Call>> {
        let mut os = boot();
        let mut goose = Goose::new(seed);
        (0..steps).map(|_| goose.step(&mut os)).collect()
    }

    #[test]
    fn same_seed_same_transcript() {
        let a = transcript(7, 40);
        let b = transcript(7, 40);
        assert_eq!(a, b);
        assert!(a.iter().any(Option::is_some));
    }

    #[test]
    fn different_seeds_diverge() {
        assert_ne!(transcript(7, 40), transcript(8, 40));
    }

    #[test]
    fn plays_reads_the_hints_alone() {
        let wall = Wall.actions(&Iden::new("aria"));
        let named = |name: &str| wall.iter().find(|v| v.name == name).unwrap();
        assert!(Goose::plays(named("wall.hit")));
        assert!(!Goose::plays(named("wall.dream")));
        assert!(!Goose::plays(&Verb::new("v", json!({ "n": "int 9..0" }))));
        assert!(!Goose::plays(&Verb::new(
            "v",
            json!({ "k": {}, "v": "of k" })
        )));
        assert!(!Goose::plays(&Verb::new("v", json!({ "v": "of k" }))));
        assert!(!Goose::plays(&Verb::new(
            "v",
            json!({ "k": "string", "v": "of k" })
        )));
        assert!(!Goose::plays(&Verb::new(
            "v",
            json!({ "k": { "a": "any" }, "v": "of k" })
        )));
        assert!(Goose::plays(&Verb::new(
            "v",
            json!({ "at": { "x": "int 0..3", "y": "int 0..3" } })
        )));
    }

    #[test]
    fn a_menu_pairs_the_key_with_its_own_value() {
        let mut rng = Rng::new(4);
        let args = json!({
            "key": { "volume": "int 0..3", "loud": "bool" },
            "value": "of key",
        });
        for _ in 0..40 {
            let filled = Goose::fill(&mut rng, &args).unwrap();
            match filled["key"].as_str() {
                Some("volume") => assert!((0..=3).contains(&filled["value"].as_i64().unwrap())),
                Some("loud") => assert!(matches!(filled["value"], Json::Bool(_))),
                other => panic!("the menu invented {other:?}"),
            }
        }
    }

    #[test]
    fn the_goose_lands_a_menu_call() {
        let mut os = boot();
        let mut goose = Goose::new(7);
        let calls: Vec<Call> = (0..60).filter_map(|_| goose.step(&mut os)).collect();
        assert!(calls.iter().any(|call| call.verb == "toy.set"));
    }

    #[test]
    fn game_over_triggers_reset() {
        let mut os = boot();
        os.call(Call::new("toy.pick", json!({ "n": 3 })));
        let mut goose = Goose::new(1);
        let call = goose.step(&mut os).expect("goose resets a finished game");
        assert_eq!(call.verb, "toy.reset");
        assert!(call.args["seed"].is_number());
    }

    #[test]
    fn stalls_end_in_reset() {
        let mut os = Os::new(Iden::new("aria")).install(Box::new(Wall));
        let mut goose = Goose::new(2);
        assert_eq!(goose.step(&mut os), None);
        assert_eq!(goose.step(&mut os), None);
        let call = goose.step(&mut os).expect("patience runs out");
        assert_eq!(call.verb, "wall.reset");
    }

    #[test]
    fn roll_covers_the_hint_grammar() {
        let mut rng = Rng::new(5);
        let dir = Goose::roll(&mut rng, "dir", "up | down | left | right").unwrap();
        assert!(["up", "down", "left", "right"].contains(&dir.as_str().unwrap()));
        let cell = Goose::roll(&mut rng, "cell", "int 0..8")
            .unwrap()
            .as_i64()
            .unwrap();
        assert!((0..=8).contains(&cell));
        let small = Goose::roll(&mut rng, "n", "int").unwrap().as_i64().unwrap();
        assert!((0..=15).contains(&small));
        let seed = Goose::roll(&mut rng, "seed", "int")
            .unwrap()
            .as_i64()
            .unwrap();
        assert!((0..=999_999_999).contains(&seed));
        let square = Goose::roll(&mut rng, "square", "square").unwrap();
        let square = square.as_str().unwrap();
        assert!(square.len() == 2);
        assert!(("a"..="h").contains(&&square[0..1]));
        assert!(("1"..="8").contains(&&square[1..2]));
        let word = Goose::roll(&mut rng, "text", "string").unwrap();
        assert!(WORDS.contains(&word.as_str().unwrap()));
        let below = Goose::roll(&mut rng, "cold", "int -8..-2")
            .unwrap()
            .as_i64()
            .unwrap();
        assert!((-8..=-2).contains(&below));
        let sides = Goose::roll(&mut rng, "sides", "4 | 6 | 20").unwrap();
        assert!([4, 6, 20].contains(&sides.as_i64().unwrap()));
        let stroke = Goose::roll(&mut rng, "points", "points 0..7 2..5").unwrap();
        let stroke = stroke.as_array().unwrap();
        assert!((1..=STROKE as usize).contains(&stroke.len()));
        for point in stroke {
            assert!((0..=7).contains(&point[0].as_i64().unwrap()));
            assert!((2..=5).contains(&point[1].as_i64().unwrap()));
        }
        assert!(Goose::roll(&mut rng, "value", "any").is_none());
        assert!(Goose::roll(&mut rng, "n", "int 9..0").is_none());
        assert!(Goose::roll(&mut rng, "points", "points 0..7").is_none());
    }
}
