use crate::shaders::Program;
use mrlybang::Bang;
use mrlycalculator::Calculator;
use mrlycalendar::Calendar;
use mrlycaptcha::Captcha;
use mrlychess::Chess;
use mrlyclock::Clock;
use mrlycolors::Colors;
use mrlycore::json::Map;
use mrlycore::Json;
use mrlycrush::Crush;
use mrlydice::Dice;
use mrlyemojis::Emojis;
use mrlyescape::Escape;
use mrlyfiles::Files;
use mrlyfonts::Fonts;
use mrlygraph::Graph;
use mrlyhash::Hash;
use mrlyiden::Identity;
use mrlyjulia::Julia;
use mrlylife::Life;
use mrlylog::Log;
use mrlymandelbrot::Mandelbrot;
use mrlymatrix::Matrix;
use mrlymemory::Memory;
use mrlymenu::Menu;
use mrlymines::Mines;
use mrlymoire::Moire;
use mrlynotes::Notes;
use mrlyos::kernel::{App, Iden, Os};
use mrlypaint::Paint;
use mrlyphotos::Photos;
use mrlyquiz::Quiz;
use mrlysettings::Settings;
use mrlysix::Six;
use mrlyskin::paint::Raster;
use mrlyskin::Skin;
use mrlysleep::Sleep;
use mrlysnake::Snake;
use mrlysolids::Solids;
use mrlystudio::Studio;
use mrlytennis::Tennis;
use mrlythree::Three;
use mrlytile::Tile;
use mrlytimer::Timer;
use mrlyttt::Ttt;
use mrlytwenty48::Twenty48;
use mrlytwo::Two;

/// Builds one of every app, menu first.
pub fn catalogue() -> Vec<Box<dyn App>> {
    vec![
        Box::new(Menu::new()),
        Box::new(Calculator::new()),
        Box::new(Notes::new()),
        Box::new(Settings::new()),
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
        Box::new(Paint::new()),
        Box::new(Solids::new()),
        Box::new(Fonts::new()),
        Box::new(Two::new()),
        Box::new(Three::new()),
        Box::new(Bang::new()),
        Box::new(Tile::new()),
        Box::new(Six::new()),
        Box::new(Graph::new()),
        Box::new(Chess::new()),
        Box::new(Moire::new()),
        Box::new(Hash::new()),
        Box::new(Colors::new()),
        Box::new(Emojis::new()),
        Box::new(Studio::new()),
        Box::new(Log::new()),
        Box::new(Files::new()),
        Box::new(Identity::new()),
    ]
}

const SYSTEM: [&str; 5] = ["menu", "settings", "iden", "log", "files"];

const ARCADE: [&str; 24] = [
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
    "graph",
    "tile",
    "moire",
    "julia",
    "mandelbrot",
    "matrix",
    "sleep",
    "solids",
];

/// Builds the named loadout: arcade keeps the system and the games, any other name means everything.
pub fn loadout(name: &str) -> Vec<Box<dyn App>> {
    if name != "arcade" {
        return catalogue();
    }
    catalogue()
        .into_iter()
        .filter(|app| SYSTEM.contains(&app.route()) || ARCADE.contains(&app.route()))
        .collect()
}

/// Lists the gpu programs with their fragment sources and uniform sizes.
pub fn programs() -> Vec<Program> {
    vec![
        Program {
            name: "mandelbrot",
            fragment: mrlymandelbrot::FRAGMENT,
            floats: mrlymandelbrot::FLOATS,
            whole: false,
        },
        Program {
            name: "julia",
            fragment: mrlyjulia::FRAGMENT,
            floats: mrlyjulia::FLOATS,
            whole: false,
        },
        Program {
            name: "mesh",
            fragment: mrlymath::space::MESH_WGSL,
            floats: 24,
            whole: true,
        },
        Program {
            name: "moire",
            fragment: mrlymoire::FRAGMENT,
            floats: mrlymoire::FLOATS,
            whole: false,
        },
        Program {
            name: "sleep",
            fragment: mrlysleep::FRAGMENT,
            floats: mrlysleep::FLOATS,
            whole: false,
        },
    ]
}

/// A mesh route's resting camera: where its eye sits before anyone spins it.
pub struct Rig {
    /// The route the camera belongs to.
    pub route: &'static str,
    /// The yaw, in turn steps.
    pub yaw: i64,
    /// The pitch, in turn steps.
    pub pitch: i64,
    /// The distance, in quarters.
    pub dist: i64,
    /// The pan offset, in sixteenths.
    pub pan: [i64; 2],
    /// Whether the lens is orthographic.
    pub ortho: bool,
}

/// Lists the resting camera of every mesh route.
pub fn rigs() -> Vec<Rig> {
    vec![
        Rig {
            route: "bang",
            yaw: 32,
            pitch: 20,
            dist: 12,
            pan: [0, 0],
            ortho: false,
        },
        Rig {
            route: "graph",
            yaw: 32,
            pitch: 20,
            dist: 12,
            pan: [0, 0],
            ortho: false,
        },
        Rig {
            route: "solids",
            yaw: 32,
            pitch: 20,
            dist: 12,
            pan: [0, 0],
            ortho: false,
        },
        Rig {
            route: "three",
            yaw: 32,
            pitch: 20,
            dist: 12,
            pan: [0, 0],
            ortho: true,
        },
    ]
}

