use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::ink::Ramp;
use mrlyfig::{field, ink, save};
use mrlylab::moire::presets::{self, Preset};
use mrlylab::moire::sample;

const LIMIT: usize = 9;
const LAYERS: usize = 5;
const PANEL: usize = 420;
const GUTTER: f64 = 20.0;

fn values(preset: &Preset) -> Result<Vec<f64>> {
    let sheet = preset.field(PANEL)?;
    assert_eq!(sheet.data.len(), PANEL * PANEL);
    Ok(sheet.as_f64())
}

fn main() -> Result<()> {
    let recipes = presets::all(LIMIT);
    assert_eq!(recipes.len(), 4);
    for recipe in &recipes {
        assert_eq!(recipe.numbers, vec![1, 3, 5, 7, 9]);
        assert_eq!(recipe.numbers.len(), LAYERS);
    }
    let table = sample::membership(recipes[3].spec.code, 3, 2)?;
    assert_eq!(table.iter().filter(|&&lit| lit).count(), 8);
    assert!(!table[sample::pack(&[1, 1], 3)]);

    let sheets = recipes
        .iter()
        .map(values)
        .collect::<Result<Vec<Vec<f64>>>>()?;
    let woven = &sheets[1];
    assert!(woven.iter().all(|&v| v == 0.0 || v == 1.0));
    for counted in [&sheets[0], &sheets[2], &sheets[3]] {
        let high = counted.iter().copied().fold(f64::MIN, f64::max);
        assert_eq!(high, LAYERS as f64);
    }

    let ramp = Ramp::new(vec![
        ink::mix(ink::ground(), ink::blue(), 0.38),
        ink::blue(),
        ink::yellow(),
    ]);
    let mut board = Board::square();
    let side = 2.0 * PANEL as f64 + GUTTER;
    let edge = (board.width as f64 - side) / 2.0;
    for (index, sheet) in sheets.iter().enumerate() {
        let x = edge + (index % 2) as f64 * (PANEL as f64 + GUTTER);
        let y = edge + (index / 2) as f64 * (PANEL as f64 + GUTTER);
        let frame = Frame::new(x, y, PANEL as f64, PANEL as f64);
        field::draw(&mut board, frame, PANEL, PANEL, sheet, &ramp);
    }
    save("demo-moire", &board)?;
    Ok(())
}
