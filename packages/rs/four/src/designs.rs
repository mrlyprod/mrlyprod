use crate::models::{Cell4d, Result};
use crate::tag::{self, Move};

pub fn zeros(width: usize, height: usize) -> Cell4d {
    Cell4d::new(width, height)
}

pub fn ones(width: usize, height: usize) -> Cell4d {
    let mut out = Cell4d::new(width, height);
    out.types.fill(1);
    out
}

fn fitted(width: usize, height: usize, count: usize) -> Result<()> {
    if width == 0 || height == 0 {
        return Err("a cell wants a width and a height above zero.".to_string());
    }
    if count != width * height {
        return Err(format!(
            "{width}x{height} wants {} values, got {count}.",
            width * height
        ));
    }
    Ok(())
}

pub fn from_squares(width: usize, height: usize, squares: &[u8]) -> Result<Cell4d> {
    fitted(width, height, squares.len())?;
    let mut out = Cell4d::new(width, height);
    for (flat, &value) in squares.iter().enumerate() {
        out.put(flat % width, flat / width, [value; 4]);
    }
    Ok(out)
}

pub fn from_tags(width: usize, height: usize, tags: &[u8]) -> Result<Cell4d> {
    fitted(width, height, tags.len())?;
    let mut out = Cell4d::new(width, height);
    for (flat, &value) in tags.iter().enumerate() {
        out.put(flat % width, flat / width, tag::sides(value));
    }
    Ok(out)
}

pub fn from_turns(width: usize, height: usize, motif: [u8; 4], turns: &[u8]) -> Result<Cell4d> {
    fitted(width, height, turns.len())?;
    let mut out = Cell4d::new(width, height);
    for (flat, &turn) in turns.iter().enumerate() {
        out.put(
            flat % width,
            flat / width,
            Move::Turn(usize::from(turn)).sides(motif),
        );
    }
    Ok(out)
}

pub fn random(
    width: usize,
    height: usize,
    tags: &[u8],
    mut draw: impl FnMut(usize) -> usize,
) -> Result<Cell4d> {
    if tags.is_empty() {
        return Err("a random cell wants at least one tag.".to_string());
    }
    let picks: Vec<u8> = (0..width * height)
        .map(|_| tags[draw(tags.len()) % tags.len()])
        .collect();
    from_tags(width, height, &picks)
}
