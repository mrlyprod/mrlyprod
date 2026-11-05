use crate::disk::save;
use crate::leaf::{self, Leaf};
use crate::model::{
    Cross, Function, Hand, Manifest, Result, SelfKind, Source, Ty, Type, TypeCross, TypeKind, big,
    by_dim, hand, ranked, row, row_of, rust, stream, undecided,
};
use std::collections::BTreeMap;
use std::path::Path;

pub const DOORS: &str = "src/doors.rs";

pub fn write_leaf(manifest: &Manifest, root: &Path, leaf: &Leaf) -> Result<()> {
    let book = Book::new(manifest);
    let doors = doors(&book, &leaf::only(manifest, &leaf.unit))?;
    let dir = root.join(&leaf.cli);
    save(&dir.join(DOORS), &render(&doors))?;
    save(&dir.join("Cargo.toml"), &leaf_cargo(leaf))?;
    save(&dir.join("src/main.rs"), &leaf_main(leaf))
}

fn leaf_cargo(leaf: &Leaf) -> String {
    let unit = &leaf.unit;
    format!(
        "[package]\nname = \"mrly-{unit}-cli\"\nversion.workspace = true\nedition.workspace = true\nlicense.workspace = true\nrepository.workspace = true\ndescription = \"The mrly-{unit} command: every public function of mrly-{unit} from the shell, JSON in, JSON out.\"\npublish = false\n\n[[bin]]\nname = \"mrly-{unit}\"\npath = \"src/main.rs\"\n\n[dependencies]\n{}serde_json.workspace = true\n",
        leaf::deps(leaf, &leaf.base, "cli")
    )
}

fn leaf_main(leaf: &Leaf) -> String {
    format!(
        "use mrly_{}::hand::cli::*;\n\nmod doors;\n\nfn main() -> std::process::ExitCode {{\n    run(\"mrly-{}\", doors::DOORS)\n}}\n",
        leaf.base, leaf.unit
    )
}

struct Book<'a> {
    types: BTreeMap<&'a str, &'a Type>,
    hands: BTreeMap<&'a str, &'a str>,
}

impl<'a> Book<'a> {
    fn new(manifest: &'a Manifest) -> Book<'a> {
        let mut types = BTreeMap::new();
        let mut hands = BTreeMap::new();
        for ty in &manifest.types {
            types.insert(ty.path.as_str(), ty);
            if let TypeCross::Hand { name } = &ty.cross
                && ty.kind != TypeKind::Alias
            {
                hands.insert(name.as_str(), ty.path.as_str());
            }
        }
        Book { types, hands }
    }
    fn hand(&self, name: &str) -> Result<String> {
        let path = self
            .hands
            .get(name)
            .ok_or_else(|| format!("no hand type named {name}"))?;
        Ok(rust(path))
    }
    fn derives(&self, path: &str, what: &str) -> bool {
        self.types
            .get(path)
            .is_some_and(|ty| ty.derives.iter().any(|derive| derive == what))
    }
    fn wide(&self, path: &str) -> bool {
        self.types.get(path).is_some_and(|ty| ty.const_generic)
    }
    fn owner(&self, path: &str) -> Result<Ty> {
        let ty = self
            .types
            .get(path)
            .ok_or_else(|| format!("no type named {path}"))?;
        Ok(match &ty.cross {
            TypeCross::Hand { name } => Ty::Hand {
                name: name.clone(),
                dim: None,
            },
            TypeCross::Class => Ty::Class {
                path: path.to_string(),
                dim: None,
            },
            TypeCross::Plain => Ty::Plain {
                path: path.to_string(),
            },
            TypeCross::Enum { .. } => Ty::Enum {
                path: path.to_string(),
            },
            TypeCross::Uncrossable { reason } => return Err(format!("{path}: {reason}")),
        })
    }
}

struct Door {
    name: String,
    sig: String,
    docs: String,
    ident: String,
    body: Result<String>,
}

enum Made {
    Open(String),
    Shut(String),
}

enum Bind {
    Open(String, String),
    Shut(String),
}

type Args = Vec<(String, Ty)>;

fn bare(ty: &Ty) -> &Ty {
    match ty {
        Ty::Ref { item, .. } | Ty::Result { item } => bare(item),
        other => other,
    }
}

fn crossing(name: &str) -> Result<&'static (String, String)> {
    row(name)?
        .cli
        .as_ref()
        .ok_or_else(|| format!("{name} has no cli reader and writer"))
}

