use mrlycore::Json;

pub mod creativity;
pub mod design;
pub mod games;
pub mod math;
pub mod physics;
pub mod puzzles;
pub mod system;
pub mod tools;
pub mod toys;

pub use creativity::notes::Notes;
pub use creativity::photos::Photos;
pub use creativity::piano::Piano;
pub use design::colors::Colors;
pub use design::emoji::Emoji;
pub use design::font::Font;
pub use design::pixel::Pixel;
pub use games::crush::Crush;
pub use games::escape::Escape;
pub use games::snake::Snake;
pub use games::tennis::Tennis;
pub use math::bang::Bang;
pub use math::life::Life;
pub use math::moire::Moire;
pub use math::six::Six;
pub use math::three::Three;
pub use math::tile::Tile;
pub use math::two::Two;
pub use physics::billiards::Billiards;
pub use physics::lasers::Lasers;
pub use physics::waves::Waves;
pub use puzzles::captcha::Captcha;
pub use puzzles::chess::Chess;
pub use puzzles::memory::Memory;
pub use puzzles::mines::Mines;
pub use puzzles::quiz::Quiz;
pub use puzzles::ttt::Ttt;
pub use puzzles::twenty48::Twenty48;
pub use system::files::Files;
pub use system::iden::Identity;
pub use system::log::Log;
pub use system::menu::Menu;
pub use system::settings::Settings;
pub use system::ui::Ui;
pub use tools::calculator::Calculator;
pub use tools::calendar::Calendar;
pub use tools::clock::Clock;
pub use tools::dice::Dice;
pub use tools::hash::Hash;
pub use tools::timer::Timer;
pub use toys::julia::Julia;
pub use toys::mandelbrot::Mandelbrot;
pub use toys::matrix::Matrix;
pub use toys::sleep::Sleep;
pub use toys::solids::Solids;

pub fn asked(shape: Option<&Json>, key: &str) -> bool {
    match shape.and_then(|s| s.as_object()) {
        Some(keys) => keys.contains_key(key),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::json;

    #[test]
    fn no_shape_asks_for_everything() {
        assert!(asked(None, "frame"));
    }

    #[test]
    fn a_take_all_shape_asks_for_everything() {
        assert!(asked(Some(&json!(1)), "frame"));
    }

    #[test]
    fn a_keyed_shape_asks_only_for_its_keys() {
        let shape = json!({ "steps": 1 });
        assert!(!asked(Some(&shape), "frame"));
        assert!(asked(Some(&shape), "steps"));
    }
}
