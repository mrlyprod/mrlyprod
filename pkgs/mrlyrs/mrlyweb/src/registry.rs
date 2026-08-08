use mrlyapps::{
    Bang, Calculator, Calendar, Captcha, Chess, Clock, Colors, Crush, Dice, Emoji, Escape, Files,
    Font, Hash, Identity, Julia, Life, Log, Mandelbrot, Matrix, Memory, Menu, Mines, Moire, Notes,
    Photos, Piano, Pixel, Quiz, Settings, Six, Sleep, Snake, Solids, Tennis, Three, Tile, Timer,
    Ttt, Twenty48, Two, Ui,
};
use mrlyos::kernel::{App, Iden, Os};

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
        Box::new(Two::new()),
        Box::new(Three::new()),
        Box::new(Bang::new()),
        Box::new(Tile::new()),
        Box::new(Six::new()),
        Box::new(Chess::new()),
        Box::new(Moire::new()),
        Box::new(Hash::new()),
        Box::new(Colors::new()),
        Box::new(Emoji::new()),
        Box::new(Piano::new()),
        Box::new(Log::new()),
        Box::new(Files::new()),
        Box::new(Identity::new()),
    ]
}

const SYSTEM: [&str; 6] = ["menu", "settings", "ui", "iden", "log", "files"];

const ARCADE: [&str; 23] = [
    "snake",
    "crush",
    "tennis",
    "escape",
    "chess",
    "ttt",
    "memory",
    "mines",
    "twenty48",
    "quiz",
    "captcha",
    "life",
    "bang",
    "two",
    "three",
    "six",
    "tile",
    "moire",
    "julia",
    "mandelbrot",
    "matrix",
    "sleep",
    "solids",
];

pub fn loadout(name: &str) -> Vec<Box<dyn App>> {
    if name != "arcade" {
        return catalogue();
    }
    catalogue()
        .into_iter()
        .filter(|app| SYSTEM.contains(&app.route()) || ARCADE.contains(&app.route()))
        .collect()
}

pub fn boot(name: &str) -> Os {
    let mut os = Os::new(Iden::new("guest"));
    for app in loadout(name) {
        os = os.install(app);
    }
    os
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlyos::kernel::{Iden, Os};

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
        assert_eq!(os.envelope(None).route.unwrap().app, "menu");
    }
    #[test]
    fn the_arcade_keeps_the_system_and_the_games() {
        let routes: Vec<String> = loadout("arcade")
            .iter()
            .map(|a| a.route().to_string())
            .collect();
        assert_eq!(routes.len(), 29);
        assert_eq!(routes[0], "menu");
        assert!(routes.contains(&"snake".to_string()));
        assert!(routes.contains(&"solids".to_string()));
        for left in [
            "notes",
            "calculator",
            "calendar",
            "clock",
            "timer",
            "photos",
            "piano",
            "colors",
            "emoji",
            "font",
            "pixel",
            "dice",
            "hash",
        ] {
            assert!(!routes.contains(&left.to_string()), "{left} rode along");
        }
    }
    #[test]
    fn an_unknown_loadout_boots_everything() {
        assert_eq!(loadout("full").len(), catalogue().len());
        assert_eq!(loadout("").len(), catalogue().len());
        assert_eq!(boot("full").catalogue().len(), catalogue().len());
    }
    #[test]
    fn every_named_route_exists() {
        let routes: Vec<String> = catalogue().iter().map(|a| a.route().to_string()).collect();
        for name in SYSTEM.iter().chain(ARCADE.iter()) {
            assert!(routes.contains(&name.to_string()), "{name} names no app");
        }
    }
}
