use crate::model::{
    Const, Cross, Field, Function, Manifest, Result, SelfKind, Source, Ty, Type, TypeCross,
    TypeKind, big, by_dim, hand, ranked, row, row_of, rust, stream, undecided,
};
use std::collections::BTreeMap;
use std::fmt::Write as _;

const TYPED: &[&str] = &[
    "u8", "u16", "u32", "i8", "i16", "i32", "f32", "f64", "usize", "u64", "i64",
];
const NUMBER: &[&str] = &[
    "u8", "u16", "u32", "usize", "i8", "i16", "i32", "f32", "f64", "bool",
];

pub struct Direct {
    pub id: usize,
    pub boolean: bool,
    pub whole: bool,
    pub fails: bool,
    pub owner: bool,
    pub count: usize,
}

pub struct ClassDoors {
    pub fields: Vec<(String, bool)>,
    pub methods: Vec<String>,
    pub statics: Vec<String>,
    pub from: bool,
    pub json: bool,
}

pub struct Table {
    pub rust: String,
    pub names: Vec<String>,
    pub direct: BTreeMap<String, Direct>,
    pub classes: BTreeMap<String, ClassDoors>,
    pub consts: Vec<String>,
    pub skipped: Vec<(String, String)>,
}

struct Book<'a> {
    unit: &'a str,
    types: BTreeMap<&'a str, &'a Type>,
    hands: BTreeMap<&'a str, &'a str>,
}

impl<'a> Book<'a> {
    fn new(manifest: &'a Manifest, unit: &'a str) -> Book<'a> {
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
        Book { unit, types, hands }
    }
    fn hand(&self, name: &str) -> Result<String> {
        if let Some(path) = self.hands.get(name) {
            return Ok(rust(path));
        }
        row(name)?
            .rust
            .first()
            .cloned()
            .ok_or_else(|| format!("no hand type named {name}"))
    }
    fn derives(&self, path: &str, what: &str) -> bool {
        self.types
            .get(path)
            .is_some_and(|ty| ty.derives.iter().any(|derive| derive == what))
    }
    fn wide(&self, path: &str) -> bool {
        self.types.get(path).is_some_and(|ty| ty.const_generic)
    }
    fn holds(&self, path: &str) -> bool {
        path.split("::").next() == Some(self.unit)
    }
    fn copy(&self, path: &str) -> bool {
        self.derives(path, "Copy")
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

enum Made {
    Open(String),
    Shut(String),
}

enum Bind {
    Open(String, String),
    Shut(String),
}

type Args = Vec<(String, Ty)>;

fn typed(ty: &Ty) -> bool {
    matches!(ty, Ty::Scalar { name } if TYPED.contains(&name.as_str()))
}

fn wide(name: &str) -> bool {
    name == "u64" || name == "i64"
}

fn pure_out(ty: &Ty) -> bool {
    match ty {
        Ty::Unit | Ty::Str | Ty::String | Ty::Json | Ty::Plain { .. } | Ty::Enum { .. } => true,
        Ty::Scalar { name } => !wide(name),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Array { item, .. } => {
            !typed(item) && pure_out(item)
        }
        Ty::Option { item } => pure_out(item),
        Ty::Tuple { items } => items.iter().all(pure_out),
        _ => false,
    }
}

fn lasting(ty: &Ty) -> bool {
    !ty.any(&|one| matches!(one, Ty::Ref { .. } | Ty::Str | Ty::Slice { .. }))
}

fn colored(name: &str) -> bool {
    name == "Color"
}

fn special(book: &Book, ty: &Ty) -> bool {
    match ty {
        Ty::U128 | Ty::I128 | Ty::Code => true,
        Ty::Hand { name, .. } => colored(name) || hand(name).is_some_and(|row| row.big),
        Ty::Class { path, .. } => book.holds(path),
        Ty::Vec { item }
        | Ty::Slice { item, .. }
        | Ty::Option { item }
        | Ty::Array { item, .. }
        | Ty::Ref { item, .. }
        | Ty::Set { item }
        | Ty::Result { item } => special(book, item),
        Ty::Tuple { items } => items.iter().any(|item| special(book, item)),
        Ty::Map { key, value } => special(book, key) || special(book, value),
        _ => false,
    }
}

fn lacks(book: &Book, ty: &Ty, what: &str) -> Option<String> {
    let mut reason = None;
    ty.walk(&mut |seen| {
        let hit = match seen {
            Ty::Plain { path } | Ty::Enum { path } => Some(path),
            Ty::Class { path, .. } if !book.holds(path) => Some(path),
            _ => None,
        };
        if let Some(path) = hit
            && reason.is_none()
            && !book.derives(path, what)
        {
            reason = Some(format!("{path} has no {what}"));
        }
    });
    reason
}

fn bare(ty: &Ty) -> &Ty {
    match ty {
        Ty::Ref { item, .. } | Ty::Result { item } => bare(item),
        other => other,
    }
}

fn last(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
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
        Ty::Json => "mrly_core::Json".to_string(),
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
                match dim {
                    Some(n) => format!("{}<{n}>", rust(path)),
                    None => return Err(format!("{path} has a const generic")),
                }
            } else {
                rust(path)
            }
        }
        Ty::Opaque { path } => return Err(format!("{path} does not cross")),
        Ty::Unknown { text } => return Err(format!("{text} does not cross")),
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

