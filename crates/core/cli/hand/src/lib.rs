pub use serde_json::{self, Value};
use std::fmt;
use std::io::{self, IsTerminal, Read, Write};
use std::process::ExitCode;

// PLUMBING

pub type Done = Result<Value, Error>;

pub type Call = fn(&str, &[Value]) -> Done;

#[macro_export]
macro_rules! take {
    ($name:expr, $at:expr, $value:expr) => {
        match $crate::serde_json::from_value($crate::Value::clone($value)) {
            Ok(value) => value,
            Err(error) => return Err($crate::bad($name, $at, &error.to_string())),
        }
    };
}

#[macro_export]
macro_rules! give {
    ($value:expr) => {
        match $crate::serde_json::to_value($value) {
            Ok(value) => value,
            Err(error) => return Err($crate::Error::Library(error.to_string())),
        }
    };
}

pub struct Function {
    pub name: &'static str,
    pub signature: &'static str,
    pub docs: &'static str,
    pub uncallable: Option<&'static str>,
    pub door: Option<Call>,
}

impl Function {
    pub fn callable(&self) -> bool {
        self.door.is_some()
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Usage(String),
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

// BOOK

#[derive(Clone, Copy)]
pub struct Book {
    pub bin: &'static str,
    pub doors: &'static [Function],
}

impl Book {
    pub fn call(&self, name: &str, args: &[Value]) -> Result<Value, Error> {
        self.find(name)?.call(args)
    }

    pub fn find(&self, name: &str) -> Result<&'static Function, Error> {
        self.doors
            .binary_search_by_key(&name, |function| function.name)
            .map(|at| &self.doors[at])
            .map_err(|_| Error::Usage(self.unknown(name)))
    }

    pub fn near(&self, name: &str) -> Vec<&'static str> {
        let want = name.to_lowercase();
        let word = last(&want);
        let mut scored: Vec<(usize, &'static str)> = self
            .doors
            .iter()
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

    fn unknown(&self, name: &str) -> String {
        let close = self.near(name);
        if close.is_empty() {
            format!(
                "no function named {name:?}; {} list prints them all",
                self.bin
            )
        } else {
            format!(
                "no function named {name:?}; did you mean {}?",
                close.join(", ")
            )
        }
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

pub fn usage(what: String) -> Error {
    Error::Usage(what)
}

pub fn bad(name: &str, at: usize, why: &str) -> Error {
    usage(format!("{name} argument {}: {why}", at + 1))
}

pub fn count(name: &str, args: &[Value], want: usize) -> Result<(), Error> {
    if args.len() == want {
        return Ok(());
    }
    let noun = if want == 1 { "argument" } else { "arguments" };
    Err(usage(format!(
        "{name} takes {want} {noun}, got {}",
        args.len()
    )))
}

pub fn slot<'a>(name: &str, args: &'a [Value], at: usize) -> Result<&'a Value, Error> {
    args.get(at)
        .ok_or_else(|| usage(format!("{name} wants an argument {}", at + 1)))
}

pub fn big(name: &str, at: usize, value: &Value) -> Result<u128, Error> {
    let read = match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_u64().map(u128::from),
    };
    read.ok_or_else(|| bad(name, at, "a u128 is a decimal string"))
}

pub fn small(name: &str, at: usize, value: &Value) -> Result<i128, Error> {
    let read = match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_i64().map(i128::from),
    };
    read.ok_or_else(|| bad(name, at, "an i128 is a decimal string"))
}

pub fn seed(name: &str, at: usize, value: &Value) -> Result<u64, Error> {
    value
        .as_u64()
        .ok_or_else(|| bad(name, at, "an Rng is a seed number"))
}

pub fn color(name: &str, at: usize, value: &Value) -> Result<mrly_core::Color, Error> {
    let bytes: [u8; 4] = match serde_json::from_value(value.clone()) {
        Ok(bytes) => bytes,
        Err(_) => return Err(bad(name, at, "a Color is four bytes, [r, g, b, a]")),
    };
    Ok(mrly_core::Color {
        r: bytes[0],
        g: bytes[1],
        b: bytes[2],
        a: bytes[3],
    })
}

