use super::Cell2d;
use super::census;
use super::geometry;
use mrly_core::error::{Result, value_error};
use mrly_core::tensor::Tensor;

const HEADER: usize = 32;

#[rustfmt::skip]
const FRAME: [u8; 25] = [
    0, 1, 1, 1, 0,
    2, 3, 3, 3, 2,
    2, 3, 3, 3, 2,
    2, 3, 3, 3, 2,
    0, 1, 1, 1, 0,
];

pub fn frame() -> Tensor {
    let mut frame = Tensor::new(vec![5, 5]);
    for (flat, &value) in FRAME.iter().enumerate().take(frame.size()) {
        frame.put(flat, i64::from(value));
    }
    frame
}

pub fn capacity(cell: &Cell2d) -> usize {
    census::fills(cell).saturating_sub(HEADER) / 8
}

pub fn embed(cell: &Cell2d, payload: &[u8]) -> Result<Cell2d> {
    let sites = sites(cell);
    let bits = message(payload);
    if bits.len() > sites.len() {
        return value_error(format!(
            "payload needs {} sites, the cell carries {}.",
            bits.len(),
            sites.len()
        ));
    }
    let mut out = cell.clone();
    for (k, &site) in sites.iter().enumerate() {
        out.cell.types.put(site, i64::from(bits[k % bits.len()]));
    }
    Ok(out)
}

pub fn extract(carrier: &Cell2d, carried: &Cell2d) -> Result<Vec<u8>> {
    if carrier.types().shape != carried.types().shape {
        return value_error("carrier and carried must share one shape.");
    }
    let sites = sites(carrier);
    if sites.len() < HEADER {
        return value_error("carrier holds too few sites for a header.");
    }
    let bits: Vec<u8> = sites
        .iter()
        .map(|&site| u8::from(carried.types().at(site) != 0))
        .collect();
    let length = bits[..HEADER]
        .iter()
        .fold(0usize, |acc, &bit| acc << 1 | bit as usize);
    if HEADER + length * 8 > bits.len() {
        return value_error("carried length runs past the carrier.");
    }
    Ok(bits[HEADER..HEADER + length * 8]
        .chunks(8)
        .map(|byte| byte.iter().fold(0u8, |acc, &bit| acc << 1 | bit))
        .collect())
}

pub fn sheet(cells: &[Cell2d; 4], payload: &[u8]) -> Result<Cell2d> {
    let carried = embed(&cells[3], payload)?;
    let laid = [
        cells[0].clone(),
        cells[1].clone(),
        cells[2].clone(),
        carried,
    ];
    geometry::mosaic(&frame(), &laid)
}

pub fn read(sheet: &Cell2d, carrier: &Cell2d) -> Result<Vec<u8>> {
    let (w, h) = (carrier.width(), carrier.height());
    if sheet.width() != w * 5 || sheet.height() != h * 5 {
        return value_error("sheet must be five carriers across and down.");
    }
    extract(carrier, &block(sheet, 1, 1, w, h)?)
}

fn sites(cell: &Cell2d) -> Vec<usize> {
    let types = cell.types();
    (0..types.size()).filter(|&i| types.at(i) == 1).collect()
}

fn message(payload: &[u8]) -> Vec<u8> {
    let mut bits = spread(&(payload.len() as u32).to_be_bytes());
    bits.extend(spread(payload));
    bits
}

fn spread(bytes: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .flat_map(|&byte| (0..8).rev().map(move |k| byte >> k & 1))
        .collect()
}

fn block(cell: &Cell2d, row: usize, col: usize, width: usize, height: usize) -> Result<Cell2d> {
    let source = cell.types();
    let mut types = Tensor::new(vec![height, width]);
    for y in 0..height {
        for x in 0..width {
            let from = source.index(&[row * height + y, col * width + x]);
            types.put(types.index(&[y, x]), source.at(from));
        }
    }
    Cell2d::new(types)
}
