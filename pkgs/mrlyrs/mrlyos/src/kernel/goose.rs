use super::app::{Call, Verb};
use super::os::Os;
use mrlycore::json::Map;
use mrlycore::rng::Rng;
use mrlycore::{json, Json};

const TRIES: usize = 8;
const PATIENCE: u32 = 3;
const WORDS: [&str; 8] = [
    "goose", "honk", "gray", "lake", "reed", "wing", "nest", "sky",
];
const FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

pub struct Goose {
    rng: Rng,
    stalls: u32,
}

impl Goose {
    pub fn new(seed: u64) -> Goose {
        Goose {
            rng: Rng::new(seed),
            stalls: 0,
        }
    }
    pub fn step(&mut self, os: &mut Os) -> Option<Call> {
        let verbs = os.frame(Some(&json!({}))).view?.actions;
        let pool: Vec<&Verb> = verbs
            .iter()
            .filter(|v| !v.name.ends_with(".reset"))
            .filter(|v| Self::fill(&mut self.rng.clone(), &v.args).is_some())
            .collect();
        if pool.is_empty() {
            return self.reset(os, &verbs);
        }
        for _ in 0..TRIES {
            let verb = pool[self.rng.below(pool.len())];
            let args = Self::fill(&mut self.rng, &verb.args)?;
            let call = Call::new(&verb.name, args);
            if os.act(call.clone()).ok {
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
        os.act(call.clone());
        Some(call)
    }
    fn fill(rng: &mut Rng, args: &Json) -> Option<Json> {
        let fields = args.as_object()?;
        let mut out = Map::new();
        for (key, hint) in fields {
            out.insert(key.clone(), Self::roll(rng, key, hint.as_str()?)?);
        }
        Some(Json::Obj(out))
    }
    fn roll(rng: &mut Rng, key: &str, hint: &str) -> Option<Json> {
        if hint.contains('|') {
            let options: Vec<&str> = hint.split('|').map(str::trim).collect();
            return Some(json!(*rng.choice(&options)));
        }
        if let Some(range) = hint.strip_prefix("int ") {
            let (lo, hi) = range.split_once("..")?;
            let lo: i64 = lo.trim().parse().ok()?;
            let hi: i64 = hi.trim().parse().ok()?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::app::{App, Outcome};
    use crate::kernel::iden::Iden;

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
            }
            out.push(Verb::new("toy.reset", json!({ "seed": "int" })));
            out.push(Verb::new(
                "toy.set",
                json!({ "key": "string", "value": "any" }),
            ));
            out
        }
        fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
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
                Verb::new("wall.reset", json!({})),
            ]
        }
        fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
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
    fn unfillable_verbs_never_picked() {
        let calls = transcript(7, 60);
        assert!(calls.iter().flatten().all(|call| call.verb != "toy.set"));
    }

    #[test]
    fn game_over_triggers_reset() {
        let mut os = boot();
        os.act(Call::new("toy.pick", json!({ "n": 3 })));
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
        assert!(Goose::roll(&mut rng, "value", "any").is_none());
        assert!(Goose::roll(&mut rng, "points", "[[x, y], ...]").is_none());
    }
}
