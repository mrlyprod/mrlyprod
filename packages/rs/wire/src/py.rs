#![allow(
    clippy::missing_transmute_annotations,
    clippy::needless_range_loop,
    clippy::deref_addrof,
    clippy::not_unsafe_ptr_arg_deref
)]

use crate::blob::inbox;
use crate::call::{Function, failed, out_len, out_ptr, run_host};
use crate::held::keep_result;
use crate::ser::shapes_of;
use crate::*;
use std::cell::RefCell;
use std::ffi::{CString, c_char, c_int, c_long, c_void};
use std::sync::OnceLock;

type Obj = *mut c_void;

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
}

#[cfg(target_os = "macos")]
const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;
#[cfg(not(target_os = "macos"))]
const RTLD_DEFAULT: *mut c_void = std::ptr::null_mut();

macro_rules! api {
    ($($name:ident : fn($($arg:ty),*) $(-> $ret:ty)?;)* | $($global:ident;)*) => {
        #[allow(non_snake_case)]
        struct Api {
            $($name: unsafe extern "C" fn($($arg),*) $(-> $ret)?,)*
            $($global: Obj,)*
        }
        fn load() -> Api {
            unsafe {
                Api {
                    $($name: std::mem::transmute(dlsym(RTLD_DEFAULT, concat!(stringify!($name), "\0").as_ptr() as *const c_char)),)*
                    $($global: dlsym(RTLD_DEFAULT, concat!(stringify!($global), "\0").as_ptr() as *const c_char),)*
                }
            }
        }
    };
}

#[repr(C)]
struct Buffer {
    buf: *mut c_void,
    obj: Obj,
    len: isize,
    itemsize: isize,
    readonly: c_int,
    ndim: c_int,
    format: *mut c_char,
    shape: *mut isize,
    strides: *mut isize,
    suboffsets: *mut isize,
    internal: *mut c_void,
}

#[repr(C)]
pub struct MethodDef {
    name: *const c_char,
    call: *const c_void,
    flags: c_int,
    doc: *const c_char,
}

api! {
    PyLong_FromLongLong: fn(i64) -> Obj;
    PyLong_FromUnsignedLongLong: fn(u64) -> Obj;
    PyLong_FromDouble: fn(f64) -> Obj;
    PyLong_AsLongLongAndOverflow: fn(Obj, *mut c_int) -> i64;
    PyLong_FromString: fn(*const c_char, *mut *mut c_char, c_int) -> Obj;
    PyLong_AsLong: fn(Obj) -> c_long;
    PyFloat_FromDouble: fn(f64) -> Obj;
    PyFloat_AsDouble: fn(Obj) -> f64;
    PyBool_FromLong: fn(c_long) -> Obj;
    PyUnicode_FromStringAndSize: fn(*const c_char, isize) -> Obj;
    PyUnicode_AsUTF8AndSize: fn(Obj, *mut isize) -> *const c_char;
    PyUnicode_InternFromString: fn(*const c_char) -> Obj;
    PyObject_Str: fn(Obj) -> Obj;
    PyList_New: fn(isize) -> Obj;
    PyList_SetItem: fn(Obj, isize, Obj) -> c_int;
    PyList_Size: fn(Obj) -> isize;
    PyList_GetItem: fn(Obj, isize) -> Obj;
    PyTuple_New: fn(isize) -> Obj;
    PyTuple_SetItem: fn(Obj, isize, Obj) -> c_int;
    PyTuple_Size: fn(Obj) -> isize;
    PyTuple_GetItem: fn(Obj, isize) -> Obj;
    PyDict_New: fn() -> Obj;
    PyDict_SetItem: fn(Obj, Obj, Obj) -> c_int;
    PyDict_Next: fn(Obj, *mut isize, *mut Obj, *mut Obj) -> c_int;
    PyDict_Size: fn(Obj) -> isize;
    PyDict_GetItemString: fn(Obj, *const c_char) -> Obj;
    PyErr_SetString: fn(Obj, *const c_char);
    PyErr_Occurred: fn() -> Obj;
    PyErr_Clear: fn();
    PyObject_CallObject: fn(Obj, Obj) -> Obj;
    PyEval_SaveThread: fn() -> *mut c_void;
    PyEval_RestoreThread: fn(*mut c_void);
    PyCMethod_New: fn(*mut MethodDef, Obj, Obj, Obj) -> Obj;
    PyInstanceMethod_New: fn(Obj) -> Obj;
    PyObject_CheckBuffer: fn(Obj) -> c_int;
    PyObject_GetBuffer: fn(Obj, *mut Buffer, c_int) -> c_int;
    PyBuffer_Release: fn(*mut Buffer);
    PyType_GetFlags: fn(Obj) -> u64;
    PyObject_IsInstance: fn(Obj, Obj) -> c_int;
    Py_IncRef: fn(Obj);
    Py_DecRef: fn(Obj);
    |
    PyLong_Type;
    PyFloat_Type;
    PyUnicode_Type;
    PyList_Type;
    PyTuple_Type;
    PyDict_Type;
    _Py_NoneStruct;
    _Py_TrueStruct;
    _Py_FalseStruct;
}

