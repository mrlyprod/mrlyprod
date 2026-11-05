use crate::blob::TAKEN;
use crate::*;
use serde::Serialize;
use serde::ser::{self, Serializer};
use std::cell::{Cell, RefCell};
use std::fmt::Display;
use std::sync::OnceLock;

const SLOTS: usize = 4096;
const SYMBOLS: u32 = u32::MAX;
const UNSET: u16 = u16::MAX;

struct Node {
    parent: u32,
    sym: u16,
    id: u16,
}

pub struct Keys {
    fast: Vec<(u32, usize, usize, u32)>,
    syms: Vec<&'static str>,
    told: Vec<bool>,
    nodes: Vec<Node>,
    edges: Vec<(u32, u16, u32)>,
    pending: Vec<u8>,
    next: u16,
}

static KNOWN: OnceLock<&'static [&'static [&'static str]]> = OnceLock::new();

thread_local! {
    pub(crate) static KEYS: RefCell<Keys> = const { RefCell::new(Keys::empty()) };
    static READY: Cell<bool> = const { Cell::new(false) };
}

pub fn shapes_of() -> &'static [&'static [&'static str]] {
    KNOWN.get().copied().unwrap_or(&[])
}

pub(crate) fn set_known(known: &'static [&'static [&'static str]]) {
    let _ = KNOWN.set(known);
}

#[inline]
pub(crate) fn ready_keys() {
    if !READY.replace(true) {
        KEYS.with_borrow_mut(Keys::restart);
    }
}

impl Keys {
    pub(crate) const fn empty() -> Keys {
        Keys {
            fast: Vec::new(),
            syms: Vec::new(),
            told: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            pending: Vec::new(),
            next: 0,
        }
    }
    pub(crate) fn restart(&mut self) {
        self.next = shapes_of().len() as u16;
    }
    fn sym_of(&mut self, key: &'static str) -> u16 {
        if let Some(at) = self.syms.iter().position(|one| *one == key) {
            return at as u16;
        }
        self.syms.push(key);
        self.told.push(false);
        self.syms.len() as u16 - 1
    }
    fn tell_sym(&mut self, id: u16) {
        if self.told[id as usize] {
            return;
        }
        self.told[id as usize] = true;
        let key = self.syms[id as usize];
        self.pending.push(1);
        self.pending.extend_from_slice(&id.to_le_bytes());
        self.pending
            .extend_from_slice(&(key.len() as u32).to_le_bytes());
        self.pending.extend_from_slice(key.as_bytes());
    }
    #[inline]
    fn slot(node: u32, at: usize, len: usize) -> usize {
        ((at >> 1) ^ (at >> 11) ^ len ^ (node as usize).wrapping_mul(0x9e37)) & (SLOTS - 1)
    }
    #[inline(never)]
    pub(crate) fn sym(&mut self, key: &'static str) -> u16 {
        let at = key.as_ptr() as usize;
        let len = key.len();
        let slot = Self::slot(SYMBOLS, at, len);
        if let Some(seen) = self.fast.get(slot) {
            if seen.0 == SYMBOLS && seen.1 == at && seen.2 == len {
                return seen.3 as u16;
            }
        } else {
            self.fast = vec![(0, 0, 0, 0); SLOTS];
        }
        let id = self.sym_of(key);
        self.tell_sym(id);
        self.fast[slot] = (SYMBOLS, at, len, id as u32);
        id
    }
    #[inline(never)]
    fn step(&mut self, node: u32, key: &'static str) -> u32 {
        let at = key.as_ptr() as usize;
        let len = key.len();
        let slot = Self::slot(node, at, len);
        if let Some(seen) = self.fast.get(slot) {
            if seen.0 == node && seen.1 == at && seen.2 == len {
                return seen.3;
            }
        } else {
            self.fast = vec![(0, 0, 0, 0); SLOTS];
        }
        let sym = self.sym_of(key);
        let child = match self
            .edges
            .iter()
            .find(|edge| edge.0 == node && edge.1 == sym)
        {
            Some(edge) => edge.2,
            None => {
                let child = self.nodes.len() as u32;
                self.nodes.push(Node {
                    parent: node,
                    sym,
                    id: UNSET,
                });
                self.edges.push((node, sym, child));
                child
            }
        };
        self.fast[slot] = (node, at, len, child);
        child
    }
    #[inline]
    fn root(&mut self) -> u32 {
        if self.nodes.is_empty() {
            self.nodes.push(Node {
                parent: 0,
                sym: 0,
                id: UNSET,
            });
        }
        0
    }
    #[inline]
    fn end(&mut self, node: u32) -> u16 {
        let id = self.nodes[node as usize].id;
        if id != UNSET {
            return id;
        }
        self.settle(node)
    }
    #[inline(never)]
    fn settle(&mut self, node: u32) -> u16 {
        let mut syms = Vec::new();
        let mut at = node;
        while at != 0 {
            let one = &self.nodes[at as usize];
            syms.push(one.sym);
            at = one.parent;
        }
        syms.reverse();
        let names: Vec<&'static str> = syms.iter().map(|sym| self.syms[*sym as usize]).collect();
        let known = shapes_of();
        let id = match known.binary_search_by(|one| (*one).cmp(names.as_slice())) {
            Ok(id) => id as u16,
            Err(_) => {
                let id = self.next;
                self.next += 1;
                for sym in &syms {
                    self.tell_sym(*sym);
                }
                self.pending.push(2);
                self.pending.extend_from_slice(&id.to_le_bytes());
                self.pending
                    .extend_from_slice(&(syms.len() as u16).to_le_bytes());
                for sym in &syms {
                    self.pending.extend_from_slice(&sym.to_le_bytes());
                }
                id
            }
        };
        self.nodes[node as usize].id = id;
        id
    }
    pub(crate) fn flush_if(&mut self, out: &mut Vec<u8>, ok: bool) {
        if ok && !self.pending.is_empty() {
            let at = out.len() as u32;
            out[0..4].copy_from_slice(&at.to_le_bytes());
            out.extend_from_slice(&self.pending);
            self.pending.clear();
        }
    }
}

#[derive(Debug)]
pub struct WireError(pub String);

impl Display for WireError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for WireError {}

impl serde::de::Error for WireError {
    fn custom<T: Display>(message: T) -> Self {
        WireError(message.to_string())
    }
}

impl ser::Error for WireError {
    fn custom<T: Display>(message: T) -> Self {
        WireError(message.to_string())
    }
}

pub struct Ser<'a> {
    pub(crate) out: &'a mut Vec<u8>,
    pub(crate) keys: &'a mut Keys,
}

pub struct Compound<'s, 'a> {
    ser: &'s mut Ser<'a>,
    at: usize,
    n: u32,
}

pub struct Shaped<'s, 'a> {
    ser: &'s mut Ser<'a>,
    at: usize,
    node: u32,
}

pub(crate) fn write_int(out: &mut Vec<u8>, value: i64) {
    if (0..128).contains(&value) {
        out.push(SMALL | value as u8);
    } else if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
        out.push(I32);
        out.extend_from_slice(&(value as i32).to_le_bytes());
    } else if value.unsigned_abs() <= SAFE {
        out.push(I53);
        out.extend_from_slice(&(value as f64).to_le_bytes());
    } else {
        write_big(out, BIG, value);
    }
}

