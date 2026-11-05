use crate::args::Arg;
use crate::call::{Error, bad};
use crate::*;
use std::any::Any;
use std::cell::{RefCell, UnsafeCell};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::{Arc, Mutex};

const INDEX_BITS: u32 = 20;
const INDEX_MASK: u32 = (1 << INDEX_BITS) - 1;

struct Slot {
    value: UnsafeCell<Box<dyn Any>>,
    borrows: AtomicIsize,
}

unsafe impl Send for Slot {}
unsafe impl Sync for Slot {}

struct Entry {
    generation: u32,
    slot: Option<Arc<Slot>>,
}

struct Table {
    entries: Vec<Entry>,
    open: Vec<u32>,
    start: u32,
}

static HANDLES: Mutex<Table> = Mutex::new(Table {
    entries: Vec::new(),
    open: Vec::new(),
    start: 0,
});

struct Parked(#[allow(dead_code)] Box<dyn Any>);

unsafe impl Send for Parked {}

static KEPT: Mutex<Vec<Option<Parked>>> = Mutex::new(Vec::new());

thread_local! {
    static PARK: RefCell<Vec<Box<dyn Any>>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn park(value: Box<dyn Any>) {
    PARK.with_borrow_mut(|park| park.push(value));
}

pub(crate) fn clear_park() {
    PARK.with_borrow_mut(Vec::clear);
}

pub fn parked() -> usize {
    PARK.with_borrow(Vec::len)
}

pub fn seed(epoch: u32) {
    HANDLES.lock().unwrap_or_else(|e| e.into_inner()).start = epoch.wrapping_mul(97) & 0xfff;
}

#[inline(never)]
pub(crate) fn store(value: Box<dyn Any>) -> u32 {
    let mut table = HANDLES.lock().unwrap_or_else(|e| e.into_inner());
    if table.entries.is_empty() {
        let start = table.start;
        table.entries.push(Entry {
            generation: start,
            slot: None,
        });
    }
    let slot = Some(Arc::new(Slot {
        value: UnsafeCell::new(value),
        borrows: AtomicIsize::new(0),
    }));
    let index = match table.open.pop() {
        Some(index) => {
            table.entries[index as usize].slot = slot;
            index
        }
        None => {
            let generation = table.start;
            table.entries.push(Entry { generation, slot });
            table.entries.len() as u32 - 1
        }
    };
    index | (table.entries[index as usize].generation << INDEX_BITS)
}

pub fn drop_handle(id: u32) {
    let mut table = HANDLES.lock().unwrap_or_else(|e| e.into_inner());
    let index = id & INDEX_MASK;
    let Some(entry) = table.entries.get_mut(index as usize) else {
        return;
    };
    if entry.generation != id >> INDEX_BITS || entry.slot.is_none() {
        return;
    }
    entry.slot = None;
    entry.generation = (entry.generation + 1) & 0xfff;
    table.open.push(index);
}

pub struct Held<T> {
    at: *mut T,
    slot: Option<Arc<Slot>>,
    mine: bool,
    own: Option<Box<T>>,
}

impl<T> Deref for Held<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*self.at }
    }
}

impl<T> DerefMut for Held<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.at }
    }
}

impl<T> Drop for Held<T> {
    fn drop(&mut self) {
        if let Some(slot) = &self.slot {
            if self.mine {
                slot.borrows.store(0, Ordering::Release);
            } else {
                slot.borrows.fetch_sub(1, Ordering::Release);
            }
        }
    }
}

impl<T: Clone> Held<T> {
    pub fn owned(mut self) -> T {
        match self.own.take() {
            Some(own) => *own,
            None => (*self).clone(),
        }
    }
}

#[inline(never)]
fn find<T: 'static>(name: &str, at: usize, id: u32, mine: bool) -> Result<Held<T>, Error> {
    let slot = {
        let table = HANDLES.lock().unwrap_or_else(|e| e.into_inner());
        table
            .entries
            .get((id & INDEX_MASK) as usize)
            .filter(|entry| entry.generation == id >> INDEX_BITS)
            .and_then(|entry| entry.slot.clone())
    }
    .ok_or_else(|| bad(name, at, "this object was already freed"))?;
    let taken = if mine {
        slot.borrows
            .compare_exchange(0, -1, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    } else {
        let before = slot.borrows.fetch_add(1, Ordering::Acquire);
        if before < 0 {
            slot.borrows.fetch_sub(1, Ordering::Release);
            false
        } else {
            true
        }
    };
    if !taken {
        return Err(bad(name, at, "this object is in use by another call"));
    }
    let value = unsafe { &mut *slot.value.get() };
    let Some(value) = value.downcast_mut::<T>() else {
        if mine {
            slot.borrows.store(0, Ordering::Release);
        } else {
            slot.borrows.fetch_sub(1, Ordering::Release);
        }
        return Err(bad(name, at, "an object of another class"));
    };
    let at = value as *mut T;
    Ok(Held {
        at,
        slot: Some(slot),
        mine,
        own: None,
    })
}

fn handle_in(value: &Arg) -> Option<u32> {
    let b = value.bytes();
    (b.first() == Some(&HANDLE)).then(|| u32::from_le_bytes([b[1], b[2], b[3], b[4]]))
}

#[inline]
pub fn bare<T>(value: T) -> Held<T> {
    let mut own = Box::new(value);
    let at = &mut *own as *mut T;
    Held {
        at,
        slot: None,
        mine: false,
        own: Some(own),
    }
}

#[inline]
pub fn held<T: 'static>(name: &str, at: usize, value: &Arg) -> Result<Option<Held<T>>, Error> {
    match handle_in(value) {
        Some(id) => find(name, at, id, false).map(Some),
        None => Ok(None),
    }
}

#[inline]
pub fn held_mut<T: 'static>(name: &str, at: usize, value: &Arg) -> Result<Option<Held<T>>, Error> {
    match handle_in(value) {
        Some(id) => find(name, at, id, true).map(Some),
        None => Ok(None),
    }
}

#[inline]
pub fn held_id<T: 'static>(id: u32) -> Result<Held<T>, Error> {
    find("", 0, id, false)
}

#[inline]
pub fn held_mut_id<T: 'static>(id: u32) -> Result<Held<T>, Error> {
    find("", 0, id, true)
}

pub fn keep_result() -> u32 {
    let parked: Vec<Box<dyn Any>> = PARK.with_borrow_mut(std::mem::take);
    let mut kept = KEPT.lock().unwrap_or_else(|e| e.into_inner());
    let value = Some(Parked(Box::new(parked)));
    match kept.iter().position(Option::is_none) {
        Some(at) => {
            kept[at] = value;
            at as u32
        }
        None => {
            kept.push(value);
            kept.len() as u32 - 1
        }
    }
}

pub fn release(id: u32) {
    let mut kept = KEPT.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(slot) = kept.get_mut(id as usize) {
        *slot = None;
    }
}