const FASTCALL: c_int = 0x0080;
const LONG_SUBCLASS: u64 = 1 << 24;
const FLAT_KEYS: [&str; 4] = ["data", "types", "colors", "tags"];

struct Tools {
    api: Api,
    doors: &'static [Function],
    error: Obj,
    array: Obj,
    wrap: Obj,
    keep: Obj,
    form: Obj,
    back: Obj,
    base: Obj,
    door_def: *mut MethodDef,
    direct_def: *mut MethodDef,
    known: Vec<Vec<Obj>>,
}

unsafe impl Send for Tools {}
unsafe impl Sync for Tools {}

static TOOLS: OnceLock<Tools> = OnceLock::new();

struct Quick {
    at: usize,
    n: u8,
    ret: u8,
}

static mut QUICK: Vec<Quick> = Vec::new();

thread_local! {
    static FRAME: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static SYMS: RefCell<Vec<Obj>> = const { RefCell::new(Vec::new()) };
    static SHAPES: RefCell<Vec<Vec<Obj>>> = const { RefCell::new(Vec::new()) };
}

#[inline(always)]
fn tools() -> &'static Tools {
    TOOLS.get().unwrap()
}

#[inline(always)]
unsafe fn type_of(object: Obj) -> Obj {
    unsafe { *(object as *const Obj).add(1) }
}

fn kind_of(format: *const c_char, itemsize: isize) -> Option<u32> {
    let mut at = format as *const u8;
    if at.is_null() {
        return Some(0);
    }
    let code = unsafe {
        if matches!(*at, b'<' | b'=' | b'@' | b'|') {
            at = at.add(1);
        }
        *at
    };
    Some(match (code, itemsize) {
        (b'B', 1) | (b'c', 1) | (b'?', 1) => 0,
        (b'H', 2) => 1,
        (b'I', 4) | (b'L', 4) => 2,
        (b'b', 1) => 3,
        (b'h', 2) => 4,
        (b'i', 4) | (b'l', 4) => 5,
        (b'f', 4) => 6,
        (b'd', 8) => 7,
        (b'L', 8) | (b'Q', 8) => 8,
        (b'l', 8) | (b'q', 8) => 9,
        _ => return None,
    })
}

unsafe fn text_of(api: &Api, object: Obj, out: &mut Vec<u8>, tag: u8) -> Result<(), String> {
    unsafe {
        let mut n = 0isize;
        let at = (api.PyUnicode_AsUTF8AndSize)(object, &mut n);
        if at.is_null() {
            (api.PyErr_Clear)();
            return Err("a string that cannot be read".to_string());
        }
        out.push(tag);
        out.extend_from_slice(&(n as u32).to_le_bytes());
        out.extend_from_slice(std::slice::from_raw_parts(at as *const u8, n as usize));
    }
    Ok(())
}