fn take(ty: &Ty, at: usize, value: &str) -> String {
    match ty {
        Ty::Vec { item } | Ty::Slice { item, .. } if typed(item) => {
            format!("take_list(name, {at}, {value})?")
        }
        _ => format!("take(name, {at}, {value})?"),
    }
}

fn class_value(book: &Book, path: &str, at: usize, value: &str) -> String {
    let parse = if book.derives(path, "Deserialize") {
        format!("take(name, {at}, {value})?")
    } else {
        format!(
            "return Err(usage(format!(\"{{name}} argument {}: an object\")))",
            at + 1
        )
    };
    let ty = rust(path);
    let read = format!("held::<{ty}>(name, {at}, {value})?");
    if book.copy(path) {
        format!("match {read} {{ Some(one) => *one, None => {parse} }}")
    } else {
        format!("match {read} {{ Some(one) => (*one).clone(), None => {parse} }}")
    }
}

fn read(book: &Book, ty: &Ty, at: usize, value: &str, deep: usize) -> Result<String> {
    if !special(book, ty) {
        return Ok(take(ty, at, value));
    }
    Ok(match ty {
        Ty::U128 => format!("({value}).big(name, {at})?"),
        Ty::I128 => format!("({value}).small(name, {at})?"),
        Ty::Code => format!(
            "{}::from(({value}).big(name, {at})?)",
            book.hand(&big()?.name)?
        ),
        Ty::Class { path, .. } => class_value(book, path, at, value),
        Ty::Hand { name, .. } if hand(name).is_some_and(|row| row.big) => {
            format!("{}::from(({value}).big(name, {at})?)", book.hand(name)?)
        }
        Ty::Hand { name, .. } if colored(name) => {
            let kind = book.hand(name)?;
            format!("{{ let [r, g, b, a] = ({value}).rgba(name, {at})?; {kind} {{ r, g, b, a }} }}")
        }
        Ty::Ref { item, .. } | Ty::Result { item } => read(book, item, at, value, deep)?,
        Ty::Option { item } => format!(
            "if ({value}).is_null() {{ None }} else {{ Some({}) }}",
            read(book, item, at, value, deep)?
        ),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => {
            let one = read(book, item, at, &format!("item{deep}"), deep + 1)?;
            let run = format!(
                "{{ let mut list{deep} = Vec::new(); for item{deep} in ({value}).items_of(name, {at})? {{ list{deep}.push({one}); }} list{deep} }}"
            );
            match ty {
                Ty::Set { .. } => format!("{run}.into_iter().collect()"),
                _ => run,
            }
        }
        Ty::Array { item, len } => {
            let one = read(book, item, at, &format!("item{deep}"), deep + 1)?;
            format!(
                "{{ let mut list{deep} = Vec::new(); for item{deep} in ({value}).fixed(name, {at}, {len})? {{ list{deep}.push({one}); }} match list{deep}.try_into() {{ Ok(fixed) => fixed, Err(_) => return Err(usage(format!(\"{{name}}: a fixed array\"))) }} }}"
            )
        }
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for (index, item) in items.iter().enumerate() {
                parts.push(read(
                    book,
                    item,
                    at,
                    &format!("parts{deep}[{index}]"),
                    deep + 1,
                )?);
            }
            format!(
                "{{ let parts{deep} = ({value}).fixed(name, {at}, {})?; ({}) }}",
                items.len(),
                parts.join(", ")
            )
        }
        Ty::Map { key, value: held } => {
            let one = read(book, held, at, &format!("item{deep}"), deep + 1)?;
            let keyed = owned(book, key, None)?;
            let held = owned(book, held, None)?;
            format!(
                "{{ let mut pairs{deep}: Vec<({keyed}, {held})> = Vec::new(); for (text{deep}, item{deep}) in ({value}).fields_of(name, {at})? {{ let key{deep} = match text{deep}.parse() {{ Ok(key) => key, Err(_) => return Err(usage(format!(\"{{name}}: an object key\"))) }}; pairs{deep}.push((key{deep}, {one})); }} pairs{deep} }}.into_iter().collect()"
            )
        }
        other => return Err(format!("{other:?} cannot be read")),
    })
}

