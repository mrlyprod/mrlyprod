use mrlycore::{json, Json};
use mrlylab::ledger::{closed, keys, terms, Cost, Key, Tier, TERMS};
use std::sync::{Mutex, OnceLock};
use wasm_bindgen::prelude::*;

const CAP: usize = 48;
const CELLS: u128 = 100_000;
const WINDOW: i128 = 1_000;
const BLOCK: usize = 8;
const DEPTHS: [usize; 4] = [8, 16, 32, 48];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stop {
    Ceiling,
    Cap,
    Budget,
}

impl Stop {
    fn slug(self) -> &'static str {
        match self {
            Stop::Ceiling => "ceiling",
            Stop::Cap => "cap",
            Stop::Budget => "budget",
        }
    }
    fn slot(self) -> usize {
        match self {
            Stop::Ceiling => 0,
            Stop::Cap => 1,
            Stop::Budget => 2,
        }
    }
}

struct Row {
    key: Key,
    tier: Tier,
    stop: Stop,
    depth: usize,
    repeats: usize,
    low: usize,
    writes: Vec<(u16, u8)>,
}

struct Snap {
    depth: usize,
    never: usize,
    once: usize,
    many: usize,
    first: usize,
    run: usize,
    incidences: u64,
    deep: usize,
}

#[derive(Default)]
struct Sheet {
    listed: Vec<(Key, Tier)>,
    tiers: Vec<(Tier, usize)>,
    rows: Vec<Option<Row>>,
    built: usize,
    order: Vec<usize>,
    cursor: usize,
    stage: usize,
    counts: Vec<u32>,
    incidences: u64,
    pairs: u64,
    low: u64,
    stops: [usize; 3],
    blank: usize,
    unread: usize,
    snaps: Vec<Snap>,
}

fn sheet() -> &'static Mutex<Sheet> {
    static SHEET: OnceLock<Mutex<Sheet>> = OnceLock::new();
    SHEET.get_or_init(|| Mutex::new(Sheet::default()))
}

fn listing(sheet: &mut Sheet) {
    if !sheet.listed.is_empty() {
        return;
    }
    sheet.counts = vec![0; WINDOW as usize + 1];
    for tier in Tier::ALL {
        let batch = keys(tier);
        sheet.tiers.push((tier, batch.len()));
        sheet
            .listed
            .extend(batch.into_iter().map(|key| (key, tier)));
    }
    sheet.order = (0..sheet.listed.len()).collect();
    sheet.rows = (0..sheet.listed.len()).map(|_| None).collect();
}

fn footprint(key: &Key, index: usize) -> Option<u128> {
    let (number, level) = key.axis.place(index, key.number());
    let number = number as u128;
    let dimension = key.dimension as u32;
    match key.measure.cost() {
        Cost::Closed => Some(1),
        Cost::Convolved => {
            let tile = number.checked_pow(dimension)?;
            let side = number.checked_pow(level)?;
            let span = key.dimension as u128 * (side - 1) + 1;
            tile.checked_add(span.checked_mul(level as u128)?)
        }
        Cost::Grid => number.checked_pow(dimension.checked_mul(level)?),
    }
}

fn allowance(key: &Key) -> usize {
    (0..CAP)
        .take_while(|&index| footprint(key, index).is_some_and(|cells| cells <= CELLS))
        .count()
}

fn ceiling_stop(read: &[i128]) -> Option<usize> {
    let mut previous: Option<i128> = None;
    for (index, &term) in read.iter().enumerate() {
        if previous.is_some_and(|last| term <= last) {
            return None;
        }
        if term > WINDOW {
            return Some(index);
        }
        previous = Some(term);
    }
    None
}

fn gather(window: &[i128]) -> (Vec<(u16, u8)>, usize, usize) {
    let low = window.iter().filter(|&&term| term < 1).count();
    let mut writes: Vec<(u16, u8)> = Vec::new();
    let mut inside = 0;
    for (index, &term) in window.iter().enumerate() {
        if !(1..=WINDOW).contains(&term) {
            continue;
        }
        inside += 1;
        let value = term as u16;
        if let Err(slot) = writes.binary_search_by_key(&value, |&(term, _)| term) {
            writes.insert(slot, (value, index as u8));
        }
    }
    let repeats = inside - writes.len();
    (writes, repeats, low)
}

