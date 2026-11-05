use crate::args::Arg;
use crate::blob::{Prim, peek_len, take_blob};
use crate::call::{Error, bad};
use crate::ser::WireError;
use crate::*;
use serde::de::{self, Deserialize, DeserializeOwned, Visitor};

fn u32_at(b: &[u8], p: usize) -> u32 {
    u32::from_le_bytes([b[p], b[p + 1], b[p + 2], b[p + 3]])
}

fn f64_at(b: &[u8], p: usize) -> f64 {
    let mut word = [0; 8];
    word.copy_from_slice(&b[p..p + 8]);
    f64::from_le_bytes(word)
}

pub struct WireDe<'de> {
    pub(crate) b: &'de [u8],
    pub(crate) p: usize,
}

impl<'de> WireDe<'de> {
    pub fn new(b: &'de [u8]) -> WireDe<'de> {
        WireDe { b, p: 0 }
    }
    fn tag(&mut self) -> u8 {
        let tag = self.b[self.p];
        self.p += 1;
        tag
    }
    fn u32(&mut self) -> u32 {
        let value = u32_at(self.b, self.p);
        self.p += 4;
        value
    }
    fn text(&mut self) -> Result<&'de str, WireError> {
        let n = self.u32() as usize;
        let bytes = &self.b[self.p..self.p + n];
        self.p += n;
        std::str::from_utf8(bytes).map_err(de::Error::custom)
    }
    pub(crate) fn skip(&mut self) {
        let tag = self.tag();
        match tag {
            I32 | F32 | BLOB_IN => self.p += 4,
            F64 | I53 => self.p += 8,
            STR | BIG | WIDE => {
                let n = self.u32() as usize;
                self.p += n;
            }
            ARR => {
                for _ in 0..self.u32() {
                    self.skip();
                }
            }
            MAP => {
                for _ in 0..self.u32() {
                    self.skip();
                    self.skip();
                }
            }
            HANDLE => self.p += 4,
            _ => {}
        }
    }
}

struct Items<'a, 'de> {
    de: &'a mut WireDe<'de>,
    left: u32,
}

impl<'de> de::SeqAccess<'de> for Items<'_, 'de> {
    type Error = WireError;
    fn next_element_seed<S: de::DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, WireError> {
        if self.left == 0 {
            return Ok(None);
        }
        self.left -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.left as usize)
    }
}

impl<'de> de::MapAccess<'de> for Items<'_, 'de> {
    type Error = WireError;
    fn next_key_seed<S: de::DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, WireError> {
        if self.left == 0 {
            return Ok(None);
        }
        self.left -= 1;
        if self.de.tag() != STR {
            return Err(de::Error::custom("a map key"));
        }
        let text = self.de.text()?;
        seed.deserialize(Key(text)).map(Some)
    }
    fn next_value_seed<S: de::DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<S::Value, WireError> {
        seed.deserialize(&mut *self.de)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.left as usize)
    }
}

struct Key<'de>(&'de str);

macro_rules! key_ints {
    ($($method:ident $visit:ident $t:ty),* $(,)?) => {
        $(
            fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, WireError> {
                match self.0.parse::<$t>() {
                    Ok(value) => visitor.$visit(value),
                    Err(_) => visitor.visit_borrowed_str(self.0),
                }
            }
        )*
    };
}

impl<'de> de::Deserializer<'de> for Key<'de> {
    type Error = WireError;
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, WireError> {
        visitor.visit_borrowed_str(self.0)
    }
    key_ints! {
        deserialize_u8 visit_u8 u8, deserialize_u16 visit_u16 u16, deserialize_u32 visit_u32 u32,
        deserialize_u64 visit_u64 u64, deserialize_i8 visit_i8 i8, deserialize_i16 visit_i16 i16,
        deserialize_i32 visit_i32 i32, deserialize_i64 visit_i64 i64,
    }
    serde::forward_to_deserialize_any! {
        bool i128 u128 f32 f64 char str string bytes byte_buf option unit
        unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}

