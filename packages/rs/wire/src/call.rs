use crate::args::{Args, split};
use crate::blob::clear_inbox;
use crate::held::clear_park;
use crate::out::Out;
use crate::ser::{KEYS, set_known};
use std::cell::{Cell, RefCell};
use std::fmt::{self, Display};

pub type Door = fn(&str, &Args, &mut Out) -> Result<(), Error>;

pub struct Function {
    #[cfg(not(target_arch = "wasm32"))]
    pub name: &'static str,
    #[cfg(not(target_arch = "wasm32"))]
    pub signature: &'static str,
    #[cfg(not(target_arch = "wasm32"))]
    pub docs: &'static str,
    pub door: Door,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Usage(String),
    Library(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usage(message) | Error::Library(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Error {}

#[inline(never)]
pub fn library<E: Display>(error: E) -> Error {
    Error::Library(error.to_string())
}

#[inline(never)]
pub fn usage(what: String) -> Error {
    Error::Usage(what)
}

#[inline(never)]
pub fn bad(name: &str, at: usize, why: &str) -> Error {
    Error::Usage(format!("{name} argument {}: {why}", at + 1))
}

static INSTALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

thread_local! {
    static HOST: Cell<bool> = const { Cell::new(true) };
    static BORROW: Cell<bool> = const { Cell::new(false) };
    static ARGS: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    pub(crate) static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static FAILED: Cell<i32> = const { Cell::new(0) };
    static PANIC: RefCell<String> = const { RefCell::new(String::new()) };
}

#[cfg(target_arch = "wasm32")]
#[inline(always)]
pub const fn on() -> bool {
    true
}

#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub fn on() -> bool {
    HOST.get()
}

#[inline(always)]
pub(crate) fn on_borrow() -> bool {
    BORROW.get()
}

#[inline(always)]
pub(crate) fn set_borrow(value: bool) {
    BORROW.set(value);
}

pub fn install(shapes: &'static [&'static [&'static str]]) {
    if !INSTALLED.swap(true, std::sync::atomic::Ordering::Relaxed) {
        set_known(shapes);
        crate::ser::ready_keys();
        hook();
    }
}

#[cfg(target_arch = "wasm32")]
fn hook() {
    std::panic::set_hook(Box::new(|info| {
        PANIC.with_borrow_mut(|text| *text = info.to_string());
    }));
}

#[cfg(not(target_arch = "wasm32"))]
fn hook() {}

pub fn panic_ptr() -> *const u8 {
    PANIC.with_borrow(|text| text.as_ptr())
}

pub fn panic_len() -> usize {
    PANIC.with_borrow(String::len)
}

pub use crate::held::seed;

pub fn buf(n: usize) -> *mut u8 {
    ARGS.with_borrow_mut(|args| {
        if args.len() < n {
            args.resize(n, 0);
        }
        args.as_mut_ptr()
    })
}

pub fn out_ptr() -> *const u8 {
    OUT.with_borrow(|out| out.as_ptr())
}

pub fn out_len() -> usize {
    OUT.with_borrow(Vec::len)
}

fn say(status: i32, message: &str) -> i32 {
    OUT.with_borrow_mut(|out| {
        out.clear();
        out.extend_from_slice(message.as_bytes());
    });
    status
}

#[inline(never)]
pub fn direct_fail(error: Error) -> f64 {
    let (code, message) = match error {
        Error::Usage(message) => (1, message),
        Error::Library(message) => (2, message),
    };
    say(code, &message);
    FAILED.set(code);
    f64::NAN
}

pub fn failed() -> i32 {
    FAILED.replace(0)
}

fn enter(door: Door, name: &str, args: &Args) -> i32 {
    let mut bytes = OUT.with_borrow_mut(std::mem::take);
    bytes.clear();
    bytes.extend_from_slice(&[0, 0, 0, 0]);
    let mut keys = KEYS.with_borrow_mut(|keys| std::mem::replace(keys, crate::ser::Keys::empty()));
    let done = {
        let mut out = Out {
            buf: &mut bytes,
            keys: &mut keys,
        };
        door(name, args, &mut out)
    };
    keys.flush_if(&mut bytes, done.is_ok());
    KEYS.with_borrow_mut(|at| *at = keys);
    OUT.with_borrow_mut(|out| *out = bytes);
    match done {
        Ok(()) => 0,
        Err(Error::Usage(message)) => say(1, &message),
        Err(Error::Library(message)) => say(2, &message),
    }
}

pub fn call(doors: &'static [Function], door: i32, n: usize) -> i32 {
    clear_park();
    let function = &doors[door as usize];
    let args_bytes = ARGS.with_borrow_mut(std::mem::take);
    let status = {
        let mut parts: [&[u8]; 24] = [&[]; 24];
        let count = split(&args_bytes[..n], &mut parts);
        #[cfg(target_arch = "wasm32")]
        let name = "";
        #[cfg(not(target_arch = "wasm32"))]
        let name = function.name;
        enter(function.door, name, &Args::new(&parts[..count]))
    };
    ARGS.with_borrow_mut(|args| *args = args_bytes);
    clear_inbox();
    if status != 0 {
        return -status;
    }
    OUT.with_borrow(|out| out.as_ptr() as i32)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_host(function: &'static Function, parts: &[&[u8]], host: bool) -> i32 {
    HOST.set(host);
    crate::ser::ready_keys();
    clear_park();
    let args = Args::new(parts);
    let status = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        enter(function.door, function.name, &args)
    })) {
        Ok(status) => status,
        Err(payload) => {
            KEYS.with_borrow_mut(crate::ser::Keys::restart);
            say(3, &panic_text(payload))
        }
    };
    clear_inbox();
    status
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn panic_text(payload: Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|text| text.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "a Rust panic".to_string())
}
