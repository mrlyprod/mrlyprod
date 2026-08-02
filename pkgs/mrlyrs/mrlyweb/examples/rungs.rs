use mrlyweb::drive::Driver;
use std::fs;

fn main() {
    let out = std::env::args().nth(1).expect("an output directory");
    let app = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "menu".to_string());
    for at in 0..mrlyui::tokens::RUNGS.len() {
        let (w, h) = mrlyui::tokens::rung(at);
        let mut driver = Driver::scripted(0);
        driver.fit_sheet(w, h);
        driver.open(&app);
        let png = driver.shot().expect("a shot");
        let path = format!("{out}/{app}-{w}x{h}.png");
        fs::write(&path, png).unwrap();
        println!("{path}");
    }
}