fn last(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
}

fn special(ty: &Ty) -> bool {
    match ty {
        Ty::U128 | Ty::I128 | Ty::Code => true,
        Ty::Hand { name, .. } => hand(name).is_some_and(|row| row.cli.is_some()),
        Ty::Vec { item }
        | Ty::Slice { item, .. }
        | Ty::Option { item }
        | Ty::Array { item, .. }
        | Ty::Ref { item, .. }
        | Ty::Set { item }
        | Ty::Result { item } => special(item),
        Ty::Tuple { items } => items.iter().any(special),
        Ty::Map { key, value } => special(key) || special(value),
        _ => false,
    }
}

fn lacks(book: &Book, ty: &Ty, what: &str) -> Option<String> {
    let mut reason = None;
    ty.walk(&mut |seen| {
        if let Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } = seen
            && reason.is_none()
            && !book.derives(path, what)
        {
            reason = Some(format!("{path} has no {what}"));
        }
    });
    reason
}

fn borrowed(ty: &Ty) -> bool {
    match ty {
        Ty::Str | Ty::Slice { .. } => true,
        Ty::Vec { item } | Ty::Option { item } | Ty::Array { item, .. } | Ty::Ref { item, .. } => {
            borrowed(item)
        }
        Ty::Tuple { items } => items.iter().any(borrowed),
        _ => false,
    }
}

pub fn show(ty: &Ty, dim: Option<u8>) -> String {
    match ty {
        Ty::Unit => "null".to_string(),
        Ty::Scalar { name } => name.clone(),
        Ty::Str => "str".to_string(),
        Ty::String => "String".to_string(),
        Ty::U128 => "u128".to_string(),
        Ty::I128 => "i128".to_string(),
        Ty::Code => big().map_or("code", |row| row.name.as_str()).to_string(),
        Ty::Json => "Json".to_string(),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => {
            format!("[{}]", show(item, dim))
        }
        Ty::Option { item } => format!("{}?", show(item, dim)),
        Ty::Array { item, len } => format!("[{}; {len}]", show(item, dim)),
        Ty::Tuple { items } => {
            let parts: Vec<String> = items.iter().map(|item| show(item, dim)).collect();
            format!("({})", parts.join(", "))
        }
        Ty::Map { key, value } => format!("{{{}: {}}}", show(key, dim), show(value, dim)),
        Ty::Ref { mutable, item, .. } => {
            if *mutable && stream(item) {
                format!("{}(seed)", show(item, dim))
            } else {
                show(item, dim)
            }
        }
        Ty::Result { item } => show(item, dim),
        Ty::Hand { name, dim: own } => match hand(name) {
            Some(row) if row.generic() => match by_dim(&row.rust, own.or(dim)) {
                Some(path) => last(path).to_string(),
                None => {
                    let all: Vec<&str> = row.rust.iter().map(|path| last(path)).collect();
                    all.join("|")
                }
            },
            _ => name.clone(),
        },
        Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } | Ty::Opaque { path } => {
            path.replace("::", ".")
        }
        Ty::Unknown { text } => text.clone(),
    }
}

fn owned(book: &Book, ty: &Ty, dim: Option<u8>) -> Result<String> {
    Ok(match ty {
        Ty::Unit => "()".to_string(),
        Ty::Scalar { name } => name.clone(),
        Ty::Str | Ty::String => "String".to_string(),
        Ty::U128 => "u128".to_string(),
        Ty::I128 => "i128".to_string(),
        Ty::Code => book.hand(&big()?.name)?,
        Ty::Json => "serde_json::Value".to_string(),
        Ty::Vec { item } | Ty::Slice { item, .. } => {
            format!("Vec<{}>", owned(book, item, dim)?)
        }
        Ty::Set { item } => format!("std::collections::BTreeSet<{}>", owned(book, item, dim)?),
        Ty::Option { item } => format!("Option<{}>", owned(book, item, dim)?),
        Ty::Array { item, len } => format!("[{}; {len}]", owned(book, item, dim)?),
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for item in items {
                parts.push(owned(book, item, dim)?);
            }
            format!("({})", parts.join(", "))
        }
        Ty::Ref { item, .. } | Ty::Result { item } => owned(book, item, dim)?,
        Ty::Map { .. } => "_".to_string(),
        Ty::Hand { name, dim: own } => {
            let path = book.hand(name)?;
            match (row(name)?.generic(), own.or(dim)) {
                (true, Some(n)) => format!("{path}<{n}>"),
                (true, None) => return Err(format!("{name} with no dimension")),
                (false, _) => path,
            }
        }
        Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } => {
            if book.wide(path) {
                return Err(format!("{path} has a const generic"));
            }
            rust(path)
        }
        Ty::Opaque { path } => return Err(format!("{path} does not cross")),
        Ty::Unknown { text } => return Err(format!("{text} does not cross")),
    })
}

