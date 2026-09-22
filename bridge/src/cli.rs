use crate::model::{Cross, Function, Manifest, Result, SelfKind, Source, Ty, Type, TypeCross, TypeKind};
use std::collections::BTreeMap;
use std::path::Path;

pub fn write(manifest: &Manifest, root: &Path) -> Result<()> {
    let shop = Shop::new(manifest);
    let doors = doors(&shop, manifest)?;
    let file = root.join("pkgs/mrlyrs/src/bin/mrly.rs");
    let folder = file.parent().ok_or("mrly.rs needs a folder")?;
    std::fs::create_dir_all(folder).map_err(|error| error.to_string())?;
    std::fs::write(&file, render(&doors)).map_err(|error| error.to_string())
}

// SHOP

struct Shop<'a> {
    types: BTreeMap<&'a str, &'a Type>,
    hands: BTreeMap<&'a str, &'a str>,
}

impl<'a> Shop<'a> {
    fn new(manifest: &'a Manifest) -> Shop<'a> {
        let mut types = BTreeMap::new();
        let mut hands = BTreeMap::new();
        for ty in &manifest.types {
            types.insert(ty.path.as_str(), ty);
            if let TypeCross::Hand { name } = &ty.cross {
                if ty.kind != TypeKind::Alias {
                    hands.insert(name.as_str(), ty.path.as_str());
                }
            }
        }
        Shop { types, hands }
    }
    fn hand(&self, name: &str) -> Result<String> {
        let path = self
            .hands
            .get(name)
            .ok_or_else(|| format!("no hand type named {name}"))?;
        Ok(format!("mrlyrs::{path}"))
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

// SHAPES

struct Door {
    name: String,
    sig: String,
    ident: String,
    body: Option<String>,
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

fn rng(ty: &Ty) -> bool {
    matches!(ty, Ty::Hand { name, .. } if name == "Rng")
}

fn special(ty: &Ty) -> bool {
    match ty {
        Ty::U128 | Ty::I128 | Ty::Code => true,
        Ty::Hand { name, .. } => name == "Color",
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

// NAMES

fn show(shop: &Shop, ty: &Ty, dim: Option<u8>) -> String {
    match ty {
        Ty::Unit => "null".to_string(),
        Ty::Scalar { name } => name.clone(),
        Ty::Str => "str".to_string(),
        Ty::String => "String".to_string(),
        Ty::U128 => "u128".to_string(),
        Ty::I128 => "i128".to_string(),
        Ty::Code => "Code".to_string(),
        Ty::Json => "Json".to_string(),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => {
            format!("[{}]", show(shop, item, dim))
        }
        Ty::Option { item } => format!("{}?", show(shop, item, dim)),
        Ty::Array { item, len } => format!("[{}; {len}]", show(shop, item, dim)),
        Ty::Tuple { items } => {
            let parts: Vec<String> = items.iter().map(|item| show(shop, item, dim)).collect();
            format!("({})", parts.join(", "))
        }
        Ty::Map { key, value } => format!("{{{}: {}}}", show(shop, key, dim), show(shop, value, dim)),
        Ty::Ref { mutable, item, .. } => {
            if *mutable && rng(item) {
                "Rng(seed)".to_string()
            } else {
                show(shop, item, dim)
            }
        }
        Ty::Result { item } => show(shop, item, dim),
        Ty::Hand { name, dim: own } => match (name.as_str(), own.or(dim)) {
            ("CellNd", Some(n)) => format!("Cell{n}d"),
            ("CellNd", None) => "Cell".to_string(),
            (other, _) => other.to_string(),
        },
        Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } | Ty::Opaque { path } => {
            path.replace("::", ".")
        }
        Ty::Unknown { text } => text.clone(),
    }
}

fn owned(shop: &Shop, ty: &Ty, dim: Option<u8>) -> Result<String> {
    Ok(match ty {
        Ty::Unit => "()".to_string(),
        Ty::Scalar { name } => name.clone(),
        Ty::Str | Ty::String => "String".to_string(),
        Ty::U128 => "u128".to_string(),
        Ty::I128 => "i128".to_string(),
        Ty::Code => shop.hand("Code")?,
        Ty::Json => "serde_json::Value".to_string(),
        Ty::Vec { item } | Ty::Slice { item, .. } => {
            format!("Vec<{}>", owned(shop, item, dim)?)
        }
        Ty::Set { item } => format!("std::collections::BTreeSet<{}>", owned(shop, item, dim)?),
        Ty::Option { item } => format!("Option<{}>", owned(shop, item, dim)?),
        Ty::Array { item, len } => format!("[{}; {len}]", owned(shop, item, dim)?),
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for item in items {
                parts.push(owned(shop, item, dim)?);
            }
            format!("({})", parts.join(", "))
        }
        Ty::Ref { item, .. } | Ty::Result { item } => owned(shop, item, dim)?,
        Ty::Map { .. } => "_".to_string(),
        Ty::Hand { name, dim: own } => {
            let path = shop.hand(name)?;
            match (name.as_str(), own.or(dim)) {
                ("CellNd", Some(n)) => format!("{path}<{n}>"),
                ("CellNd", None) => return Err("a cell with no dimension".to_string()),
                _ => path,
            }
        }
        Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } => {
            if shop.wide(path) {
                return Err(format!("{path} has a const generic"));
            }
            format!("mrlyrs::{path}")
        }
        Ty::Opaque { path } => return Err(format!("{path} does not cross")),
        Ty::Unknown { text } => return Err(format!("{text} does not cross")),
    })
}

// READING

fn take(at: usize, value: &str) -> String {
    format!("take!(name, {at}, {value})")
}

fn read(shop: &Shop, ty: &Ty, at: usize, value: &str, deep: usize) -> Result<String> {
    if !special(ty) {
        return Ok(take(at, value));
    }
    Ok(match ty {
        Ty::U128 => format!("big(name, {at}, {value})?"),
        Ty::I128 => format!("small(name, {at}, {value})?"),
        Ty::Code => format!("{}::from(big(name, {at}, {value})?)", shop.hand("Code")?),
        Ty::Hand { .. } => format!("color(name, {at}, {value})?"),
        Ty::Ref { item, .. } | Ty::Result { item } => read(shop, item, at, value, deep)?,
        Ty::Option { item } => format!(
            "if Value::is_null({value}) {{ None }} else {{ Some({}) }}",
            read(shop, item, at, value, deep)?
        ),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => {
            let one = read(shop, item, at, &format!("item{deep}"), deep + 1)?;
            let run = format!(
                "{{ let mut list{deep} = Vec::new(); for item{deep} in items(name, {at}, {value})? {{ list{deep}.push({one}); }} list{deep} }}"
            );
            match ty {
                Ty::Set { .. } => format!("{run}.into_iter().collect()"),
                _ => run,
            }
        }
        Ty::Array { item, len } => {
            let one = read(shop, item, at, &format!("item{deep}"), deep + 1)?;
            format!(
                "{{ let mut list{deep} = Vec::new(); for item{deep} in parts(name, {at}, {value}, {len})? {{ list{deep}.push({one}); }} match list{deep}.try_into() {{ Ok(fixed) => fixed, Err(_) => return Err(bad(name, {at}, \"a fixed array\")) }} }}"
            )
        }
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for (index, item) in items.iter().enumerate() {
                parts.push(read(
                    shop,
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
            let one = read(shop, held, at, &format!("item{deep}"), deep + 1)?;
            let keyed = owned(shop, key, None)?;
            let held = owned(shop, held, None)?;
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

fn seen(shop: &Shop, ty: &Ty, dim: Option<u8>) -> Result<String> {
    Ok(match ty {
        Ty::Str | Ty::String => "&str".to_string(),
        Ty::Vec { item } | Ty::Slice { item, .. } => format!("&[{}]", owned(shop, item, dim)?),
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for item in items {
                parts.push(seen(shop, item, dim)?);
            }
            format!("({})", parts.join(", "))
        }
        other => owned(shop, other, dim)?,
    })
}

// WRITING

fn give(shop: &Shop, ty: &Ty, value: &str, deep: usize) -> Result<String> {
    if !special(ty) {
        return Ok(match ty {
            Ty::Unit => "Value::Null".to_string(),
            _ => format!("give!({value})"),
        });
    }
    Ok(match ty {
        Ty::U128 | Ty::I128 => format!("Value::String({value}.to_string())"),
        Ty::Code => format!("Value::String({value}.get().to_string())"),
        Ty::Hand { .. } => format!("tint({value})"),
        Ty::Ref { item, .. } | Ty::Result { item } => give(shop, item, value, deep)?,
        Ty::Option { item } => format!(
            "match {value} {{ Some(item{deep}) => {}, None => Value::Null }}",
            give(shop, item, &format!("item{deep}"), deep + 1)?
        ),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } | Ty::Array { item, .. } => {
            format!(
                "{{ let mut list{deep} = Vec::new(); for item{deep} in {value} {{ list{deep}.push({}); }} Value::Array(list{deep}) }}",
                give(shop, item, &format!("item{deep}"), deep + 1)?
            )
        }
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for (index, item) in items.iter().enumerate() {
                parts.push(give(shop, item, &format!("parts{deep}.{index}"), deep + 1)?);
            }
            format!(
                "{{ let parts{deep} = {value}; Value::Array(vec![{}]) }}",
                parts.join(", ")
            )
        }
        Ty::Map { value: held, .. } => format!(
            "{{ let mut pairs{deep} = serde_json::Map::new(); for (key{deep}, item{deep}) in {value} {{ pairs{deep}.insert(key{deep}.to_string(), {}); }} Value::Object(pairs{deep}) }}",
            give(shop, held, &format!("item{deep}"), deep + 1)?
        ),
        other => return Err(format!("{other:?} cannot be written")),
    })
}

// DOORS

fn args_of(shop: &Shop, function: &Function) -> Result<Args> {
    let mut out = Vec::new();
    if let Some(kind) = function.self_kind {
        let owner = function
            .owner
            .as_ref()
            .ok_or_else(|| format!("{}: a self with no owner", function.path))?;
        let ty = shop.owner(owner)?;
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

fn sign(shop: &Shop, function: &Function) -> Result<String> {
    let dim = match function.dims.as_slice() {
        [one] => Some(*one),
        _ => None,
    };
    let args = args_of(shop, function)?;
    let parts: Vec<String> = args
        .iter()
        .map(|(name, ty)| format!("{name}: {}", show(shop, ty, dim)))
        .collect();
    Ok(format!(
        "({}) -> {}",
        parts.join(", "),
        show(shop, &function.ret, dim)
    ))
}

fn callee(shop: &Shop, function: &Function, dim: Option<u8>) -> String {
    let name = &function.name;
    match (&function.owner, &function.via) {
        (Some(owner), Some(via)) if function.source == Source::Trait => {
            format!("<mrlyrs::{owner} as mrlyrs::{via}>::{name}")
        }
        (Some(owner), _) => {
            let turbo = match dim {
                Some(n) if shop.wide(owner) => format!("::<{n}>"),
                _ => String::new(),
            };
            format!("mrlyrs::{owner}{turbo}::{name}")
        }
        (None, _) => {
            let turbo = match dim {
                Some(n) => format!("::<{n}>"),
                None => String::new(),
            };
            format!("mrlyrs::{}{turbo}", function.path)
        }
    }
}

fn hold(shop: &Shop, ty: &Ty, at: usize, dim: Option<u8>) -> Result<Bind> {
    let slot = format!("args[{at}]");
    if let Ty::Ref { mutable: true, item, .. } = ty {
        if rng(item) {
            let open = format!("mrlyrs::core::Rng::new(seed(name, {at}, &{slot})?)");
            return Ok(Bind::Open(
                format!("let mut a{at} = {open};"),
                format!("&mut a{at}"),
            ));
        }
        return Ok(Bind::Shut("mutates its argument in place".to_string()));
    }
    if let Ty::Option { item } = ty {
        if let Ty::Ref { mutable: true, item: held, .. } = item.as_ref() {
            if rng(held) {
                let open = format!("mrlyrs::core::Rng::new(seed(name, {at}, &{slot})?)");
                return Ok(Bind::Open(
                    format!("let mut a{at} = if {slot}.is_null() {{ None }} else {{ Some({open}) }};"),
                    format!("a{at}.as_mut()"),
                ));
            }
            return Ok(Bind::Shut("mutates its argument in place".to_string()));
        }
    }
    let known = bare(ty);
    if let Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } = known {
        if !shop.derives(path, "Deserialize") {
            return Ok(Bind::Shut(format!("{path} has no Deserialize")));
        }
    }
    let start = match read(shop, ty, at, &format!("&{slot}"), 0) {
        Ok(text) => text,
        Err(reason) => return Ok(Bind::Shut(reason)),
    };
    let mut lets = match owned(shop, ty, dim) {
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
            let sight = match seen(shop, item, dim) {
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

fn probe(shop: &Shop, function: &Function) -> Result<String> {
    let args = args_of(shop, function)?;
    let cell = |ty: &Ty| matches!(ty, Ty::Hand { name, dim: None } if name == "CellNd");
    for (at, (_, ty)) in args.iter().enumerate() {
        let inner = bare(ty);
        if cell(inner) {
            return Ok(format!("cell_rank(name, args, {at})?"));
        }
        if let Ty::Slice { item, .. } | Ty::Vec { item } = inner {
            if cell(bare(item)) {
                return Ok(format!("cells_rank(name, args, {at})?"));
            }
        }
    }
    for (at, (_, ty)) in args.iter().enumerate() {
        if matches!(bare(ty), Ty::Hand { name, .. } if name == "Tensor") {
            return Ok(format!("tensor_rank(name, args, {at})?"));
        }
    }
    Err(format!("{}: no cell or tensor to rank", function.path))
}

fn one(shop: &Shop, function: &Function, dim: Option<u8>) -> Result<Made> {
    let args = args_of(shop, function)?;
    let mut lines = vec![format!("count(name, args, {})?;", args.len())];
    let mut passes = Vec::new();
    for (at, (_, ty)) in args.iter().enumerate() {
        match hold(shop, ty, at, dim)? {
            Bind::Open(lets, pass) => {
                lines.push(lets);
                passes.push(pass);
            }
            Bind::Shut(reason) => return Ok(Made::Shut(reason)),
        }
    }
    let call = format!("{}({})", callee(shop, function, dim), passes.join(", "));
    let known = bare(&function.ret);
    if let Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } = known {
        if !shop.derives(path, "Serialize") {
            return Ok(Made::Shut(format!("{path} has no Serialize")));
        }
    }
    let tail = match &function.ret {
        Ty::Result { item } => {
            let good = match item.as_ref() {
                Ty::Unit => "Ok(()) => Ok(Value::Null)".to_string(),
                other => match give(shop, other, "value", 0) {
                    Ok(text) => format!("Ok(value) => Ok({text})"),
                    Err(reason) => return Ok(Made::Shut(reason)),
                },
            };
            format!("match {call} {{ {good}, Err(error) => Err(Fail::Error(error.to_string())) }}")
        }
        Ty::Unit => format!("{call};\nOk(Value::Null)"),
        other => match give(shop, other, &call, 0) {
            Ok(text) => format!("Ok({text})"),
            Err(reason) => return Ok(Made::Shut(reason)),
        },
    };
    lines.push(tail);
    Ok(Made::Open(lines.join("\n")))
}

fn build(shop: &Shop, group: &[&Function]) -> Result<Made> {
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
        return one(shop, function, dim);
    }
    let mut body = format!("match {} {{\n", probe(shop, branches[0].1)?);
    for (dim, function) in &branches {
        let dim = dim.ok_or("a branch with no dimension")?;
        match one(shop, function, Some(dim))? {
            Made::Open(text) => body.push_str(&format!("{dim} => {{\n{text}\n}}\n")),
            shut => return Ok(shut),
        }
    }
    body.push_str(
        "other => Err(usage(format!(\"{name} wants a 2d or 3d argument, got {other}d\"))),\n}",
    );
    Ok(Made::Open(body))
}

fn doors(shop: &Shop, manifest: &Manifest) -> Result<Vec<Door>> {
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
            signs.push(sign(shop, function)?);
        }
        let stem = format!("door_{}", name.replace('.', "_").to_lowercase());
        let count = taken.entry(stem.clone()).or_insert(0);
        *count += 1;
        let ident = match *count {
            1 => stem,
            other => format!("{stem}_{other}"),
        };
        let (sig, body) = match build(shop, &group)? {
            Made::Open(body) => (signs.join(" | "), Some(body)),
            Made::Shut(reason) => (
                format!("{} # uncallable: {reason}", signs.join(" | ")),
                None,
            ),
        };
        out.push(Door {
            name,
            sig,
            ident,
            body,
        });
    }
    Ok(out)
}

// TEXT

const HEAD: &str = r#"use serde_json::Value;
use std::process::ExitCode;

// PLUMBING

enum Fail {
    Usage(String),
    Error(String),
}

type Done = std::result::Result<Value, Fail>;

type Call = fn(&str, &[Value]) -> Done;

type Door = (&'static str, &'static str, Option<Call>);

const USAGE: &str = "usage: mrly list | mrly <module.fn> '[json, args]'";

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
            Err(error) => return Err(Fail::Error(error.to_string())),
        }
    };
}

fn usage(what: String) -> Fail {
    Fail::Usage(format!("mrly: {what}; {USAGE}"))
}

fn bad(name: &str, at: usize, why: &str) -> Fail {
    usage(format!("{name} argument {at}: {why}"))
}

fn count(name: &str, args: &[Value], want: usize) -> std::result::Result<(), Fail> {
    if args.len() == want {
        Ok(())
    } else {
        Err(usage(format!(
            "{name} takes {want} arguments, got {}",
            args.len()
        )))
    }
}

fn slot<'a>(name: &str, args: &'a [Value], at: usize) -> std::result::Result<&'a Value, Fail> {
    args.get(at)
        .ok_or_else(|| usage(format!("{name} wants an argument {at}")))
}

fn big(name: &str, at: usize, value: &Value) -> std::result::Result<u128, Fail> {
    let read = match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_u64().map(u128::from),
    };
    match read {
        Some(number) => Ok(number),
        None => Err(bad(name, at, "a u128 is a decimal string")),
    }
}