unsafe fn raw_buffer(
    api: &Api,
    object: Obj,
    flat: bool,
    out: &mut Vec<u8>,
    blobs: &mut u32,
) -> Result<Option<Vec<usize>>, String> {
    unsafe {
        if (api.PyObject_CheckBuffer)(object) == 0 {
            return Ok(None);
        }
        let mut view: Buffer = std::mem::zeroed();
        if (api.PyObject_GetBuffer)(object, &mut view, 0x4 | 0x8 | 0x20) != 0 {
            (api.PyErr_Clear)();
            return Err("an array that is not contiguous".to_string());
        }
        let Some(kind) = kind_of(view.format, view.itemsize) else {
            (api.PyBuffer_Release)(&mut view);
            return Err("an array of an unknown type".to_string());
        };
        let n = view.len as usize / view.itemsize.max(1) as usize;
        let at = inbox(kind, n);
        std::ptr::copy_nonoverlapping(view.buf as *const u8, at, view.len as usize);
        let id = *blobs;
        *blobs += 1;
        let shape: Vec<usize> = (0..view.ndim as usize)
            .map(|axis| *view.shape.add(axis) as usize)
            .collect();
        if view.ndim >= 2 && !flat {
            out.push(MAP);
            out.extend_from_slice(&2u32.to_le_bytes());
            write_shape(out, &shape);
            out.push(STR);
            out.extend_from_slice(&4u32.to_le_bytes());
            out.extend_from_slice(b"data");
        }
        out.push(BLOB_IN);
        out.extend_from_slice(&id.to_le_bytes());
        (api.PyBuffer_Release)(&mut view);
        Ok(Some(shape))
    }
}

fn write_shape(out: &mut Vec<u8>, shape: &[usize]) {
    out.push(STR);
    out.extend_from_slice(&5u32.to_le_bytes());
    out.extend_from_slice(b"shape");
    out.push(ARR);
    out.extend_from_slice(&(shape.len() as u32).to_le_bytes());
    for &length in shape {
        out.push(I32);
        out.extend_from_slice(&(length as i32).to_le_bytes());
    }
}

unsafe fn put(
    api: &Api,
    object: Obj,
    flat: bool,
    out: &mut Vec<u8>,
    blobs: &mut u32,
) -> Result<(), String> {
    unsafe {
        if object == api._Py_NoneStruct {
            out.push(NUL);
            return Ok(());
        }
        if object == api._Py_TrueStruct {
            out.push(TRUE);
            return Ok(());
        }
        if object == api._Py_FalseStruct {
            out.push(FALSE);
            return Ok(());
        }
        let ty = type_of(object);
        if ty == api.PyLong_Type {
            let mut over = 0;
            let value = (api.PyLong_AsLongLongAndOverflow)(object, &mut over);
            if over != 0 {
                let text = (api.PyObject_Str)(object);
                let done = text_of(api, text, out, BIG);
                (api.Py_DecRef)(text);
                return done;
            }
            crate::ser::write_int(out, value);
            return Ok(());
        }
        if ty == api.PyFloat_Type {
            out.push(F64);
            out.extend_from_slice(&(api.PyFloat_AsDouble)(object).to_le_bytes());
            return Ok(());
        }
        if ty == api.PyUnicode_Type {
            return text_of(api, object, out, STR);
        }
        if ty == api.PyList_Type {
            let n = (api.PyList_Size)(object);
            out.push(ARR);
            out.extend_from_slice(&(n as u32).to_le_bytes());
            for at in 0..n {
                put(api, (api.PyList_GetItem)(object, at), false, out, blobs)?;
            }
            return Ok(());
        }
        if ty == api.PyTuple_Type {
            let n = (api.PyTuple_Size)(object);
            out.push(ARR);
            out.extend_from_slice(&(n as u32).to_le_bytes());
            for at in 0..n {
                put(api, (api.PyTuple_GetItem)(object, at), false, out, blobs)?;
            }
            return Ok(());
        }
        if ty == api.PyDict_Type {
            return put_dict(api, object, out, blobs);
        }
        if (api.PyType_GetFlags)(ty) & LONG_SUBCLASS != 0 {
            if (api.PyObject_IsInstance)(object, tools().base) != 1 {
                (api.PyErr_Clear)();
                return Err(
                    "an object made by another mrly unit; make it with this one".to_string()
                );
            }
            let id = (api.PyLong_AsLong)(object) as u32;
            out.push(HANDLE);
            out.extend_from_slice(&id.to_le_bytes());
            return Ok(());
        }
        if raw_buffer(api, object, flat, out, blobs)?.is_some() {
            return Ok(());
        }
        let value = (api.PyFloat_AsDouble)(object);
        if value == -1.0 && !(api.PyErr_Occurred)().is_null() {
            (api.PyErr_Clear)();
            return Err("a value that cannot cross".to_string());
        }
        if value.fract() == 0.0 && value.abs() <= SAFE as f64 {
            crate::ser::write_int(out, value as i64);
        } else {
            out.push(F64);
            out.extend_from_slice(&value.to_le_bytes());
        }
    }
    Ok(())
}

