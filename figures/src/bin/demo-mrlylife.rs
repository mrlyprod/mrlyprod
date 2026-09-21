use mrlyfig::out::root;
use mrlyfig::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::core::tensor::Tensor;
use mrlyrs::core::MrlyError;
use mrlyrs::core::Rng;
use mrlyrs::life::{design_mask, lattice_index, next_grid, Boundary};
use mrlyrs::math::two::Cell2d;
use std::path::PathBuf;

const NAME: &str = "demo-mrlylife";
const SIDE: usize = 192;
const GENERATIONS: usize = 64;
const SEED: u64 = 1729;
const DENSITY: f64 = 0.05;
const CELL: f64 = 4.0;
const STAMP: f64 = 8.0;
const MASK: usize = 9;
const SITES: usize = 64;

// LIFE

fn soup(seed: u64) -> Cell2d {
    let mut rng = Rng::new(seed);
    let mut types = Tensor::new(vec![SIDE, SIDE]);
    for slot in types.bytes_mut().iter_mut() {
        *slot = u8::from(rng.chance(DENSITY));
    }
    Cell2d::new(types)
}

// DATA

fn path() -> PathBuf {
    root()
        .join("files")
        .join("figures")
        .join("data")
        .join(format!("{NAME}.json"))
}

fn write_data(mask: &[u8], cells: &[u8]) -> Result<PathBuf> {
    let file = path();
    let folder = file.parent().unwrap().to_path_buf();
    std::fs::create_dir_all(&folder)
        .map_err(|e| MrlyError::Value(format!("cannot make {folder:?}: {e}")))?;
    let one: Vec<String> = mask.iter().map(|bit| bit.to_string()).collect();
    let two: Vec<String> = cells.iter().map(|bit| bit.to_string()).collect();
    let text = format!(
        "{{\"mask\":[{}],\"cells\":[{}]}}",
        one.join(","),
        two.join(",")
    );
    std::fs::write(&file, text)
        .map_err(|e| MrlyError::Value(format!("cannot write {file:?}: {e}")))?;
    Ok(file)
}

fn field(text: &str, name: &str) -> Vec<u8> {
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

fn read_data() -> Result<(Vec<u8>, Vec<u8>)> {
    let file = path();
    let raw = std::fs::read_to_string(&file)
        .map_err(|e| MrlyError::Value(format!("cannot read {file:?}: {e}; run -- compute")))?;
    let text: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    Ok((field(&text, "mask"), field(&text, "cells")))
}

// PRESS

fn compute() -> Result<()> {
    let mask = design_mask(2, 7, 3, 2)?;
    assert_eq!(mask.shape, vec![MASK, MASK]);
    assert_eq!((0..mask.size()).filter(|&i| mask.at(i) == 1).count(), SITES);
    assert_eq!(mask.get(&[MASK / 2, MASK / 2]), 0);
    assert_eq!(lattice_index(&mask), 1);

    let mut cell = soup(SEED);
    for _ in 0..GENERATIONS {
        cell = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Wrap)?;
    }
    assert_eq!(cell.width(), SIDE);
    assert_eq!(cell.height(), SIDE);
    let types = cell.types();
    let live = (0..SIDE * SIDE).filter(|&i| types.at(i) != 0).count();
    assert!(live > 0);

    let stamp: Vec<u8> = (0..MASK * MASK).map(|i| mask.at(i) as u8).collect();
    let board: Vec<u8> = (0..SIDE * SIDE)
        .map(|i| u8::from(types.at(i) != 0))
        .collect();
    let file = write_data(&stamp, &board)?;
    println!("{NAME} {SITES} sites {live} live after {GENERATIONS} -> {file:?}");
    Ok(())
}

fn draw() -> Result<()> {
    let (mask, cells) = read_data()?;
    assert_eq!(mask.len(), MASK * MASK);
    assert_eq!(cells.len(), SIDE * SIDE);
    assert!(cells.iter().any(|&bit| bit != 0));

    let mut board = Board::square();
    let area = board.frame(0.08);
    let block = SIDE as f64 * CELL;
    let ox = (area.x + area.w - block).round();
    let oy = (area.y + area.h - block).round();
    for row in 0..SIDE {
        for col in 0..SIDE {
            if cells[row * SIDE + col] != 0 {
                let x = ox + col as f64 * CELL;
                let y = oy + row as f64 * CELL;
                board.rect(x, y, CELL, CELL, ink::blue());
            }
        }
    }

    let sx = area.x.round();
    let sy = area.y.round();
    let mut stamped = 0usize;
    for row in 0..MASK {
        for col in 0..MASK {
            if mask[row * MASK + col] == 1 {
                let x = sx + col as f64 * STAMP;
                let y = sy + row as f64 * STAMP;
                board.rect(x, y, STAMP, STAMP, ink::yellow());
                stamped += 1;
            }
        }
    }
    assert_eq!(stamped, SITES);
    save(NAME, &board)?;
    Ok(())
}

fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() == Some("compute") {
        return compute();
    }
    draw()
}