fn render(key: &Key, tier: Tier, depth: usize) -> Option<Row> {
    let full = allowance(key);
    let allowed = full.min(depth);
    let mut count = BLOCK.min(allowed);
    let window;
    let stop;
    loop {
        let (read, capped) = terms(key, count, CELLS).ok()?;
        let short = capped || read.len() < count;
        if let Some(edge) = ceiling_stop(&read) {
            window = read[..=edge].to_vec();
            stop = Stop::Ceiling;
            break;
        }
        if short {
            window = read;
            stop = Stop::Budget;
            break;
        }
        if count >= allowed {
            window = read;
            stop = if allowed < full || full == CAP {
                Stop::Cap
            } else {
                Stop::Budget
            };
            break;
        }
        count = (count * 2).min(allowed);
    }
    let (writes, repeats, low) = gather(&window);
    Some(Row {
        key: *key,
        tier,
        stop,
        depth: window.len(),
        repeats,
        low,
        writes,
    })
}

impl Sheet {
    fn add(&mut self, row: &Row, sign: i64) {
        for &(value, _) in &row.writes {
            let slot = &mut self.counts[value as usize];
            *slot = (*slot as i64 + sign) as u32;
        }
        let terms = row.writes.len() as i64;
        self.incidences = (self.incidences as i64 + sign * terms) as u64;
        self.pairs = (self.pairs as i64 + sign * (terms + row.repeats as i64)) as u64;
        self.low = (self.low as i64 + sign * row.low as i64) as u64;
        self.stops[row.stop.slot()] = (self.stops[row.stop.slot()] as i64 + sign) as usize;
        if row.writes.is_empty() {
            self.blank = (self.blank as i64 + sign) as usize;
        }
    }
    fn tally(&self) -> (usize, usize, usize) {
        let mut never = 0;
        let mut once = 0;
        let mut many = 0;
        for &count in &self.counts[1..] {
            match count {
                0 => never += 1,
                1 => once += 1,
                _ => many += 1,
            }
        }
        (never, once, many)
    }
    fn first_miss(&self) -> usize {
        self.counts[1..]
            .iter()
            .position(|&count| count == 0)
            .map_or(0, |index| index + 1)
    }
    fn longest_run(&self) -> usize {
        let mut best = 0;
        let mut run = 0;
        for &count in &self.counts[1..] {
            run = if count == 0 { 0 } else { run + 1 };
            best = best.max(run);
        }
        best
    }
    fn deepenable(&self) -> Vec<usize> {
        self.rows
            .iter()
            .enumerate()
            .filter(|(_, row)| row.as_ref().is_some_and(|row| row.stop == Stop::Cap))
            .map(|(index, _)| index)
            .collect()
    }
    fn snap(&mut self, depth: usize) {
        let (never, once, many) = self.tally();
        let deep = self.deepenable().len();
        self.snaps.push(Snap {
            depth,
            never,
            once,
            many,
            first: self.first_miss(),
            run: self.longest_run(),
            incidences: self.incidences,
            deep,
        });
    }
    fn bands(&self) -> Vec<Json> {
        let mut out = Vec::new();
        let mut first = 1usize;
        while first <= WINDOW as usize {
            let last = (first * 10 - 1).min(WINDOW as usize);
            let missed = self.counts[first..=last]
                .iter()
                .filter(|&&count| count == 0)
                .count();
            let width = last - first + 1;
            out.push(json!({
                "first": first,
                "last": last,
                "width": width,
                "missed": missed,
                "density": missed as f64 / width as f64,
            }));
            first *= 10;
        }
        out
    }
    fn coverage(&self) -> Vec<Json> {
        let width = WINDOW as usize + 1;
        let mut seen: Vec<Vec<bool>> = Tier::ALL.iter().map(|_| vec![false; width]).collect();
        let mut counted = [0usize; 4];
        for row in self.rows.iter().flatten() {
            let slot = Tier::ALL
                .iter()
                .position(|&tier| tier == row.tier)
                .unwrap_or(0);
            counted[slot] += 1;
            for &(value, _) in &row.writes {
                seen[slot][value as usize] = true;
            }
        }
        Tier::ALL
            .iter()
            .enumerate()
            .map(|(slot, tier)| {
                let written = seen[slot][1..].iter().filter(|&&hit| hit).count();
                let alone = (1..width)
                    .filter(|&value| {
                        seen[slot][value] && seen.iter().filter(|tier| tier[value]).count() == 1
                    })
                    .count();
                json!({
                    "tier": tier.slug(),
                    "rows": counted[slot],
                    "written": written,
                    "alone": alone,
                })
            })
            .collect()
    }
    fn depth(&self) -> usize {
        DEPTHS[self.stage.min(DEPTHS.len() - 1)]
    }
    fn shallow(&self) -> usize {
        self.snaps.last().map_or(0, |snap| snap.depth)
    }
}