fn take(at: usize, value: &str) -> String {
    format!("take!(name, {at}, {value})")
}

fn read(book: &Book, ty: &Ty, at: usize, value: &str, deep: usize) -> Result<String> {
    if !special(ty) {
        return Ok(take(at, value));
    }
    Ok(match ty {
        Ty::U128 => format!("big(name, {at}, {value})?"),
        Ty::I128 => format!("small(name, {at}, {value})?"),
        Ty::Code => format!(
            "{}::from(big(name, {at}, {value})?)",
            book.hand(&big()?.name)?
        ),
        Ty::Hand { name, .. } => format!("{}(name, {at}, {value})?", crossing(name)?.0),
        Ty::Ref { item, .. } | Ty::Result { item } => read(book, item, at, value, deep)?,
        Ty::Option { item } => format!(
            "if Value::is_null({value}) {{ None }} else {{ Some({}) }}",
            read(book, item, at, value, deep)?
        ),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => {
            let one = read(book, item, at, &format!("item{deep}"), deep + 1)?;
            let run = format!(
                "{{ let mut list{deep} = Vec::new(); for item{deep} in items(name, {at}, {value})? {{ list{deep}.push({one}); }} list{deep} }}"
            );
            match ty {
                Ty::Set { .. } => format!("{run}.into_iter().collect()"),
                _ => run,
            }
        }
        Ty::Array { item, len } => {
            let one = read(book, item, at, &format!("item{deep}"), deep + 1)?;
            format!(
                "{{ let mut list{deep} = Vec::new(); for item{deep} in parts(name, {at}, {value}, {len})? {{ list{deep}.push({one}); }} match list{deep}.try_into() {{ Ok(fixed) => fixed, Err(_) => return Err(bad(name, {at}, \"a fixed array\")) }} }}"
            )
        }
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for (index, item) in items.iter().enumerate() {
                parts.push(read(
                    book,
                    item,
                    at,
                    &format!("&parts{deep}[{index}]"),
                    deep + 1,
                )?);
            }
            format!(
                "{{ let parts{deep} = parts(name, {at}, {value}, {})?; ({}) }}",
                items.len(),
                parts.join(", ")
            )
        }
        Ty::Map { key, value: held } => {
            let one = read(book, held, at, &format!("item{deep}"), deep + 1)?;
            let keyed = owned(book, key, None)?;
            let held = owned(book, held, None)?;
            format!(
                "{{ let mut pairs{deep}: Vec<({keyed}, {held})> = Vec::new(); for (text{deep}, item{deep}) in fields(name, {at}, {value})? {{ let key{deep} = match text{deep}.parse() {{ Ok(key) => key, Err(_) => return Err(bad(name, {at}, \"an object key\")) }}; pairs{deep}.push((key{deep}, {one})); }} pairs{deep} }}.into_iter().collect()"
            )
        }
        other => return Err(format!("{other:?} cannot be read")),
    })
}

fn view(ty: &Ty, at: &str, top: bool) -> String {
    match ty {
        Ty::Str | Ty::String => format!("{at}.as_str()"),
        Ty::Vec { .. } | Ty::Slice { .. } => format!("{at}.as_slice()"),
        Ty::Tuple { items } => {
            let parts: Vec<String> = items
                .iter()
                .enumerate()
                .map(|(index, item)| view(item, &format!("{at}.{index}"), false))
                .collect();
            format!("({})", parts.join(", "))
        }
        _ if top => format!("*{at}"),
        _ => at.to_string(),
    }
}

fn seen(book: &Book, ty: &Ty, dim: Option<u8>) -> Result<String> {
    Ok(match ty {
        Ty::Str | Ty::String => "&str".to_string(),
        Ty::Vec { item } | Ty::Slice { item, .. } => format!("&[{}]", owned(book, item, dim)?),
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for item in items {
                parts.push(seen(book, item, dim)?);
            }
            format!("({})", parts.join(", "))
        }
        other => owned(book, other, dim)?,
    })
}

