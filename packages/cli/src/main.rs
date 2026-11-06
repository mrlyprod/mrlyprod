use mrly_json::{Json as Value, Map, Number};
use mrly_wire::*;
use std::io::{self, IsTerminal, Read, Write};
use std::process::ExitCode;

use mrly_blob_all::{DOORS, SHAPES};

const HELP: &str = "usage:
  mrly list [prefix]
  mrly help <name>
  mrly <name> '<json>'
  mrly <name> [-]
  mrly <name> -- '<json>'
  mrly --version

options:
  -r, --raw
  -h, --help

exit codes:
  0  ok
  1  the function returned an error
  2  the command was written wrong";

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

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    match command(&words) {
        Ok(()) | Err(Stop::Quiet) => ExitCode::SUCCESS,
        Err(Stop::Library(message)) => {
            eprintln!("mrly: {message}");
            ExitCode::from(1)
        }
        Err(Stop::Usage(message)) => {
            eprintln!("mrly: {message}");
            ExitCode::from(2)
        }
    }
}

fn command(words: &[String]) -> Result<(), Stop> {
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
                    "no option {flag}; mrly --help lists them"
                )));
            }
            other => rest.push(other),
        }
    }
    if version {
        return print(&format!("mrly {}", env!("CARGO_PKG_VERSION")));
    }
    match (help, rest.as_slice()) {
        (_, []) | (_, ["help"]) | (true, ["list", ..]) => print(HELP),
        (_, ["help", name]) | (true, [name]) | (true, [name, _]) => describe(name),
        (_, ["list"]) => catalog(""),
        (_, ["list", prefix]) => catalog(prefix),
        (_, [name]) => answer(name, None, raw),
        (_, [name, text]) => answer(name, Some(text), raw),
        _ => Err(Stop::Usage(
            "too many words; pass the arguments as one JSON array in quotes".to_string(),
        )),
    }
}

fn shown(function: &Function) -> bool {
    !function.name.contains('$') && !function.name.starts_with("Rng.")
}

fn catalog(prefix: &str) -> Result<(), Stop> {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut any = false;
    for function in DOORS
        .iter()
        .filter(|f| shown(f) && f.name.starts_with(prefix))
    {
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

fn find(name: &str) -> Option<(&'static Function, &'static [&'static [&'static str]])> {
    let at = DOORS
        .binary_search_by_key(&name, |function| function.name)
        .ok()?;
    Some((&DOORS[at], SHAPES))
}

fn describe(name: &str) -> Result<(), Stop> {
    let Some((function, _)) = find(name) else {
        return catalog(&format!("{}.", name.trim_end_matches('.')));
    };
    let mut text = line(function);
    if !function.docs.is_empty() {
        text.push_str("\n\n");
        text.push_str(function.docs);
    }
    print(&text)
}

fn answer(name: &str, text: Option<&str>, raw: bool) -> Result<(), Stop> {
    let Some((function, shapes)) = find(name) else {
        return Err(Stop::Usage(format!(
            "no function named {name:?}; mrly list prints them all"
        )));
    };
    let text = match text {
        Some("-") => stdin()?,
        Some(text) => text.to_string(),
        None if function.signature.starts_with("()") || io::stdin().is_terminal() => String::new(),
        None => stdin()?,
    };
    let args = match text.trim() {
        "" => Vec::new(),
        text => match mrly_json::parse(text) {
            Ok(Value::Array(args)) => args,
            Ok(one) => vec![one],
            Err(error) => {
                return Err(Stop::Usage(format!("the arguments to {name}: {error}")));
            }
        },
    };
    install(shapes);
    let frames: Vec<Vec<u8>> = args
        .iter()
        .map(|arg| {
            let mut out = Vec::new();
            encode(arg, &mut out);
            out
        })
        .collect();
    let parts: Vec<&[u8]> = frames.iter().map(Vec::as_slice).collect();
    let status = run_host(function, &parts, false);
    let bytes = unsafe { std::slice::from_raw_parts(out_ptr(), out_len()) };
    match status {
        0 => {}
        1 => {
            let said = String::from_utf8_lossy(bytes);
            return Err(Stop::Usage(format!("{said}\n  {}", line(function))));
        }
        _ => return Err(Stop::Library(String::from_utf8_lossy(bytes).into_owned())),
    }
    let value = decode(bytes).map_err(Stop::Library)?;
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

fn json(word: &str) -> bool {
    mrly_json::parse(word).is_ok()
}

fn print(text: &str) -> Result<(), Stop> {
    let mut out = io::stdout().lock();
    writeln!(out, "{text}")?;
    out.flush()?;
    Ok(())
}

fn line(function: &Function) -> String {
    format!("{}{}", function.name, function.signature)
}

fn stdin() -> Result<String, Stop> {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .map_err(|error| Stop::Usage(format!("stdin: {error}")))?;
    Ok(text)
}

fn encode(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Null => out.push(NUL),
        Value::Bool(flag) => out.push(if *flag { TRUE } else { FALSE }),
        Value::Number(number) => {
            if let Some(whole) = number.as_i64() {
                if (0..128).contains(&whole) {
                    out.push(SMALL | whole as u8);
                } else if whole >= i32::MIN as i64 && whole <= i32::MAX as i64 {
                    out.push(I32);
                    out.extend_from_slice(&(whole as i32).to_le_bytes());
                } else {
                    text(out, BIG, &whole.to_string());
                }
            } else if let Some(whole) = number.as_u64() {
                text(out, BIG, &whole.to_string());
            } else {
                out.push(F64);
                out.extend_from_slice(&number.as_f64().unwrap_or(f64::NAN).to_le_bytes());
            }
        }
        Value::String(one) => text(out, STR, one),
        Value::Array(items) => {
            out.push(ARR);
            out.extend_from_slice(&(items.len() as u32).to_le_bytes());
            for item in items {
                encode(item, out);
            }
        }
        Value::Object(map) => {
            out.push(MAP);
            out.extend_from_slice(&(map.len() as u32).to_le_bytes());
            for (key, item) in map {
                text(out, STR, key);
                encode(item, out);
            }
        }
    }
}

fn text(out: &mut Vec<u8>, tag: u8, value: &str) {
    out.push(tag);
    out.extend_from_slice(&(value.len() as u32).to_le_bytes());
    out.extend_from_slice(value.as_bytes());
}

struct Reader<'a> {
    b: &'a [u8],
    p: usize,
    syms: Vec<String>,
    shapes: Vec<Vec<String>>,
}

