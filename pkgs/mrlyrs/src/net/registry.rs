use crate::apps::{
    Bang, Billiards, Calculator, Calendar, Captcha, Chess, Clock, Colors, Crush, Dice, Emoji,
    Escape, Extras, Files, Font, Hash, Identity, Julia, Lasers, Life, Log, Mandelbrot, Matrix,
    Memory, Menu, Mines, Moire, Notes, Pages, Photos, Piano, Pixel, Quiz, Settings, Six, Sleep,
    Snake, Solids, Tennis, Text, Three, Tile, Timer, Ttt, Twenty48, Two, Ui, Waves,
};
use crate::os::kernel::App;

pub fn catalogue() -> Vec<Box<dyn App>> {
    vec![
        Box::new(Menu::new()),
        Box::new(Calculator::new()),
        Box::new(Notes::new()),
        Box::new(Settings::new()),
        Box::new(Ui::new()),
        Box::new(Life::new()),
        Box::new(Clock::new()),
        Box::new(Timer::new()),
        Box::new(Calendar::new()),
        Box::new(Dice::new()),
        Box::new(Photos::new()),
        Box::new(Pages::new()),
        Box::new(Snake::new()),
        Box::new(Julia::new()),
        Box::new(Mandelbrot::new()),
        Box::new(Matrix::new()),
        Box::new(Sleep::new()),
        Box::new(Ttt::new()),
        Box::new(Memory::new()),
        Box::new(Mines::new()),
        Box::new(Twenty48::new()),
        Box::new(Crush::new()),
        Box::new(Tennis::new()),
        Box::new(Escape::new()),
        Box::new(Quiz::new()),
        Box::new(Captcha::new()),
        Box::new(Pixel::new()),
        Box::new(Solids::new()),
        Box::new(Font::new()),
        Box::new(Text::new()),
        Box::new(Two::new()),
        Box::new(Three::new()),
        Box::new(Bang::new()),
        Box::new(Tile::new()),
        Box::new(Six::new()),
        Box::new(Waves::new()),
        Box::new(Billiards::new()),
        Box::new(Lasers::new()),
        Box::new(Chess::new()),
        Box::new(Moire::new()),
        Box::new(Hash::new()),
        Box::new(Colors::new()),
        Box::new(Emoji::new()),
        Box::new(Piano::new()),
        Box::new(Extras::new()),
        Box::new(Log::new()),
        Box::new(Files::new()),
        Box::new(Identity::new()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::{Iden, Os};

    #[test]
    fn routes_are_unique_and_menu_leads() {
        let apps = catalogue();
        assert_eq!(apps[0].route(), "menu");
        let mut routes: Vec<String> = apps.iter().map(|a| a.route().to_string()).collect();
        let count = routes.len();
        routes.sort();
        routes.dedup();
        assert_eq!(routes.len(), count);
    }
    #[test]
    fn the_catalogue_installs() {
        let mut os = Os::new(Iden::new("aria"));
        for app in catalogue() {
            os = os.install(app);
        }
        assert_eq!(os.catalogue().len(), catalogue().len());
        assert_eq!(os.frame().route.unwrap().app, "menu");
    }
}