fn give(ty: &Ty, value: &str, deep: usize) -> Result<String> {
    if !special(ty) {
        return Ok(match ty {
            Ty::Unit => "Value::Null".to_string(),
            _ => format!("give!({value})"),
        });
    }
    Ok(match ty {
        Ty::U128 | Ty::I128 => format!("Value::String({value}.to_string())"),
        Ty::Code => format!("Value::String({value}.get().to_string())"),
        Ty::Hand { name, .. } => format!("{}({value})", crossing(name)?.1),
        Ty::Ref { item, .. } | Ty::Result { item } => give(item, value, deep)?,
        Ty::Option { item } => format!(
            "match {value} {{ Some(item{deep}) => {}, None => Value::Null }}",
            give(item, &format!("item{deep}"), deep + 1)?
        ),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } | Ty::Array { item, .. } => {
            format!(
                "{{ let mut list{deep} = Vec::new(); for item{deep} in {value} {{ list{deep}.push({}); }} Value::Array(list{deep}) }}",
                give(item, &format!("item{deep}"), deep + 1)?
            )
        }
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for (index, item) in items.iter().enumerate() {
                parts.push(give(item, &format!("parts{deep}.{index}"), deep + 1)?);
            }
            format!(
                "{{ let parts{deep} = {value}; Value::Array(vec![{}]) }}",
                parts.join(", ")
            )
        }
        Ty::Map { value: held, .. } => format!(
            "{{ let mut pairs{deep} = serde_json::Map::new(); for (key{deep}, item{deep}) in {value} {{ pairs{deep}.insert(key{deep}.to_string(), {}); }} Value::Object(pairs{deep}) }}",
            give(held, &format!("item{deep}"), deep + 1)?
        ),
        other => return Err(format!("{other:?} cannot be written")),
    })
}

fn args_of(book: &Book, function: &Function) -> Result<Args> {
    let mut out = Vec::new();
    if let Some(kind) = function.self_kind {
        let owner = function
            .owner
            .as_ref()
            .ok_or_else(|| format!("{}: a self with no owner", function.path))?;
        let ty = book.owner(owner)?;
        out.push((
            "self".to_string(),
            match kind {
                SelfKind::Value => ty,
                SelfKind::Ref => Ty::Ref {
                    mutable: false,
                    lifetime: None,
                    item: Box::new(ty),
                },
                SelfKind::Mut => Ty::Ref {
                    mutable: true,
                    lifetime: None,
                    item: Box::new(ty),
                },
            },
        ));
    }
    for param in &function.params {
        out.push((param.name.clone(), param.ty.clone()));
    }
    Ok(out)
}

fn sign(book: &Book, function: &Function) -> Result<String> {
    let dim = match function.dims.as_slice() {
        [one] => Some(*one),
        _ => None,
    };
    let args = args_of(book, function)?;
    let parts: Vec<String> = args
        .iter()
        .map(|(name, ty)| format!("{name}: {}", show(ty, dim)))
        .collect();
    Ok(format!(
        "({}) -> {}",
        parts.join(", "),
        show(&function.ret, dim)
    ))
}

fn callee(book: &Book, function: &Function, dim: Option<u8>) -> String {
    let name = &function.name;
    match (&function.owner, &function.via) {
        (Some(owner), Some(via)) if function.source == Source::Trait => {
            format!("<{} as {}>::{name}", rust(owner), rust(via))
        }
        (Some(owner), _) => {
            let turbo = match dim {
                Some(n) if book.wide(owner) => format!("::<{n}>"),
                _ => String::new(),
            };
            format!("{}{turbo}::{name}", rust(owner))
        }
        (None, _) => {
            let turbo = match dim {
                Some(n) => format!("::<{n}>"),
                None => String::new(),
            };
            format!("{}{turbo}", rust(&function.path))
        }
    }
}

