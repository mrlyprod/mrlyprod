#![doc = include_str!("../README.md")]

use serde_json::Value;
use std::fmt;

// PLUMBING

type Done = Result<Value, Error>;

type Call = fn(&str, &[Value]) -> Done;

macro_rules! take {
    ($name:expr, $at:expr, $value:expr) => {
        match serde_json::from_value(Value::clone($value)) {
            Ok(value) => value,
            Err(error) => return Err(bad($name, $at, &error.to_string())),
        }
    };
}

macro_rules! give {
    ($value:expr) => {
        match serde_json::to_value($value) {
            Ok(value) => value,
            Err(error) => return Err(Error::Library(error.to_string())),
        }
    };
}

#[path = "doors.gen.rs"]
mod doors;

/// One function of mrly as the command line sees it.
pub struct Function {
    /// The dotted path after `mrly::`, as `mrly list` prints it: `num.factor.gcd`.
    pub name: &'static str,
    /// The parameters and the return, in the type names `mrly list` prints.
    pub signature: &'static str,
    /// The doc comment as plain text, empty when there is none.
    pub docs: &'static str,
    /// Why the command line cannot call it, when it cannot.
    pub uncallable: Option<&'static str>,
    door: Option<Call>,
}

impl Function {
    /// Whether `call` can run this function.
    pub fn callable(&self) -> bool {
        self.door.is_some()
    }

    /// Runs this function with one JSON value per parameter and returns its result as JSON.
    pub fn call(&self, args: &[Value]) -> Result<Value, Error> {
        match self.door {
            Some(door) => door(self.name, args),
            None => Err(Error::Usage(format!(
                "{} cannot be called from the command line: {}",
                self.name,
                self.uncallable.unwrap_or("its types do not cross as JSON")
            ))),
        }
    }
}

/// Why a call returned no value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The call was written wrong: an unknown name, the wrong number of arguments, or an argument of the wrong shape.
    Usage(String),
    /// The function ran and returned an error.
    Library(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usage(message) | Error::Library(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for Error {}

/// Calls the function `name` with one JSON value per parameter, and returns its result as JSON.
pub fn call(name: &str, args: &[Value]) -> Result<Value, Error> {
    find(name)?.call(args)
}

/// Every function, sorted by name.
pub fn list() -> impl Iterator<Item = &'static Function> {
    doors::DOORS.iter()
}

/// The function named `name`, or a usage error that suggests close names.
pub fn find(name: &str) -> Result<&'static Function, Error> {
    doors::DOORS
        .binary_search_by_key(&name, |function| function.name)
        .map(|at| &doors::DOORS[at])
        .map_err(|_| Error::Usage(unknown(name)))
}

/// Up to five names close to `name`, closest first: the same last word, or a few typed letters off.
pub fn near(name: &str) -> Vec<&'static str> {
    let want = name.to_lowercase();
    let word = last(&want);
    let mut scored: Vec<(usize, &'static str)> = list()
        .filter_map(|function| {
            let have = function.name.to_lowercase();
            let whole = distance(&want, &have);
            let tail = distance(word, last(&have)) + usize::from(want.contains('.'));
            let score = whole.min(tail + 1);
            (score <= 2.max(word.len() / 3)).then_some((score, function.name))
        })
        .collect();
    scored.sort();
    scored.into_iter().take(5).map(|(_, name)| name).collect()
}

fn unknown(name: &str) -> String {
    let close = near(name);
    if close.is_empty() {
        format!("no function named {name:?}; mrly list prints them all")
    } else {
        format!(
            "no function named {name:?}; did you mean {}?",
            close.join(", ")
        )
    }
}

fn last(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, x) in a.chars().enumerate() {
        let mut corner = row[0];
        row[0] = i + 1;
        for (j, y) in b.iter().enumerate() {
            let above = row[j + 1];
            row[j + 1] = (corner + usize::from(x != *y))
                .min(row[j] + 1)
                .min(above + 1);
            corner = above;
        }
    }
    row[b.len()]
}

// READING

fn usage(what: String) -> Error {
    Error::Usage(what)
}

fn bad(name: &str, at: usize, why: &str) -> Error {
    usage(format!("{name} argument {}: {why}", at + 1))
}

fn count(name: &str, args: &[Value], want: usize) -> Result<(), Error> {
    if args.len() == want {
        return Ok(());
    }
    let noun = if want == 1 { "argument" } else { "arguments" };
    Err(usage(format!(
        "{name} takes {want} {noun}, got {}",
        args.len()
    )))
}

fn slot<'a>(name: &str, args: &'a [Value], at: usize) -> Result<&'a Value, Error> {
    args.get(at)
        .ok_or_else(|| usage(format!("{name} wants an argument {}", at + 1)))
}

fn big(name: &str, at: usize, value: &Value) -> Result<u128, Error> {
    let read = match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_u64().map(u128::from),
    };
    read.ok_or_else(|| bad(name, at, "a u128 is a decimal string"))
}

fn small(name: &str, at: usize, value: &Value) -> Result<i128, Error> {
    let read = match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_i64().map(i128::from),
    };
    read.ok_or_else(|| bad(name, at, "an i128 is a decimal string"))
}

fn seed(name: &str, at: usize, value: &Value) -> Result<u64, Error> {
    value
        .as_u64()
        .ok_or_else(|| bad(name, at, "an Rng is a seed number"))
}

fn color(name: &str, at: usize, value: &Value) -> Result<mrly::core::Color, Error> {
    let bytes: [u8; 4] = match serde_json::from_value(value.clone()) {
        Ok(bytes) => bytes,
        Err(_) => return Err(bad(name, at, "a Color is four bytes, [r, g, b, a]")),
    };
    Ok(mrly::core::Color {
        r: bytes[0],
        g: bytes[1],
        b: bytes[2],
        a: bytes[3],
    })
}

fn items<'a>(name: &str, at: usize, value: &'a Value) -> Result<&'a [Value], Error> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| bad(name, at, "a JSON array"))
}

fn parts<'a>(name: &str, at: usize, value: &'a Value, want: usize) -> Result<&'a [Value], Error> {
    match value.as_array() {
        Some(list) if list.len() == want => Ok(list),
        _ => Err(bad(name, at, &format!("a JSON array of {want}"))),
    }
}

fn fields<'a>(
    name: &str,
    at: usize,
    value: &'a Value,
) -> Result<&'a serde_json::Map<String, Value>, Error> {
    value
        .as_object()
        .ok_or_else(|| bad(name, at, "a JSON object"))
}

fn axes(value: &Value) -> Option<usize> {
    Some(value.get("shape")?.as_array()?.len())
}

fn walls(value: &Value) -> Option<usize> {
    axes(value.get("cell")?.get("types")?)
}

fn tensor_rank(name: &str, args: &[Value], at: usize) -> Result<usize, Error> {
    axes(slot(name, args, at)?).ok_or_else(|| bad(name, at, "a Tensor with a shape"))
}

fn cell_rank(name: &str, args: &[Value], at: usize) -> Result<usize, Error> {
    walls(slot(name, args, at)?).ok_or_else(|| bad(name, at, "a Cell2d or Cell3d"))
}

fn cells_rank(name: &str, args: &[Value], at: usize) -> Result<usize, Error> {
    slot(name, args, at)?
        .as_array()
        .and_then(|list| list.first())
        .and_then(walls)
        .ok_or_else(|| bad(name, at, "a list of Cell2d or Cell3d"))
}

// WRITING

fn tint(color: mrly::core::Color) -> Value {
    Value::Array(vec![
        color.r.into(),
        color.g.into(),
        color.b.into(),
        color.a.into(),
    ])
}
