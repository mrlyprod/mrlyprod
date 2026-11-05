use crate::blob::Prim;
use crate::call::{Error, library, on, set_borrow};
use crate::held::{park, store};
use crate::ser::{Keys, Ser, write_big, write_int};
use crate::*;
use serde::Serialize;
use std::fmt::Display;

pub struct Out<'a> {
    pub(crate) buf: &'a mut Vec<u8>,
    pub(crate) keys: &'a mut Keys,
}

impl Out<'_> {
    #[inline(never)]
    pub fn value<T: Serialize + 'static>(&mut self, value: T) -> Result<(), Error> {
        set_borrow(true);
        let mut ser = Ser {
            out: &mut *self.buf,
            keys: &mut *self.keys,
        };
        let done = value.serialize(&mut ser).map_err(library);
        set_borrow(false);
        park(Box::new(value));
        done
    }
    #[inline(never)]
    pub fn view<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        let mut ser = Ser {
            out: &mut *self.buf,
            keys: &mut *self.keys,
        };
        value.serialize(&mut ser).map_err(library)
    }
    pub fn null(&mut self) {
        self.buf.push(NUL);
    }
    pub fn array(&mut self, n: usize) {
        self.buf.push(ARR);
        self.buf.extend_from_slice(&(n as u32).to_le_bytes());
    }
    pub fn map(&mut self, n: usize) {
        self.buf.push(MAP);
        self.buf.extend_from_slice(&(n as u32).to_le_bytes());
    }
    pub fn text(&mut self, value: &str) {
        self.buf.push(STR);
        self.buf
            .extend_from_slice(&(value.len() as u32).to_le_bytes());
        self.buf.extend_from_slice(value.as_bytes());
    }
    pub fn bigint<T: Long>(&mut self, value: T) {
        if on() {
            let (tag, bytes) = value.raw();
            self.buf.push(tag);
            self.buf.extend_from_slice(&bytes);
        } else {
            write_big(self.buf, WIDE, value);
        }
    }
    pub fn wide<T: Display>(&mut self, value: T) {
        write_big(self.buf, WIDE, value);
    }
    pub fn int(&mut self, value: i64) {
        write_int(self.buf, value);
    }
    #[inline(never)]
    pub fn small<T: Prim + Serialize + Clone>(&mut self, value: &[T]) {
        if cfg!(target_arch = "wasm32") && on() {
            self.buf.push(INLINE);
            self.buf.push(T::KIND as u8);
            self.buf
                .extend_from_slice(&(value.len() as u32).to_le_bytes());
            let bytes = unsafe {
                std::slice::from_raw_parts(value.as_ptr().cast::<u8>(), size_of_val(value))
            };
            self.buf.extend_from_slice(bytes);
        } else {
            self.blob(value.to_vec());
        }
    }
    #[inline(never)]
    pub fn blob<T: Prim + Serialize>(&mut self, value: Vec<T>) {
        if !on() {
            self.array(value.len());
            for one in &value {
                let _ = self.view(one);
            }
            return;
        }
        self.buf.push(BLOB_OUT);
        self.buf.push(T::KIND as u8);
        self.buf
            .extend_from_slice(&(value.as_ptr() as u64).to_le_bytes());
        self.buf
            .extend_from_slice(&(value.len() as u64).to_le_bytes());
        park(Box::new(value));
    }
    #[inline(never)]
    pub fn held<T: 'static>(&mut self, value: T, class: &'static str) {
        let id = store(Box::new(value));
        let sym = self.keys.sym(class);
        self.buf.push(HANDLE);
        self.buf.extend_from_slice(&id.to_le_bytes());
        self.buf.extend_from_slice(&sym.to_le_bytes());
    }
    pub fn rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.array(4);
        for one in [r, g, b, a] {
            write_int(self.buf, one as i64);
        }
    }
    pub fn back(&mut self, n: usize) {
        self.buf.push(BACK);
        self.buf.push(n as u8);
    }
    pub fn at(&mut self, index: usize) {
        self.buf.push(index as u8);
    }
}

pub trait Long: Display + Copy {
    fn raw(self) -> (u8, [u8; 8]);
}

impl Long for i64 {
    fn raw(self) -> (u8, [u8; 8]) {
        (BIG64, self.to_le_bytes())
    }
}

impl Long for u64 {
    fn raw(self) -> (u8, [u8; 8]) {
        (UBIG64, self.to_le_bytes())
    }
}

impl<T: Long> Long for &T {
    fn raw(self) -> (u8, [u8; 8]) {
        (*self).raw()
    }
}