fn hold(book: &Book, ty: &Ty, at: usize, dim: Option<u8>) -> Result<Bind> {
    let slot = format!("args[{at}]");
    if let Ty::Ref {
        mutable: true,
        item,
        ..
    } = ty
    {
        if stream(item) {
            let open = format!(
                "{}::new(seed(name, {at}, &{slot})?)",
                owned(book, item, dim)?
            );
            return Ok(Bind::Open(
                format!("let mut a{at} = {open};"),
                format!("&mut a{at}"),
            ));
        }
        return Ok(Bind::Shut("mutates its argument in place".to_string()));
    }
    if let Ty::Option { item } = ty
        && let Ty::Ref {
            mutable: true,
            item: held,
            ..
        } = item.as_ref()
    {
        if stream(held) {
            let open = format!(
                "{}::new(seed(name, {at}, &{slot})?)",
                owned(book, held, dim)?
            );
            return Ok(Bind::Open(
                format!("let mut a{at} = if {slot}.is_null() {{ None }} else {{ Some({open}) }};"),
                format!("a{at}.as_mut()"),
            ));
        }
        return Ok(Bind::Shut("mutates its argument in place".to_string()));
    }
    if let Some(reason) = lacks(book, ty, "Deserialize") {
        return Ok(Bind::Shut(reason));
    }
    let start = match read(book, ty, at, &format!("&{slot}"), 0) {
        Ok(text) => text,
        Err(reason) => return Ok(Bind::Shut(reason)),
    };
    let mut lets = match owned(book, ty, dim) {
        Ok(text) if text != "_" => format!("let a{at}: {text} = {start};"),
        _ => format!("let a{at} = {start};"),
    };
    let stripped = match ty {
        Ty::Ref { item, .. } => item.as_ref(),
        other => other,
    };
    let pass = match stripped {
        Ty::Str => format!("a{at}.as_str()"),
        Ty::Slice { item, .. } | Ty::Vec { item } if borrowed(item) => {
            let sight = match seen(book, item, dim) {
                Ok(text) => text,
                Err(reason) => return Ok(Bind::Shut(reason)),
            };
            lets.push_str(&format!(
                " let b{at}: Vec<{sight}> = a{at}.iter().map(|item| {}).collect();",
                view(item, "item", true)
            ));
            format!("&b{at}")
        }
        Ty::Slice { .. } => format!("&a{at}"),
        Ty::Option { item } => match item.as_ref() {
            Ty::Ref { .. } => format!("a{at}.as_ref()"),
            Ty::Slice { .. } | Ty::Str => format!("a{at}.as_deref()"),
            _ => format!("a{at}"),
        },
        _ if matches!(ty, Ty::Ref { .. }) => format!("&a{at}"),
        _ => format!("a{at}"),
    };
    Ok(Bind::Open(lets, pass))
}

fn word(ty: &Ty) -> &'static str {
    row_of(ty).map_or("", |row: &'static Hand| row.word.as_str())
}

fn probe(book: &Book, function: &Function) -> Result<String> {
    let args = args_of(book, function)?;
    for (at, (_, ty)) in args.iter().enumerate() {
        let inner = bare(ty);
        if undecided(inner) {
            return Ok(format!("{}_rank(name, args, {at})?", word(inner)));
        }
        if let Ty::Slice { item, .. } | Ty::Vec { item } = inner
            && undecided(bare(item))
        {
            return Ok(format!("{}s_rank(name, args, {at})?", word(bare(item))));
        }
    }
    for (at, (_, ty)) in args.iter().enumerate() {
        if ranked(bare(ty)) {
            return Ok(format!("{}_rank(name, args, {at})?", word(bare(ty))));
        }
    }
    Err(format!("{}: nothing to rank", function.path))
}

fn one(book: &Book, function: &Function, dim: Option<u8>) -> Result<Made> {
    let args = args_of(book, function)?;
    let mut lines = vec![format!("count(name, args, {})?;", args.len())];
    let mut passes = Vec::new();
    for (at, (_, ty)) in args.iter().enumerate() {
        match hold(book, ty, at, dim)? {
            Bind::Open(lets, pass) => {
                lines.push(lets);
                passes.push(pass);
            }
            Bind::Shut(reason) => return Ok(Made::Shut(reason)),
        }
    }
    let call = format!("{}({})", callee(book, function, dim), passes.join(", "));
    if let Some(reason) = lacks(book, &function.ret, "Serialize") {
        return Ok(Made::Shut(reason));
    }
    let tail = match &function.ret {
        Ty::Result { item } => {
            let good = match item.as_ref() {
                Ty::Unit => "Ok(()) => Ok(Value::Null)".to_string(),
                other => match give(other, "value", 0) {
                    Ok(text) => format!("Ok(value) => Ok({text})"),
                    Err(reason) => return Ok(Made::Shut(reason)),
                },
            };
            format!(
                "match {call} {{ {good}, Err(error) => Err(Error::Library(error.to_string())) }}"
            )
        }
        Ty::Unit => format!("{call};\nOk(Value::Null)"),
        other => match give(other, &call, 0) {
            Ok(text) => format!("Ok({text})"),
            Err(reason) => return Ok(Made::Shut(reason)),
        },
    };
    lines.push(tail);
    Ok(Made::Open(lines.join("\n")))
}