unsafe fn put_dict(
    api: &Api,
    object: Obj,
    out: &mut Vec<u8>,
    blobs: &mut u32,
) -> Result<(), String> {
    unsafe {
        let shaped = !(api.PyDict_GetItemString)(object, c"shape".as_ptr()).is_null();
        let types = (api.PyDict_GetItemString)(object, c"types".as_ptr());
        let celled = !types.is_null();
        let mut n = (api.PyDict_Size)(object) as u32;
        let mut extra = None;
        if celled && !shaped && (api.PyObject_CheckBuffer)(types) != 0 {
            let mut view: Buffer = std::mem::zeroed();
            if (api.PyObject_GetBuffer)(types, &mut view, 0x8) == 0 {
                extra = Some(
                    (0..view.ndim as usize)
                        .map(|axis| *view.shape.add(axis) as usize)
                        .collect::<Vec<usize>>(),
                );
                (api.PyBuffer_Release)(&mut view);
            } else {
                (api.PyErr_Clear)();
            }
        }
        if extra.is_some() {
            n += 1;
        }
        out.push(MAP);
        let count_at = out.len();
        out.extend_from_slice(&n.to_le_bytes());
        if let Some(shape) = extra {
            write_shape(out, &shape);
        }
        let mut at = 0isize;
        let mut key: Obj = std::ptr::null_mut();
        let mut item: Obj = std::ptr::null_mut();
        let mut skipped = 0;
        while (api.PyDict_Next)(object, &mut at, &mut key, &mut item) != 0 {
            let mut size = 0isize;
            let text = (api.PyUnicode_AsUTF8AndSize)(key, &mut size);
            if text.is_null() {
                (api.PyErr_Clear)();
                let shown = (api.PyObject_Str)(key);
                let done = text_of(api, shown, out, STR);
                (api.Py_DecRef)(shown);
                done?;
                put(api, item, false, out, blobs)?;
                continue;
            }
            let name = std::slice::from_raw_parts(text as *const u8, size as usize);
            if item == api._Py_NoneStruct
                && (celled || shaped)
                && FLAT_KEYS.iter().any(|one| one.as_bytes() == name)
            {
                skipped += 1;
                continue;
            }
            text_of(api, key, out, STR)?;
            let flat = (celled || shaped) && FLAT_KEYS.iter().any(|one| one.as_bytes() == name);
            put(api, item, flat, out, blobs)?;
        }
        if skipped != 0 {
            out[count_at..count_at + 4].copy_from_slice(&(n - skipped).to_le_bytes());
        }
    }
    Ok(())
}

struct Reader<'a> {
    b: &'a [u8],
    p: usize,
    keep: Obj,
}

