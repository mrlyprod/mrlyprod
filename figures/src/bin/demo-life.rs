use mrlyfig::out::root;
use mrlyfig::{ink, save, Board, Ramp};
use mrlyrs::core::errors::Result;
use mrlyrs::core::tensor::Tensor;
use mrlyrs::core::MrlyError;
use mrlyrs::life::{design_mask, next_grid, Boundary};
use mrlyrs::math::two::Cell2d;
use std::path::PathBuf;

const NAME: &str = "demo-life";
const SIDE: usize = 96;
const STEPS: usize = 512;
const SEED: [(usize, usize); 5] = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];

// DATA

fn path() -> PathBuf {
    root()
        .join("files")
        .join("figures")
        .join("data")
        .join(format!("{NAME}.json"))
}

fn write_data(first: &[i32], live: &[u8]) -> Result<PathBuf> {
    let file = path();
    let folder = file.parent().unwrap().to_path_buf();
    std::fs::create_dir_all(&folder)
        .map_err(|e| MrlyError::Value(format!("cannot make {folder:?}: {e}")))?;
    let one: Vec<String> = first.iter().map(|step| step.to_string()).collect();
    let two: Vec<String> = live.iter().map(|bit| bit.to_string()).collect();
    let text = format!(
        "{{\"first\":[{}],\"live\":[{}]}}",
        one.join(","),
        two.join(",")
    );
    std::fs::write(&file, text)
        .map_err(|e| MrlyError::Value(format!("cannot write {file:?}: {e}")))?;
    Ok(file)
}

fn field(text: &str, name: &str) -> Vec<i32> {
    let head = format!("\"{name}\":[");
    let Some(start) = text.find(&head) else {
        return Vec::new();
    };
    let body = &text[start + head.len()..];
    let end = body.find(']').unwrap_or(0);
    body[..end]
        .split(',')
        .filter_map(|token| token.parse().ok())
        .collect()
}

fn read_data() -> Result<(Vec<i32>, Vec<i32>)> {
    let file = path();
    let raw = std::fs::read_to_string(&file)
        .map_err(|e| MrlyError::Value(format!("cannot read {file:?}: {e}; run -- compute")))?;
    let text: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    Ok((field(&text, "first"), field(&text, "live")))
}

// PRESS

fn compute() -> Result<()> {
    let mask = design_mask(2, 7, 3, 1)?;
    assert_eq!(mask.shape, vec![3, 3]);
    assert_eq!((0..mask.size()).filter(|&i| mask.at(i) == 1).count(), 8);

    let mut types = Tensor::new(vec![SIDE, SIDE]);
    let corner = SIDE / 2 - 1;
    for (x, y) in SEED {
        types.set(&[corner + y, corner + x], 1);
    }
    assert_eq!(types.bytes().iter().filter(|&&on| on == 1).count(), 5);

    let mut cell = Cell2d::new(types);
    let mut first = vec![-1i32; SIDE * SIDE];
    for step in 0..=STEPS {
        let live = cell.types();
        for (slot, mark) in first.iter_mut().enumerate() {
            if *mark < 0 && live.at(slot) != 0 {
                *mark = step as i32;
            }
        }
        if step < STEPS {
            cell = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Constant)?;
        }
    }
    let ever = first.iter().filter(|mark| **mark >= 0).count();
    let standing: Vec<u8> = (0..SIDE * SIDE)
        .map(|slot| u8::from(cell.types().at(slot) != 0))
        .collect();
    let alive = standing.iter().filter(|&&bit| bit != 0).count();
    assert!(alive > 0);
    assert!(ever > alive);

    let file = write_data(&first, &standing)?;
    println!("{NAME} {ever} ever {alive} live after {STEPS} -> {file:?}");
    Ok(())
}

fn draw() -> Result<()> {
    let (first, live) = read_data()?;
    assert_eq!(first.len(), SIDE * SIDE);
    assert_eq!(live.len(), SIDE * SIDE);
    assert!(first.iter().any(|&mark| mark >= 0));
    assert!(live.iter().any(|&bit| bit != 0));

    let mut board = Board::square();
    let area = board.frame(0.08);
    let scale = (area.w / SIDE as f64).floor().max(1.0);
    let block = SIDE as f64 * scale;
    let ox = ((board.width as f64 - block) / 2.0).round();
    let oy = ((board.height as f64 - block) / 2.0).round();
    let ramp = Ramp::tone(ink::green(), ink::line());
    for row in 0..SIDE {
        for col in 0..SIDE {
            let slot = row * SIDE + col;
            let x = ox + col as f64 * scale;
            let y = oy + row as f64 * scale;
            if first[slot] >= 0 {
                let t = first[slot] as f64 / STEPS as f64;
                board.rect(x, y, scale, scale, ramp.at(t));
            }
            if live[slot] != 0 {
                board.rect(x, y, scale, scale, ink::yellow());
            }
        }
    }
    save(NAME, &board)?;
    Ok(())
}

fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() == Some("compute") {
        return compute();
    }
    draw()
}
