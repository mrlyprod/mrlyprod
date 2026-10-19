#![doc = include_str!("../README.md")]

use mrly_cli::{Error, Function, find, list};
use serde_json::Value;
use std::io::{self, IsTerminal, Read, Write};
use std::process::ExitCode;

const HELP: &str = r#"mrly: every public function of the mrly library from the shell, JSON in, JSON out.

usage:
  mrly list [prefix]       every function with its signature, one per line
  mrly help <name>         one function's signature and docs
  mrly <name> '<json>'     call a function and print its result as JSON
  mrly <name> [-]          the same, reading the JSON from stdin
  mrly <name> -- '<json>'  the same, for JSON that starts with a dash
  mrly --version           the version

options:
  -r, --raw                print a string result as it is, not as a JSON string
  -h, --help               this page

arguments:
  a JSON array holds one value per parameter, in order: '[3, 1]'
  any other JSON value is the one argument, so one call's result can pipe into the next
  an optional parameter takes null; a Color is [r, g, b, a]
  Rng(seed) is a seed number; u128, i128 and Code are decimal strings

exit codes:
  0  the call ran and its result is on stdout
  1  the function returned an error; the message is on stderr
  2  the command was written wrong; the message is on stderr

examples:
  mrly num.factor.gcd '["1071", "462"]'
  mrly list math.two
  mrly math.two.carpet '[3, 1]' | mrly math.cell.models.invert
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

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    match run(&words) {
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

fn run(words: &[String]) -> Result<(), Stop> {
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
        (_, []) | (_, ["help"]) | (true, ["list", ..]) => print(HELP.trim_end()),
        (_, ["help", name]) | (true, [name]) => describe(name),
        (true, [name, _]) => describe(name),
        (_, ["list"]) => catalog(""),
        (_, ["list", prefix]) => catalog(prefix),
        (_, [name]) => answer(name, None, raw),
        (_, [name, text]) => answer(name, Some(text), raw),
        _ => Err(Stop::Usage(
            "too many words; pass the arguments as one JSON array in quotes".to_string(),
        )),
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

fn catalog(prefix: &str) -> Result<(), Stop> {
    let mut out = io::BufWriter::new(io::stdout().lock());
    let mut any = false;
    for function in list().filter(|function| function.name.starts_with(prefix)) {
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

fn describe(name: &str) -> Result<(), Stop> {
    let module = format!("{}.", name.trim_end_matches('.'));
    if find(name).is_err() && list().any(|function| function.name.starts_with(&module)) {
        return catalog(&module);
    }
    let function = find(name).map_err(stop)?;
    let mut text = line(function);
    if !function.docs.is_empty() {
        text.push_str("\n\n");
        text.push_str(function.docs);
    }
    print(&text)
}

fn answer(name: &str, text: Option<&str>, raw: bool) -> Result<(), Stop> {
    let function = find(name).map_err(stop)?;
    let text = match text {
        Some("-") => stdin()?,
        Some(text) => text.to_string(),
        None if function.signature.starts_with("()") || io::stdin().is_terminal() => String::new(),
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

fn stdin() -> Result<String, Stop> {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .map_err(|error| Stop::Usage(format!("stdin: {error}")))?;
    Ok(text)
}