impl Reader<'_> {
    #[inline]
    fn u16(&mut self) -> u16 {
        let value = u16::from_le_bytes([self.b[self.p], self.b[self.p + 1]]);
        self.p += 2;
        value
    }
    #[inline]
    fn u32(&mut self) -> u32 {
        let value = u32::from_le_bytes([
            self.b[self.p],
            self.b[self.p + 1],
            self.b[self.p + 2],
            self.b[self.p + 3],
        ]);
        self.p += 4;
        value
    }
    #[inline]
    fn u64(&mut self) -> u64 {
        let mut word = [0; 8];
        word.copy_from_slice(&self.b[self.p..self.p + 8]);
        self.p += 8;
        u64::from_le_bytes(word)
    }
    unsafe fn sym(&mut self, api: &Api) -> Obj {
        let id = self.u16() as usize;
        let object = SYMS.with_borrow(|syms| syms[id]);
        unsafe { (api.Py_IncRef)(object) };
        object
    }
    unsafe fn big(&mut self, api: &Api) -> Obj {
        let n = self.u32() as usize;
        let text = CString::new(&self.b[self.p..self.p + n]).unwrap_or_default();
        self.p += n;
        unsafe { (api.PyLong_FromString)(text.as_ptr(), std::ptr::null_mut(), 10) }
    }
    unsafe fn read(&mut self, tools: &Tools) -> Obj {
        let api = &tools.api;
        let tag = self.b[self.p];
        self.p += 1;
        unsafe {
            match tag {
                NUL => {
                    (api.Py_IncRef)(api._Py_NoneStruct);
                    api._Py_NoneStruct
                }
                FALSE => (api.PyBool_FromLong)(0),
                TRUE => (api.PyBool_FromLong)(1),
                I32 => (api.PyLong_FromLongLong)(self.u32() as i32 as i64),
                F32 => (api.PyFloat_FromDouble)(f32::from_bits(self.u32()) as f64),
                F64 => (api.PyFloat_FromDouble)(f64::from_bits(self.u64())),
                I53 => (api.PyLong_FromLongLong)(f64::from_bits(self.u64()) as i64),
                STR => {
                    let n = self.u32() as usize;
                    let at = self.b[self.p..].as_ptr() as *const c_char;
                    self.p += n;
                    (api.PyUnicode_FromStringAndSize)(at, n as isize)
                }
                BIG | WIDE => self.big(api),
                BIG64 => (api.PyLong_FromLongLong)(self.u64() as i64),
                UBIG64 => (api.PyLong_FromUnsignedLongLong)(self.u64()),
                ARR => {
                    let n = self.u32() as isize;
                    let list = (api.PyList_New)(n);
                    for at in 0..n {
                        (api.PyList_SetItem)(list, at, self.read(tools));
                    }
                    list
                }
                MAP => {
                    let n = self.u32();
                    let map = (api.PyDict_New)();
                    for _ in 0..n {
                        let key = self.read(tools);
                        let item = self.read(tools);
                        (api.PyDict_SetItem)(map, key, item);
                        (api.Py_DecRef)(key);
                        (api.Py_DecRef)(item);
                    }
                    map
                }
                SHP => {
                    let id = self.u16() as usize;
                    let keys = if id < tools.known.len() {
                        tools.known[id].clone()
                    } else {
                        SHAPES.with_borrow(|shapes| shapes[id - tools.known.len()].clone())
                    };
                    let map = (api.PyDict_New)();
                    for key in keys {
                        let item = self.read(tools);
                        (api.PyDict_SetItem)(map, key, item);
                        (api.Py_DecRef)(item);
                    }
                    map
                }
                SYM => self.sym(api),
                VAR => {
                    let key = self.sym(api);
                    let item = self.read(tools);
                    let map = (api.PyDict_New)();
                    (api.PyDict_SetItem)(map, key, item);
                    (api.Py_DecRef)(key);
                    (api.Py_DecRef)(item);
                    map
                }
                FORM => {
                    let kind = self.b[self.p] as i64;
                    self.p += 1;
                    let value = self.read(tools);
                    let args = (api.PyTuple_New)(2);
                    (api.PyTuple_SetItem)(args, 0, (api.PyLong_FromLongLong)(kind));
                    (api.PyTuple_SetItem)(args, 1, value);
                    let made = (api.PyObject_CallObject)(tools.form, args);
                    (api.Py_DecRef)(args);
                    made
                }
                BLOB_OUT => {
                    let kind = self.b[self.p] as i64;
                    self.p += 1;
                    let at = self.u64() as i64;
                    let n = self.u64() as i64;
                    if self.keep.is_null() {
                        let id = keep_result() as i64;
                        let args = (api.PyTuple_New)(1);
                        (api.PyTuple_SetItem)(args, 0, (api.PyLong_FromLongLong)(id));
                        self.keep = (api.PyObject_CallObject)(tools.keep, args);
                        (api.Py_DecRef)(args);
                    }
                    let args = (api.PyTuple_New)(4);
                    (api.PyTuple_SetItem)(args, 0, (api.PyLong_FromLongLong)(kind));
                    (api.PyTuple_SetItem)(args, 1, (api.PyLong_FromLongLong)(at));
                    (api.PyTuple_SetItem)(args, 2, (api.PyLong_FromLongLong)(n));
                    (api.Py_IncRef)(self.keep);
                    (api.PyTuple_SetItem)(args, 3, self.keep);
                    let made = (api.PyObject_CallObject)(tools.array, args);
                    (api.Py_DecRef)(args);
                    made
                }
                HANDLE => {
                    let id = self.u32() as i64;
                    let class = self.sym(api);
                    let args = (api.PyTuple_New)(2);
                    (api.PyTuple_SetItem)(args, 0, (api.PyLong_FromLongLong)(id));
                    (api.PyTuple_SetItem)(args, 1, class);
                    let made = (api.PyObject_CallObject)(tools.wrap, args);
                    (api.Py_DecRef)(args);
                    made
                }
                BACK => {
                    let n = self.b[self.p] as usize;
                    self.p += 1;
                    let list = (api.PyList_New)(n as isize);
                    for at in 0..n {
                        let index = self.b[self.p] as i64;
                        self.p += 1;
                        let value = self.read(tools);
                        let pair = (api.PyTuple_New)(2);
                        (api.PyTuple_SetItem)(pair, 0, (api.PyLong_FromLongLong)(index));
                        (api.PyTuple_SetItem)(pair, 1, value);
                        (api.PyList_SetItem)(list, at as isize, pair);
                    }
                    let ret = self.read(tools);
                    let args = (api.PyTuple_New)(2);
                    (api.PyTuple_SetItem)(args, 0, list);
                    (api.PyTuple_SetItem)(args, 1, ret);
                    args
                }
                SMALL..=0xff => (api.PyLong_FromLongLong)((tag & 0x7f) as i64),
                _ => std::ptr::null_mut(),
            }
        }
    }
}