pub fn items<'a>(name: &str, at: usize, value: &'a Value) -> Result<&'a [Value], Error> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| bad(name, at, "a JSON array"))
}

pub fn parts<'a>(
    name: &str,
    at: usize,
    value: &'a Value,
    want: usize,
) -> Result<&'a [Value], Error> {
    match value.as_array() {
        Some(list) if list.len() == want => Ok(list),
        _ => Err(bad(name, at, &format!("a JSON array of {want}"))),
    }
}

pub fn fields<'a>(
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

pub fn tensor_rank(name: &str, args: &[Value], at: usize) -> Result<usize, Error> {
    axes(slot(name, args, at)?).ok_or_else(|| bad(name, at, "a Tensor with a shape"))
}

pub fn cell_rank(name: &str, args: &[Value], at: usize) -> Result<usize, Error> {
    walls(slot(name, args, at)?).ok_or_else(|| bad(name, at, "a Cell2d or Cell3d"))
}

pub fn cells_rank(name: &str, args: &[Value], at: usize) -> Result<usize, Error> {
    slot(name, args, at)?
        .as_array()
        .and_then(|list| list.first())
        .and_then(walls)
        .ok_or_else(|| bad(name, at, "a list of Cell2d or Cell3d"))
}

// WRITING

pub fn tint(color: mrly_core::Color) -> Value {
    Value::Array(vec![
        color.r.into(),
        color.g.into(),
        color.b.into(),
        color.a.into(),
    ])
}

// COMMAND

const HELP: &str = r#"BIN: every public function of its crate from the shell, JSON in, JSON out.

usage:
  BIN list [prefix]       every function with its signature, one per line
  BIN help <name>         one function's signature and docs
  BIN <name> '<json>'     call a function and print its result as JSON
  BIN <name> [-]          the same, reading the JSON from stdin
  BIN <name> -- '<json>'  the same, for JSON that starts with a dash
  BIN --version           the version

options:
  -r, --raw               print a string result as it is, not as a JSON string
  -h, --help              this page

arguments:
  a JSON array holds one value per parameter, in order: '[3, 1]'
  any other JSON value is the one argument, so one call's result can pipe into the next
  an optional parameter takes null; a Color is [r, g, b, a]
  Rng(seed) is a seed number; u128, i128 and Code are decimal strings

exit codes:
  0  the call ran and its result is on stdout
  1  the function returned an error; the message is on stderr
  2  the command was written wrong; the message is on stderr
"#;

enum Stop {
    Quiet,
    Usage(String),
    Library(String),
}

impl From<io::Error> for Stop {
    fn from(error: io::Error) -> Stop {
        match error.kind() {
            io::ErrorKind::BrokenPipe => Stop::Quiet,
            _ => Stop::Library(error.to_string()),
        }
    }
}

fn stop(error: Error) -> Stop {
    match error {
        Error::Usage(message) => Stop::Usage(message),
        Error::Library(message) => Stop::Library(message),
    }
}

pub fn run(bin: &'static str, doors: &'static [Function]) -> ExitCode {
    let book = Book { bin, doors };
    let words: Vec<String> = std::env::args().skip(1).collect();
    match book.command(&words) {
        Ok(()) | Err(Stop::Quiet) => ExitCode::SUCCESS,
        Err(Stop::Library(message)) => {
            eprintln!("{bin}: {message}");
            ExitCode::from(1)
        }
        Err(Stop::Usage(message)) => {
            eprintln!("{bin}: {message}");
            ExitCode::from(2)
        }
    }
}

