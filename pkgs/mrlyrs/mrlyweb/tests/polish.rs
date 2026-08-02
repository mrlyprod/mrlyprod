use mrlyui::face::{HEIGHT, WIDTH};
use mrlyweb::drive::{Act, Driver};

const PROMOTED: [&str; 4] = ["menu", "snake", "twenty48", "mandelbrot"];

fn open(app: &str) -> Driver {
    let mut driver = Driver::scripted(0);
    driver.open(app);
    driver
}

#[test]
fn every_hit_stays_on_the_sheet() {
    for app in PROMOTED {
        let driver = open(app);
        for hit in &driver.scene().hits {
            assert!(hit.w > 0 && hit.h > 0, "{app} has an empty hit");
            assert!(hit.x + hit.w <= WIDTH, "{app} hit runs off the side");
            assert!(hit.y + hit.h <= HEIGHT, "{app} hit runs off the bottom");
        }
    }
}

#[test]
fn no_two_stops_claim_the_same_centre() {
    for app in PROMOTED {
        let driver = open(app);
        let stops: Vec<&mrlyweb::drive::Hit> = driver
            .scene()
            .hits
            .iter()
            .filter(|hit| matches!(hit.act, Act::Tap { .. } | Act::Edit { .. }))
            .collect();
        for (i, hit) in stops.iter().enumerate() {
            let (cx, cy) = (hit.x + hit.w / 2, hit.y + hit.h / 2);
            let over = stops
                .iter()
                .skip(i + 1)
                .filter(|other| other.holds(cx, cy))
                .count();
            assert_eq!(over, 0, "{app} buries a stop under another");
        }
    }
}

#[test]
fn every_plain_verb_is_one_tap_away() {
    for app in PROMOTED {
        let driver = open(app);
        let Some(view) = driver.os().frame(None).view else {
            panic!("{app} has no view")
        };
        let plain: Vec<String> = view
            .actions
            .iter()
            .filter(|verb| verb.args.as_object().is_none_or(|args| args.is_empty()))
            .map(|verb| verb.name.clone())
            .collect();
        for name in plain {
            let found = driver
                .scene()
                .hits
                .iter()
                .any(|hit| matches!(&hit.act, Act::Tap { call } if call.verb == name));
            assert!(found, "{app} hides {name} from the pointer");
        }
    }
}

#[test]
fn a_promoted_app_survives_a_mash() {
    for app in PROMOTED {
        let mut driver = open(app);
        let mut seed: u64 = 7;
        for _ in 0..400 {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let x = (seed >> 33) as usize % WIDTH;
            let y = (seed >> 17) as usize % HEIGHT;
            driver.hover(Some((x, y)));
            driver.tap_at(x, y);
            if driver.route() != app {
                driver.open(app);
            }
        }
        assert_eq!(driver.scene().frame.height, HEIGHT, "{app} lost its sheet");
    }
}

#[test]
fn the_sheet_is_whole_at_every_scale_it_can_take() {
    for scale in 1..=4usize {
        let (bw, bh) = (WIDTH * scale, HEIGHT * scale);
        let (ox, oy, got) = mrlyui::face::fit(bw, bh, WIDTH, HEIGHT);
        assert_eq!(got, scale, "a {scale}x window did not read as {scale}x");
        assert!(
            ox + WIDTH * got <= bw && oy + HEIGHT * got <= bh,
            "the sheet overflows a {scale}x window"
        );
    }
    for (bw, bh) in [
        (WIDTH + WIDTH / 2, HEIGHT + HEIGHT / 2),
        (WIDTH * 4, HEIGHT),
        (WIDTH, HEIGHT * 4),
        (1, 1),
    ] {
        let (ox, oy, got) = mrlyui::face::fit(bw, bh, WIDTH, HEIGHT);
        assert!(got >= 1, "the scale never drops below 1x");
        assert!(
            ox + WIDTH * got <= bw.max(WIDTH) && oy + HEIGHT * got <= bh.max(HEIGHT),
            "a {bw}x{bh} window let the sheet run out"
        );
    }
    let (_, _, lopsided) = mrlyui::face::fit(WIDTH * 4, HEIGHT, WIDTH, HEIGHT);
    assert_eq!(lopsided, 1, "a wide-but-short window must stay at 1x");
}

#[test]
fn the_desk_takes_its_colour_from_the_setting() {
    let mut driver = Driver::scripted(0);
    let spare = [1, 2, 3, 255];
    driver.act(
        "settings.set",
        mrlycore::json!({ "key": "background", "value": "teal" }),
    );
    let teal = mrlycore::colors::named("teal").unwrap();
    assert_eq!(driver.desk(spare), [teal.r, teal.g, teal.b, 255]);
    driver.act(
        "settings.set",
        mrlycore::json!({ "key": "background", "value": "black" }),
    );
    let black = mrlycore::colors::named("black").unwrap();
    assert_eq!(driver.desk(spare), [black.r, black.g, black.b, 255]);
    assert_ne!(driver.desk(spare), spare, "the desk ignored the setting");
}

#[test]
fn the_desk_washes_from_the_board_down_to_a_colour() {
    let mut driver = Driver::scripted(0);
    driver.open("menu");
    let board = [255, 255, 255, 255];
    let (top, foot) = driver.wash(board);
    assert_eq!(top, board, "the wash starts at the board");
    assert_ne!(
        foot, board,
        "an unset background still washes to the accent"
    );
    driver.act(
        "settings.set",
        mrlycore::json!({ "key": "background", "value": "teal" }),
    );
    driver.open("menu");
    let (top, teal) = driver.wash(board);
    assert_eq!(top, board);
    assert_eq!(
        teal,
        [
            mrlycore::colors::TEAL.r,
            mrlycore::colors::TEAL.g,
            mrlycore::colors::TEAL.b,
            255
        ],
        "the background setting picks the base"
    );
    driver.act(
        "settings.set",
        mrlycore::json!({ "key": "background", "value": "white" }),
    );
    driver.open("snake");
    let (_, snake) = driver.wash(board);
    driver.open("twenty48");
    let (_, other) = driver.wash(board);
    assert_ne!(snake, other, "each app washes toward its own accent");
}
