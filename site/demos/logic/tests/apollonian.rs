use demos::apollonian::*;
use mrlyrs::core::error::parse;
use mrlyrs::core::Json;

fn read(root: &str, cap: u32, order: usize) -> Json {
    parse(&apollonian_read(root, cap, order).unwrap()).unwrap()
}

#[test]
fn the_growth_lands_on_the_counts_the_generator_prints() {
    let strip = read("strip", 2048, 32);
    assert_eq!(
        format!(
            "{} {} {} {} {}",
            strip["circles"], strip["drawn"], strip["quads"], strip["broken"], strip["strayed"]
        ),
        "2448 2450 2449 0 0"
    );
    assert_eq!(read("strip", 1000, 32)["circles"], 950);
    assert_eq!(read("-1,2,2,3", 1000, 32)["circles"], 3325);
    assert_eq!(read("-2,3,6,7", 1000, 32)["circles"], 1297);
    assert_eq!(read("-3,4,12,13", 1000, 32)["circles"], 741);
    assert_eq!(
        format!("{:.6}", strip["exponent"].as_f64().unwrap()),
        "1.292481"
    );
    assert_eq!(strip["from"], 512);
}

#[test]
fn every_circle_comes_back_as_a_centre_a_radius_and_its_curvature() {
    let circles = apollonian("strip", 2048).unwrap();
    assert_eq!(circles.len(), 2448 * 5);
    assert_eq!(circles[..5], [0.5, 0.125, 0.125, 8.0, 2.0]);
    for block in circles.chunks(5) {
        let [x, y, r, k, den] = [block[0], block[1], block[2], block[3], block[4]];
        assert!((r * k.abs() - 1.0).abs() < 1e-12);
        assert!(x > -r && x < 1.0 + r && y > 0.0 && y < 1.0);
        if den > 0.0 {
            assert!((y - r).abs() < 1e-12 && (k - 2.0 * den * den).abs() < 1e-12);
        }
    }
    assert_eq!(
        apollonian_root("strip").unwrap(),
        [0.0, 0.5, 0.5, 2.0, 1.0, 1.0, 0.5, 0.5, 2.0, 1.0]
    );
    let bounded = read("-1,2,2,3", 1000, 32);
    assert_eq!(apollonian_root("-1,2,2,3").unwrap().len(), 20);
    assert_eq!(bounded["frame"].to_string(), "[-1.0,-1.0,1.0,1.0]");
    assert_eq!(
        read("strip", 2048, 32)["frame"].to_string(),
        "[0.0,0.0,1.0,1.0]"
    );
}

#[test]
fn the_ford_circles_rest_on_the_reduced_fractions() {
    let strip = read("strip", 2048, 32);
    assert_eq!(format!("{} {}", strip["line"], strip["ford"]), "323 323");
    let marks = apollonian_touches("strip", 2048).unwrap();
    assert_eq!(marks.len(), 323 * 4);
    assert_eq!(marks[..4], [0.03125, 1.0, 32.0, 2048.0]);
    assert_eq!(marks[marks.len() - 4..], [0.96875, 31.0, 32.0, 2048.0]);
    let mut last = 0.0;
    for block in marks.chunks(4) {
        let [at, num, den, k] = [block[0], block[1], block[2], block[3]];
        assert!(at > last && (at - num / den).abs() < 1e-12);
        assert_eq!(k, 2.0 * den * den);
        last = at;
    }
    assert!(apollonian_touches("-1,2,2,3", 2048).unwrap().is_empty());
    assert_eq!(read("-1,2,2,3", 1000, 32)["line"], 0);
}

#[test]
fn the_stack_is_the_shadow_and_the_brightness_closes() {
    let deep = read("strip", 2048, 32);
    let shadow = &deep["shadow"];
    assert_eq!(
        format!(
            "{} {} {} {} {} {} {} {}",
            shadow["reach"],
            shadow["covered"],
            shadow["nodes"],
            shadow["touched"],
            shadow["missed"],
            shadow["offford"],
            shadow["bright"],
            shadow["want"]
        ),
        "2048 true 323 323 0 0 528 528"
    );
    let shallow = read("strip", 2048, 16);
    assert_eq!(
        format!(
            "{} {} {} {}",
            shallow["shadow"]["nodes"],
            shallow["shadow"]["touched"],
            shallow["shadow"]["missed"],
            shallow["shadow"]["bright"]
        ),
        "79 79 0 136"
    );
    assert_eq!(read("strip", 512, 32)["shadow"]["covered"], false);
    assert_eq!(read("-1,2,2,3", 1000, 32)["shadow"]["nodes"], 0);
}

#[test]
fn a_root_off_the_list_or_a_cap_past_the_ceiling_is_refused() {
    let caps = parse(&apollonian_caps()).unwrap();
    assert_eq!(
        format!(
            "{} {} {}",
            caps["curvature"], caps["circles"], caps["order"]
        ),
        "8192 200000 64"
    );
    assert_eq!(caps["roots"].as_array().unwrap().len(), 4);
    assert!(apollonian("gasket", 2048).is_err());
    assert!(apollonian("strip", 8193).is_err());
    assert!(apollonian("strip", 1).is_err());
    assert!(apollonian_read("strip", 2048, 65).is_err());
}