fn small(name: &str, at: usize, value: &Value) -> std::result::Result<i128, Fail> {
    let read = match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_i64().map(i128::from),
    };
    match read {
        Some(number) => Ok(number),
        None => Err(bad(name, at, "an i128 is a decimal string")),
    }
}

fn seed(name: &str, at: usize, value: &Value) -> std::result::Result<u64, Fail> {
    match value.as_u64() {
        Some(number) => Ok(number),
        None => Err(bad(name, at, "an Rng is a seed number")),
    }
}

fn color(name: &str, at: usize, value: &Value) -> std::result::Result<mrlyrs::core::Color, Fail> {
    let bytes: [u8; 4] = match serde_json::from_value(value.clone()) {
        Ok(bytes) => bytes,
        Err(_) => return Err(bad(name, at, "a Color is four bytes")),
    };
    Ok(mrlyrs::core::Color {
        r: bytes[0],
        g: bytes[1],
        b: bytes[2],
        a: bytes[3],
    })
}

fn tint(color: mrlyrs::core::Color) -> Value {
    Value::Array(vec![
        color.r.into(),
        color.g.into(),
        color.b.into(),
        color.a.into(),
    ])
}

fn items<'a>(name: &str, at: usize, value: &'a Value) -> std::result::Result<&'a [Value], Fail> {
    match value.as_array() {
        Some(list) => Ok(list),
        None => Err(bad(name, at, "a JSON array")),
    }
}

