use mrlyweb::crop::crop_circle;

struct Circle {
    seen: Vec<u32>,
    inside: Vec<u32>,
    cut: Vec<u32>,
}

fn circle(code: &str, dimension: usize, level: usize, centre: &str) -> Circle {
    let flat = crop_circle(code, 3, level, 2, dimension, centre).unwrap();
    let width = flat.len() / 3;
    Circle {
        seen: flat[..width].to_vec(),
        inside: flat[width..2 * width].to_vec(),
        cut: flat[2 * width..].to_vec(),
    }
}

#[test]
fn the_circle_count_the_page_prints() {
    let carpet = circle("7", 2, 6, "corner");
    println!("crop_circle carpet corner radii {}", carpet.seen.len());
    assert_eq!(carpet.seen.len(), 729);
    println!("crop_circle carpet corner 1..12 {:?}", &carpet.seen[1..=12]);
    assert_eq!(
        carpet.seen[1..=12],
        [1, 3, 7, 12, 16, 22, 30, 38, 48, 63, 77, 91]
    );
    let powers: Vec<u32> = (1..=5).map(|k| carpet.seen[3usize.pow(k)]).collect();
    println!("crop_circle carpet N(3^k) {powers:?}");
    assert_eq!(powers, [7, 48, 385, 3080, 24610]);
    let defect = carpet.seen[81] as i64 - 8 * carpet.seen[27] as i64;
    println!("crop_circle carpet delta(27) {defect}");
    assert_eq!(defect, 0);
    let cuts = [carpet.cut[27], carpet.cut[81], carpet.cut[243]];
    println!("crop_circle carpet C(3^k) {cuts:?}");
    assert_eq!(cuts, [42, 114, 306]);
    println!(
        "crop_circle carpet brackets 243 {} {} {}",
        carpet.inside[243],
        carpet.seen[243],
        carpet.inside[243] + carpet.cut[243]
    );
    for r in 1..carpet.seen.len() {
        assert!(carpet.inside[r] <= carpet.seen[r], "in r={r}");
        assert!(
            carpet.seen[r] <= carpet.inside[r] + carpet.cut[r],
            "out r={r}"
        );
    }
    let sponge = circle("23", 3, 4, "corner");
    println!("crop_circle sponge corner radii {}", sponge.seen.len());
    assert_eq!(sponge.seen.len(), 81);
    println!("crop_circle sponge corner 1..8 {:?}", &sponge.seen[1..=8]);
    assert_eq!(sponge.seen[1..=8], [1, 4, 13, 28, 47, 65, 95, 137]);
    let hole = circle("7", 2, 6, "centre");
    let first = (1..hole.seen.len()).find(|&r| hole.seen[r] > 0).unwrap();
    println!(
        "crop_circle carpet centre radii {} first {first}",
        hole.seen.len()
    );
    assert_eq!((hole.seen.len(), first), (365, 122));
    let deep = circle("23", 3, 4, "centre");
    let hit = (1..deep.seen.len()).find(|&r| deep.seen[r] > 0).unwrap();
    println!(
        "crop_circle sponge centre radii {} first {hit}",
        deep.seen.len()
    );
    assert_eq!((deep.seen.len(), hit), (41, 20));
    assert!(crop_circle("7", 3, 6, 2, 2, "edge").is_err());
    assert!(crop_circle("7", 3, 9, 2, 2, "corner").is_err());
}
