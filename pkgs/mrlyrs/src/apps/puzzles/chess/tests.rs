use super::*;
use crate::os::kernel::testkit::iden;

fn game() -> Chess {
    let mut g = Chess::new();
    g.act(&iden(), &Call::new("chess.reset", json!({ "seed": 1 })));
    g
}
fn custom(layout: &str) -> Chess {
    let mut g = game();
    g.act(
        &iden(),
        &Call::new("chess.set", json!({ "key": "layout", "value": layout })),
    );
    g
}
fn mv(g: &mut Chess, from: &str, to: &str) -> Outcome {
    g.act(
        &iden(),
        &Call::new("chess.move", json!({ "from": from, "to": to })),
    )
}

#[test]
fn turn_alternates() {
    let mut g = game();
    assert_eq!(g.state(&iden())["turn"], json!("white"));
    assert!(mv(&mut g, "e2", "e4").ok);
    assert_eq!(g.state(&iden())["turn"], json!("black"));
    assert!(mv(&mut g, "e7", "e5").ok);
    assert_eq!(g.state(&iden())["turn"], json!("white"));
}
#[test]
fn illegal_fails_honestly() {
    let mut g = game();
    let out = mv(&mut g, "e4", "e3");
    assert!(!out.ok);
    assert_eq!(out.note.as_deref(), Some("illegal move"));
    let out = mv(&mut g, "e7", "e5");
    assert!(!out.ok);
    assert_eq!(g.state(&iden())["turn"], json!("white"));
    assert_eq!(g.state(&iden())["steps"], json!(0));
    let out = mv(&mut g, "z9", "e5");
    assert!(!out.ok);
    assert_eq!(out.note.as_deref(), Some("no such square"));
}
#[test]
fn pawn_pushes_and_captures() {
    let g = custom("k7/8/8/8/8/8/P7/K7");
    let v = g.valid(&g.board, g.ep, 0, 6);
    assert!(v.contains(&g.cell(0, 5)), "single push");
    assert!(v.contains(&g.cell(0, 4)), "double push");
    let blocked = custom("k7/8/8/8/8/N7/P7/K7");
    assert!(blocked.valid(&blocked.board, blocked.ep, 0, 6).is_empty());
    let cap = custom("k7/8/8/8/8/1n6/P7/7K");
    let v = cap.valid(&cap.board, cap.ep, 0, 6);
    assert!(v.contains(&cap.cell(1, 5)), "diagonal capture");
    assert!(v.contains(&cap.cell(0, 5)), "forward still legal");
}
#[test]
fn en_passant_window() {
    let mut g = custom("3k4/3p4/8/4P3/8/8/8/4K3");
    assert!(mv(&mut g, "e1", "e2").ok);
    assert_eq!(g.ep, None);
    assert!(mv(&mut g, "d7", "d5").ok);
    let d6 = g.cell(3, 2);
    assert_eq!(g.ep, Some(d6));
    let out = mv(&mut g, "e5", "d6");
    assert!(out.ok);
    assert_eq!(g.board[g.cell(3, 3)].kind, 0);
    assert_eq!(g.board[d6].kind, 1);
    assert_eq!(g.ep, None);
}
#[test]
fn castling_rules() {
    let both = custom("r3k2r/8/8/8/8/8/8/R3K2R");
    let v = both.valid(&both.board, both.ep, 4, 7);
    assert!(v.contains(&both.cell(6, 7)), "kingside");
    assert!(v.contains(&both.cell(2, 7)), "queenside");
    let blocked = custom("r3k2r/8/8/8/8/8/8/R3KB1R");
    let v = blocked.valid(&blocked.board, blocked.ep, 4, 7);
    assert!(!v.contains(&blocked.cell(6, 7)), "kingside blocked");
    assert!(v.contains(&blocked.cell(2, 7)), "queenside open");
    let in_check = custom("4r3/8/8/8/8/8/8/R3K2R");
    let v = in_check.valid(&in_check.board, in_check.ep, 4, 7);
    assert!(!v.contains(&in_check.cell(6, 7)) && !v.contains(&in_check.cell(2, 7)));
    let path = custom("5r2/8/8/8/8/8/8/R3K2R");
    let v = path.valid(&path.board, path.ep, 4, 7);
    assert!(!v.contains(&path.cell(6, 7)), "path attacked");
    assert!(v.contains(&path.cell(2, 7)), "queenside still ok");
}
#[test]
fn castling_unavailable_after_king_moves() {
    let mut g = custom("r3k2r/8/8/8/8/8/8/R3K2R");
    assert!(mv(&mut g, "e1", "e2").ok);
    assert!(mv(&mut g, "e8", "e7").ok);
    assert!(mv(&mut g, "e2", "e1").ok);
    assert!(mv(&mut g, "e7", "e8").ok);
    let v = g.valid(&g.board, g.ep, 4, 7);
    assert!(!v.contains(&g.cell(6, 7)) && !v.contains(&g.cell(2, 7)));
}
#[test]
fn promotion_defaults_to_queen() {
    let mut g = custom("7k/P7/8/8/8/K7");
    assert_eq!((g.w, g.h), (8, 6));
    assert!(mv(&mut g, "a5", "a6").ok);
    let q = g.board[g.cell(0, 0)];
    assert_eq!(q.kind, 5);
    assert_eq!(q.team, 0);
}
#[test]
fn promotion_honors_the_choice() {
    let mut g = custom("7k/P7/8/8/8/K7");
    let out = g.act(
        &iden(),
        &Call::new(
            "chess.move",
            json!({ "from": "a5", "to": "a6", "promote": "knight" }),
        ),
    );
    assert!(out.ok);
    assert_eq!(g.board[g.cell(0, 0)].kind, 2);
    let mut g = custom("7k/P7/8/8/8/K7");
    let out = g.act(
        &iden(),
        &Call::new(
            "chess.move",
            json!({ "from": "a5", "to": "a6", "promote": "king" }),
        ),
    );
    assert!(!out.ok);
    let mut g = game();
    let out = g.act(
        &iden(),
        &Call::new(
            "chess.move",
            json!({ "from": "e2", "to": "e4", "promote": "queen" }),
        ),
    );
    assert!(!out.ok);
    assert_eq!(out.note.as_deref(), Some("nothing to promote"));
}
#[test]
fn fools_mate_ends_the_game() {
    let mut g = game();
    assert!(mv(&mut g, "f2", "f3").ok);
    assert!(mv(&mut g, "e7", "e5").ok);
    assert!(mv(&mut g, "g2", "g4").ok);
    let out = mv(&mut g, "d8", "h4");
    assert!(out.ok);
    assert_eq!(out.data["over"], json!(true));
    assert_eq!(out.data["winner"], json!("black"));
    let state = g.state(&iden());
    assert_eq!(state["winner"], json!("black"));
    assert!(state["moves"].as_array().unwrap().is_empty());
    let again = mv(&mut g, "a2", "a3");
    assert!(!again.ok);
    assert_eq!(again.note.as_deref(), Some("round over, reset to continue"));
}
#[test]
fn stalemate_is_a_draw() {
    let mut m = custom("7k/8/8/6Q1/8/8/8/K7");
    let out = mv(&mut m, "g5", "g6");
    assert!(out.ok);
    assert_eq!(out.data["over"], json!(true));
    assert_eq!(out.data["winner"], json!("draw"));
    assert_eq!(m.state(&iden())["winner"], json!("draw"));
}
#[test]
fn check_is_a_fact() {
    let mut g = game();
    assert!(mv(&mut g, "e2", "e4").ok);
    assert!(mv(&mut g, "f7", "f6").ok);
    assert!(!g.state(&iden())["check"].as_bool().unwrap());
    assert!(mv(&mut g, "d1", "h5").ok);
    assert!(g.state(&iden())["check"].as_bool().unwrap());
}
#[test]
fn moves_list_the_action_space() {
    let g = game();
    let state = g.state(&iden());
    let moves = state["moves"].as_array().unwrap();
    assert_eq!(moves.len(), 20);
    assert!(moves
        .iter()
        .any(|m| m["from"] == json!("e2") && m["to"] == json!("e4")));
}
#[test]
fn custom_dims_parse() {
    let g = custom("k2/3/2K");
    assert_eq!((g.w, g.h), (3, 3));
    assert_eq!(g.board[g.cell(0, 0)].kind, 6);
    assert_eq!(g.board[g.cell(2, 2)].kind, 6);
    assert_eq!(g.state(&iden())["board"][0][0], json!("k"));
}
#[test]
fn deterministic_appearance() {
    let mut a = game();
    let mut b = game();
    for g in [&mut a, &mut b] {
        g.act(&iden(), &Call::new("chess.reset", json!({ "seed": 7 })));
    }
    assert_eq!(a.piece_colors, b.piece_colors);
    assert_eq!(a.board_colors, b.board_colors);
    assert_eq!(a.glyphs, b.glyphs);
    let mut ob = game();
    ob.act(
        &iden(),
        &Call::new("chess.set", json!({ "key": "obfuscate", "value": true })),
    );
    let mut ob2 = game();
    ob2.act(
        &iden(),
        &Call::new("chess.set", json!({ "key": "obfuscate", "value": true })),
    );
    assert_eq!(ob.glyphs, ob2.glyphs);
    assert_ne!(ob.glyphs, default_glyphs());
}
#[test]
fn renders_two_layers() {
    let g = game();
    let cell = g.render().composite();
    assert_eq!(cell.width(), 8 * 5);
    assert_eq!(cell.height(), 8 * 5);
    let colors = cell.cell.colors.clone().unwrap();
    let mid = (4 * 5) * cell.width() + (4 * 5);
    assert_eq!(colors[mid], g.board_colors[(4 + 4) % 2]);
}
#[test]
fn seed_reproduces() {
    let mut a = game();
    let mut b = game();
    for g in [&mut a, &mut b] {
        g.act(&iden(), &Call::new("chess.reset", json!({ "seed": 123 })));
        mv(g, "e2", "e4");
        mv(g, "e7", "e5");
    }
    assert_eq!(a.state(&iden()), b.state(&iden()));
    assert_eq!(a.save(), b.save());
}
#[test]
fn save_load_roundtrips_and_continues() {
    let mut a = game();
    a.act(
        &iden(),
        &Call::new("chess.set", json!({ "key": "reskin", "value": 2 })),
    );
    mv(&mut a, "e2", "e4");
    mv(&mut a, "e7", "e5");
    mv(&mut a, "g1", "f3");
    let mut b = Chess::new();
    b.load(&a.save());
    assert_eq!(b.state(&iden()), a.state(&iden()));
    assert_eq!(b.save(), a.save());
    for g in [&mut a, &mut b] {
        mv(g, "b8", "c6");
    }
    assert_eq!(b.state(&iden()), a.state(&iden()));
}
#[test]
fn set_validates_and_resets_the_round() {
    let mut g = game();
    mv(&mut g, "e2", "e4");
    let out = g.act(
        &iden(),
        &Call::new("chess.set", json!({ "key": "tile", "value": 8 })),
    );
    assert!(out.ok);
    let state = g.state(&iden());
    assert_eq!(state["settings"]["tile"], json!(8));
    assert_eq!(state["steps"], json!(0));
    assert!(
        !g.act(
            &iden(),
            &Call::new("chess.set", json!({ "key": "tile", "value": 99 }))
        )
        .ok
    );
    assert!(
        !g.act(
            &iden(),
            &Call::new("chess.set", json!({ "key": "volume", "value": 1 }))
        )
        .ok
    );
    assert!(
        !g.act(
            &iden(),
            &Call::new(
                "chess.set",
                json!({ "key": "layout", "value": "99999/99999/99999" })
            )
        )
        .ok
    );
}
#[test]
fn reset_seed_defaults_to_now() {
    let mut g = Chess::new();
    let out = g.act(&iden(), &Call::new("chess.reset", json!({})).at(5000));
    assert!(out.ok);
    assert_eq!(out.data["seed"], json!(5000));
    assert_eq!(g.state(&iden())["seed"], json!(5000));
}
#[test]
fn load_survives_garbage() {
    let mut g = Chess::new();
    g.load(&json!({ "seed": "soup", "board": [[9, 9, 9]], "settings": 7 }));
    let state = g.state(&iden());
    assert_eq!(state["steps"], json!(0));
    assert_eq!(state["seed"], json!(0));
    assert_eq!(state["turn"], json!("white"));
    assert_eq!(state["moves"].as_array().unwrap().len(), 20);
}
#[test]
fn actions_offer_the_natural_verbs() {
    let g = game();
    let names: Vec<String> = g.actions(&iden()).iter().map(|v| v.name.clone()).collect();
    assert_eq!(
        names,
        vec!["chess.select", "chess.move", "chess.reset", "chess.set"]
    );
}
fn select(g: &mut Chess, square: &str) -> Outcome {
    g.act(
        &iden(),
        &Call::new("chess.select", json!({ "square": square })),
    )
}