struct Variant<'a, 'de> {
    de: &'a mut WireDe<'de>,
    key: &'de str,
    has: bool,
}

impl<'a, 'de> de::EnumAccess<'de> for Variant<'a, 'de> {
    type Error = WireError;
    type Variant = Self;
    fn variant_seed<S: de::DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, Self), WireError> {
        let picked = seed.deserialize(Key(self.key))?;
        Ok((picked, self))
    }
}

impl<'de> de::VariantAccess<'de> for Variant<'_, 'de> {
    type Error = WireError;
    fn unit_variant(self) -> Result<(), WireError> {
        Ok(())
    }
    fn newtype_variant_seed<S: de::DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<S::Value, WireError> {
        if !self.has {
            return Err(de::Error::custom("a variant value"));
        }
        seed.deserialize(&mut *self.de)
    }
    fn tuple_variant<V: Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, WireError> {
        if !self.has {
            return Err(de::Error::custom("a variant list"));
        }
        de::Deserializer::deserialize_any(self.de, visitor)
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, WireError> {
        if !self.has {
            return Err(de::Error::custom("a variant object"));
        }
        de::Deserializer::deserialize_any(self.de, visitor)
    }
}

struct OneBlob(u32, bool);

impl<'de> de::MapAccess<'de> for OneBlob {
    type Error = WireError;
    fn next_key_seed<S: de::DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<Option<S::Value>, WireError> {
        if self.1 {
            return Ok(None);
        }
        self.1 = true;
        seed.deserialize(Key("$b")).map(Some)
    }
    fn next_value_seed<S: de::DeserializeSeed<'de>>(
        &mut self,
        seed: S,
    ) -> Result<S::Value, WireError> {
        seed.deserialize(de::value::U32Deserializer::new(self.0))
    }
}