fn give(book: &Book, ty: &Ty, value: &str, kept: bool, deep: usize) -> Result<String> {
    let kept = kept && !matches!(ty, Ty::Str | Ty::Slice { .. });
    if pure_out(ty) {
        return Ok(match ty {
            Ty::Unit => "out.null();".to_string(),
            _ if kept && lasting(ty) => format!("out.value({value})?;"),
            _ if kept => format!("out.view(&{value})?;"),
            Ty::Str => format!("out.view({value})?;"),
            _ => format!("out.view({value})?;"),
        });
    }
    Ok(match ty {
        Ty::Scalar { name } if wide(name) => format!("out.bigint({value});"),
        Ty::U128 | Ty::I128 => format!("out.wide({value});"),
        Ty::Code => format!("out.wide({value}.get());"),
        Ty::Vec { item } if typed(item) && kept => format!("out.blob({value});"),
        Ty::Array { item, .. } if typed(item) => format!("out.small(&{value}[..]);"),
        Ty::Vec { item } | Ty::Slice { item, .. } if typed(item) => {
            format!("out.blob({value}.to_vec());")
        }
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Array { item, .. } | Ty::Set { item } => {
            let walk = if kept {
                value.to_string()
            } else {
                format!("{value}.iter()")
            };
            format!(
                "{{ let list{deep}: Vec<_> = {walk}.into_iter().collect(); out.array(list{deep}.len()); for item{deep} in list{deep} {{ {} }} }}",
                give(book, item, &format!("item{deep}"), kept, deep + 1)?
            )
        }
        Ty::Option { item } => format!(
            "match {value} {{ Some(item{deep}) => {{ {} }} None => out.null(), }}",
            give(book, item, &format!("item{deep}"), kept, deep + 1)?
        ),
        Ty::Tuple { items } => {
            let mut parts = String::new();
            for (index, item) in items.iter().enumerate() {
                let one = if kept {
                    format!("parts{deep}.{index}")
                } else {
                    format!("(&parts{deep}.{index})")
                };
                parts.push_str(&give(book, item, &one, kept, deep + 1)?);
            }
            format!(
                "{{ let parts{deep} = {value}; out.array({}); {parts} }}",
                items.len()
            )
        }
        Ty::Ref { item, .. } => give(book, item, value, false, deep)?,
        Ty::Result { item } => give(book, item, value, kept, deep)?,
        Ty::Map { value: held, .. } => {
            let walk = if kept {
                value.to_string()
            } else {
                format!("{value}.iter()")
            };
            format!(
                "{{ let pairs{deep}: Vec<_> = {walk}.into_iter().collect(); out.map(pairs{deep}.len()); for (key{deep}, item{deep}) in pairs{deep} {{ out.text(&key{deep}.to_string()); {} }} }}",
                give(book, held, &format!("item{deep}"), kept, deep + 1)?
            )
        }
        Ty::Hand { name, .. } if colored(name) => {
            format!("{{ let one = {value}; out.rgba(one.r, one.g, one.b, one.a); }}")
        }
        Ty::Hand { name, .. } => {
            let row = row(name)?;
            if row.stream {
                let one = if kept {
                    value.to_string()
                } else {
                    format!("{value}.clone()")
                };
                format!("out.held({one}, \"Rng\");")
            } else if row.big {
                format!("out.wide({value}.get());")
            } else if kept {
                format!("out.value({value})?;")
            } else {
                format!("out.view({value})?;")
            }
        }
        Ty::Class { path, .. } if book.holds(path) => {
            let one = match (kept, book.copy(path)) {
                (true, _) => value.to_string(),
                (false, true) => format!("*{value}"),
                (false, false) => format!("{value}.clone()"),
            };
            let class = path.replace("::", ".");
            if book.derives(path, "Serialize") {
                format!("if on() {{ out.held({one}, {class:?}); }} else {{ out.view(&{one})?; }}")
            } else {
                format!("out.held({one}, {class:?});")
            }
        }
        Ty::Class { .. } => {
            if kept {
                format!("out.value({value})?;")
            } else {
                format!("out.view({value})?;")
            }
        }
        Ty::Opaque { path } => return Err(format!("{path} does not cross")),
        Ty::Unknown { text } => return Err(format!("{text} does not cross")),
        other => return Err(format!("{other:?} cannot be written")),
    })
}