unsafe fn define(tools: &Tools, b: &[u8], at: usize) {
    let api = &tools.api;
    let base = tools.known.len();
    let mut p = at;
    while p < b.len() {
        let kind = b[p];
        let id = u16::from_le_bytes([b[p + 1], b[p + 2]]) as usize;
        p += 3;
        if kind == 1 {
            let n = u32::from_le_bytes([b[p], b[p + 1], b[p + 2], b[p + 3]]) as usize;
            p += 4;
            let text = CString::new(&b[p..p + n]).unwrap_or_default();
            p += n;
            let object = unsafe { (api.PyUnicode_InternFromString)(text.as_ptr()) };
            SYMS.with_borrow_mut(|syms| {
                if syms.len() <= id {
                    syms.resize(id + 1, std::ptr::null_mut());
                }
                syms[id] = object;
            });
        } else {
            let n = u16::from_le_bytes([b[p], b[p + 1]]) as usize;
            p += 2;
            let mut list = Vec::with_capacity(n);
            for _ in 0..n {
                let sym = u16::from_le_bytes([b[p], b[p + 1]]) as usize;
                p += 2;
                list.push(SYMS.with_borrow(|syms| syms[sym]));
            }
            SHAPES.with_borrow_mut(|shapes| {
                let at = id - base;
                if shapes.len() <= at {
                    shapes.resize(at + 1, Vec::new());
                }
                shapes[at] = list;
            });
        }
    }
}

unsafe fn raise(tools: &Tools, message: &str) -> Obj {
    let text = CString::new(message.replace('\0', " ")).unwrap_or_default();
    unsafe { (tools.api.PyErr_SetString)(tools.error, text.as_ptr()) };
    std::ptr::null_mut()
}

