use super::census;
use super::geometry;
use super::models::Cell2d;
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;

const HEADER: usize = 32;

#[rustfmt::skip]
const FRAME: [u8; 25] = [
    0, 1, 1, 1, 0,
    2, 3, 3, 3, 2,
    2, 3, 3, 3, 2,
    2, 3, 3, 3, 2,
    0, 1, 1, 1, 0,
];

/// The five by five mask a carried mosaic lays its four tiles out under.
pub fn frame() -> Tensor {
    Tensor::of(FRAME.to_vec(), vec![5, 5])
}

/// Returns the payload bytes the cell's filled sites can hold, its length header paid for.
///
/// ```
/// let cell = mrlymath::two::carpet(3, 3).unwrap();
/// assert_eq!(mrlymath::two::carry::capacity(&cell), 60);
/// ```
pub fn capacity(cell: &Cell2d) -> usize {
    census::fills(cell).saturating_sub(HEADER) / 8
}

/// Writes the payload over the cell's filled sites, repeating it until every site is spoken for.
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

/// Reads the payload back, the plain cell naming the sites the carried one wrote over.
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

/// Builds the framed sheet of four same-sized cells, the fourth carrying the payload.
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

/// Reads the payload back from a framed sheet, the plain fourth cell naming the sites.
pub fn read(sheet: &Cell2d, carrier: &Cell2d) -> Result<Vec<u8>> {
    let (w, h) = (carrier.width(), carrier.height());
    if sheet.width() != w * 5 || sheet.height() != h * 5 {
        return value_error("sheet must be five carriers across and down.");
    }
    extract(carrier, &block(sheet, 1, 1, w, h))
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

fn block(cell: &Cell2d, row: usize, col: usize, width: usize, height: usize) -> Cell2d {
    let mut types = Tensor::new(vec![height, width]);
    for y in 0..height {
        for x in 0..width {
            types.set(
                &[y, x],
                cell.types().get(&[row * height + y, col * width + x]),
            );
        }
    }
    Cell2d::new(types)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two::designs;

    fn carrier() -> Cell2d {
        designs::carpet(3, 3).unwrap()
    }

    #[test]
    fn payload_round_trips_through_the_filled_sites() {
        let plain = carrier();
        let payload = b"Hello, World!";
        let carried = embed(&plain, payload).unwrap();
        assert_eq!(carried.types().shape, plain.types().shape);
        assert_ne!(carried.types(), plain.types());
        assert_eq!(extract(&plain, &carried).unwrap(), payload);
    }

    #[test]
    fn every_payload_length_survives() {
        let plain = carrier();
        for length in [0usize, 1, 2, 7, 8, 59, 60] {
            let payload: Vec<u8> = (0..length).map(|i| (i * 37 % 251) as u8).collect();
            let carried = embed(&plain, &payload).unwrap();
            assert_eq!(
                extract(&plain, &carried).unwrap(),
                payload,
                "length {length}"
            );
        }
    }

    #[test]
    fn capacity_is_the_fills_less_the_header() {
        let plain = carrier();
        assert_eq!(census::fills(&plain), 512);
        assert_eq!(capacity(&plain), 60);
        let full = vec![7u8; capacity(&plain)];
        assert!(embed(&plain, &full).is_ok());
        let over = vec![7u8; capacity(&plain) + 1];
        assert!(embed(&plain, &over).is_err());
        assert_eq!(capacity(&designs::ones(2, 1).unwrap()), 0);
    }

    #[test]
    fn the_payload_repeats_across_the_spare_sites() {
        let plain = carrier();
        let carried = embed(&plain, b"ab").unwrap();
        let sites = sites(&plain);
        let bits = message(b"ab");
        for (k, &site) in sites.iter().enumerate() {
            assert_eq!(
                carried.types().at(site) as u8,
                bits[k % bits.len()],
                "site {site}"
            );
        }
    }

    #[test]
    fn the_carrier_names_the_sites() {
        let plain = carrier();
        let carried = embed(&plain, b"secret").unwrap();
        let wrong = designs::net(3, 3).unwrap();
        assert!(extract(&wrong, &carried).unwrap_or_default() != b"secret".to_vec());
        assert!(extract(&plain, &designs::carpet(3, 2).unwrap()).is_err());
        assert!(extract(&designs::ones(2, 1).unwrap(), &designs::ones(2, 1).unwrap()).is_err());
    }

    #[test]
    fn a_stray_length_is_refused() {
        let plain = carrier();
        let mut carried = embed(&plain, b"x").unwrap();
        for &site in sites(&plain).iter().take(HEADER) {
            carried.cell.types.put(site, 1);
        }
        assert!(extract(&plain, &carried).is_err());
    }

    #[test]
    fn the_sheet_frames_the_carrier() {
        let tiles = [
            designs::carpet(3, 3).unwrap(),
            designs::vtree(3, 3).unwrap().rotate(1),
            designs::vtree(3, 3).unwrap(),
            designs::carpet(3, 3).unwrap(),
        ];
        let framed = sheet(&tiles, b"Hello, World!").unwrap();
        assert_eq!((framed.width(), framed.height()), (135, 135));
        assert_eq!(read(&framed, &tiles[3]).unwrap(), b"Hello, World!");
        let corner = block(&framed, 0, 0, 27, 27);
        assert_eq!(corner.types(), tiles[0].types());
        assert!(read(&tiles[3].clone(), &tiles[3]).is_err());
    }

    #[test]
    fn the_frame_names_four_tiles() {
        let mask = frame();
        assert_eq!(mask.shape, vec![5, 5]);
        assert_eq!(mask.get(&[0, 0]), 0);
        assert_eq!(mask.get(&[0, 1]), 1);
        assert_eq!(mask.get(&[1, 0]), 2);
        assert_eq!(mask.get(&[2, 2]), 3);
        let mut seen = [0usize; 4];
        for &value in mask.bytes() {
            seen[value as usize] += 1;
        }
        assert_eq!(seen, [4, 6, 6, 9]);
    }
}
