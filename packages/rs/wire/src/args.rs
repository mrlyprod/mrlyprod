use crate::call::{Error, bad, usage};
use crate::de::WireDe;
use crate::*;
use std::ops::Index;

#[repr(transparent)]
pub struct Arg([u8]);

impl Arg {
    fn from(part: &[u8]) -> &Arg {
        unsafe { &*(part as *const [u8] as *const Arg) }
    }
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
    pub fn is_null(&self) -> bool {
        matches!(self.0.first(), None | Some(&NUL))
    }
    pub fn number(&self) -> Option<f64> {
        let b = &self.0;
        match *b.first()? {
            I32 => Some(i32::from_le_bytes([b[1], b[2], b[3], b[4]]) as f64),
            F32 => Some(f32::from_le_bytes([b[1], b[2], b[3], b[4]]) as f64),
            F64 | I53 => {
                let mut word = [0; 8];
                word.copy_from_slice(&b[1..9]);
                Some(f64::from_le_bytes(word))
            }
            tag @ SMALL..=0xff => Some((tag & 0x7f) as f64),
            _ => None,
        }
    }
    pub fn text(&self) -> Option<&str> {
        let b = &self.0;
        match *b.first()? {
            STR | BIG | WIDE => {
                let n = u32::from_le_bytes([b[1], b[2], b[3], b[4]]) as usize;
                std::str::from_utf8(&b[5..5 + n]).ok()
            }
            _ => None,
        }
    }
    pub fn items(&self) -> Option<Vec<&Arg>> {
        let b = &self.0;
        if b.first() != Some(&ARR) {
            return None;
        }
        let n = u32::from_le_bytes([b[1], b[2], b[3], b[4]]) as usize;
        let mut walk = WireDe { b, p: 5 };
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            let from = walk.p;
            walk.skip();
            out.push(Arg::from(&b[from..walk.p]));
        }
        Some(out)
    }
    pub fn fields(&self) -> Option<Vec<(&str, &Arg)>> {
        let b = &self.0;
        if b.first() != Some(&MAP) {
            return None;
        }
        let n = u32::from_le_bytes([b[1], b[2], b[3], b[4]]) as usize;
        let mut walk = WireDe { b, p: 5 };
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            let from = walk.p;
            walk.skip();
            let key = Arg::from(&b[from..walk.p]).text()?;
            let from = walk.p;
            walk.skip();
            out.push((key, Arg::from(&b[from..walk.p])));
        }
        Some(out)
    }
    pub fn field(&self, name: &str) -> Option<&Arg> {
        self.fields()?
            .into_iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value)
    }
    pub fn big(&self, name: &str, at: usize) -> Result<u128, Error> {
        let read = match self.text() {
            Some(text) => text.parse().ok(),
            None => self
                .number()
                .filter(|n| n.fract() == 0.0 && *n >= 0.0)
                .map(|n| n as u128),
        };
        read.ok_or_else(|| bad(name, at, "a u128 is a decimal string"))
    }
    pub fn small(&self, name: &str, at: usize) -> Result<i128, Error> {
        let read = match self.text() {
            Some(text) => text.parse().ok(),
            None => self
                .number()
                .filter(|n| n.fract() == 0.0)
                .map(|n| n as i128),
        };
        read.ok_or_else(|| bad(name, at, "an i128 is a decimal string"))
    }
    pub fn seed(&self, name: &str, at: usize) -> Result<u64, Error> {
        let read = match self.text() {
            Some(text) => text.parse().ok(),
            None => self
                .number()
                .filter(|n| n.fract() == 0.0 && *n >= 0.0)
                .map(|n| n as u64),
        };
        read.ok_or_else(|| bad(name, at, "a seed is a whole number"))
    }
    pub fn rgba(&self, name: &str, at: usize) -> Result<[u8; 4], Error> {
        let bytes: Vec<u8> = crate::de::take(name, at, self)
            .map_err(|_| bad(name, at, "a color is three or four bytes"))?;
        match bytes[..] {
            [r, g, b, a] => Ok([r, g, b, a]),
            [r, g, b] => Ok([r, g, b, 255]),
            _ => Err(bad(name, at, "a color is three or four bytes")),
        }
    }
    pub fn items_of(&self, name: &str, at: usize) -> Result<Vec<&Arg>, Error> {
        self.items().ok_or_else(|| bad(name, at, "a list"))
    }
    pub fn fixed(&self, name: &str, at: usize, want: usize) -> Result<Vec<&Arg>, Error> {
        match self.items() {
            Some(list) if list.len() == want => Ok(list),
            _ => Err(bad(name, at, &format!("a list of {want}"))),
        }
    }
    pub fn fields_of(&self, name: &str, at: usize) -> Result<Vec<(&str, &Arg)>, Error> {
        self.fields().ok_or_else(|| bad(name, at, "an object"))
    }
    fn rank_here(&self) -> Option<usize> {
        if let Some(shape) = self.field("shape") {
            return Some(shape.items()?.len());
        }
        if let Some(types) = self.field("types") {
            return types.rank_here();
        }
        self.field("cell")?.rank_here()
    }
}

pub struct Args<'a> {
    parts: &'a [&'a [u8]],
}

impl<'a> Args<'a> {
    pub fn new(parts: &'a [&'a [u8]]) -> Args<'a> {
        Args { parts }
    }
    pub fn len(&self) -> usize {
        self.parts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }
    #[inline(never)]
    pub fn count(&self, name: &str, want: usize) -> Result<(), Error> {
        let have = self.parts.len();
        if have == want
            || (have > want
                && self.parts[want..]
                    .iter()
                    .all(|part| part.first() == Some(&NUL)))
        {
            return Ok(());
        }
        let noun = if want == 1 { "argument" } else { "arguments" };
        Err(usage(format!("{name} takes {want} {noun}, got {have}")))
    }
    pub fn rank(&self, name: &str, at: usize) -> Result<usize, Error> {
        self.parts
            .get(at)
            .and_then(|part| Arg::from(part).rank_here())
            .ok_or_else(|| bad(name, at, "an object with a shape"))
    }
    pub fn ranks(&self, name: &str, at: usize) -> Result<usize, Error> {
        self.parts
            .get(at)
            .and_then(|part| Arg::from(part).items())
            .and_then(|list| list.first().and_then(|one| one.rank_here()))
            .ok_or_else(|| bad(name, at, "a list of objects with a shape"))
    }
}

impl Index<usize> for Args<'_> {
    type Output = Arg;
    #[inline]
    fn index(&self, at: usize) -> &Arg {
        match self.parts.get(at) {
            Some(part) => Arg::from(part),
            None => Arg::from(&[NUL]),
        }
    }
}

pub fn split<'a>(bytes: &'a [u8], parts: &mut [&'a [u8]; 24]) -> usize {
    let count = bytes[0] as usize;
    let mut p = 1;
    for part in parts.iter_mut().take(count) {
        let len = u32::from_le_bytes([bytes[p], bytes[p + 1], bytes[p + 2], bytes[p + 3]]) as usize;
        p += 4;
        *part = &bytes[p..p + len];
        p += len;
    }
    count
}