unsafe extern "C" fn door_call(me: Obj, args: *const Obj, nargs: isize) -> Obj {
    let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
        door_body(me, args, nargs)
    }));
    match run {
        Ok(made) => made,
        Err(payload) => {
            crate::call::OUT.with_borrow_mut(Vec::clear);
            crate::blob::clear_inbox();
            unsafe { raise(tools(), &crate::call::panic_text(payload)) }
        }
    }
}

unsafe fn door_body(me: Obj, args: *const Obj, nargs: isize) -> Obj {
    let tools = tools();
    let api = &tools.api;
    unsafe {
        let packed = (api.PyLong_AsLong)(me) as usize;
        let door = packed & 0xffff;
        let want = (packed >> 16) as isize;
        let count = want.max(nargs).min(24);
        let mut frame = FRAME.with_borrow_mut(std::mem::take);
        frame.clear();
        let mut blobs = 0;
        let mut ends = [0usize; 24];
        for at in 0..count as usize {
            if at < nargs as usize {
                if let Err(message) = put(api, *args.add(at), false, &mut frame, &mut blobs) {
                    crate::blob::clear_inbox();
                    FRAME.with_borrow_mut(|keep| *keep = frame);
                    return raise(
                        tools,
                        &format!("{} argument {}: {message}", tools.doors[door].name, at + 1),
                    );
                }
            } else {
                frame.push(NUL);
            }
            ends[at] = frame.len();
        }
        let mut parts: [&[u8]; 24] = [&[]; 24];
        let mut from = 0;
        for at in 0..count as usize {
            parts[at] = &frame[from..ends[at]];
            from = ends[at];
        }
        let state = (api.PyEval_SaveThread)();
        let status = run_host(&tools.doors[door], &parts[..count as usize], true);
        (api.PyEval_RestoreThread)(state);
        let bytes = crate::call::OUT.with_borrow_mut(std::mem::take);
        let done = if status != 0 {
            raise(tools, &String::from_utf8_lossy(&bytes))
        } else {
            let defs = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
            if defs != 0 {
                define(tools, &bytes, defs);
            }
            let mut reader = Reader {
                b: &bytes,
                p: 4,
                keep: std::ptr::null_mut(),
            };
            let made = reader.read(tools);
            if !reader.keep.is_null() {
                (api.Py_DecRef)(reader.keep);
            }
            if !made.is_null() && bytes[4] == BACK {
                let call = (api.PyTuple_New)(nargs.max(0) + 1);
                (api.Py_IncRef)(made);
                (api.PyTuple_SetItem)(call, 0, made);
                for at in 0..nargs {
                    let item = *args.add(at as usize);
                    (api.Py_IncRef)(item);
                    (api.PyTuple_SetItem)(call, at + 1, item);
                }
                let result = (api.PyObject_CallObject)(tools.back, call);
                (api.Py_DecRef)(call);
                (api.Py_DecRef)(made);
                result
            } else {
                made
            }
        };
        crate::call::OUT.with_borrow_mut(|out| {
            if out.is_empty() {
                *out = bytes;
            }
        });
        FRAME.with_borrow_mut(|keep| *keep = frame);
        done
    }
}

type F0 = unsafe extern "C-unwind" fn() -> f64;
type F1 = unsafe extern "C-unwind" fn(f64) -> f64;
type F2 = unsafe extern "C-unwind" fn(f64, f64) -> f64;
type F3 = unsafe extern "C-unwind" fn(f64, f64, f64) -> f64;
type F4 = unsafe extern "C-unwind" fn(f64, f64, f64, f64) -> f64;
type F5 = unsafe extern "C-unwind" fn(f64, f64, f64, f64, f64) -> f64;
type F6 = unsafe extern "C-unwind" fn(f64, f64, f64, f64, f64, f64) -> f64;