fn parts<'a>(
    name: &str,
    at: usize,
    value: &'a Value,
    want: usize,
) -> std::result::Result<&'a [Value], Fail> {
    match value.as_array() {
        Some(list) if list.len() == want => Ok(list),
        _ => Err(bad(name, at, &format!("a JSON array of {want}"))),
    }
}

fn fields<'a>(
    name: &str,
    at: usize,
    value: &'a Value,
) -> std::result::Result<&'a serde_json::Map<String, Value>, Fail> {
    match value.as_object() {
        Some(map) => Ok(map),
        None => Err(bad(name, at, "a JSON object")),
    }
}

fn axes(value: &Value) -> Option<usize> {
    Some(value.get("shape")?.as_array()?.len())
}

fn walls(value: &Value) -> Option<usize> {
    axes(value.get("cell")?.get("types")?)
}

fn tensor_rank(name: &str, args: &[Value], at: usize) -> std::result::Result<usize, Fail> {
    match axes(slot(name, args, at)?) {
        Some(rank) => Ok(rank),
        None => Err(bad(name, at, "a Tensor with a shape")),
    }
}

fn cell_rank(name: &str, args: &[Value], at: usize) -> std::result::Result<usize, Fail> {
    match walls(slot(name, args, at)?) {
        Some(rank) => Ok(rank),
        None => Err(bad(name, at, "a cell")),
    }
}

