mod args;
mod blob;
mod call;
mod de;
mod held;
mod out;
#[cfg(not(target_arch = "wasm32"))]
pub mod py;
mod ser;

pub use args::{Arg, Args, split};
pub use blob::{Blob, Prim, Raw, flat_rgba, inbox, take_blob, write_ref};
#[cfg(not(target_arch = "wasm32"))]
pub use call::run_host;
pub use call::{
    Door, Error, Function, bad, buf, call, direct_fail, failed, install, library, on, out_len,
    out_ptr, panic_len, panic_ptr, seed, usage,
};
pub use de::{WireDe, take, take_list};
pub use held::{
    Held, bare, drop_handle, held, held_id, held_mut, held_mut_id, keep_result, parked, release,
};
pub use out::Out;
pub use ser::{Keys, Ser, form, shapes_of};

pub const NUL: u8 = 0x00;
pub const FALSE: u8 = 0x01;
pub const TRUE: u8 = 0x02;
pub const I32: u8 = 0x03;
pub const F32: u8 = 0x04;
pub const F64: u8 = 0x05;
pub const I53: u8 = 0x06;
pub const STR: u8 = 0x07;
pub const ARR: u8 = 0x08;
pub const MAP: u8 = 0x09;
pub const SHP: u8 = 0x0a;
pub const SYM: u8 = 0x0b;
pub const BLOB_OUT: u8 = 0x0c;
pub const BLOB_IN: u8 = 0x0d;
pub const HANDLE: u8 = 0x0e;
pub const BIG: u8 = 0x0f;
pub const VAR: u8 = 0x10;
pub const FORM: u8 = 0x11;
pub const WIDE: u8 = 0x12;
pub const BACK: u8 = 0x13;
pub const BIG64: u8 = 0x14;
pub const UBIG64: u8 = 0x15;
pub const INLINE: u8 = 0x16;
pub const SMALL: u8 = 0x80;

pub const SAFE: u64 = 9007199254740991;

#[macro_export]
macro_rules! export {
    ($doors:path, $shapes:path) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_buf(n: usize) -> *mut u8 {
            $crate::buf(n)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_in(kind: u32, n: usize) -> *mut u8 {
            $crate::inbox(kind, n)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_call(door: i32, n: usize) -> i32 {
            $crate::install($shapes);
            $crate::call($doors, door, n)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_out_ptr() -> *const u8 {
            $crate::out_ptr()
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_out_len() -> usize {
            $crate::out_len()
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_drop(id: u32) {
            $crate::drop_handle(id)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_keep() -> u32 {
            $crate::keep_result()
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_release(id: u32) {
            $crate::release(id)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_failed() -> i32 {
            $crate::failed()
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_seed(epoch: u32) {
            $crate::seed(epoch)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_panic_ptr() -> *const u8 {
            $crate::panic_ptr()
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_panic_len() -> usize {
            $crate::panic_len()
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_py_init(
            error: *mut std::ffi::c_void,
            array: *mut std::ffi::c_void,
            wrap: *mut std::ffi::c_void,
            keep: *mut std::ffi::c_void,
            form: *mut std::ffi::c_void,
            back: *mut std::ffi::c_void,
            base: *mut std::ffi::c_void,
        ) {
            $crate::install($shapes);
            $crate::py::init($doors, error, array, wrap, keep, form, back, base)
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_py_door(door: usize, want: usize) -> *mut std::ffi::c_void {
            $crate::py::door(door, want)
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_py_direct(at: usize, n: usize, ret: usize) -> *mut std::ffi::c_void {
            $crate::py::direct(at, n, ret)
        }
        #[cfg(not(target_arch = "wasm32"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn mrly_py_method(function: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
            $crate::py::method(function)
        }
    };
}