fn decode(b: &[u8]) -> Result<Value, String> {
    let known: Vec<Vec<String>> = shapes_of()
        .iter()
        .map(|fields| fields.iter().map(|field| field.to_string()).collect())
        .collect();
    let mut reader = Reader {
        b,
        p: 4,
        syms: Vec::new(),
        shapes: known,
    };
    let defs = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize;
    if defs != 0 {
        reader.define(defs)?;
    }
    reader.read()
}

impl Reader<'_> {
    fn u16(&mut self) -> usize {
        let value = u16::from_le_bytes([self.b[self.p], self.b[self.p + 1]]);
        self.p += 2;
        value as usize
    }
    fn u32(&mut self) -> usize {
        let b = self.b;
        let value = u32::from_le_bytes([b[self.p], b[self.p + 1], b[self.p + 2], b[self.p + 3]]);
        self.p += 4;
        value as usize
    }
    fn f64(&mut self) -> f64 {
        let mut word = [0; 8];
        word.copy_from_slice(&self.b[self.p..self.p + 8]);
        self.p += 8;
        f64::from_le_bytes(word)
    }
    fn string(&mut self) -> Result<&str, String> {
        let n = self.u32();
        let at = self.p;
        self.p += n;
        std::str::from_utf8(&self.b[at..at + n]).map_err(|error| error.to_string())
    }
    fn define(&mut self, at: usize) -> Result<(), String> {
        let keep = self.p;
        self.p = at;
        while self.p < self.b.len() {
            let kind = self.b[self.p];
            self.p += 1;
            let id = self.u16();
            if kind == 1 {
                let name = self.string()?.to_string();
                if self.syms.len() <= id {
                    self.syms.resize(id + 1, String::new());
                }
                self.syms[id] = name;
            } else {
                let n = self.u16();
                let mut fields = Vec::with_capacity(n);
                for _ in 0..n {
                    let sym = self.u16();
                    fields.push(self.syms[sym].clone());
                }
                if self.shapes.len() <= id {
                    self.shapes.resize(id + 1, Vec::new());
                }
                self.shapes[id] = fields;
            }
        }
        self.p = keep;
        Ok(())
    }
    fn read(&mut self) -> Result<Value, String> {
        let tag = self.b[self.p];
        self.p += 1;
        Ok(match tag {
            NUL => Value::Null,
            FALSE => Value::Bool(false),
            TRUE => Value::Bool(true),
            I32 => (self.u32() as u32 as i32).into(),
            F32 => {
                let value = f32::from_bits(self.u32() as u32);
                Number::from_f64(value as f64).map_or(Value::Null, Value::Number)
            }
            F64 => Number::from_f64(self.f64()).map_or(Value::Null, Value::Number),
            I53 => (self.f64() as i64).into(),
            STR | WIDE => Value::String(self.string()?.to_string()),
            BIG => {
                let text = self.string()?.to_string();
                if let Ok(value) = text.parse::<u64>() {
                    value.into()
                } else if let Ok(value) = text.parse::<i64>() {
                    value.into()
                } else {
                    Value::String(text)
                }
            }
            ARR => {
                let n = self.u32();
                let mut items = Vec::with_capacity(n);
                for _ in 0..n {
                    items.push(self.read()?);
                }
                Value::Array(items)
            }
            MAP => {
                let n = self.u32();
                let mut map = Map::new();
                for _ in 0..n {
                    let key = match self.read()? {
                        Value::String(text) => text,
                        other => other.to_string(),
                    };
                    map.insert(key, self.read()?);
                }
                Value::Object(map)
            }
            SHP => {
                let id = self.u16();
                let fields = self.shapes.get(id).cloned().unwrap_or_default();
                let mut map = Map::new();
                for field in fields {
                    map.insert(field, self.read()?);
                }
                Value::Object(map)
            }
            SYM => {
                let id = self.u16();
                Value::String(self.syms.get(id).cloned().unwrap_or_default())
            }
            VAR => {
                let id = self.u16();
                let key = self.syms.get(id).cloned().unwrap_or_default();
                let mut map = Map::new();
                map.insert(key, self.read()?);
                Value::Object(map)
            }
            BACK => {
                let n = self.b[self.p] as usize;
                self.p += 1;
                let mut backs = Vec::with_capacity(n);
                for _ in 0..n {
                    self.p += 1;
                    backs.push(self.read()?);
                }
                match self.read()? {
                    Value::Null if backs.len() == 1 => backs.remove(0),
                    Value::Null => Value::Array(backs),
                    one => {
                        backs.insert(0, one);
                        Value::Array(backs)
                    }
                }
            }
            HANDLE => {
                self.p += 4;
                self.u16();
                return Err("this result is an object and cannot print".to_string());
            }
            SMALL..=0xff => ((tag & 0x7f) as u64).into(),
            other => return Err(format!("an unknown wire tag {other}")),
        })
    }
}
