use mrlycore::colors::LIGHT;
use mrlydemo::set_theme;
use mrlydemo::spiral::spiral_pixels;

#[test]
fn the_light_theme_paints_the_ground_white() {
    set_theme(false);
    let sheet = spiral_pixels("square", 61, 4, -2, 41, "prime", false, 180).unwrap();
    let centre = (90 * 180 + 90) * 4;
    let ground = LIGHT.ground;
    assert_eq!(
        sheet.rgba[centre..centre + 3],
        [ground.r, ground.g, ground.b]
    );
    set_theme(true);
}