pub(crate) fn write_big(out: &mut Vec<u8>, tag: u8, value: impl Display) {
    let text = value.to_string();
    out.push(tag);
    out.extend_from_slice(&(text.len() as u32).to_le_bytes());
    out.extend_from_slice(text.as_bytes());
}

impl Ser<'_> {
    #[inline(never)]
    fn open(&mut self, tag: u8) -> usize {
        self.out.push(tag);
        let at = self.out.len();
        self.out.extend_from_slice(&[0, 0, 0, 0]);
        at
    }
    #[inline(never)]
    fn int(&mut self, value: i64) {
        write_int(self.out, value);
    }
}

impl Compound<'_, '_> {
    #[inline(never)]
    fn close(self) -> Result<(), WireError> {
        self.ser.out[self.at..self.at + 4].copy_from_slice(&self.n.to_le_bytes());
        Ok(())
    }
}

pub fn form<S: Serializer, T: Serialize + ?Sized>(
    serializer: S,
    name: &'static str,
    value: &T,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_newtype_struct(name, value)
}

impl<'s, 'a> Serializer for &'s mut Ser<'a> {
    type Ok = ();
    type Error = WireError;
    type SerializeSeq = Compound<'s, 'a>;
    type SerializeTuple = Compound<'s, 'a>;
    type SerializeTupleStruct = Compound<'s, 'a>;
    type SerializeTupleVariant = Compound<'s, 'a>;
    type SerializeMap = Compound<'s, 'a>;
    type SerializeStruct = Shaped<'s, 'a>;
    type SerializeStructVariant = Shaped<'s, 'a>;

