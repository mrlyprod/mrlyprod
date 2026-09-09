use crate::Fault;
use mrlycore::json;
use mrlynum::spiral::{self, Growth, Snail};
use wasm_bindgen::prelude::*;

const TOP: u32 = 2000;
const BASE: u32 = 8;

fn read(base: u32, top: u32, growth: &str) -> Result<Snail, Fault> {
    let growth =
        Growth::named(growth).ok_or_else(|| Fault::new("the growth is prime or every."))?;
    if !(2..=BASE).contains(&base) {
        return Err(Fault::new(format!("the base is 2 to {BASE}.")));
    }
    if top == 0 || top > TOP {
        return Err(Fault::new(format!("the top is 1 to {TOP}.")));
    }
    Ok(spiral::snail(u64::from(base), u64::from(top), growth))
}

/// Packs the snail of the whole numbers to the top: five numbers a tile, the x and y of its lower-left corner, its side, its level and one when the number is prime, in the order one, two, three and on.
#[wasm_bindgen]
pub fn snail_cells(base: u32, top: u32, growth: &str) -> Result<Vec<i32>, Fault> {
    let shell = read(base, top, growth)?;
    let mut out = Vec::with_capacity(5 * shell.tiles.len());
    for tile in &shell.tiles {
        out.push(tile.x as i32);
        out.push(tile.y as i32);
        out.push(tile.side as i32);
        out.push(tile.level as i32);
        out.push(i32::from(tile.prime));
    }
    Ok(out)
}

/// Reads the snail: the count of tiles, the primes at or below the top, the tiles grown past a unit cell, the tally at each level, the largest side, the drawn area and the box the tiles fill, as JSON.
#[wasm_bindgen]
pub fn snail_read(base: u32, top: u32, growth: &str) -> Result<String, Fault> {
    let shell = read(base, top, growth)?;
    let grown = shell.tiles.iter().filter(|tile| tile.level > 0).count();
    let side = shell.tiles.iter().map(|tile| tile.side).max().unwrap_or(1);
    Ok(json!({
        "tiles": shell.tiles.len(),
        "primes": shell.primes,
        "grown": grown,
        "levels": shell.levels,
        "peak": shell.levels.len() - 1,
        "side": side,
        "area": shell.area as u64,
        "low": [shell.low.0, shell.low.1],
        "high": [shell.high.0, shell.high.1],
        "width": shell.high.0 - shell.low.0,
        "height": shell.high.1 - shell.low.1,
    })
    .to_string())
}