unsafe extern "C" fn direct_call(me: Obj, args: *const Obj, nargs: isize) -> Obj {
    let tools = tools();
    let api = &tools.api;
    unsafe {
        let quick = &*(&raw const QUICK);
        let one = &quick[(api.PyLong_AsLong)(me) as usize];
        let n = one.n as usize;
        if nargs as usize != n {
            return raise(tools, &format!("takes {n} arguments, got {nargs}"));
        }
        let mut a = [0f64; 6];
        for at in 0..n {
            let value = (api.PyFloat_AsDouble)(*args.add(at));
            if value == -1.0 && !(api.PyErr_Occurred)().is_null() {
                return std::ptr::null_mut();
            }
            a[at] = value;
        }
        let run = std::panic::catch_unwind(|| match n {
            0 => std::mem::transmute::<usize, F0>(one.at)(),
            1 => std::mem::transmute::<usize, F1>(one.at)(a[0]),
            2 => std::mem::transmute::<usize, F2>(one.at)(a[0], a[1]),
            3 => std::mem::transmute::<usize, F3>(one.at)(a[0], a[1], a[2]),
            4 => std::mem::transmute::<usize, F4>(one.at)(a[0], a[1], a[2], a[3]),
            5 => std::mem::transmute::<usize, F5>(one.at)(a[0], a[1], a[2], a[3], a[4]),
            _ => std::mem::transmute::<usize, F6>(one.at)(a[0], a[1], a[2], a[3], a[4], a[5]),
        });
        let value = match run {
            Ok(value) => value,
            Err(payload) => return raise(tools, &crate::call::panic_text(payload)),
        };
        if value.is_nan() && failed() != 0 {
            let text = std::slice::from_raw_parts(out_ptr(), out_len());
            return raise(tools, &String::from_utf8_lossy(text));
        }
        match one.ret {
            0 => (api.PyFloat_FromDouble)(value),
            1 => (api.PyLong_FromDouble)(value),
            _ => (api.PyBool_FromLong)((value != 0.0) as c_long),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn init(
    doors: &'static [Function],
    error: Obj,
    array: Obj,
    wrap: Obj,
    keep: Obj,
    form: Obj,
    back: Obj,
    base: Obj,
) {
    let api = load();
    let known: Vec<Vec<Obj>> = shapes_of()
        .iter()
        .map(|fields| {
            fields
                .iter()
                .map(|field| {
                    let text = CString::new(*field).unwrap_or_default();
                    unsafe { (api.PyUnicode_InternFromString)(text.as_ptr()) }
                })
                .collect()
        })
        .collect();
    unsafe {
        for object in [error, array, wrap, keep, form, back, base] {
            (api.Py_IncRef)(object);
        }
    }
    let door_def = Box::into_raw(Box::new(MethodDef {
        name: c"door".as_ptr(),
        call: door_call as *const c_void,
        flags: FASTCALL,
        doc: std::ptr::null(),
    }));
    let direct_def = Box::into_raw(Box::new(MethodDef {
        name: c"direct".as_ptr(),
        call: direct_call as *const c_void,
        flags: FASTCALL,
        doc: std::ptr::null(),
    }));
    let _ = TOOLS.set(Tools {
        api,
        doors,
        error,
        array,
        wrap,
        keep,
        form,
        back,
        base,
        door_def,
        direct_def,
        known,
    });
}

pub fn door(door: usize, want: usize) -> Obj {
    let tools = tools();
    let api = &tools.api;
    unsafe {
        let me = (api.PyLong_FromLongLong)((door | (want << 16)) as i64);
        let made = (api.PyCMethod_New)(
            tools.door_def,
            me,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        (api.Py_DecRef)(me);
        made
    }
}

pub fn direct(at: usize, n: usize, ret: usize) -> Obj {
    let tools = tools();
    let api = &tools.api;
    unsafe {
        let quick = &mut *(&raw mut QUICK);
        quick.push(Quick {
            at,
            n: n as u8,
            ret: ret as u8,
        });
        let me = (api.PyLong_FromLongLong)(quick.len() as i64 - 1);
        let made = (api.PyCMethod_New)(
            tools.direct_def,
            me,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        (api.Py_DecRef)(me);
        made
    }
}

pub fn method(function: Obj) -> Obj {
    unsafe { (tools().api.PyInstanceMethod_New)(function) }
}