    fn serialize_bool(self, value: bool) -> Result<(), WireError> {
        self.out.push(if value { TRUE } else { FALSE });
        Ok(())
    }
    fn serialize_i8(self, value: i8) -> Result<(), WireError> {
        self.int(value as i64);
        Ok(())
    }
    fn serialize_i16(self, value: i16) -> Result<(), WireError> {
        self.int(value as i64);
        Ok(())
    }
    fn serialize_i32(self, value: i32) -> Result<(), WireError> {
        self.int(value as i64);
        Ok(())
    }
    fn serialize_i64(self, value: i64) -> Result<(), WireError> {
        self.int(value);
        Ok(())
    }
    fn serialize_i128(self, value: i128) -> Result<(), WireError> {
        write_big(self.out, BIG, value);
        Ok(())
    }
    fn serialize_u8(self, value: u8) -> Result<(), WireError> {
        self.int(value as i64);
        Ok(())
    }
    fn serialize_u16(self, value: u16) -> Result<(), WireError> {
        self.int(value as i64);
        Ok(())
    }
    fn serialize_u32(self, value: u32) -> Result<(), WireError> {
        self.int(value as i64);
        Ok(())
    }
    fn serialize_u64(self, value: u64) -> Result<(), WireError> {
        if value <= SAFE {
            self.int(value as i64);
        } else {
            write_big(self.out, BIG, value);
        }
        Ok(())
    }
    fn serialize_u128(self, value: u128) -> Result<(), WireError> {
        write_big(self.out, BIG, value);
        Ok(())
    }
    #[inline(never)]
    fn serialize_f32(self, value: f32) -> Result<(), WireError> {
        self.out.push(F32);
        self.out.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }
    #[inline(never)]
    fn serialize_f64(self, value: f64) -> Result<(), WireError> {
        self.out.push(F64);
        self.out.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }
    fn serialize_char(self, value: char) -> Result<(), WireError> {
        let mut buffer = [0; 4];
        self.serialize_str(value.encode_utf8(&mut buffer))
    }
    #[inline(never)]
    fn serialize_str(self, value: &str) -> Result<(), WireError> {
        self.out.push(STR);
        self.out
            .extend_from_slice(&(value.len() as u32).to_le_bytes());
        self.out.extend_from_slice(value.as_bytes());
        Ok(())
    }
    fn serialize_bytes(self, value: &[u8]) -> Result<(), WireError> {
        let at = self.open(ARR);
        for &byte in value {
            self.int(byte as i64);
        }
        self.out[at..at + 4].copy_from_slice(&(value.len() as u32).to_le_bytes());
        Ok(())
    }
    fn serialize_none(self) -> Result<(), WireError> {
        self.out.push(NUL);
        Ok(())
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<(), WireError> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<(), WireError> {
        self.out.push(NUL);
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), WireError> {
        self.out.push(NUL);
        Ok(())
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<(), WireError> {
        let sym = self.keys.sym(variant);
        self.out.push(SYM);
        self.out.extend_from_slice(&sym.to_le_bytes());
        Ok(())
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<(), WireError> {
        match name {
            "$b" => {
                let (kind, at, len) = TAKEN.get();
                self.out.push(BLOB_OUT);
                self.out.push(kind as u8);
                self.out.extend_from_slice(&(at as u64).to_le_bytes());
                self.out.extend_from_slice(&(len as u64).to_le_bytes());
                Ok(())
            }
            "$t" | "$c" | "$i" => {
                self.out.push(FORM);
                self.out.push(name.as_bytes()[1]);
                value.serialize(self)
            }
            _ => value.serialize(self),
        }
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), WireError> {
        let sym = self.keys.sym(variant);
        self.out.push(VAR);
        self.out.extend_from_slice(&sym.to_le_bytes());
        value.serialize(self)
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, WireError> {
        let at = self.open(ARR);
        Ok(Compound {
            ser: self,
            at,
            n: 0,
        })
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, WireError> {
        self.serialize_seq(None)
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, WireError> {
        self.serialize_seq(None)
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, WireError> {
        let sym = self.keys.sym(variant);
        self.out.push(VAR);
        self.out.extend_from_slice(&sym.to_le_bytes());
        self.serialize_seq(None)
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, WireError> {
        let at = self.open(MAP);
        Ok(Compound {
            ser: self,
            at,
            n: 0,
        })
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, WireError> {
        self.out.push(SHP);
        let at = self.out.len();
        self.out.extend_from_slice(&[0, 0]);
        let node = self.keys.root();
        Ok(Shaped {
            ser: self,
            at,
            node,
        })
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, WireError> {
        let sym = self.keys.sym(variant);
        self.out.push(VAR);
        self.out.extend_from_slice(&sym.to_le_bytes());
        self.serialize_struct(name, len)
    }
}

macro_rules! compound {
    ($($tr:ident $method:ident),* $(,)?) => {
        $(
            impl ser::$tr for Compound<'_, '_> {
                type Ok = ();
                type Error = WireError;
                fn $method<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), WireError> {
                    self.n += 1;
                    value.serialize(&mut *self.ser)
                }
                fn end(self) -> Result<(), WireError> {
                    self.close()
                }
            }
        )*
    };
}

compound! {
    SerializeSeq serialize_element,
    SerializeTuple serialize_element,
    SerializeTupleStruct serialize_field,
    SerializeTupleVariant serialize_field,
}

impl ser::SerializeMap for Compound<'_, '_> {
    type Ok = ();
    type Error = WireError;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), WireError> {
        self.n += 1;
        key.serialize(&mut *self.ser)
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), WireError> {
        value.serialize(&mut *self.ser)
    }
    fn end(self) -> Result<(), WireError> {
        self.close()
    }
}

impl Shaped<'_, '_> {
    #[inline]
    fn field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), WireError> {
        self.node = self.ser.keys.step(self.node, key);
        value.serialize(&mut *self.ser)
    }
    #[inline(never)]
    fn done(self) -> Result<(), WireError> {
        let id = self.ser.keys.end(self.node);
        self.ser.out[self.at..self.at + 2].copy_from_slice(&id.to_le_bytes());
        Ok(())
    }
}

impl ser::SerializeStruct for Shaped<'_, '_> {
    type Ok = ();
    type Error = WireError;
    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), WireError> {
        self.field(key, value)
    }
    fn end(self) -> Result<(), WireError> {
        self.done()
    }
}

impl ser::SerializeStructVariant for Shaped<'_, '_> {
    type Ok = ();
    type Error = WireError;
    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), WireError> {
        self.field(key, value)
    }
    fn end(self) -> Result<(), WireError> {
        self.done()
    }
}
