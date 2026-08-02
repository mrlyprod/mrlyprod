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
        let fit = (bw / WIDTH).min(bh / HEIGHT).max(1);
        assert_eq!(fit, scale, "a {scale}x window did not read as {scale}x");
        assert!(
            WIDTH * fit <= bw && HEIGHT * fit <= bh,
            "the sheet overflows"
        );
    }
    let (bw, bh) = (WIDTH + WIDTH / 2, HEIGHT + HEIGHT / 2);
    let fit = (bw / WIDTH).min(bh / HEIGHT).max(1);
    assert_eq!(fit, 1, "a one-and-a-half window must stay at 1x");
    assert!(WIDTH * fit <= bw, "the sheet must sit inside its window");
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