fn head(row: &Row) -> Vec<String> {
    terms(&row.key, TERMS.min(row.depth), CELLS)
        .map(|(read, _)| read.iter().map(|term| term.to_string()).collect())
        .unwrap_or_default()
}

fn spell(row: &Row, value: u16) -> Json {
    let key = &row.key;
    let at = row
        .writes
        .binary_search_by_key(&value, |&(term, _)| term)
        .map_or(0, |slot| row.writes[slot].1);
    let side = key.axis.place(at as usize, key.number()).0;
    json!({
        "name": key.name(),
        "code": key.code.to_string(),
        "d": key.dimension,
        "q": key.base,
        "measure": key.measure.slug(),
        "axis": key.axis.slug(),
        "number": key.number(),
        "start": key.axis.start(),
        "tier": row.tier.slug(),
        "closed": closed(key).ok().flatten().map_or(String::new(), |form| form.text()),
        "stop": row.stop.slug(),
        "depth": row.depth,
        "index": at,
        "term": key.axis.start() + at as i32,
        "side": side,
        "writes": row.writes.len(),
        "head": head(row),
    })
}

fn snapshot(snap: &Snap) -> Json {
    json!({
        "depth": snap.depth,
        "never": snap.never,
        "once": snap.once,
        "multiple": snap.many,
        "written": snap.once + snap.many,
        "first_miss": snap.first,
        "run": snap.run,
        "incidences": snap.incidences,
        "deepenable": snap.deep,
    })
}

/// Prints the pinned window of the census: the term cap, the cells a term, the ceiling, the deepening passes and the registry the walk reads, as JSON.
#[wasm_bindgen]
pub fn census_window() -> String {
    let mut guard = sheet().lock().expect("the census sheet is not poisoned");
    listing(&mut guard);
    let tiers: Vec<Json> = guard
        .tiers
        .iter()
        .map(|(tier, count)| json!({ "tier": tier.slug(), "keys": count }))
        .collect();
    json!({
        "cap": CAP,
        "cells": CELLS.to_string(),
        "ceiling": WINDOW.to_string(),
        "head": TERMS,
        "depths": DEPTHS.to_vec(),
        "registry": guard.listed.len(),
        "tiers": tiers,
    })
    .to_string()
}

/// Walks the next span of rows at the pass's depth, deepening only the rows the last pass cut at the depth, and reports how far the walk is, as JSON.
#[wasm_bindgen]
pub fn census_walk(span: usize) -> String {
    let mut guard = sheet().lock().expect("the census sheet is not poisoned");
    listing(&mut guard);
    let depth = guard.depth();
    let stop = guard.order.len().min(guard.cursor.saturating_add(span));
    for slot in guard.cursor..stop {
        let at = guard.order[slot];
        let (key, tier) = guard.listed[at];
        let Some(row) = render(&key, tier, depth) else {
            guard.unread += 1;
            continue;
        };
        guard.add(&row, 1);
        if let Some(old) = guard.rows[at].replace(row) {
            guard.add(&old, -1);
        } else {
            guard.built += 1;
        }
    }
    guard.cursor = stop;
    let done = stop;
    let total = guard.order.len();
    if done == total {
        guard.snap(depth);
        if guard.stage + 1 < DEPTHS.len() {
            guard.stage += 1;
            guard.order = guard.deepenable();
            guard.cursor = 0;
        } else {
            guard.order = Vec::new();
            guard.cursor = 0;
        }
    }
    let (never, once, many) = guard.tally();
    json!({
        "depth": depth,
        "done": done,
        "total": total,
        "rows": guard.built,
        "pending": guard.order.len().saturating_sub(guard.cursor),
        "next": guard.depth(),
        "complete": guard.shallow() == CAP,
        "never": never,
        "once": once,
        "multiple": many,
    })
    .to_string()
}