#[test]
fn select_highlights_then_moves() {
    let mut g = game();
    let out = select(&mut g, "e2");
    assert!(out.ok);
    assert_eq!(out.data["selected"], json!("e2"));
    let state = g.state(&iden());
    assert_eq!(state["selected"], json!("e2"));
    let targets = state["targets"].as_array().unwrap();
    assert!(targets.contains(&json!("e3")) && targets.contains(&json!("e4")));
    let out = select(&mut g, "e4");
    assert!(out.ok);
    assert_eq!(out.data["from"], json!("e2"));
    assert_eq!(out.data["to"], json!("e4"));
    let state = g.state(&iden());
    assert_eq!(state["selected"], Json::Null);
    assert!(state["targets"].as_array().unwrap().is_empty());
    assert_eq!(state["last_move"], json!({ "from": "e2", "to": "e4" }));
    assert_eq!(state["turn"], json!("black"));
}
#[test]
fn select_reselects_and_clears() {
    let mut g = game();
    assert!(select(&mut g, "e2").ok);
    let out = select(&mut g, "d2");
    assert!(out.ok);
    assert_eq!(out.data["selected"], json!("d2"));
    let out = select(&mut g, "d5");
    assert!(out.ok);
    assert_eq!(out.data["selected"], Json::Null);
    assert_eq!(g.state(&iden())["selected"], Json::Null);
    assert!(g.state(&iden())["targets"].as_array().unwrap().is_empty());
    let out = select(&mut g, "e7");
    assert!(out.ok);
    assert_eq!(out.data["selected"], Json::Null);
}
#[test]
fn select_then_move_matches_move() {
    let mut a = game();
    select(&mut a, "g1");
    select(&mut a, "f3");
    let mut b = game();
    mv(&mut b, "g1", "f3");
    assert_eq!(a.state(&iden()), b.state(&iden()));
    assert_eq!(a.save(), b.save());
}
#[test]
fn select_accepts_grid_coords() {
    let mut g = game();
    let out = g.act(
        &iden(),
        &Call::new("chess.select", json!({ "x": 4, "y": 6 })),
    );
    assert!(out.ok);
    assert_eq!(out.data["selected"], json!("e2"));
    assert!(
        !g.act(
            &iden(),
            &Call::new("chess.select", json!({ "x": 9, "y": 0 }))
        )
        .ok
    );
    assert!(!select(&mut g, "z9").ok);
}
#[test]
fn select_respects_the_round() {
    let mut g = game();
    mv(&mut g, "f2", "f3");
    mv(&mut g, "e7", "e5");
    mv(&mut g, "g2", "g4");
    mv(&mut g, "d8", "h4");
    let out = select(&mut g, "e2");
    assert!(!out.ok);
    assert_eq!(out.note.as_deref(), Some("round over, reset to continue"));
}
#[test]
fn targets_match_the_moves_fact() {
    let mut g = game();
    select(&mut g, "g1");
    let state = g.state(&iden());
    let mut targets: Vec<String> = state["targets"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t.as_str().unwrap().to_string())
        .collect();
    targets.sort();
    let mut knight: Vec<String> = state["moves"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|m| m["from"] == json!("g1"))
        .map(|m| m["to"].as_str().unwrap().to_string())
        .collect();
    knight.sort();
    assert_eq!(targets, vec!["f3", "h3"]);
    assert_eq!(targets, knight);
}
#[test]
fn moves_emit_cues() {
    let mut g = game();
    let out = select(&mut g, "e2");
    assert_eq!(out.effects[0].kind, "sound");
    assert_eq!(out.effects[0].data, cue::payload("blip"));
    let out = select(&mut g, "e4");
    assert_eq!(out.effects[0].data, cue::payload("blip"));
    mv(&mut g, "d7", "d5");
    let out = mv(&mut g, "e4", "d5");
    assert_eq!(out.effects[0].data, cue::payload("good"));
    let out = mv(&mut g, "d5", "d4");
    assert!(!out.ok);
    assert_eq!(out.effects[0].data, cue::payload("bad"));
    let mut g = game();
    mv(&mut g, "f2", "f3");
    mv(&mut g, "e7", "e5");
    mv(&mut g, "g2", "g4");
    let out = mv(&mut g, "d8", "h4");
    assert_eq!(out.effects[0].data, cue::payload("win"));
    let mut m = custom("7k/8/8/6Q1/8/8/8/K7");
    let out = mv(&mut m, "g5", "g6");
    assert_eq!(out.effects[0].data, cue::payload("lose"));
}
#[test]
fn check_sounds_the_alarm() {
    let mut g = game();
    mv(&mut g, "e2", "e4");
    mv(&mut g, "f7", "f6");
    let out = mv(&mut g, "d1", "h5");
    assert_eq!(out.effects[0].data, cue::payload("bad"));
}
#[test]
fn surface_and_skin_validate() {
    let mut g = game();
    let out = g.act(
        &iden(),
        &Call::new("chess.set", json!({ "key": "surface", "value": "canvas" })),
    );
    assert!(out.ok);
    assert!(
        !g.act(
            &iden(),
            &Call::new("chess.set", json!({ "key": "skin", "value": "emojis" }))
        )
        .ok
    );
    assert!(
        !g.act(
            &iden(),
            &Call::new("chess.set", json!({ "key": "skin", "value": "tiles" }))
        )
        .ok
    );
    assert!(
        !g.act(
            &iden(),
            &Call::new("chess.set", json!({ "key": "surface", "value": "cube" }))
        )
        .ok
    );
    let out = g.act(
        &iden(),
        &Call::new("chess.set", json!({ "key": "surface", "value": "grid" })),
    );
    assert!(out.ok);
    let out = g.act(
        &iden(),
        &Call::new("chess.set", json!({ "key": "skin", "value": "emojis" })),
    );
    assert!(out.ok);
    assert!(
        !g.act(
            &iden(),
            &Call::new("chess.set", json!({ "key": "surface", "value": "canvas" }))
        )
        .ok
    );
}
#[test]
fn from_json_resets_legality() {
    let set = Set::from_json(&json!({ "surface": "canvas", "skin": "emojis" }));
    assert!(!(set.skin == "emojis" && set.surface != "grid"));
    let set = Set::from_json(&json!({ "tile": 8 }));
    assert_eq!(set.surface, "grid");
    assert_eq!(set.skin, "digits");
    let mut g = Chess::new();
    g.load(&json!({ "seed": 3, "settings": { "surface": "canvas", "skin": "emojis" } }));
    let settings = g.state(&iden())["settings"].clone();
    assert!(!(settings["skin"] == json!("emojis") && settings["surface"] == json!("canvas")));
}
#[test]
fn selection_is_transient_across_save() {
    let mut a = game();
    mv(&mut a, "e2", "e4");
    select(&mut a, "e7");
    let saved = a.save();
    assert_eq!(saved.get("selected"), None);
    let mut b = Chess::new();
    b.load(&saved);
    let state = b.state(&iden());
    assert_eq!(state["selected"], Json::Null);
    assert!(state["targets"].as_array().unwrap().is_empty());
    assert_eq!(state["last_move"], json!({ "from": "e2", "to": "e4" }));
    assert_eq!(state["board"], a.state(&iden())["board"]);
}
#[test]
fn last_move_load_rejects_garbage() {
    let mut a = game();
    mv(&mut a, "e2", "e4");
    let mut saved = a.save();
    saved["last_move"] = json!([999, 1]);
    let mut b = Chess::new();
    b.load(&saved);
    assert_eq!(b.state(&iden())["last_move"], Json::Null);
}
#[test]
fn facts_stay_semantic() {
    let mut g = game();
    select(&mut g, "e2");
    let state = g.state(&iden());
    let mut keys: Vec<&str> = state
        .as_object()
        .unwrap()
        .keys()
        .map(|k| k.as_str())
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        vec![
            "board",
            "check",
            "frame",
            "last_move",
            "moves",
            "over",
            "score",
            "seed",
            "selected",
            "settings",
            "steps",
            "targets",
            "turn",
            "winner",
        ]
    );
}

#[test]
fn state_carries_an_indexed_frame() {
    let g = game();
    let state = g.state(&iden());
    let palette = state["frame"]["palette"].as_array().unwrap();
    assert!(!palette.is_empty());
    let rows = state["frame"]["rows"].as_array().unwrap();
    assert_eq!(
        rows.len(),
        state["frame"]["height"].as_u64().unwrap() as usize
    );
    assert_eq!(state["board"].as_array().unwrap().len(), 8);
}