fn build(book: &Book, group: &[&Function]) -> Result<Made> {
    let mut branches: Vec<(Option<u8>, &Function)> = Vec::new();
    for function in group {
        if function.dims.is_empty() {
            branches.push((None, function));
        } else {
            for dim in &function.dims {
                branches.push((Some(*dim), function));
            }
        }
    }
    if branches.len() == 1 {
        let (dim, function) = branches[0];
        return one(book, function, dim);
    }
    let mut body = format!("match {} {{\n", probe(book, branches[0].1)?);
    for (dim, function) in &branches {
        let dim = dim.ok_or("a branch with no dimension")?;
        match one(book, function, Some(dim))? {
            Made::Open(text) => body.push_str(&format!("{dim} => {{\n{text}\n}}\n")),
            shut => return Ok(shut),
        }
    }
    body.push_str(
        "other => Err(usage(format!(\"{name} wants a 2d or 3d argument, got {other}d\"))),\n}",
    );
    Ok(Made::Open(body))
}

fn doors(book: &Book, manifest: &Manifest) -> Result<Vec<Door>> {
    let mut groups: BTreeMap<String, Vec<&Function>> = BTreeMap::new();
    for function in &manifest.functions {
        if function.cross != Cross::Ok {
            continue;
        }
        groups
            .entry(function.path.replace("::", "."))
            .or_default()
            .push(function);
    }
    let mut taken: BTreeMap<String, usize> = BTreeMap::new();
    let mut out = Vec::new();
    for (name, group) in groups {
        let mut signs = Vec::new();
        for function in &group {
            signs.push(sign(book, function)?);
        }
        let stem = format!("door_{}", name.replace('.', "_").to_lowercase());
        let count = taken.entry(stem.clone()).or_insert(0);
        *count += 1;
        let ident = match *count {
            1 => stem,
            other => format!("{stem}_{other}"),
        };
        let body = match build(book, &group)? {
            Made::Open(body) => Ok(body),
            Made::Shut(reason) => Err(reason),
        };
        out.push(Door {
            name,
            sig: signs.join(" | "),
            docs: docs_of(&group),
            ident,
            body,
        });
    }
    Ok(out)
}

fn docs_of(group: &[&Function]) -> String {
    let lines = group
        .iter()
        .map(|function| &function.docs)
        .find(|docs| !docs.is_empty())
        .cloned()
        .unwrap_or_default();
    let mut fenced = false;
    let mut out: Vec<&str> = Vec::new();
    for line in &lines {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        let blank = line.trim().is_empty();
        if fenced || (blank && out.last().is_none_or(|last| last.is_empty())) {
            continue;
        }
        out.push(if blank { "" } else { line });
    }
    out.join("\n").trim().to_string()
}

fn render(doors: &[Door]) -> String {
    let mut out = String::from(
        "use super::*;\nuse serde_json::Value;\n\npub(crate) static DOORS: &[Function] = &[\n",
    );
    for door in doors {
        let (uncallable, call) = match &door.body {
            Ok(_) => ("None".to_string(), format!("Some({})", door.ident)),
            Err(reason) => (format!("Some({reason:?})"), "None".to_string()),
        };
        out.push_str(&format!(
            "    Function {{ name: {:?}, signature: {:?}, docs: {:?}, uncallable: {uncallable}, door: {call} }},\n",
            door.name, door.sig, door.docs
        ));
    }
    out.push_str("];\n");
    for door in doors {
        let Ok(body) = &door.body else { continue };
        out.push_str(&format!(
            "\nfn {}(name: &str, args: &[Value]) -> Done {{\n",
            door.ident
        ));
        for line in body.lines() {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
        out.push_str("}\n");
    }
    out
}