/// Reads the census so far: the never, once and multiple counts of the window, the first miss, the longest written run, the incidences both ways, the truncation tally, the miss density by decade, the tier coverage and every completed depth, as JSON.
#[wasm_bindgen]
pub fn census_report() -> String {
    let guard = sheet().lock().expect("the census sheet is not poisoned");
    let (never, once, many) = guard.tally();
    let written = once + many;
    let depths: Vec<Json> = guard.snaps.iter().map(snapshot).collect();
    json!({
        "ceiling": WINDOW.to_string(),
        "depth": guard.shallow(),
        "rows": guard.built,
        "registry": guard.listed.len(),
        "never": never,
        "once": once,
        "multiple": many,
        "written": written,
        "share": written as f64 / WINDOW as f64,
        "first_miss": guard.first_miss(),
        "run": guard.longest_run(),
        "incidences": guard.incidences,
        "pairs": guard.pairs,
        "repeats": guard.pairs - guard.incidences,
        "low": guard.low,
        "ceiling_stopped": guard.stops[0],
        "cap_stopped": guard.stops[1],
        "budget_stopped": guard.stops[2],
        "blank": guard.blank,
        "unread": guard.unread,
        "bands": guard.bands(),
        "tiers": guard.coverage(),
        "depths": depths,
    })
    .to_string()
}

/// Counts the rows writing every integer of the window in turn, the first entry the integer one.
#[wasm_bindgen]
pub fn census_counts() -> Vec<u32> {
    let guard = sheet().lock().expect("the census sheet is not poisoned");
    guard.counts[1..].to_vec()
}

/// Lists the integers the most rows write, the heaviest first and the least integer ahead on a tie, as JSON.
#[wasm_bindgen]
pub fn census_champions(take: usize) -> String {
    let guard = sheet().lock().expect("the census sheet is not poisoned");
    let mut all: Vec<(usize, u32)> = guard
        .counts
        .iter()
        .enumerate()
        .skip(1)
        .map(|(value, &count)| (value, count))
        .collect();
    all.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(&right.0)));
    all.truncate(take);
    let rows: Vec<Json> = all
        .iter()
        .map(|&(value, count)| json!({ "value": value, "rows": count }))
        .collect();
    json!(rows).to_string()
}

/// Lists the least integers of the window no row writes, as JSON.
#[wasm_bindgen]
pub fn census_misses(take: usize) -> String {
    let guard = sheet().lock().expect("the census sheet is not poisoned");
    let rows: Vec<usize> = guard
        .counts
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, &count)| count == 0)
        .map(|(value, _)| value)
        .take(take)
        .collect();
    json!(rows).to_string()
}

/// Reads every registry row writing one integer: the count, the tally by tier, and one page of rows with the design, the measure, the closed form and the index the integer lands on, as JSON.
#[wasm_bindgen]
pub fn census_writers(value: usize, page: usize, rows: usize) -> String {
    let guard = sheet().lock().expect("the census sheet is not poisoned");
    let inside = (1..=WINDOW as usize).contains(&value);
    let wanted = value as u16;
    let found: Vec<&Row> = if inside {
        guard
            .rows
            .iter()
            .flatten()
            .filter(|row| {
                row.writes
                    .binary_search_by_key(&wanted, |&(term, _)| term)
                    .is_ok()
            })
            .collect()
    } else {
        Vec::new()
    };
    let tiers: Vec<Json> = Tier::ALL
        .iter()
        .map(|tier| {
            let count = found.iter().filter(|row| row.tier == *tier).count();
            json!({ "tier": tier.slug(), "rows": count })
        })
        .collect();
    let shown: Vec<Json> = found
        .iter()
        .skip(page * rows)
        .take(rows)
        .map(|row| spell(row, wanted))
        .collect();
    json!({
        "value": value,
        "inside": inside,
        "count": guard.counts.get(value).copied().unwrap_or(0),
        "rows": found.len(),
        "tiers": tiers,
        "shown": shown,
    })
    .to_string()
}