impl<'de> de::Deserializer<'de> for &mut WireDe<'de> {
    type Error = WireError;
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, WireError> {
        let tag = self.tag();
        match tag {
            NUL => visitor.visit_unit(),
            FALSE => visitor.visit_bool(false),
            TRUE => visitor.visit_bool(true),
            I32 => {
                let value = self.u32() as i32;
                if value >= 0 {
                    visitor.visit_u64(value as u64)
                } else {
                    visitor.visit_i64(value as i64)
                }
            }
            F32 => {
                let value = f32::from_bits(self.u32());
                visitor.visit_f64(value as f64)
            }
            F64 => {
                let value = f64_at(self.b, self.p);
                self.p += 8;
                visitor.visit_f64(value)
            }
            I53 => {
                let value = f64_at(self.b, self.p) as i64;
                self.p += 8;
                if value >= 0 {
                    visitor.visit_u64(value as u64)
                } else {
                    visitor.visit_i64(value)
                }
            }
            STR => visitor.visit_borrowed_str(self.text()?),
            BIG | WIDE => {
                let text = self.text()?;
                if let Ok(value) = text.parse::<u64>() {
                    visitor.visit_u64(value)
                } else if let Ok(value) = text.parse::<i64>() {
                    visitor.visit_i64(value)
                } else if let Ok(value) = text.parse::<u128>() {
                    visitor.visit_u128(value)
                } else {
                    visitor.visit_i128(text.parse::<i128>().map_err(de::Error::custom)?)
                }
            }
            ARR => {
                let left = self.u32();
                visitor.visit_seq(Items { de: self, left })
            }
            MAP => {
                let left = self.u32();
                visitor.visit_map(Items { de: self, left })
            }
            BLOB_IN => {
                let id = self.u32();
                visitor.visit_map(OneBlob(id, false))
            }
            SMALL..=0xff => visitor.visit_u64((tag & 0x7f) as u64),
            HANDLE => Err(de::Error::custom(
                "an object cannot cross here; pass its plain data",
            )),
            _ => Err(de::Error::custom("an unknown wire tag")),
        }
    }
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, WireError> {
        if self.b[self.p] == NUL {
            self.p += 1;
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, WireError> {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, WireError> {
        match self.b[self.p] {
            STR => {
                self.p += 1;
                let key = self.text()?;
                visitor.visit_enum(Variant {
                    de: self,
                    key,
                    has: false,
                })
            }
            MAP => {
                self.p += 1;
                if self.u32() != 1 {
                    return Err(de::Error::custom("an enum object of one key"));
                }
                if self.tag() != STR {
                    return Err(de::Error::custom("an enum key"));
                }
                let key = self.text()?;
                visitor.visit_enum(Variant {
                    de: self,
                    key,
                    has: true,
                })
            }
            _ => Err(de::Error::custom("an enum")),
        }
    }
    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit
        unit_struct seq tuple tuple_struct map struct identifier ignored_any
    }
}

#[inline(never)]
fn take_bytes<T: DeserializeOwned>(name: &str, at: usize, b: &[u8]) -> Result<T, Error> {
    let mut de = WireDe::new(b);
    let first = T::deserialize(&mut de);
    if first.is_err() && b.first() == Some(&BLOB_IN) {
        let wire = spread(u32_at(b, 1) as usize);
        if let Some(wire) = wire {
            let mut de = WireDe::new(&wire);
            return T::deserialize(&mut de).map_err(|error| bad(name, at, &error.0));
        }
    }
    first.map_err(|error| bad(name, at, &error.0))
}

fn spread(id: usize) -> Option<Vec<u8>> {
    peek_len(id)?;
    let blob = take_blob(id)?;
    let mut out = Vec::new();
    macro_rules! each {
        ($v:expr, $to:expr) => {{
            out.push(ARR);
            out.extend_from_slice(&($v.len() as u32).to_le_bytes());
            for one in $v {
                let to: fn(_) -> i128 = $to;
                let value = to(one);
                crate::ser::write_big(&mut out, BIG, value);
            }
        }};
    }
    match blob {
        Blob::U8(v) => each!(v, |x: u8| x as i128),
        Blob::U16(v) => each!(v, |x: u16| x as i128),
        Blob::U32(v) => each!(v, |x: u32| x as i128),
        Blob::I8(v) => each!(v, |x: i8| x as i128),
        Blob::I16(v) => each!(v, |x: i16| x as i128),
        Blob::I32(v) => each!(v, |x: i32| x as i128),
        Blob::U64(v) => each!(v, |x: u64| x as i128),
        Blob::I64(v) => each!(v, |x: i64| x as i128),
        Blob::F32(v) => {
            out.push(ARR);
            out.extend_from_slice(&(v.len() as u32).to_le_bytes());
            for one in v {
                out.push(F32);
                out.extend_from_slice(&one.to_le_bytes());
            }
        }
        Blob::F64(v) => {
            out.push(ARR);
            out.extend_from_slice(&(v.len() as u32).to_le_bytes());
            for one in v {
                out.push(F64);
                out.extend_from_slice(&one.to_le_bytes());
            }
        }
    }
    Some(out)
}

#[inline]
pub fn take<T: DeserializeOwned>(name: &str, at: usize, value: &Arg) -> Result<T, Error> {
    take_bytes(name, at, value.bytes())
}

#[inline(never)]
pub fn take_list<T: Prim + DeserializeOwned>(
    name: &str,
    at: usize,
    value: &Arg,
) -> Result<Vec<T>, Error> {
    let b = value.bytes();
    if b.first() == Some(&BLOB_IN) {
        let id = u32_at(b, 1) as usize;
        return match take_blob(id) {
            Some(blob) => Ok(T::from_blob(blob)),
            None => Err(bad(name, at, "a byte buffer was read twice")),
        };
    }
    let mut de = WireDe::new(b);
    Vec::<T>::deserialize(&mut de).map_err(|error| bad(name, at, &error.0))
}
