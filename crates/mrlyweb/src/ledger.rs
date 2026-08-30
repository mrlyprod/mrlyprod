use crate::{checked, code_of, Fault};
use mrlycore::{json, Json};
use mrlylab::ledger::{
    self, identify, keys, numbers, search, sequence, Axis, Key, Measure, Sequence, Tier, BUDGET,
    RECORDS,
};
use mrlymath::bang::factory;
use mrlymath::formulas;
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct Shelf {
    rows: Vec<Sequence>,
    tiers: BTreeMap<Tier, (Vec<Key>, usize)>,
}

fn key(code: &str, dimension: usize, base: usize, measure: &str, axis: &str) -> Result<Key, Fault> {
    let code = checked(code, dimension, base)?;
    Ok(Key::new(
        code,
        dimension,
        base,
        Measure::parse(measure)?,
        Axis::parse(axis)?,
    ))
}

fn shelf() -> &'static Mutex<Shelf> {
    static SHELF: OnceLock<Mutex<Shelf>> = OnceLock::new();
    SHELF.get_or_init(|| Mutex::new(Shelf::default()))
}

fn grow(shelf: &mut Shelf, tier: Tier, count: usize, span: usize) -> (usize, usize) {
    let (keys, cursor) = shelf.tiers.entry(tier).or_insert_with(|| (keys(tier), 0));
    let stop = keys.len().min(cursor.saturating_add(span));
    for key in &keys[*cursor..stop] {
        if let Ok(row) = sequence(key, count, BUDGET) {
            shelf.rows.push(row);
        }
    }
    *cursor = stop;
    (stop, keys.len())
}

fn row(sequence: &Sequence) -> Json {
    let key = &sequence.key;
    json!({
        "name": key.name(),
        "code": key.code.to_string(),
        "d": key.dimension,
        "q": key.base,
        "measure": key.measure.slug(),
        "axis": key.axis.slug(),
        "number": key.number(),
        "start": key.axis.start(),
        "terms": sequence.terms.iter().map(|term| term.to_string()).collect::<Vec<String>>(),
        "capped": sequence.capped,
        "closed": sequence.closed.as_ref().map_or(String::new(), |form| form.text()),
        "oeis": sequence.record.map_or("", |(record, _)| record.id),
        "shift": sequence.record.map_or(0, |(_, shift)| shift),
        "tag": sequence.tag.map_or("", |tag| tag.text()),
    })
}

/// Names every measure the ledger reads.
#[wasm_bindgen]
pub fn ledger_measures() -> Vec<String> {
    Measure::ALL
        .iter()
        .map(|measure| measure.slug().to_string())
        .collect()
}

/// Lists the designs of a dimension and base, one code per orbit, as decimal strings.
#[wasm_bindgen]
pub fn ledger_designs(dimension: usize, base: usize) -> Result<Vec<String>, Fault> {
    Ok(ledger::designs(dimension, base)?
        .iter()
        .map(|code| code.to_string())
        .collect())
}

/// Builds one tier of the catalog into memory, once, and returns the rows the catalog holds.
#[wasm_bindgen]
pub fn ledger_build(tier: &str, count: usize) -> Result<usize, Fault> {
    let tier = Tier::parse(tier)?;
    let mut guard = shelf().lock().expect("the shelf is not poisoned");
    grow(&mut guard, tier, count, usize::MAX);
    Ok(guard.rows.len())
}

/// Builds the next span of keys of one tier into memory and reports the rows so far with the tier's keys done and in all, as JSON, so a page can build between frames.
#[wasm_bindgen]
pub fn ledger_grow(tier: &str, count: usize, span: usize) -> Result<String, Fault> {
    let tier = Tier::parse(tier)?;
    let mut guard = shelf().lock().expect("the shelf is not poisoned");
    let (done, total) = grow(&mut guard, tier, count, span);
    Ok(json!({ "rows": guard.rows.len(), "done": done, "total": total }).to_string())
}

