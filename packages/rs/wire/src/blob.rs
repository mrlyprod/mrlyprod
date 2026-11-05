use crate::call::on_borrow;
use crate::held::park;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::Serializer;
use std::cell::{Cell, RefCell};
use std::fmt;

thread_local! {
    pub(crate) static TAKEN: Cell<(u32, usize, usize)> = const { Cell::new((0, 0, 0)) };
    static INBOX: RefCell<Vec<Option<Blob>>> = const { RefCell::new(Vec::new()) };
}

pub enum Blob {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    U64(Vec<u64>),
    I64(Vec<i64>),
}

pub trait Prim: Copy + 'static {
    const KIND: u32;
    fn from_blob(blob: Blob) -> Vec<Self>;
}

#[inline]
fn pass<A: Prim, B: Prim>(v: Vec<A>, each: fn(A) -> B) -> Vec<B> {
    if A::KIND == B::KIND && size_of::<A>() == size_of::<B>() {
        let mut v = std::mem::ManuallyDrop::new(v);
        unsafe { Vec::from_raw_parts(v.as_mut_ptr() as *mut B, v.len(), v.capacity()) }
    } else {
        v.into_iter().map(each).collect()
    }
}

macro_rules! prim {
    ($($t:ty => $kind:expr),* $(,)?) => {
        $(
            impl Prim for $t {
                const KIND: u32 = $kind;
                fn from_blob(blob: Blob) -> Vec<Self> {
                    match blob {
                        Blob::U8(v) => pass(v, |x| x as $t),
                        Blob::U16(v) => pass(v, |x| x as $t),
                        Blob::U32(v) => pass(v, |x| x as $t),
                        Blob::I8(v) => pass(v, |x| x as $t),
                        Blob::I16(v) => pass(v, |x| x as $t),
                        Blob::I32(v) => pass(v, |x| x as $t),
                        Blob::F32(v) => pass(v, |x| x as $t),
                        Blob::F64(v) => pass(v, |x| x as $t),
                        Blob::U64(v) => pass(v, |x| x as $t),
                        Blob::I64(v) => pass(v, |x| x as $t),
                    }
                }
            }
        )*
    };
}

prim! { u8 => 0, u16 => 1, u32 => 2, i8 => 3, i16 => 4, i32 => 5, f32 => 6, f64 => 7, u64 => 8, i64 => 9 }

#[cfg(target_pointer_width = "32")]
prim! { usize => 2, isize => 5 }

#[cfg(target_pointer_width = "64")]
prim! { usize => 8, isize => 9 }

fn fresh<T: Copy>(n: usize) -> (Vec<T>, *mut u8) {
    let mut v: Vec<T> = Vec::with_capacity(n);
    #[allow(clippy::uninit_vec)]
    unsafe {
        v.set_len(n)
    };
    let at = v.as_mut_ptr() as *mut u8;
    (v, at)
}

pub fn inbox(kind: u32, n: usize) -> *mut u8 {
    macro_rules! make {
        ($t:ty, $v:ident) => {{
            let (v, at) = fresh::<$t>(n);
            (Blob::$v(v), at)
        }};
    }
    let (blob, at) = match kind {
        0 => make!(u8, U8),
        1 => make!(u16, U16),
        2 => make!(u32, U32),
        3 => make!(i8, I8),
        4 => make!(i16, I16),
        5 => make!(i32, I32),
        6 => make!(f32, F32),
        7 => make!(f64, F64),
        8 => make!(u64, U64),
        _ => make!(i64, I64),
    };
    INBOX.with_borrow_mut(|list| list.push(Some(blob)));
    at
}

pub fn take_blob(id: usize) -> Option<Blob> {
    INBOX.with_borrow_mut(|list| list.get_mut(id).and_then(Option::take))
}

pub(crate) fn clear_inbox() {
    INBOX.with_borrow_mut(Vec::clear);
}

pub(crate) fn peek_len(id: usize) -> Option<usize> {
    INBOX.with_borrow(|list| {
        list.get(id)
            .and_then(Option::as_ref)
            .map(|blob| match blob {
                Blob::U8(v) => v.len(),
                Blob::U16(v) => v.len(),
                Blob::U32(v) => v.len(),
                Blob::I8(v) => v.len(),
                Blob::I16(v) => v.len(),
                Blob::I32(v) => v.len(),
                Blob::F32(v) => v.len(),
                Blob::F64(v) => v.len(),
                Blob::U64(v) => v.len(),
                Blob::I64(v) => v.len(),
            })
    })
}

pub fn write_ref<S: Serializer, T: Prim>(serializer: S, data: &[T]) -> Result<S::Ok, S::Error> {
    let at = if on_borrow() {
        data.as_ptr() as usize
    } else {
        let copy = data.to_vec();
        let at = copy.as_ptr() as usize;
        park(Box::new(copy));
        at
    };
    TAKEN.set((T::KIND, at, data.len()));
    serializer.serialize_newtype_struct("$b", &())
}

pub struct Raw(pub Blob);

impl Raw {
    pub fn len(&self) -> usize {
        match &self.0 {
            Blob::U8(v) => v.len(),
            Blob::U16(v) => v.len(),
            Blob::U32(v) => v.len(),
            Blob::I8(v) => v.len(),
            Blob::I16(v) => v.len(),
            Blob::I32(v) => v.len(),
            Blob::F32(v) => v.len(),
            Blob::F64(v) => v.len(),
            Blob::U64(v) => v.len(),
            Blob::I64(v) => v.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

struct RawVisitor;

impl<'de> Visitor<'de> for RawVisitor {
    type Value = Raw;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("raw bytes or a list of numbers")
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Raw, A::Error> {
        let Some((key, id)) = map.next_entry::<&str, usize>()? else {
            return Err(de::Error::custom("empty raw bytes"));
        };
        if key != "$b" {
            return Err(de::Error::custom("raw bytes"));
        }
        take_blob(id)
            .map(Raw)
            .ok_or_else(|| de::Error::custom("a byte buffer was read twice"))
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Raw, A::Error> {
        let mut out = Vec::with_capacity(seq.size_hint().unwrap_or(0));
        while let Some(one) = seq.next_element::<u8>()? {
            out.push(one);
        }
        Ok(Raw(Blob::U8(out)))
    }
}

impl<'de> Deserialize<'de> for Raw {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Raw, D::Error> {
        deserializer.deserialize_any(RawVisitor)
    }
}

pub fn flat_rgba(flat: &[u8]) -> Vec<[u8; 4]> {
    flat.chunks_exact(4)
        .map(|x| [x[0], x[1], x[2], x[3]])
        .collect()
}