fn class_handle(book: &Book, path: &str, at: usize, mutable: bool) -> String {
    let ty = rust(path);
    let slot = format!("&args[{at}]");
    let reader = if mutable { "held_mut" } else { "held" };
    let parse = if book.derives(path, "Deserialize") {
        format!(
            "{{ #[cfg(target_arch = \"wasm32\")] {{ return Err(usage(format!(\"{{name}} argument {}: an object\"))); }} #[cfg(not(target_arch = \"wasm32\"))] {{ bare(take(name, {at}, {slot})?) }} }}",
            at + 1
        )
    } else {
        format!(
            "{{ return Err(usage(format!(\"{{name}} argument {}: an object\"))); }}",
            at + 1
        )
    };
    format!("match {reader}::<{ty}>(name, {at}, {slot})? {{ Some(one) => one, None => {parse} }}")
}

fn stream_handle(book: &Book, item: &Ty, at: usize, dim: Option<u8>) -> Result<String> {
    let ty = owned(book, item, dim)?;
    let slot = format!("&args[{at}]");
    Ok(format!(
        "match held_mut::<{ty}>(name, {at}, {slot})? {{ Some(one) => one, None => bare({ty}::new(({slot}).seed(name, {at})?)) }}"
    ))
}

fn hold(book: &Book, ty: &Ty, at: usize, dim: Option<u8>, backs: &mut Vec<usize>) -> Result<Bind> {
    let slot = format!("&args[{at}]");
    if let Ty::Ref {
        mutable: true,
        item,
        ..
    } = ty
    {
        if stream(item) {
            return Ok(Bind::Open(
                format!("let mut a{at} = {};", stream_handle(book, item, at, dim)?),
                format!("&mut *a{at}"),
            ));
        }
        if let Ty::Class { path, .. } = item.as_ref()
            && book.holds(path)
        {
            return Ok(Bind::Open(
                format!("let mut a{at} = {};", class_handle(book, path, at, true)),
                format!("&mut *a{at}"),
            ));
        }
        if row_of(item).is_some_and(|row| row.write_back.is_some()) {
            let start = read(book, item, at, &slot, 0)?;
            let kind = owned(book, item, dim)?;
            backs.push(at);
            return Ok(Bind::Open(
                format!("let mut a{at}: {kind} = {start};"),
                format!("&mut a{at}"),
            ));
        }
        return Ok(Bind::Shut("mutates a value that cannot cross".to_string()));
    }
    if let Ty::Option { item } = ty
        && let Ty::Ref {
            mutable: true,
            item: held,
            ..
        } = item.as_ref()
    {
        if stream(held) {
            return Ok(Bind::Open(
                format!(
                    "let mut a{at} = if ({slot}).is_null() {{ None }} else {{ Some({}) }};",
                    stream_handle(book, held, at, dim)?
                ),
                format!("a{at}.as_deref_mut()"),
            ));
        }
        return Ok(Bind::Shut("mutates a value that cannot cross".to_string()));
    }
    if let Ty::Ref {
        mutable: false,
        item,
        ..
    } = ty
        && let Ty::Class { path, .. } = item.as_ref()
        && book.holds(path)
    {
        return Ok(Bind::Open(
            format!("let a{at} = {};", class_handle(book, path, at, false)),
            format!("&*a{at}"),
        ));
    }
    if let Some(reason) = lacks(book, ty, "Deserialize") {
        return Ok(Bind::Shut(reason));
    }
    let start = match read(book, ty, at, &slot, 0) {
        Ok(text) => text,
        Err(reason) => return Ok(Bind::Shut(reason)),
    };
    let mut lets = match owned(book, ty, dim) {
        Ok(text) if text != "_" => format!("let a{at}: {text} = {start};"),
        Ok(_) => format!("let a{at} = {start};"),
        Err(reason) => return Ok(Bind::Shut(reason)),
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

fn probe(book: &Book, function: &Function) -> Result<String> {
    let args = args_of(book, function)?;
    for (at, (_, ty)) in args.iter().enumerate() {
        let inner = bare(ty);
        if undecided(inner) {
            return Ok(format!("args.rank(name, {at})?"));
        }
        if let Ty::Slice { item, .. } | Ty::Vec { item } = inner
            && undecided(bare(item))
        {
            return Ok(format!("args.ranks(name, {at})?"));
        }
    }
    for (at, (_, ty)) in args.iter().enumerate() {
        if ranked(bare(ty)) {
            return Ok(format!("args.rank(name, {at})?"));
        }
    }
    Err(format!("{}: nothing to rank", function.path))
}

fn suffix_dim(name: &str) -> Option<u8> {
    if name.ends_with("_2d") {
        Some(2)
    } else if name.ends_with("_3d") || name.ends_with("_6d") {
        Some(3)
    } else {
        None
    }
}

fn one(book: &Book, function: &Function, dim: Option<u8>) -> Result<Made> {
    let args = args_of(book, function)?;
    let sized = dim.or_else(|| suffix_dim(&function.path));
    let mut lines = vec![format!("if !on() {{ args.count(name, {})?; }}", args.len())];
    let mut passes = Vec::new();
    let mut backs = Vec::new();
    for (at, (_, ty)) in args.iter().enumerate() {
        match hold(book, ty, at, sized, &mut backs)? {
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
    let mut head = String::new();
    if !backs.is_empty() {
        write!(head, "out.back({});", backs.len()).ok();
        for at in &backs {
            write!(head, "out.at({at}); out.view(&a{at})?;").ok();
        }
    }
    let tail = match &function.ret {
        Ty::Result { item } => {
            let good = match give(book, item, "value", true, 0) {
                Ok(text) => text,
                Err(reason) => return Ok(Made::Shut(reason)),
            };
            let bind = if matches!(item.as_ref(), Ty::Unit) {
                "Ok(())"
            } else {
                "Ok(value)"
            };
            format!(
                "match {call} {{ {bind} => {{ {head} {good} }} Err(error) => return Err(library(error)), }}"
            )
        }
        Ty::Unit => format!("{call}; {head} out.null();"),
        other => match give(book, other, "value", true, 0) {
            Ok(text) => format!("let value = {call}; {head} {text}"),
            Err(reason) => return Ok(Made::Shut(reason)),
        },
    };
    lines.push(tail);
    lines.push("Ok(())".to_string());
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

fn number(ty: &Ty) -> Option<&str> {
    match ty {
        Ty::Scalar { name } if NUMBER.contains(&name.as_str()) => Some(name),
        _ => None,
    }
}

fn direct(book: &Book, group: &[&Function], id: usize) -> Result<Option<(String, Direct)>> {
    let [function] = group else { return Ok(None) };
    if !function.dims.is_empty() {
        return Ok(None);
    }
    let (ret, fallible) = match &function.ret {
        Ty::Result { item } => (item.as_ref(), true),
        other => (other, false),
    };
    let Some(ret) = number(ret) else {
        return Ok(None);
    };
    let args = args_of(book, function)?;
    let mut lines = String::new();
    let mut passes = Vec::new();
    let mut owner = false;
    let mut raw = Vec::new();
    for (at, (_, ty)) in args.iter().enumerate() {
        raw.push(format!("r{at}: f64"));
        if let Some(name) = number(ty) {
            let cast = if name == "bool" {
                format!("r{at} != 0.0")
            } else {
                format!("r{at} as {name}")
            };
            writeln!(lines, "let a{at}: {name} = {cast};").ok();
            passes.push(format!("a{at}"));
            continue;
        }
        let Ty::Ref { mutable, item, .. } = ty else {
            return Ok(None);
        };
        let Ty::Class { path, .. } = item.as_ref() else {
            return Ok(None);
        };
        if at != 0 || !book.holds(path) {
            return Ok(None);
        }
        owner = true;
        let reader = if *mutable { "held_mut_id" } else { "held_id" };
        let kind = rust(path);
        let binding = if *mutable { "let mut" } else { "let" };
        writeln!(
            lines,
            "{binding} a{at} = {reader}::<{kind}>(r{at} as u32)?;"
        )
        .ok();
        passes.push(if *mutable {
            format!("&mut *a{at}")
        } else {
            format!("&*a{at}")
        });
    }
    let call = format!("{}({})", callee(book, function, None), passes.join(", "));
    let body = if fallible {
        format!("{call}.map_err(library)")
    } else {
        format!("Ok({call})")
    };
    let done = if ret == "bool" {
        "if value { 1.0 } else { 0.0 }"
    } else {
        "value as f64"
    };
    let names: Vec<String> = (0..raw.len()).map(|at| format!("r{at}")).collect();
    let fails = fallible || owner;
    let text = format!(
        "#[cfg(feature = \"abi\")]\n#[unsafe(no_mangle)]\npub extern \"C-unwind\" fn mrly_d{id}({}) -> f64 {{\n    fn inner({}) -> Result<{ret}, Error> {{\n{lines}{body}\n    }}\n    match inner({}) {{\n        Ok(value) => {done},\n        Err(error) => direct_fail(error),\n    }}\n}}\n",
        raw.join(", "),
        raw.join(", "),
        names.join(", ")
    );
    Ok(Some((
        text,
        Direct {
            id,
            boolean: ret == "bool",
            whole: !matches!(ret, "f32" | "f64" | "bool"),
            fails,
            owner,
            count: args.len(),
        },
    )))
}

fn field_ok(book: &Book, field: &Field, owner: &str) -> bool {
    let home = owner.split("::").next();
    let mut ok = true;
    field.ty.walk(&mut |t| match t {
        Ty::Unknown { .. } | Ty::Opaque { .. } => ok = false,
        Ty::Class { path, .. } if !book.holds(path) || path.split("::").next() != home => {
            ok = false
        }
        Ty::Ref {
            lifetime: Some(l),
            item,
            ..
        } if l == "'static" && !matches!(**item, Ty::Str) => ok = false,
        _ => {}
    });
    ok
}

struct Row {
    name: String,
    sign: String,
    docs: String,
    ident: String,
    body: String,
}

type RngDoors = (Vec<Row>, String, BTreeMap<String, Direct>);

fn rng_doors(book: &Book) -> Result<RngDoors> {
    let ty = book.hand("Rng")?;
    let rows = [
        (
            "new",
            format!("let seed = args[0].seed(name, 0)?; out.held({ty}::new(seed), \"Rng\");"),
        ),
        (
            "unit",
            format!("let mut one = {}; out.view(&one.unit())?;", rng_self(&ty)),
        ),
        (
            "below",
            format!(
                "let n: usize = take(name, 1, &args[1])?; let mut one = {}; out.view(&one.below(n))?;",
                rng_self(&ty)
            ),
        ),
        (
            "range",
            format!(
                "let lo: f64 = take(name, 1, &args[1])?; let hi: f64 = take(name, 2, &args[2])?; let mut one = {}; out.view(&(one.range(lo as i64, hi as i64) as f64))?;",
                rng_self(&ty)
            ),
        ),
        (
            "boolean",
            format!(
                "let mut one = {}; out.view(&one.boolean())?;",
                rng_self(&ty)
            ),
        ),
        (
            "chance",
            format!(
                "let p: f64 = take(name, 1, &args[1])?; let mut one = {}; out.view(&one.chance(p))?;",
                rng_self(&ty)
            ),
        ),
        (
            "sample_indices",
            format!(
                "let length: usize = take(name, 1, &args[1])?; let amount: usize = take(name, 2, &args[2])?; let mut one = {}; let picked: Vec<u32> = one.sample_indices(length, amount).into_iter().map(|at| at as u32).collect(); out.blob(picked);",
                rng_self(&ty)
            ),
        ),
        (
            "order",
            format!(
                "let length: u32 = take(name, 1, &args[1])?; let mut one = {}; let mut order: Vec<u32> = (0..length).collect(); one.shuffle(&mut order); out.blob(order);",
                rng_self(&ty)
            ),
        ),
        (
            "__state",
            format!("let one = {}; out.view(&*one)?;", rng_self(&ty)),
        ),
        (
            "__restore",
            format!(
                "let state: {ty} = take(name, 1, &args[1])?; let mut one = {}; *one = state; out.null();",
                rng_self(&ty)
            ),
        ),
    ];
    let mut out = Vec::new();
    for (method, body) in rows {
        out.push(Row {
            name: format!("Rng.{method}"),
            sign: "(...)".to_string(),
            docs: String::new(),
            ident: format!("door_rng_{}", method.trim_start_matches('_')),
            body: format!("{body}\nOk(())"),
        });
    }
    let mut direct = BTreeMap::new();
    let text = format!(
        "#[cfg(feature = \"abi\")]\n#[unsafe(no_mangle)]\npub extern \"C-unwind\" fn mrly_rng_unit(r0: f64) -> f64 {{\n    match held_mut_id::<{ty}>(r0 as u32) {{\n        Ok(mut one) => one.unit(),\n        Err(error) => direct_fail(error),\n    }}\n}}\n#[cfg(feature = \"abi\")]\n#[unsafe(no_mangle)]\npub extern \"C-unwind\" fn mrly_rng_below(r0: f64, r1: f64) -> f64 {{\n    match held_mut_id::<{ty}>(r0 as u32) {{\n        Ok(mut one) => one.below(r1 as usize) as f64,\n        Err(error) => direct_fail(error),\n    }}\n}}\n"
    );
    direct.insert(
        "Rng.unit".to_string(),
        Direct {
            id: usize::MAX,
            boolean: false,
            whole: false,
            fails: true,
            owner: true,
            count: 1,
        },
    );
    direct.insert(
        "Rng.below".to_string(),
        Direct {
            id: usize::MAX - 1,
            boolean: false,
            whole: true,
            fails: true,
            owner: true,
            count: 2,
        },
    );
    Ok((out, text, direct))
}

fn rng_self(ty: &str) -> String {
    format!(
        "match held_mut::<{ty}>(name, 0, &args[0])? {{ Some(one) => one, None => return Err(usage(format!(\"{{name}} argument 1: an Rng\"))) }}"
    )
}

fn const_door(book: &Book, c: &Const) -> Result<String> {
    let path = rust(&c.path);
    Ok(format!(
        "let value = {path}; {}\nOk(())",
        give(book, &c.ty, "value", true, 0)?
    ))
}

fn class_extras(
    book: &Book,
    t: &Type,
    methods: &[String],
    rows: &mut Vec<Row>,
) -> Result<ClassDoors> {
    let name = t.path.replace("::", ".");
    let kind = rust(&t.path);
    let stem = format!("door_x_{}", name.replace('.', "_").to_lowercase());
    let mut fields = Vec::new();
    for field in t.fields.iter().filter(|f| f.public) {
        if !field_ok(book, field, &t.path) || methods.contains(&field.name) {
            continue;
        }
        let f = &field.name;
        let getter = format!(
            "let a0 = {}; {}\nOk(())",
            class_handle(book, &t.path, 0, false),
            give(book, &field.ty, &format!("(&a0.{f})"), false, 0)?
        );
        rows.push(Row {
            name: format!("{name}.{f}$get"),
            sign: format!("(self: {name}) -> {}", show(&field.ty, None)),
            docs: String::new(),
            ident: format!("{stem}_{}_get", f.to_lowercase()),
            body: getter,
        });
        let settable = field.ty.settable();
        if settable {
            let value = read(book, &field.ty, 1, "&args[1]", 0)?;
            rows.push(Row {
                name: format!("{name}.{f}$set"),
                sign: format!("(self: {name}, value: {}) -> null", show(&field.ty, None)),
                docs: String::new(),
                ident: format!("{stem}_{}_set", f.to_lowercase()),
                body: format!(
                    "let mut a0 = {}; a0.{f} = {value}; out.null();\nOk(())",
                    class_handle(book, &t.path, 0, true)
                ),
            });
        }
        fields.push((f.clone(), settable));
    }
    let from = book.derives(&t.path, "Deserialize");
    if from {
        rows.push(Row {
            name: format!("{name}.from"),
            sign: format!("(data) -> {name}"),
            docs: String::new(),
            ident: format!("{stem}_from"),
            body: format!(
                "let one: {kind} = take(name, 0, &args[0])?; out.held(one, {name:?});\nOk(())"
            ),
        });
    }
    let json = book.derives(&t.path, "Serialize");
    if json {
        rows.push(Row {
            name: format!("{name}.toJSON"),
            sign: format!("(self: {name}) -> data"),
            docs: String::new(),
            ident: format!("{stem}_tojson"),
            body: format!(
                "let a0 = {}; out.view(&*a0)?;\nOk(())",
                class_handle(book, &t.path, 0, false)
            ),
        });
    }
    Ok(ClassDoors {
        fields,
        methods: Vec::new(),
        statics: Vec::new(),
        from,
        json,
    })
}

pub fn table(all: &Manifest, own: &Manifest, unit: &str, shapes: &[Vec<String>]) -> Result<Table> {
    let book = Book::new(all, unit);
    let mut groups: BTreeMap<String, Vec<&Function>> = BTreeMap::new();
    for function in &own.functions {
        if function.cross != Cross::Ok {
            continue;
        }
        groups
            .entry(function.path.replace("::", "."))
            .or_default()
            .push(function);
    }
    let mut taken: BTreeMap<String, usize> = BTreeMap::new();
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    let mut direct_map = BTreeMap::new();
    let mut direct_text = String::new();
    let mut classes: BTreeMap<String, ClassDoors> = BTreeMap::new();
    let mut methods: BTreeMap<String, Vec<(String, bool)>> = BTreeMap::new();
    for (name, group) in &groups {
        let mut signs = Vec::new();
        for function in group {
            signs.push(sign(&book, function)?);
        }
        let stem = format!("door_{}", name.replace('.', "_").to_lowercase());
        let count = taken.entry(stem.clone()).or_insert(0);
        *count += 1;
        let ident = match *count {
            1 => stem,
            other => format!("{stem}_{other}"),
        };
        match build(&book, group)? {
            Made::Open(body) => {
                rows.push(Row {
                    name: name.clone(),
                    sign: signs.join(" | "),
                    docs: docs_of(group),
                    ident,
                    body,
                });
                let head = group[0];
                if let Some(owner) = &head.owner {
                    methods
                        .entry(owner.clone())
                        .or_default()
                        .push((head.name.clone(), head.self_kind.is_some()));
                }
                if let Some((text, info)) = direct(&book, group, direct_map.len())? {
                    direct_text.push_str(&text);
                    direct_map.insert(name.clone(), info);
                }
            }
            Made::Shut(reason) => skipped.push((name.clone(), reason)),
        }
    }
    let mut consts = Vec::new();
    for c in &own.consts {
        if c.cross != Cross::Ok {
            continue;
        }
        let name = c.path.replace("::", ".");
        match const_door(&book, c) {
            Ok(body) => {
                rows.push(Row {
                    name: name.clone(),
                    sign: format!("() -> {}", show(&c.ty, None)),
                    docs: String::new(),
                    ident: format!("door_k_{}", name.replace('.', "_").to_lowercase()),
                    body,
                });
                consts.push(name);
            }
            Err(reason) => skipped.push((name, reason)),
        }
    }
    let mut extras = Vec::new();
    for t in all
        .types
        .iter()
        .filter(|t| t.cross == TypeCross::Class && book.holds(&t.path))
    {
        let names: Vec<String> = methods
            .get(&t.path)
            .map(|list| list.iter().map(|(name, _)| name.clone()).collect())
            .unwrap_or_default();
        let mut doors = class_extras(&book, t, &names, &mut extras)?;
        for (method, own_self) in methods.get(&t.path).into_iter().flatten() {
            if *own_self {
                doors.methods.push(method.clone());
            } else {
                doors.statics.push(method.clone());
            }
        }
        classes.insert(t.path.replace("::", "."), doors);
    }
    rows.extend(extras);
    let (rng_rows, rng_text, rng_direct) = rng_doors(&book)?;
    rows.extend(rng_rows);
    direct_text.push_str(&rng_text);
    direct_map.extend(rng_direct);
    rows.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));
    let names: Vec<String> = rows.iter().map(|row| row.name.clone()).collect();
    let mut rust = String::from(
        "#![allow(unused_mut, unused_variables, unused_imports, unused_parens, unused_braces, unreachable_code, non_snake_case, clippy::all)]\n\nuse mrly_wire::*;\n\n",
    );
    rust.push_str("#[cfg(feature = \"abi\")]\nmrly_wire::export!(DOORS, SHAPES);\n\n");
    rust.push_str("pub static SHAPES: &[&[&str]] = &[\n");
    for shape in shapes {
        writeln!(rust, "    &{shape:?},").ok();
    }
    rust.push_str("];\n\npub static DOORS: &[Function] = &[\n");
    for row in &rows {
        writeln!(
            rust,
            "    Function {{ #[cfg(not(target_arch = \"wasm32\"))] name: {:?}, #[cfg(not(target_arch = \"wasm32\"))] signature: {:?}, #[cfg(not(target_arch = \"wasm32\"))] docs: {:?}, door: {} }},",
            row.name, row.sign, row.docs, row.ident
        )
        .ok();
    }
    rust.push_str("];\n");
    for row in &rows {
        writeln!(
            rust,
            "\nfn {}(name: &str, args: &Args, out: &mut Out) -> Result<(), Error> {{\n{}\n}}",
            row.ident, row.body
        )
        .ok();
    }
    rust.push('\n');
    rust.push_str(&direct_text);
    Ok(Table {
        rust,
        names,
        direct: direct_map,
        classes,
        consts,
        skipped,
    })
}