impl Book {
    fn command(&self, words: &[String]) -> Result<(), Stop> {
        let mut raw = false;
        let mut help = false;
        let mut version = false;
        let mut rest: Vec<&str> = Vec::new();
        let mut words = words.iter().map(String::as_str);
        while let Some(word) = words.next() {
            match word {
                "-r" | "--raw" => raw = true,
                "-h" | "--help" => help = true,
                "-V" | "--version" => version = true,
                "--" => rest.extend(words.by_ref()),
                flag if flag.starts_with('-') && flag != "-" && !json(flag) => {
                    return Err(Stop::Usage(format!(
                        "no option {flag}; {} --help lists them",
                        self.bin
                    )));
                }
                other => rest.push(other),
            }
        }
        if version {
            return print(&format!("{} {}", self.bin, env!("CARGO_PKG_VERSION")));
        }
        match (help, rest.as_slice()) {
            (_, []) | (_, ["help"]) | (true, ["list", ..]) => {
                print(HELP.replace("BIN", self.bin).trim_end())
            }
            (_, ["help", name]) | (true, [name]) => self.describe(name),
            (true, [name, _]) => self.describe(name),
            (_, ["list"]) => self.catalog(""),
            (_, ["list", prefix]) => self.catalog(prefix),
            (_, [name]) => self.answer(name, None, raw),
            (_, [name, text]) => self.answer(name, Some(text), raw),
            _ => Err(Stop::Usage(
                "too many words; pass the arguments as one JSON array in quotes".to_string(),
            )),
        }
    }

    fn catalog(&self, prefix: &str) -> Result<(), Stop> {
        let mut out = io::BufWriter::new(io::stdout().lock());
        let mut any = false;
        for function in self.doors.iter().filter(|f| f.name.starts_with(prefix)) {
            writeln!(out, "{}", line(function))?;
            any = true;
        }
        out.flush()?;
        if any {
            Ok(())
        } else {
            Err(Stop::Usage(format!("no function starts with {prefix:?}")))
        }
    }

    fn describe(&self, name: &str) -> Result<(), Stop> {
        let module = format!("{}.", name.trim_end_matches('.'));
        if self.find(name).is_err() && self.doors.iter().any(|f| f.name.starts_with(&module)) {
            return self.catalog(&module);
        }
        let function = self.find(name).map_err(stop)?;
        let mut text = line(function);
        if !function.docs.is_empty() {
            text.push_str("\n\n");
            text.push_str(function.docs);
        }
        print(&text)
    }

    fn answer(&self, name: &str, text: Option<&str>, raw: bool) -> Result<(), Stop> {
        let function = self.find(name).map_err(stop)?;
        let text = match text {
            Some("-") => stdin()?,
            Some(text) => text.to_string(),
            None if function.signature.starts_with("()") || io::stdin().is_terminal() => {
                String::new()
            }
            None => stdin()?,
        };
        let args = match text.trim() {
            "" => Vec::new(),
            text => match serde_json::from_str::<Value>(text) {
                Ok(Value::Array(args)) => args,
                Ok(one) => vec![one],
                Err(error) => {
                    return Err(Stop::Usage(format!(
                        "the arguments to {name} are not JSON: {error}"
                    )));
                }
            },
        };
        let value = function.call(&args).map_err(|error| match error {
            Error::Usage(message) if function.callable() => {
                Stop::Usage(format!("{message}\n  {}", line(function)))
            }
            other => stop(other),
        })?;
        match value {
            Value::String(text) if raw => {
                let mut out = io::stdout().lock();
                out.write_all(text.as_bytes())?;
                if !text.ends_with('\n') {
                    out.write_all(b"\n")?;
                }
                out.flush()?;
                Ok(())
            }
            value => print(&value.to_string()),
        }
    }
}

fn json(word: &str) -> bool {
    serde_json::from_str::<Value>(word).is_ok()
}

fn print(text: &str) -> Result<(), Stop> {
    let mut out = io::stdout().lock();
    writeln!(out, "{text}")?;
    out.flush()?;
    Ok(())
}

fn line(function: &Function) -> String {
    match function.uncallable {
        Some(reason) => format!(
            "{}{} # uncallable: {reason}",
            function.name, function.signature
        ),
        None => format!("{}{}", function.name, function.signature),
    }
}

fn stdin() -> Result<String, Stop> {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .map_err(|error| Stop::Usage(format!("stdin: {error}")))?;
    Ok(text)
}
