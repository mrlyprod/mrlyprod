use mrlycore::errors::{value_error, Result};
use mrlycore::{json, Json};
use std::sync::OnceLock;

pub(crate) const SIDE: usize = 30;

const ONE: &[u8] = include_bytes!("../corpus/one.bin");

const TWO: &[u8] = include_bytes!("../corpus/two.bin");

static PACKS: [OnceLock<Result<Corpus>>; 2] = [OnceLock::new(), OnceLock::new()];

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Grid {
    pub w: usize,
    pub h: usize,
    pub cells: Vec<u8>,
}

impl Grid {
    pub fn blank(w: usize, h: usize) -> Grid {
        Grid {
            w,
            h,
            cells: vec![0; w * h],
        }
    }
    pub fn to_json(&self) -> Json {
        Json::Arr(
            self.cells
                .chunks(self.w)
                .map(|row| json!(row.to_vec()))
                .collect(),
        )
    }
    pub fn from_json(value: &Json) -> Option<Grid> {
        let rows = value.as_array()?;
        let h = rows.len();
        let w = rows.first()?.as_array()?.len();
        if h == 0 || h > SIDE || w == 0 || w > SIDE {
            return None;
        }
        let mut cells = Vec::with_capacity(w * h);
        for row in rows {
            let row = row.as_array()?;
            if row.len() != w {
                return None;
            }
            for cell in row {
                match cell.as_u64() {
                    Some(v) if v <= 9 => cells.push(v as u8),
                    _ => return None,
                }
            }
        }
        Some(Grid { w, h, cells })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Pair {
    pub input: Grid,
    pub output: Grid,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TaskData {
    pub id: String,
    pub pairs: Vec<Pair>,
    pub tests: Vec<Pair>,
}

pub(crate) struct Corpus {
    pub train: Vec<TaskData>,
    pub eval: Vec<TaskData>,
}

pub(crate) fn corpus(set: &str) -> Option<&'static Corpus> {
    let (lock, bytes) = match set {
        "one" => (&PACKS[0], ONE),
        "two" => (&PACKS[1], TWO),
        _ => return None,
    };
    lock.get_or_init(|| unpack(bytes)).as_ref().ok()
}

fn unpack(bytes: &[u8]) -> Result<Corpus> {
    let raw = mrlycore::inflate(bytes)?;
    if raw.len() < 9 || raw[..4] != *b"ARCP" || raw[4] != 1 {
        return value_error("not an arc pack.");
    }
    let train = u16::from_le_bytes([raw[5], raw[6]]) as usize;
    let eval = u16::from_le_bytes([raw[7], raw[8]]) as usize;
    let mut at = 9;
    let mut out = Corpus {
        train: Vec::with_capacity(train),
        eval: Vec::with_capacity(eval),
    };
    for _ in 0..train {
        out.train.push(task(&raw, &mut at)?);
    }
    for _ in 0..eval {
        out.eval.push(task(&raw, &mut at)?);
    }
    if at != raw.len() {
        return value_error("trailing pack bytes.");
    }
    Ok(out)
}

fn take<'a>(raw: &'a [u8], at: &mut usize, n: usize) -> Result<&'a [u8]> {
    let end = at.checked_add(n).filter(|&end| end <= raw.len());
    match end {
        Some(end) => {
            let slice = &raw[*at..end];
            *at = end;
            Ok(slice)
        }
        None => value_error("truncated pack."),
    }
}

fn task(raw: &[u8], at: &mut usize) -> Result<TaskData> {
    let id = take(raw, at, 8)?;
    if !id.iter().all(u8::is_ascii_hexdigit) {
        return value_error("bad task id.");
    }
    let id = String::from_utf8_lossy(id).into_owned();
    let counts = take(raw, at, 2)?;
    let (pairs, tests) = (counts[0] as usize, counts[1] as usize);
    if pairs == 0 || tests == 0 {
        return value_error("empty task.");
    }
    let mut out = TaskData {
        id,
        pairs: Vec::with_capacity(pairs),
        tests: Vec::with_capacity(tests),
    };
    for _ in 0..pairs {
        out.pairs.push(pair(raw, at)?);
    }
    for _ in 0..tests {
        out.tests.push(pair(raw, at)?);
    }
    Ok(out)
}

fn pair(raw: &[u8], at: &mut usize) -> Result<Pair> {
    Ok(Pair {
        input: grid(raw, at)?,
        output: grid(raw, at)?,
    })
}

fn grid(raw: &[u8], at: &mut usize) -> Result<Grid> {
    let dims = take(raw, at, 2)?;
    let (w, h) = (dims[0] as usize, dims[1] as usize);
    if w == 0 || w > SIDE || h == 0 || h > SIDE {
        return value_error("bad grid size.");
    }
    let count = w * h;
    let packed = take(raw, at, count.div_ceil(2))?;
    let mut cells = Vec::with_capacity(count);
    for (i, byte) in packed.iter().enumerate() {
        cells.push(byte >> 4);
        if i * 2 + 1 < count {
            cells.push(byte & 0x0f);
        }
    }
    if cells.iter().any(|&cell| cell > 9) {
        return value_error("bad cell value.");
    }
    Ok(Grid { w, h, cells })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_corrupt_pack_is_an_error_not_a_panic() {
        assert!(unpack(b"not a zlib stream").is_err());
        assert!(unpack(&mrlycore::deflate(b"WRONG MAGIC HERE")).is_err());
        let starving = [b'A', b'R', b'C', b'P', 1, 1, 0, 0, 0];
        assert!(unpack(&mrlycore::deflate(&starving)).is_err());
        let mut trailing = b"ARCP\x01\x00\x00\x00\x00".to_vec();
        trailing.push(9);
        assert!(unpack(&mrlycore::deflate(&trailing)).is_err());
    }
}