fn cells_rank(name: &str, args: &[Value], at: usize) -> std::result::Result<usize, Fail> {
    let value = slot(name, args, at)?;
    match value.as_array().and_then(|list| list.first()).and_then(walls) {
        Some(rank) => Ok(rank),
        None => Err(bad(name, at, "a list of cells")),
    }
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    match run(&words) {
        Ok(()) => ExitCode::SUCCESS,
        Err(Fail::Error(message)) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
        Err(Fail::Usage(message)) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

fn run(words: &[String]) -> std::result::Result<(), Fail> {
    match words {
        [one] if one == "list" => {
            for (name, sig, _) in DOORS {
                println!("{name}{sig}");
            }
            Ok(())
        }
        [name, text] => {
            let at = DOORS
                .binary_search_by_key(&name.as_str(), |door| door.0)
                .map_err(|_| usage(format!("no door named {name:?}")))?;
            let call = DOORS[at]
                .2
                .ok_or_else(|| usage(format!("{name} is not callable from the shell")))?;
            let args = match serde_json::from_str::<Value>(text) {
                Ok(Value::Array(args)) => args,
                Ok(_) => return Err(usage(format!("{name} wants a JSON array of arguments"))),
                Err(error) => return Err(usage(format!("{name} arguments: {error}"))),
            };
            println!("{}", call(name, &args)?);
            Ok(())
        }
        _ => Err(usage("say list or a door".to_string())),
    }
}
"#;

fn render(doors: &[Door]) -> String {
    let mut out = String::from(HEAD);
    out.push_str("\n// DOORS\n\nstatic DOORS: &[Door] = &[\n");
    for door in doors {
        let call = match &door.body {
            Some(_) => format!("Some({})", door.ident),
            None => "None".to_string(),
        };
        out.push_str(&format!(
            "    ({:?}, {:?}, {call}),\n",
            door.name, door.sig
        ));
    }
    out.push_str("];\n\n// CALLS\n");
    for door in doors {
        let Some(body) = &door.body else { continue };
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