/// Searches the catalog by a window of terms or a name fragment, narrowed by measure, dimension and base where given, one page of rows at a time, as JSON.
#[wasm_bindgen]
pub fn ledger_search(
    query: &str,
    measure: &str,
    dimension: usize,
    base: usize,
    page: usize,
    rows: usize,
) -> String {
    let guard = shelf().lock().expect("the shelf is not poisoned");
    let wanted = Measure::parse(measure).ok();
    let hits: Vec<&Sequence> = search(&guard.rows, query)
        .into_iter()
        .map(|index| &guard.rows[index])
        .filter(|sequence| {
            let key = &sequence.key;
            wanted.is_none_or(|measure| key.measure == measure)
                && (dimension == 0 || key.dimension == dimension)
                && (base == 0 || key.base == base)
        })
        .collect();
    let shown: Vec<Json> = hits
        .iter()
        .skip(page * rows)
        .take(rows)
        .map(|sequence| row(sequence))
        .collect();
    json!({ "total": hits.len(), "rows": shown }).to_string()
}

/// Reads the first terms of a design sequence within a cell budget, as decimal strings, fewer than asked when the budget or a u128 stops them.
#[wasm_bindgen]
pub fn ledger_terms(
    code: &str,
    dimension: usize,
    base: usize,
    measure: &str,
    axis: &str,
    count: usize,
    cells: &str,
) -> Result<Vec<String>, Fault> {
    let key = key(code, dimension, base, measure, axis)?;
    let (terms, _) = ledger::terms(&key, count, code_of(cells)?)?;
    Ok(terms.iter().map(|term| term.to_string()).collect())
}

/// Reads one design sequence of any code the space accepts as a catalog row within a cell budget, as JSON.
#[wasm_bindgen]
pub fn ledger_row(
    code: &str,
    dimension: usize,
    base: usize,
    measure: &str,
    axis: &str,
    count: usize,
    cells: &str,
) -> Result<String, Fault> {
    let key = key(code, dimension, base, measure, axis)?;
    Ok(row(&sequence(&key, count, code_of(cells)?)?).to_string())
}

/// Counts the filled cells on every diagonal plane of a design at the side and level, as decimal strings: the strip itself in dimension one.
#[wasm_bindgen]
pub fn ledger_profile(
    code: &str,
    dimension: usize,
    base: usize,
    number: usize,
    level: u32,
) -> Result<Vec<String>, Fault> {
    let tile = factory::create(checked(code, dimension, base)?, number, dimension, base, 1)?;
    Ok(formulas::profile_of_tile(&tile, level)?
        .iter()
        .map(|count| count.to_string())
        .collect())
}

/// Finds the records holding the typed terms as a window, each with the record's index of the first typed term, as JSON.
#[wasm_bindgen]
pub fn ledger_identify(terms: &str) -> String {
    let found: Vec<Json> = numbers(terms)
        .map(|window| identify(&window))
        .unwrap_or_default()
        .iter()
        .map(|(record, shift)| {
            json!({
                "id": record.id,
                "name": record.name,
                "offset": record.offset,
                "shift": shift,
                "terms": record.terms,
            })
        })
        .collect();
    json!(found).to_string()
}

/// Lists the curated records: id, name, offset, first terms, status, formula, witness, and the key and shift where the entry names a design sequence, as JSON.
#[wasm_bindgen]
pub fn ledger_records() -> String {
    let rows: Vec<Json> = RECORDS
        .iter()
        .map(|record| {
            json!({
                "id": record.id,
                "name": record.name,
                "offset": record.offset,
                "terms": record.terms,
                "status": record.status.text(),
                "formula": record.formula,
                "witness": record.witness,
                "key": record.key.map_or(String::new(), |key| key.name()),
                "shift": record.shift,
            })
        })
        .collect();
    json!(rows).to_string()
}

/// Spells the closed form of a design sequence, or an empty string when the ledger knows none.
#[wasm_bindgen]
pub fn ledger_closed(
    code: &str,
    dimension: usize,
    base: usize,
    measure: &str,
    axis: &str,
) -> Result<String, Fault> {
    let key = key(code, dimension, base, measure, axis)?;
    Ok(ledger::closed(&key)?.map_or(String::new(), |form| form.text()))
}