/// Returns the uniform float count of the named program, or None for a stranger.
pub fn floats(name: &str) -> Option<usize> {
    programs()
        .into_iter()
        .find(|program| program.name == name)
        .map(|program| program.floats)
}

/// Lists every skinned route with its wardrobe of named skins.
pub fn wardrobes() -> Vec<(&'static str, Vec<(&'static str, Skin)>)> {
    vec![
        ("bang", mrlybang::skin::corpus()),
        ("captcha", mrlycaptcha::skin::corpus()),
        ("chess", mrlychess::skin::corpus()),
        ("crush", mrlycrush::skin::corpus()),
        ("dice", mrlydice::skin::corpus()),
        ("escape", mrlyescape::skin::corpus()),
        ("hash", mrlyhash::skin::corpus()),
        ("life", mrlylife::skin::corpus()),
        ("matrix", mrlymatrix::skin::corpus()),
        ("memory", mrlymemory::skin::corpus()),
        ("mines", mrlymines::skin::corpus()),
        ("paint", mrlypaint::skin::corpus()),
        ("quiz", mrlyquiz::skin::corpus()),
        ("snake", mrlysnake::skin::corpus()),
        ("tennis", mrlytennis::skin::corpus()),
        ("tile", mrlytile::skin::corpus()),
        ("ttt", mrlyttt::skin::corpus()),
        ("twenty48", mrlytwenty48::skin::corpus()),
        ("two", mrlytwo::skin::corpus()),
    ]
}

/// Folds every wardrobe into one Json object, keyed by route and then variant.
pub fn corpus() -> Json {
    let mut out = Map::new();
    for (route, wardrobe) in wardrobes() {
        let mut entry = Map::new();
        for (variant, skin) in wardrobe {
            entry.insert(variant.to_string(), skin.to_json());
        }
        out.insert(route.to_string(), Json::Obj(entry));
    }
    Json::Obj(out)
}

/// Dresses a route's cells at the given tile size, or None for a route with no wardrobe.
pub fn raster(route: &str, cells: &Json, tile: usize, dark: bool) -> Option<Raster> {
    let (_, wardrobe) = wardrobes().into_iter().find(|(name, _)| *name == route)?;
    mrlyskin::raster(wardrobe, cells, tile, dark)
}

/// Boots a guest os with the named loadout installed.
/// ```
/// assert_eq!(mrlyweb::registry::boot("arcade").catalogue().len(), 29);
/// ```
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
    use mrlycore::json;
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
            "studio",
            "colors",
            "emojis",
            "fonts",
            "paint",
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
    #[test]
    fn every_program_is_complete() {
        assert!(!programs().is_empty());
        for program in programs() {
            let source = crate::shaders::assemble(&program);
            let name = program.name;
            assert!(source.contains("fn vs_main"), "{name} misses vs_main");
            assert!(source.contains("fn fs_main"), "{name} misses fs_main");
            assert!(source.contains("var<uniform>"), "{name} misses uniforms");
        }
    }
    #[test]
    fn uniform_sizes_hold_the_std_layout() {
        for program in programs() {
            let name = program.name;
            assert!(program.floats >= 12, "{name} misses the shared head");
            assert_eq!(program.floats % 4, 0, "{name} is not 16 byte aligned");
        }
    }
    #[test]
    fn program_lookups_agree() {
        assert_eq!(floats("mandelbrot"), Some(20));
        assert_eq!(floats("mesh"), Some(24));
        assert!(floats("nothing").is_none());
    }
    #[test]
    fn corpus_names_every_tile_app() {
        let routes: Vec<String> = catalogue().iter().map(|a| a.route().to_string()).collect();
        let all = corpus();
        for (app, _) in wardrobes() {
            assert!(routes.contains(&app.to_string()), "{app} names no app");
            let wardrobe = all[app].as_object().unwrap_or_else(|| {
                panic!("{app} misses its corpus entry");
            });
            assert!(!wardrobe.is_empty(), "{app} is undressed");
            for (variant, skin) in wardrobe {
                assert!(
                    !skin.as_array().unwrap().is_empty(),
                    "{app} {variant} has no roles"
                );
            }
        }
        assert_eq!(all.as_object().unwrap().len(), 19);
        assert_eq!(all["ttt"]["tiles"][1]["bg"], json!({ "pen": 0 }));
        assert_eq!(all["ttt"]["tiles"][1]["motif"], json!("design"));
        assert_eq!(
            all["twenty48"]["digits"][3]["face"],
            json!({ "as": "glyph", "value": "8" })
        );
        assert_eq!(all["paint"]["tiles"][7]["bg"], json!({ "pen": 7 }));
    }
}
