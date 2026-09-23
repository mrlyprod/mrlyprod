use crate::model::*;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const NATIVE: &str = "mrlypy._mrlypy";

const PYTHON_KEYWORDS: &[&str] = &[
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
    "where", "while", "yield", "try", "gen", "box", "do", "final", "macro", "override", "priv",
    "typeof", "unsized", "virtual", "abstract", "become", "u8", "u16", "u32", "u64", "u128",
    "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32", "f64", "bool", "char", "str",
];

const HAND_SELF: &[(&str, &str)] = &[
    ("Tensor", "tensor"),
    ("Cell", "cell"),
    ("CellNd", "cell"),
    ("Cell6d", "cell"),
    ("Color", "color"),
    ("Code", "code"),
    ("Rng", "rng"),
];

const HEADER: &str = "#![allow(clippy::too_many_arguments)]\n";

const HAND_NAMES: &[&str] = &[
    "ok", "PyCell", "PyCell2d", "PyCell3d", "PyCell6d", "PyCellNd", "PyCode", "PyColor",
    "PyPixels", "PyRgba", "PyRng", "PySerde", "PyTensor",
];

pub fn write(manifest: &Manifest, root: &Path) -> Result<()> {
    let cx = Cx::new(manifest);
    let tree = build(&cx)?;
    let pkg = root.join("pkgs/mrlypy");
    save(&pkg.join("src/lib.rs"), &lib_file())?;
    save(&pkg.join("src/gen.rs"), &rust_file(&cx, &tree)?)?;
    let python = pkg.join("python/mrlypy");
    python_files(&cx, &tree, &python)?;
    save(&python.join("py.typed"), "")?;
    save(&pkg.join("tests/test_manifest.py"), &test_file(&cx))?;
    Ok(())
}

fn save(path: &Path, text: &str) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

// NAMES

fn parent(path: &str) -> &str {
    path.rsplit_once("::").map(|(head, _)| head).unwrap_or("")
}

fn last(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
}

fn dotted(path: &str) -> String {
    path.replace("::", ".")
}

fn py_name(name: &str) -> String {
    if PYTHON_KEYWORDS.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

fn rust_ident(name: &str) -> String {
    if RUST_KEYWORDS.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

fn snake(name: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = name.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if c.is_uppercase() && i > 0 && !chars[i - 1].is_uppercase() {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

fn quote(text: &str) -> String {
    format!("{text:?}")
}

fn py_str(text: &str) -> String {
    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn indent(depth: usize) -> String {
    "    ".repeat(depth)
}

// CONTEXT

struct Cx<'a> {
    manifest: &'a Manifest,
    types: BTreeMap<&'a str, &'a Type>,
}

impl<'a> Cx<'a> {
    fn new(manifest: &'a Manifest) -> Cx<'a> {
        let types = manifest
            .types
            .iter()
            .map(|t| (t.path.as_str(), t))
            .collect();
        Cx { manifest, types }
    }

    fn ty(&self, path: &str) -> Result<&'a Type> {
        self.types
            .get(path)
            .copied()
            .ok_or_else(|| format!("{path} is not a type in the manifest"))
    }

    fn wrapper(&self, path: &str) -> String {
        let mut segs: Vec<String> = parent(path)
            .split("::")
            .filter(|s| !s.is_empty())
            .map(rust_ident)
            .collect();
        segs.push(last(path).to_string());
        format!("crate::gen::{}", segs.join("::"))
    }

    fn const_generic(&self, path: &str) -> bool {
        self.types.get(path).is_some_and(|t| t.const_generic)
    }

    fn derives(&self, path: &str, derive: &str) -> bool {
        self.types
            .get(path)
            .is_some_and(|t| t.derives.iter().any(|d| d == derive))
    }

    fn is_copy(&self, ty: &Ty) -> bool {
        match ty {
            Ty::Scalar { .. } | Ty::U128 | Ty::I128 | Ty::Code => true,
            Ty::Hand { name, .. } => name == "Color" || name == "Code",
            Ty::Tuple { items } => items.iter().all(|t| self.is_copy(t)),
            Ty::Array { item, .. } | Ty::Option { item } => self.is_copy(item),
            Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } => {
                self.derives(path, "Copy")
            }
            _ => false,
        }
    }

    fn words(&self, path: &str) -> Vec<String> {
        let Some(ty) = self.types.get(path) else {
            return Vec::new();
        };
        let rename_all = ty.serde.iter().find_map(|s| {
            s.strip_prefix("rename_all = ")
                .map(|v| v.trim_matches('"').to_string())
        });
        ty.variants
            .iter()
            .map(|v| {
                if let Some(word) = v.serde.iter().find_map(|s| {
                    s.strip_prefix("rename = ")
                        .map(|w| w.trim_matches('"').to_string())
                }) {
                    return word;
                }
                match rename_all.as_deref() {
                    Some("lowercase") => v.name.to_lowercase(),
                    Some("UPPERCASE") => v.name.to_uppercase(),
                    Some("snake_case") => snake(&v.name),
                    Some("kebab-case") => snake(&v.name).replace('_', "-"),
                    _ => v.name.clone(),
                }
            })
            .collect()
    }
}

// TREE

#[derive(Default)]
struct Node<'a> {
    path: String,
    docs: Vec<String>,
    children: BTreeMap<String, Node<'a>>,
    exports: Vec<Export<'a>>,
    classes: Vec<Owned<'a>>,
    holders: Vec<Owned<'a>>,
    consts: Vec<&'a Const>,
}

struct Export<'a> {
    name: String,
    variants: Vec<(&'a Function, Option<u8>)>,
}

struct Owned<'a> {
    ty: &'a Type,
    fns: Vec<&'a Function>,
}

impl<'a> Node<'a> {
    fn name(&self) -> &str {
        last(&self.path)
    }

    fn reach(&mut self, path: &str) -> &mut Node<'a> {
        if path.is_empty() {
            return self;
        }
        let mut node = self;
        let mut sofar = String::new();
        for seg in path.split("::") {
            if !sofar.is_empty() {
                sofar.push_str("::");
            }
            sofar.push_str(seg);
            let full = sofar.clone();
            node = node
                .children
                .entry(seg.to_string())
                .or_insert_with(|| Node {
                    path: full,
                    ..Node::default()
                });
        }
        node
    }

    fn push_export(&mut self, f: &'a Function) -> Result<()> {
        let name = last(&f.path).to_string();
        let dims: Vec<Option<u8>> = if f.dims.is_empty() {
            vec![None]
        } else {
            f.dims.iter().map(|d| Some(*d)).collect()
        };
        let path = self.path.clone();
        let index = match self.exports.iter().position(|e| e.name == name) {
            Some(index) => index,
            None => {
                self.exports.push(Export {
                    name: name.clone(),
                    variants: Vec::new(),
                });
                self.exports.len() - 1
            }
        };
        let export = &mut self.exports[index];
        for dim in dims {
            let clash = export
                .variants
                .iter()
                .any(|(_, d)| d.is_none() || *d == dim)
                || (dim.is_none() && !export.variants.is_empty());
            if clash {
                return Err(format!("{name} is exported twice in {path}"));
            }
            export.variants.push((f, dim));
        }
        export.variants.sort_by_key(|(_, d)| *d);
        Ok(())
    }

    fn prune(&mut self) -> bool {
        self.children.retain(|_, child| child.prune());
        !self.children.is_empty()
            || !self.exports.is_empty()
            || !self.classes.is_empty()
            || !self.holders.is_empty()
            || !self.consts.is_empty()
    }

    fn item_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.exports.iter().map(|e| py_name(&e.name)).collect();
        names.extend(self.classes.iter().map(|c| c.ty.name.clone()));
        names.extend(self.holders.iter().map(|h| h.ty.name.clone()));
        names.extend(self.consts.iter().map(|c| c.name.clone()));
        if self.path == "core" {
            names.push("Rng".to_string());
        }
        names
    }

    fn check(&self) -> Result<()> {
        let names = self.item_names();
        for (i, name) in names.iter().enumerate() {
            if names[..i].contains(name) {
                return Err(format!(
                    "{name} is named twice in mrlypy.{}",
                    dotted(&self.path)
                ));
            }
            if self.children.contains_key(name) {
                return Err(format!(
                    "{name} is both an item and a module in mrlypy.{}",
                    dotted(&self.path)
                ));
            }
        }
        for class in &self.classes {
            let mut seen: Vec<String> = Vec::new();
            for field in class
                .ty
                .fields
                .iter()
                .filter(|f| f.public && crossable(&f.ty))
            {
                seen.push(py_name(&field.name));
            }
            for f in &class.fns {
                let name = py_name(&f.name);
                if seen.contains(&name) {
                    return Err(format!(
                        "{} has a field and a method named {name}",
                        class.ty.path
                    ));
                }
                seen.push(name);
            }
            for extra in ["from_dict", "to_dict"] {
                if seen.iter().any(|s| s == extra) {
                    return Err(format!("{} already has {extra}", class.ty.path));
                }
            }
        }
        self.children.values().try_for_each(Node::check)
    }

    fn walk<'s>(&'s self, out: &mut Vec<&'s Node<'a>>) {
        out.push(self);
        for child in self.children.values() {
            child.walk(out);
        }
    }
}

fn crossable(ty: &Ty) -> bool {
    !ty.any(&|t| matches!(t, Ty::Opaque { .. } | Ty::Unknown { .. }))
}

fn build<'a>(cx: &Cx<'a>) -> Result<Node<'a>> {
    let mut root = Node::default();
    for m in &cx.manifest.modules {
        root.reach(&m.path).docs = m.docs.clone();
    }
    let mut owned: BTreeMap<&str, Vec<&'a Function>> = BTreeMap::new();
    for f in cx
        .manifest
        .functions
        .iter()
        .filter(|f| f.cross == Cross::Ok)
    {
        match &f.owner {
            None => root.reach(&f.module).push_export(f)?,
            Some(owner) => match &cx.ty(owner)?.cross {
                TypeCross::Hand { .. } => root.reach(&f.module).push_export(f)?,
                TypeCross::Uncrossable { .. } => {
                    return Err(format!("{} is owned by an uncrossable type", f.path))
                }
                _ => owned.entry(owner.as_str()).or_default().push(f),
            },
        }
    }
    for ty in &cx.manifest.types {
        let fns = owned.remove(ty.path.as_str()).unwrap_or_default();
        match &ty.cross {
            TypeCross::Class => root.reach(parent(&ty.path)).classes.push(Owned { ty, fns }),
            TypeCross::Plain | TypeCross::Enum { .. } if !fns.is_empty() => {
                root.reach(parent(&ty.path)).holders.push(Owned { ty, fns })
            }
            _ => {}
        }
    }
    if let Some((owner, _)) = owned.iter().next() {
        return Err(format!("{owner} owns functions but is no type"));
    }
    for c in cx.manifest.consts.iter().filter(|c| c.cross == Cross::Ok) {
        root.reach(parent(&c.path)).consts.push(c);
    }
    root.prune();
    if !root.exports.is_empty() || !root.classes.is_empty() || !root.consts.is_empty() {
        return Err("the crate root holds items the Python bridge has no module for".into());
    }
    root.check()?;
    Ok(root)
}

// TYPES

fn is_u8(ty: &Ty) -> bool {
    matches!(ty, Ty::Scalar { name } if name == "u8")
}

fn is_rgba(ty: &Ty) -> bool {
    matches!(ty, Ty::Array { item, len: 4 } if is_u8(item))
}

fn hand_name(ty: &Ty) -> Option<&str> {
    match ty {
        Ty::Hand { name, .. } => Some(name),
        _ => None,
    }
}

fn subst(ty: &Ty, dim: u8) -> Ty {
    match ty {
        Ty::Hand { name, dim: None } if name == "CellNd" => Ty::Hand {
            name: name.clone(),
            dim: Some(dim),
        },
        Ty::Vec { item } => Ty::Vec {
            item: Box::new(subst(item, dim)),
        },
        Ty::Slice { mutable, item } => Ty::Slice {
            mutable: *mutable,
            item: Box::new(subst(item, dim)),
        },
        Ty::Option { item } => Ty::Option {
            item: Box::new(subst(item, dim)),
        },
        Ty::Tuple { items } => Ty::Tuple {
            items: items.iter().map(|t| subst(t, dim)).collect(),
        },
        Ty::Array { item, len } => Ty::Array {
            item: Box::new(subst(item, dim)),
            len: *len,
        },
        Ty::Ref {
            mutable,
            lifetime,
            item,
        } => Ty::Ref {
            mutable: *mutable,
            lifetime: lifetime.clone(),
            item: Box::new(subst(item, dim)),
        },
        Ty::Result { item } => Ty::Result {
            item: Box::new(subst(item, dim)),
        },
        Ty::Map { key, value } => Ty::Map {
            key: Box::new(subst(key, dim)),
            value: Box::new(subst(value, dim)),
        },
        Ty::Set { item } => Ty::Set {
            item: Box::new(subst(item, dim)),
        },
        other => other.clone(),
    }
}

fn undecided(ty: &Ty) -> bool {
    ty.any(&|t| matches!(t, Ty::Hand { name, dim: None } if name == "CellNd"))
}

fn has_tensor(ty: &Ty) -> bool {
    ty.any(&|t| matches!(t, Ty::Hand { name, .. } if name == "Tensor"))
}

fn hand_decl(name: &str, dim: Option<u8>) -> Result<String> {
    Ok(match name {
        "Tensor" => "PyTensor",
        "Cell" => "PyCell",
        "CellNd" => match dim {
            Some(2) => "PyCell2d",
            Some(3) => "PyCell3d",
            _ => return Err("a cell of undecided dimension".into()),
        },
        "Cell6d" => "PyCell6d",
        "Color" => "PyColor",
        "Code" => "PyCode",
        "Rng" => "PyRng",
        other => return Err(format!("no hand crossing for {other}")),
    }
    .to_string())
}

fn hand_into(name: &str) -> Result<&'static str> {
    Ok(match name {
        "Tensor" => "PyTensor",
        "Cell" => "PyCell",
        "CellNd" => "PyCellNd",
        "Cell6d" => "PyCell6d",
        "Color" => "PyColor",
        "Code" => "PyCode",
        "Rng" => "PyRng",
        other => return Err(format!("no hand crossing for {other}")),
    })
}

fn decl(cx: &Cx, ty: &Ty) -> Result<String> {
    Ok(match ty {
        Ty::Scalar { name } => name.clone(),
        Ty::Str | Ty::String => "String".into(),
        Ty::U128 => "u128".into(),
        Ty::I128 => "i128".into(),
        Ty::Code => "PyCode".into(),
        Ty::Json => "PySerde<serde_json::Value>".into(),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => {
            if is_rgba(item) {
                "PyPixels".into()
            } else {
                format!("Vec<{}>", decl(cx, item)?)
            }
        }
        Ty::Option { item } => format!("Option<{}>", decl(cx, item)?),
        Ty::Tuple { items } => {
            let parts = items
                .iter()
                .map(|t| decl(cx, t))
                .collect::<Result<Vec<_>>>()?;
            format!("({})", parts.join(", "))
        }
        Ty::Array { item, len } => {
            if is_rgba(ty) {
                "PyRgba".into()
            } else {
                format!("[{}; {len}]", decl(cx, item)?)
            }
        }
        Ty::Ref {
            mutable: false,
            item,
            ..
        } => match &**item {
            Ty::Class { .. } => return Err("a class by reference inside a container".into()),
            other => decl(cx, other)?,
        },
        Ty::Ref { mutable: true, .. } => return Err("a mutable borrow inside a container".into()),
        Ty::Map { key, value } => format!(
            "std::collections::HashMap<{}, {}>",
            decl(cx, key)?,
            decl(cx, value)?
        ),
        Ty::Hand { name, dim } => hand_decl(name, *dim)?,
        Ty::Plain { path } => {
            if cx.const_generic(path) {
                return Err(format!("{path} is const generic inside a container"));
            }
            format!("PySerde<mrlyrs::{path}>")
        }
        Ty::Enum { path } => format!("PySerde<mrlyrs::{path}>"),
        Ty::Class { path, .. } => cx.wrapper(path),
        Ty::Unit | Ty::Result { .. } | Ty::Opaque { .. } | Ty::Unknown { .. } => {
            return Err(format!("{ty:?} cannot cross as a parameter"))
        }
    })
}

fn own(ty: &Ty, var: &str) -> Option<String> {
    match ty {
        Ty::Code
        | Ty::Json
        | Ty::Hand { .. }
        | Ty::Plain { .. }
        | Ty::Enum { .. }
        | Ty::Class { .. } => Some(format!("{var}.0")),
        Ty::Vec { item } | Ty::Slice { item, .. } => {
            if is_rgba(item) {
                Some(format!("{var}.0"))
            } else {
                own(item, "x")
                    .map(|e| format!("{var}.into_iter().map(|x| {e}).collect::<Vec<_>>()"))
            }
        }
        Ty::Set { item } => Some(match own(item, "x") {
            Some(e) => format!("{var}.into_iter().map(|x| {e}).collect()"),
            None => format!("{var}.into_iter().collect()"),
        }),
        Ty::Option { item } => own(item, "x").map(|e| format!("{var}.map(|x| {e})")),
        Ty::Tuple { items } => {
            let mut any = false;
            let mut parts = Vec::new();
            for (i, t) in items.iter().enumerate() {
                let slot = format!("t.{i}");
                match own(t, &slot) {
                    Some(e) => {
                        any = true;
                        parts.push(e);
                    }
                    None => parts.push(slot),
                }
            }
            any.then(|| format!("{{ let t = {var}; ({}) }}", parts.join(", ")))
        }
        Ty::Array { item, .. } => {
            if is_rgba(ty) {
                Some(format!("{var}.0"))
            } else {
                own(item, "x").map(|e| format!("{var}.map(|x| {e})"))
            }
        }
        Ty::Map { value, .. } => Some(match own(value, "v") {
            Some(e) => format!("{var}.into_iter().map(|(k, v)| (k, {e})).collect()"),
            None => format!("{var}.into_iter().collect()"),
        }),
        Ty::Ref { item, .. } => own(item, var),
        _ => None,
    }
}

fn needs_view(ty: &Ty) -> bool {
    match ty {
        Ty::Str | Ty::Slice { .. } | Ty::Ref { .. } => true,
        Ty::Tuple { items } => items.iter().any(needs_view),
        _ => false,
    }
}

fn view(cx: &Cx, ty: &Ty, x: &str) -> Result<String> {
    Ok(match ty {
        Ty::Str => format!("{x}.as_str()"),
        Ty::Slice { item, .. } => {
            if needs_view(item) {
                return Err("a borrow two levels deep".into());
            }
            format!("{x}.as_slice()")
        }
        Ty::Ref { item, .. } => {
            if needs_view(item) {
                return Err("a borrow two levels deep".into());
            }
            x.to_string()
        }
        Ty::Tuple { items } => {
            let mut parts = Vec::new();
            for (i, t) in items.iter().enumerate() {
                let slot = format!("{x}.{i}");
                parts.push(if needs_view(t) {
                    view(cx, t, &slot)?
                } else if cx.is_copy(t) {
                    slot
                } else {
                    format!("{slot}.clone()")
                });
            }
            format!("({})", parts.join(", "))
        }
        _ => format!("{x}.clone()"),
    })
}

#[derive(Default)]
struct Plan {
    decl: String,
    lets: Vec<String>,
    pass: String,
    post: Vec<String>,
    optional: bool,
}

fn value(cx: &Cx, var: &str, ty: &Ty, p: &mut Plan) -> Result<()> {
    if let Ty::Plain { path } = ty {
        if cx.const_generic(path) {
            p.decl = "&Bound<'_, PyAny>".into();
            p.lets
                .push(format!("let {var} = crate::hand::serde_from_py({var})?;"));
            return Ok(());
        }
    }
    p.decl = decl(cx, ty)?;
    if let Some(e) = own(ty, var) {
        p.lets.push(format!("let {var} = {e};"));
    }
    if let Ty::Slice { item, .. } | Ty::Vec { item } = ty {
        if needs_view(item) {
            p.lets.push(format!(
                "let {var} = {var}.iter().map(|y| {}).collect::<Vec<_>>();",
                view(cx, item, "y")?
            ));
        }
    }
    Ok(())
}

fn plan(cx: &Cx, var: &str, ty: &Ty) -> Result<Plan> {
    let mut p = Plan {
        optional: matches!(ty, Ty::Option { .. }),
        ..Plan::default()
    };
    match ty {
        Ty::Str => {
            p.decl = "&str".into();
            p.pass = var.into();
        }
        Ty::Ref {
            mutable: true,
            item,
            ..
        } => match &**item {
            Ty::Hand { name, .. } if name == "Rng" => {
                p.decl = "&mut PyRng".into();
                p.pass = format!("&mut {var}.0");
            }
            Ty::Hand { name, .. } if name == "Tensor" || name == "Cell" => {
                let kind = name.to_lowercase();
                p.decl = "&Bound<'_, PyAny>".into();
                p.lets.push(format!(
                    "let mut {var}_owned = crate::hand::{kind}_from_py({var})?;"
                ));
                p.pass = format!("&mut {var}_owned");
                let carried = if name == "Cell" {
                    format!("{var}_owned")
                } else {
                    format!("&{var}_owned")
                };
                p.post.push(format!(
                    "crate::hand::{kind}_write_back({var}, {carried})?;"
                ));
            }
            Ty::Class { path, .. } => {
                p.decl = format!("PyRefMut<'_, {}>", cx.wrapper(path));
                p.lets.push(format!("let mut {var} = {var};"));
                p.pass = format!("&mut {var}.0");
            }
            other => return Err(format!("{other:?} cannot cross by mutable borrow")),
        },
        Ty::Ref {
            mutable: false,
            item,
            ..
        } => match &**item {
            Ty::Class { path, .. } => {
                p.decl = format!("PyRef<'_, {}>", cx.wrapper(path));
                p.pass = format!("&{var}.0");
            }
            Ty::Str => {
                p.decl = "&str".into();
                p.pass = var.into();
            }
            other => {
                value(cx, var, other, &mut p)?;
                p.pass = format!("&{var}");
            }
        },
        Ty::Slice { .. } => {
            value(cx, var, ty, &mut p)?;
            p.pass = format!("&{var}");
        }
        Ty::Option { item } => match &**item {
            Ty::Ref {
                mutable: true,
                item: inner,
                ..
            } if hand_name(inner) == Some("Rng") => {
                p.decl = "Option<&mut PyRng>".into();
                p.pass = format!("{var}.map(|r| &mut r.0)");
            }
            Ty::Ref {
                mutable: false,
                item: inner,
                ..
            } if !matches!(**inner, Ty::Class { .. }) => {
                value(cx, var, ty, &mut p)?;
                p.pass = format!("{var}.as_ref()");
            }
            Ty::Slice { .. } => {
                value(cx, var, ty, &mut p)?;
                p.pass = format!("{var}.as_deref()");
            }
            _ => {
                value(cx, var, ty, &mut p)?;
                p.pass = var.into();
            }
        },
        _ => {
            value(cx, var, ty, &mut p)?;
            p.pass = var.into();
        }
    }
    Ok(p)
}

fn needs_into(ty: &Ty) -> bool {
    ty.any(&|t| {
        is_rgba(t)
            || matches!(
                t,
                Ty::Result { .. }
                    | Ty::Ref { .. }
                    | Ty::Str
                    | Ty::Code
                    | Ty::Json
                    | Ty::Hand { .. }
                    | Ty::Plain { .. }
                    | Ty::Enum { .. }
                    | Ty::Class { .. }
                    | Ty::Set { .. }
            )
    })
}

fn into(cx: &Cx, ty: &Ty, e: &str) -> Result<String> {
    Ok(match ty {
        Ty::Result { item } => into(cx, item, &format!("ok({e})?"))?,
        Ty::Ref { item, .. } if matches!(**item, Ty::Str) => into(cx, item, e)?,
        Ty::Ref { item, .. } => into(cx, item, &format!("({e}).clone()"))?,
        Ty::Str => format!("({e}).to_string()"),
        Ty::Code => format!("PyCode({e})"),
        Ty::Json => format!("PySerde({e})"),
        Ty::Hand { name, .. } => format!("{}({e})", hand_into(name)?),
        Ty::Plain { .. } | Ty::Enum { .. } => format!("PySerde({e})"),
        Ty::Class { path, .. } => format!("{}({e})", cx.wrapper(path)),
        Ty::Array { .. } if is_rgba(ty) => format!("PyRgba({e})"),
        Ty::Vec { item } if is_rgba(item) => format!("PyPixels({e})"),
        Ty::Slice { item, .. } if is_rgba(item) => format!("PyPixels(({e}).to_vec())"),
        Ty::Vec { item } | Ty::Array { item, .. } => {
            if needs_into(item) {
                format!(
                    "({e}).into_iter().map({}).collect::<Vec<_>>()",
                    mapper(into(cx, item, "x")?)
                )
            } else {
                e.into()
            }
        }
        Ty::Slice { item, .. } => into(
            cx,
            &Ty::Vec { item: item.clone() },
            &format!("({e}).to_vec()"),
        )?,
        Ty::Set { item } => format!(
            "({e}).into_iter().map({}).collect::<Vec<_>>()",
            mapper(into(cx, item, "x")?)
        ),
        Ty::Option { item } => {
            if needs_into(item) {
                format!("({e}).map({})", mapper(into(cx, item, "x")?))
            } else {
                e.into()
            }
        }
        Ty::Tuple { items } => {
            if items.iter().any(needs_into) {
                let parts = items
                    .iter()
                    .enumerate()
                    .map(|(i, t)| into(cx, t, &format!("t.{i}")))
                    .collect::<Result<Vec<_>>>()?;
                format!("{{ let t = {e}; ({}) }}", parts.join(", "))
            } else {
                e.into()
            }
        }
        Ty::Map { value, .. } => {
            if needs_into(value) {
                format!(
                    "({e}).into_iter().map(|(k, v)| (k, {})).collect::<std::collections::HashMap<_, _>>()",
                    into(cx, value, "v")?
                )
            } else {
                e.into()
            }
        }
        Ty::Unit | Ty::Scalar { .. } | Ty::String | Ty::U128 | Ty::I128 => e.into(),
        Ty::Opaque { .. } | Ty::Unknown { .. } => {
            return Err(format!("{ty:?} cannot cross as a return"))
        }
    })
}

fn mapper(body: String) -> String {
    match body.strip_suffix("(x)") {
        Some(ctor)
            if !ctor.is_empty()
                && ctor
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == ':') =>
        {
            ctor.to_string()
        }
        _ => format!("|x| {body}"),
    }
}

// CALLS

#[derive(Clone, Copy, PartialEq)]
enum Place<'a> {
    Free,
    Method(&'a Type),
    Static(&'a Type),
}

struct Arg {
    py: String,
    rust: String,
    plan: Plan,
    ty: Ty,
}

fn self_ty(kind: SelfKind, inner: Ty) -> Ty {
    match kind {
        SelfKind::Value => inner,
        SelfKind::Ref => Ty::Ref {
            mutable: false,
            lifetime: None,
            item: Box::new(inner),
        },
        SelfKind::Mut => Ty::Ref {
            mutable: true,
            lifetime: None,
            item: Box::new(inner),
        },
    }
}

fn owner_ty(cx: &Cx, owner: &str, dim: Option<u8>) -> Result<Ty> {
    let ty = cx.ty(owner)?;
    Ok(match &ty.cross {
        TypeCross::Hand { name } => Ty::Hand {
            name: name.clone(),
            dim,
        },
        TypeCross::Plain => Ty::Plain {
            path: owner.to_string(),
        },
        TypeCross::Enum { .. } => Ty::Enum {
            path: owner.to_string(),
        },
        TypeCross::Class => Ty::Class {
            path: owner.to_string(),
            dim: None,
        },
        TypeCross::Uncrossable { .. } => return Err(format!("{owner} is uncrossable")),
    })
}

fn self_name(cx: &Cx, f: &Function) -> Result<String> {
    let owner = f.owner.as_deref().ok_or("no owner")?;
    let ty = cx.ty(owner)?;
    let base = match &ty.cross {
        TypeCross::Hand { name } => HAND_SELF
            .iter()
            .find(|(hand, _)| hand == name)
            .map(|(_, word)| word.to_string())
            .unwrap_or_else(|| snake(name)),
        _ => snake(&ty.name),
    };
    let base = py_name(&base);
    if f.params.iter().any(|p| py_name(&p.name) == base) {
        Ok(format!("self_{base}"))
    } else {
        Ok(base)
    }
}

fn raw_types(cx: &Cx, f: &Function, place: Place) -> Result<Vec<Ty>> {
    let mut out = Vec::new();
    if let (Some(kind), Place::Free | Place::Static(_)) = (f.self_kind, place) {
        let owner = f.owner.as_deref().ok_or("no owner")?;
        out.push(self_ty(kind, owner_ty(cx, owner, None)?));
    }
    out.extend(f.params.iter().map(|p| p.ty.clone()));
    Ok(out)
}

fn args(cx: &Cx, f: &Function, place: Place, dim: Option<u8>) -> Result<Vec<Arg>> {
    let mut out = Vec::new();
    if let (Some(kind), Place::Free | Place::Static(_)) = (f.self_kind, place) {
        let owner = f.owner.as_deref().ok_or("no owner")?;
        let ty = self_ty(kind, owner_ty(cx, owner, dim)?);
        let name = self_name(cx, f)?;
        let rust = rust_ident(&name);
        out.push(Arg {
            plan: plan(cx, &rust, &ty)?,
            py: name,
            rust,
            ty,
        });
    }
    for p in &f.params {
        let ty = match dim {
            Some(d) => subst(&p.ty, d),
            None => p.ty.clone(),
        };
        let name = py_name(&p.name);
        let rust = rust_ident(&name);
        out.push(Arg {
            plan: plan(cx, &rust, &ty)?,
            py: name,
            rust,
            ty,
        });
    }
    Ok(out)
}

fn callee(f: &Function, dim: Option<u8>) -> String {
    let turbo = match dim {
        Some(d) if !f.dims.is_empty() => format!("::<{d}>"),
        _ => String::new(),
    };
    match (&f.owner, &f.via) {
        (Some(owner), Some(via)) => {
            format!("<mrlyrs::{owner} as mrlyrs::{via}>::{}", f.name)
        }
        (Some(owner), None) => format!("mrlyrs::{owner}{turbo}::{}", f.name),
        (None, _) => format!("mrlyrs::{}{turbo}", f.path),
    }
}

fn signature(args: &[Arg]) -> String {
    let optional_from = args
        .iter()
        .rposition(|a| !a.plan.optional)
        .map_or(0, |i| i + 1);
    let parts: Vec<String> = args
        .iter()
        .enumerate()
        .map(|(i, a)| {
            if i >= optional_from {
                format!("{}=None", a.rust)
            } else {
                a.rust.clone()
            }
        })
        .collect();
    format!("({})", parts.join(", "))
}

fn summary(docs: &[String]) -> Vec<String> {
    docs.iter()
        .take_while(|line| !line.trim().is_empty())
        .cloned()
        .collect()
}

fn doc_lines(out: &mut String, depth: usize, docs: &[String]) {
    for line in docs {
        if line.is_empty() {
            out.push_str(&format!("{}///\n", indent(depth)));
        } else {
            out.push_str(&format!("{}/// {line}\n", indent(depth)));
        }
    }
}

fn merged_docs(variants: &[&Function]) -> Vec<String> {
    let first = summary(&variants[0].docs);
    let mut docs = first.clone();
    for v in &variants[1..] {
        let more = summary(&v.docs);
        if more != first {
            docs.push(String::new());
            docs.extend(more);
        }
    }
    docs
}

fn body(
    cx: &Cx,
    out: &mut String,
    depth: usize,
    f: &Function,
    place: Place,
    args: &[Arg],
    dim: Option<u8>,
) -> Result<()> {
    let pad = indent(depth);
    for a in args {
        for l in &a.plan.lets {
            out.push_str(&format!("{pad}{l}\n"));
        }
    }
    let mut passes: Vec<String> = Vec::new();
    if let (Place::Method(owner), Some(kind)) = (place, f.self_kind) {
        passes.push(
            match kind {
                SelfKind::Value if cx.derives(&owner.path, "Copy") => "self.0",
                SelfKind::Value => "self.0.clone()",
                SelfKind::Ref => "&self.0",
                SelfKind::Mut => "&mut self.0",
            }
            .to_string(),
        );
    }
    passes.extend(args.iter().map(|a| a.plan.pass.clone()));
    let ret = match dim {
        Some(d) => subst(&f.ret, d),
        None => f.ret.clone(),
    };
    let call = format!("{}({})", callee(f, dim), passes.join(", "));
    if ret == Ty::Unit {
        out.push_str(&format!("{pad}{call};\n"));
    } else {
        out.push_str(&format!("{pad}let out = {call};\n"));
    }
    for a in args {
        for l in &a.plan.post {
            out.push_str(&format!("{pad}{l}\n"));
        }
    }
    let done = if ret == Ty::Unit {
        "()".to_string()
    } else {
        format!("({})", into(cx, &ret, "out")?)
    };
    out.push_str(&format!("{pad}{done}.into_bound_py_any(py)\n"));
    Ok(())
}

fn receiver(place: Place, f: &Function) -> Option<&'static str> {
    match (place, f.self_kind) {
        (Place::Method(_), Some(SelfKind::Mut)) => Some("&mut self"),
        (Place::Method(_), Some(_)) => Some("&self"),
        _ => None,
    }
}

fn emit_simple(
    cx: &Cx,
    out: &mut String,
    depth: usize,
    f: &Function,
    place: Place,
    name: &str,
    dim: Option<u8>,
) -> Result<()> {
    let pad = indent(depth);
    let args = args(cx, f, place, dim)?;
    doc_lines(out, depth, &summary(&f.docs));
    match place {
        Place::Free => out.push_str(&format!("{pad}#[pyfunction]\n")),
        Place::Static(_) => out.push_str(&format!("{pad}#[staticmethod]\n")),
        Place::Method(_) => {}
    }
    out.push_str(&format!(
        "{pad}#[pyo3(name = {}, signature = {})]\n",
        quote(name),
        signature(&args)
    ));
    let mut params: Vec<String> = receiver(place, f).map(str::to_string).into_iter().collect();
    params.push("py: Python<'py>".to_string());
    params.extend(args.iter().map(|a| format!("{}: {}", a.rust, a.plan.decl)));
    let ident = match place {
        Place::Method(_) | Place::Static(_) if name == "new" => "new_".to_string(),
        _ => rust_ident(name),
    };
    out.push_str(&format!(
        "{pad}pub fn {ident}<'py>({}) -> PyResult<Bound<'py, PyAny>> {{\n",
        params.join(", ")
    ));
    body(cx, out, depth + 1, f, place, &args, dim)?;
    out.push_str(&format!("{pad}}}\n"));
    Ok(())
}

fn emit_dispatch(
    cx: &Cx,
    out: &mut String,
    depth: usize,
    pairs: &[(&Function, Option<u8>)],
    place: Place,
    name: &str,
) -> Result<()> {
    let pad = indent(depth);
    let variants: Vec<&Function> = pairs.iter().map(|(f, _)| *f).collect();
    let mut per: Vec<(u8, Vec<Arg>)> = Vec::new();
    for (v, dim) in pairs {
        let dim = dim.ok_or("a dispatch variant without dims")?;
        per.push((dim, args(cx, v, place, Some(dim))?));
    }
    let mut union: Vec<(String, String, String, bool, bool)> = Vec::new();
    for (_, args) in &per {
        for a in args {
            if !union.iter().any(|u| u.0 == a.py) {
                let shared = per
                    .iter()
                    .all(|(_, other)| other.iter().any(|o| o.py == a.py));
                union.push((
                    a.py.clone(),
                    a.rust.clone(),
                    a.plan.decl.clone(),
                    a.plan.optional,
                    shared,
                ));
            }
        }
    }
    let raw = raw_types(cx, variants[0], place)?;
    let undecided_at: Vec<usize> = raw
        .iter()
        .enumerate()
        .filter(|(_, t)| undecided(t))
        .map(|(i, _)| i)
        .collect();
    let pivot = undecided_at
        .first()
        .copied()
        .or_else(|| raw.iter().position(has_tensor))
        .ok_or_else(|| format!("{name} has no cell or tensor to dispatch on"))?;
    let loose: Vec<usize> = if undecided_at.is_empty() {
        vec![pivot]
    } else {
        undecided_at
    };
    let pivot_name = per[0].1[pivot].rust.clone();
    let loose_names: Vec<String> = loose.iter().map(|&i| per[0].1[i].rust.clone()).collect();
    doc_lines(out, depth, &merged_docs(&variants));
    match place {
        Place::Free => out.push_str(&format!("{pad}#[pyfunction]\n")),
        Place::Static(_) => out.push_str(&format!("{pad}#[staticmethod]\n")),
        Place::Method(_) => return Err(format!("{name} dispatches inside a class")),
    }
    let optional_from = union.iter().rposition(|u| !u.3 && u.4).map_or(0, |i| i + 1);
    let sig: Vec<String> = union
        .iter()
        .enumerate()
        .map(|(i, u)| {
            if i >= optional_from {
                format!("{}=None", u.1)
            } else {
                u.1.clone()
            }
        })
        .collect();
    out.push_str(&format!(
        "{pad}#[pyo3(name = {}, signature = ({}))]\n",
        quote(name),
        sig.join(", ")
    ));
    let params: Vec<String> = union
        .iter()
        .map(|u| {
            if loose_names.contains(&u.1) {
                format!("{}: &Bound<'_, PyAny>", u.1)
            } else if u.4 {
                format!("{}: {}", u.1, u.2)
            } else {
                format!("{}: Option<{}>", u.1, u.2)
            }
        })
        .collect();
    out.push_str(&format!(
        "{pad}pub fn {}<'py>(py: Python<'py>, {}) -> PyResult<Bound<'py, PyAny>> {{\n",
        rust_ident(name),
        params.join(", ")
    ));
    let inner = indent(depth + 1);
    out.push_str(&format!(
        "{inner}let dim = crate::hand::ndim({pivot_name})?;\n"
    ));
    out.push_str(&format!("{inner}match dim {{\n"));
    for ((dim, args), v) in per.iter().zip(&variants) {
        let arm = indent(depth + 2);
        let deep = indent(depth + 3);
        out.push_str(&format!("{arm}{dim} => {{\n"));
        for &i in &loose {
            let a = &args[i];
            out.push_str(&format!(
                "{deep}let {} = {}.extract::<{}>()?;\n",
                a.rust, a.rust, a.plan.decl
            ));
        }
        for a in args {
            let shared = union
                .iter()
                .find(|u| u.0 == a.py)
                .map(|u| u.4)
                .unwrap_or(true);
            if !shared {
                out.push_str(&format!(
                    "{deep}let {} = {}.ok_or_else(|| PyValueError::new_err({}))?;\n",
                    a.rust,
                    a.rust,
                    quote(&format!("a {dim}d {name} wants {}.", a.py))
                ));
            }
        }
        body(cx, out, depth + 3, v, place, args, Some(*dim))?;
        out.push_str(&format!("{arm}}}\n"));
    }
    out.push_str(&format!(
        "{}other => Err(PyValueError::new_err(format!({}))),\n",
        indent(depth + 2),
        quote(&format!(
            "{name} wants a 2d or 3d argument, got {{other}}d."
        ))
    ));
    out.push_str(&format!("{inner}}}\n"));
    out.push_str(&format!("{pad}}}\n"));
    Ok(())
}

fn emit_export(
    cx: &Cx,
    out: &mut String,
    depth: usize,
    export: &Export,
    place: Place,
) -> Result<()> {
    let name = py_name(&export.name);
    if export.variants.len() == 1 {
        let (f, dim) = export.variants[0];
        emit_simple(cx, out, depth, f, place, &name, dim)
    } else {
        emit_dispatch(cx, out, depth, &export.variants, place, &name)
    }
}

fn is_constructor(f: &Function) -> bool {
    let owner = f.owner.as_deref().unwrap_or("");
    let returns_self = match &f.ret {
        Ty::Class { path, .. } => path == owner,
        Ty::Result { item } => matches!(&**item, Ty::Class { path, .. } if path == owner),
        _ => false,
    };
    f.name == "new" && f.self_kind.is_none() && returns_self
}

fn emit_constructor(cx: &Cx, out: &mut String, depth: usize, f: &Function) -> Result<()> {
    let pad = indent(depth);
    let args = args(
        cx,
        f,
        Place::Static(cx.ty(f.owner.as_deref().unwrap())?),
        None,
    )?;
    doc_lines(out, depth, &summary(&f.docs));
    out.push_str(&format!("{pad}#[new]\n"));
    out.push_str(&format!("{pad}#[pyo3(signature = {})]\n", signature(&args)));
    let params: Vec<String> = args
        .iter()
        .map(|a| format!("{}: {}", a.rust, a.plan.decl))
        .collect();
    out.push_str(&format!(
        "{pad}pub fn __new__({}) -> PyResult<Self> {{\n",
        params.join(", ")
    ));
    let inner = indent(depth + 1);
    for a in &args {
        for l in &a.plan.lets {
            out.push_str(&format!("{inner}{l}\n"));
        }
    }
    let passes: Vec<String> = args.iter().map(|a| a.plan.pass.clone()).collect();
    out.push_str(&format!(
        "{inner}let out = {}({});\n",
        callee(f, None),
        passes.join(", ")
    ));
    for a in &args {
        for l in &a.plan.post {
            out.push_str(&format!("{inner}{l}\n"));
        }
    }
    let unwrapped = if matches!(f.ret, Ty::Result { .. }) {
        "ok(out)?"
    } else {
        "out"
    };
    out.push_str(&format!("{inner}Ok(Self({unwrapped}))\n"));
    out.push_str(&format!("{pad}}}\n"));
    Ok(())
}

fn emit_class(cx: &Cx, out: &mut String, depth: usize, module: &str, class: &Owned) -> Result<()> {
    let pad = indent(depth);
    let inner = indent(depth + 1);
    let ty = class.ty;
    let clone = cx.derives(&ty.path, "Clone");
    doc_lines(out, depth, &summary(&ty.docs));
    out.push_str(&format!(
        "{pad}#[pyclass(name = {}, module = {}, {})]\n",
        quote(&ty.name),
        quote(&format!("mrlypy.{}", dotted(module))),
        if clone {
            "from_py_object"
        } else {
            "skip_from_py_object"
        }
    ));
    if clone {
        out.push_str(&format!("{pad}#[derive(Clone)]\n"));
    }
    out.push_str(&format!(
        "{pad}pub struct {}(pub mrlyrs::{});\n\n",
        ty.name, ty.path
    ));
    out.push_str(&format!("{pad}#[pymethods]\n{pad}impl {} {{\n", ty.name));
    for f in class.fns.iter().filter(|f| is_constructor(f)) {
        emit_constructor(cx, out, depth + 1, f)?;
    }
    for field in ty.fields.iter().filter(|f| f.public && crossable(&f.ty)) {
        doc_lines(out, depth + 1, &summary(&field.docs));
        out.push_str(&format!("{inner}#[getter]\n"));
        out.push_str(&format!(
            "{inner}#[pyo3(name = {})]\n",
            quote(&py_name(&field.name))
        ));
        out.push_str(&format!(
            "{inner}pub fn {}<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {{\n",
            rust_ident(&py_name(&field.name))
        ));
        let taken = if matches!(field.ty, Ty::Ref { .. }) || cx.is_copy(&field.ty) {
            ""
        } else {
            ".clone()"
        };
        out.push_str(&format!(
            "{}let value = self.0.{}{taken};\n",
            indent(depth + 2),
            field.name
        ));
        out.push_str(&format!(
            "{}({}).into_bound_py_any(py)\n",
            indent(depth + 2),
            into(cx, &field.ty, "value")?
        ));
        out.push_str(&format!("{inner}}}\n"));
        if field.ty.settable() {
            emit_setter(cx, out, depth + 1, &field.name, &field.ty)?;
        }
    }
    for f in &class.fns {
        let place = if f.self_kind.is_some() {
            if f.self_kind == Some(SelfKind::Value) && !clone {
                return Err(format!("{} takes self by value without Clone", f.path));
            }
            Place::Method(ty)
        } else {
            Place::Static(ty)
        };
        emit_simple(
            cx,
            out,
            depth + 1,
            f,
            place,
            &py_name(&f.name),
            f.dims.first().copied(),
        )?;
    }
    if cx.derives(&ty.path, "Deserialize") {
        out.push_str(&format!("{inner}/// Reads plain data into the class.\n"));
        out.push_str(&format!("{inner}#[staticmethod]\n"));
        out.push_str(&format!(
            "{inner}pub fn from_dict(data: &Bound<'_, PyAny>) -> PyResult<Self> {{\n{}Ok(Self(crate::hand::serde_from_py(data)?))\n{inner}}}\n",
            indent(depth + 2)
        ));
    }
    if cx.derives(&ty.path, "Serialize") {
        out.push_str(&format!("{inner}/// Returns the value as plain data.\n"));
        out.push_str(&format!(
            "{inner}pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {{\n{}crate::hand::serde_into_py(py, &self.0)\n{inner}}}\n",
            indent(depth + 2)
        ));
    }
    out.push_str(&format!("{pad}}}\n\n"));
    Ok(())
}

fn emit_setter(cx: &Cx, out: &mut String, depth: usize, field: &str, ty: &Ty) -> Result<()> {
    let pad = indent(depth);
    let deep = indent(depth + 1);
    let plan = plan(cx, "value", ty)?;
    out.push_str(&format!("{pad}#[setter]\n"));
    out.push_str(&format!(
        "{pad}#[pyo3(name = {})]\n",
        quote(&py_name(field))
    ));
    out.push_str(&format!(
        "{pad}pub fn set_{field}(&mut self, value: {}) -> PyResult<()> {{\n",
        plan.decl
    ));
    for line in &plan.lets {
        out.push_str(&format!("{deep}{line}\n"));
    }
    out.push_str(&format!("{deep}self.0.{field} = {};\n", plan.pass));
    out.push_str(&format!("{deep}Ok(())\n{pad}}}\n"));
    Ok(())
}

fn emit_holder(
    cx: &Cx,
    out: &mut String,
    depth: usize,
    module: &str,
    holder: &Owned,
) -> Result<()> {
    let pad = indent(depth);
    let ty = holder.ty;
    doc_lines(out, depth, &summary(&ty.docs));
    out.push_str(&format!(
        "{pad}#[pyclass(name = {}, module = {}, skip_from_py_object)]\n",
        quote(&ty.name),
        quote(&format!("mrlypy.{}", dotted(module)))
    ));
    out.push_str(&format!("{pad}pub struct {};\n\n", ty.name));
    out.push_str(&format!("{pad}#[pymethods]\n{pad}impl {} {{\n", ty.name));
    for f in &holder.fns {
        emit_simple(
            cx,
            out,
            depth + 1,
            f,
            Place::Static(ty),
            &py_name(&f.name),
            f.dims.first().copied(),
        )?;
    }
    out.push_str(&format!("{pad}}}\n\n"));
    Ok(())
}

fn free_words(text: &str) -> BTreeSet<&str> {
    let mut words = BTreeSet::new();
    for line in text.lines().filter(|l| !l.trim_start().starts_with("///")) {
        let mut start = None;
        let mut quoted = false;
        let mut escaped = false;
        for (i, c) in line.char_indices().chain([(line.len(), ' ')]) {
            if quoted {
                quoted = escaped || c != '"';
                escaped = !escaped && c == '\\';
                continue;
            }
            if c.is_alphanumeric() || c == '_' {
                start.get_or_insert(i);
                continue;
            }
            if let Some(s) = start.take() {
                if !line[..s].ends_with("::") && !line[..s].ends_with('.') {
                    words.insert(&line[s..i]);
                }
            }
            quoted = c == '"';
        }
    }
    words
}

fn uses(body: &str) -> Vec<String> {
    let words = free_words(body);
    let mut lines = Vec::new();
    let hand: Vec<&str> = HAND_NAMES
        .iter()
        .copied()
        .filter(|n| words.contains(n))
        .collect();
    if !hand.is_empty() {
        lines.push(format!("use crate::hand::{{{}}};", hand.join(", ")));
    }
    if words.contains("PyValueError") {
        lines.push("use pyo3::exceptions::PyValueError;".to_string());
    }
    lines.push("use pyo3::prelude::*;".to_string());
    lines.push("use pyo3::types::PyDict;".to_string());
    if body.contains(".into_bound_py_any(") {
        lines.push("use pyo3::IntoPyObjectExt;".to_string());
    }
    lines
}

fn emit_module(cx: &Cx, out: &mut String, depth: usize, node: &Node) -> Result<()> {
    let pad = indent(depth);
    let inner = indent(depth + 1);
    let mut children = String::new();
    for child in node.children.values() {
        emit_module(cx, &mut children, depth + 1, child)?;
    }
    let mut body = String::new();
    emit_items(cx, &mut body, depth, node)?;
    doc_lines(out, depth, &summary(&node.docs));
    out.push_str(&format!("{pad}pub mod {} {{\n", rust_ident(node.name())));
    for line in uses(&body) {
        out.push_str(&format!("{inner}{line}\n"));
    }
    out.push('\n');
    out.push_str(&children);
    out.push_str(&body);
    out.push_str(&format!("{pad}}}\n\n"));
    Ok(())
}

fn emit_items(cx: &Cx, out: &mut String, depth: usize, node: &Node) -> Result<()> {
    let inner = indent(depth + 1);
    for class in &node.classes {
        emit_class(cx, out, depth + 1, &node.path, class)?;
    }
    for holder in &node.holders {
        emit_holder(cx, out, depth + 1, &node.path, holder)?;
    }
    for export in &node.exports {
        emit_export(cx, out, depth + 1, export, Place::Free)?;
        out.push('\n');
    }
    out.push_str(&format!(
        "{inner}pub fn init(py: Python<'_>, parent: &Bound<'_, PyModule>, sys: &Bound<'_, PyDict>) -> PyResult<()> {{\n"
    ));
    let deep = indent(depth + 2);
    out.push_str(&format!(
        "{deep}let m = PyModule::new(py, {})?;\n",
        quote(&format!("mrlypy.{}", dotted(&node.path)))
    ));
    if !node.docs.is_empty() {
        out.push_str(&format!(
            "{deep}m.setattr(\"__doc__\", {})?;\n",
            quote(&node.docs.join("\n"))
        ));
    }
    if node.path == "core" {
        out.push_str(&format!("{deep}m.add_class::<PyRng>()?;\n"));
    }
    for class in &node.classes {
        out.push_str(&format!("{deep}m.add_class::<{}>()?;\n", class.ty.name));
    }
    for holder in &node.holders {
        out.push_str(&format!("{deep}m.add_class::<{}>()?;\n", holder.ty.name));
    }
    for export in &node.exports {
        out.push_str(&format!(
            "{deep}m.add_function(wrap_pyfunction!({}, &m)?)?;\n",
            rust_ident(&py_name(&export.name))
        ));
    }
    for c in &node.consts {
        out.push_str(&format!(
            "{deep}m.add({}, {})?;\n",
            quote(&c.name),
            into(cx, &c.ty, &format!("mrlyrs::{}", c.path))?
        ));
    }
    let names: Vec<String> = node.item_names().iter().map(|n| quote(n)).collect();
    out.push_str(&format!(
        "{deep}let names: Vec<&str> = vec![{}];\n{deep}m.add(\"__all__\", names)?;\n",
        names.join(", ")
    ));
    for child in node.children.values() {
        out.push_str(&format!(
            "{deep}{}::init(py, &m, sys)?;\n",
            rust_ident(child.name())
        ));
    }
    out.push_str(&format!("{deep}parent.add({}, &m)?;\n", quote(node.name())));
    out.push_str(&format!(
        "{deep}sys.set_item({}, &m)?;\n",
        quote(&format!("{NATIVE}.{}", dotted(&node.path)))
    ));
    out.push_str(&format!("{deep}Ok(())\n{inner}}}\n"));
    Ok(())
}

fn lib_file() -> String {
    let module = NATIVE.rsplit_once('.').map_or(NATIVE, |(_, m)| m);
    format!(
        "mod gen;\npub mod hand;\n\nuse pyo3::prelude::*;\n\n#[pymodule]\nfn {module}(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {{\n    module.add(\"__version__\", env!(\"CARGO_PKG_VERSION\"))?;\n    gen::init(py, module)\n}}\n"
    )
}

fn rust_file(cx: &Cx, root: &Node) -> Result<String> {
    let mut out = String::from(HEADER);
    out.push_str("\nuse pyo3::prelude::*;\nuse pyo3::types::PyDict;\n\n");
    for child in root.children.values() {
        emit_module(cx, &mut out, 0, child)?;
    }
    out.push_str("pub fn init(py: Python<'_>, root: &Bound<'_, PyModule>) -> PyResult<()> {\n");
    out.push_str(
        "    let sys = py.import(\"sys\")?.getattr(\"modules\")?.cast_into::<PyDict>()?;\n",
    );
    for child in root.children.values() {
        out.push_str(&format!(
            "    {}::init(py, root, &sys)?;\n",
            rust_ident(child.name())
        ));
    }
    out.push_str("    Ok(())\n}\n");
    Ok(out)
}

// PYTHON

fn py_type(cx: &Cx, ty: &Ty, here: &str) -> String {
    match ty {
        Ty::Unit => "None".into(),
        Ty::Scalar { name } => match name.as_str() {
            "bool" => "bool".into(),
            "f32" | "f64" => "float".into(),
            "char" => "str".into(),
            _ => "int".into(),
        },
        Ty::Str | Ty::String => "str".into(),
        Ty::U128 | Ty::I128 | Ty::Code => "int".into(),
        Ty::Json => "Any".into(),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => {
            if is_u8(item) {
                "bytes".into()
            } else if is_rgba(item) {
                "NDArray[Any]".into()
            } else {
                format!("list[{}]", py_type(cx, item, here))
            }
        }
        Ty::Option { item } => format!("{} | None", py_type(cx, item, here)),
        Ty::Tuple { items } => format!(
            "tuple[{}]",
            items
                .iter()
                .map(|t| py_type(cx, t, here))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Ty::Array { item, .. } => {
            if is_rgba(ty) {
                "tuple[int, int, int, int]".into()
            } else if is_u8(item) {
                "bytes".into()
            } else {
                format!("list[{}]", py_type(cx, item, here))
            }
        }
        Ty::Ref { item, .. } | Ty::Result { item } => py_type(cx, item, here),
        Ty::Map { key, value } => format!(
            "dict[{}, {}]",
            py_type(cx, key, here),
            py_type(cx, value, here)
        ),
        Ty::Hand { name, .. } => match name.as_str() {
            "Tensor" => "NDArray[Any]".into(),
            "Color" => "tuple[int, int, int, int]".into(),
            "Code" => "int".into(),
            "Rng" => qualified("core", "Rng", here),
            _ => "dict[str, Any]".into(),
        },
        Ty::Plain { path } => match cx.types.get(path.as_str()).map(|t| t.kind) {
            Some(TypeKind::Enum) => "Any".into(),
            _ => "dict[str, Any]".into(),
        },
        Ty::Enum { path } => {
            let words = cx.words(path);
            if words.is_empty() {
                "str".into()
            } else {
                format!(
                    "Literal[{}]",
                    words
                        .iter()
                        .map(|w| py_str(w))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        Ty::Class { path, .. } => qualified(parent(path), last(path), here),
        Ty::Opaque { .. } | Ty::Unknown { .. } => "Any".into(),
    }
}

fn qualified(module: &str, name: &str, here: &str) -> String {
    if module == here {
        name.to_string()
    } else {
        format!("mrlypy.{}.{name}", dotted(module))
    }
}

fn docstring(out: &mut String, depth: usize, docs: &[String]) {
    if docs.is_empty() {
        return;
    }
    let text = docs
        .join(&format!("\n{}", indent(depth)))
        .replace('\\', "\\\\")
        .replace("\"\"\"", "\\\"\\\"\\\"");
    out.push_str(&format!("{}\"\"\"{text}\"\"\"\n", indent(depth)));
}

fn stub_params(cx: &Cx, args: &[Arg], here: &str) -> Vec<String> {
    let optional_from = args
        .iter()
        .rposition(|a| !a.plan.optional)
        .map_or(0, |i| i + 1);
    args.iter()
        .enumerate()
        .map(|(i, a)| {
            let hint = py_type(cx, &a.ty, here);
            if i >= optional_from {
                format!("{}: {hint} = None", a.py)
            } else {
                format!("{}: {hint}", a.py)
            }
        })
        .collect()
}

fn stub_fn(
    cx: &Cx,
    out: &mut String,
    depth: usize,
    export: &Export,
    place: Place,
    here: &str,
) -> Result<()> {
    let pad = indent(depth);
    let name = py_name(&export.name);
    let (f, dim) = export.variants[0];
    let mut params = stub_params(cx, &args(cx, f, place, dim)?, here);
    let mut ret = py_type(cx, &subst_opt(&f.ret, dim), here);
    if export.variants.len() > 1 {
        let mut seen: Vec<String> = params
            .iter()
            .map(|p| p.split(':').next().unwrap().to_string())
            .collect();
        for (v, d) in &export.variants[1..] {
            for a in &args(cx, v, place, *d)? {
                if !seen.contains(&a.py) {
                    seen.push(a.py.clone());
                    let hint = py_type(cx, &a.ty, here);
                    params.push(format!("{}: {hint} | None = None", a.py));
                }
            }
            let other = py_type(cx, &subst_opt(&v.ret, *d), here);
            if other != ret {
                ret = format!("{ret} | {other}");
            }
        }
    }
    let variants: Vec<&Function> = export.variants.iter().map(|(f, _)| *f).collect();
    let receiver = match place {
        Place::Method(_) => "self".to_string(),
        _ => String::new(),
    };
    let mut all = Vec::new();
    if !receiver.is_empty() {
        all.push(receiver);
    }
    all.extend(params);
    if matches!(place, Place::Static(_)) {
        out.push_str(&format!("{pad}@staticmethod\n"));
    }
    out.push_str(&format!("{pad}def {name}({}) -> {ret}:\n", all.join(", ")));
    let docs = merged_docs(&variants);
    if docs.is_empty() {
        out.push_str(&format!("{}...\n", indent(depth + 1)));
    } else {
        docstring(out, depth + 1, &docs);
    }
    Ok(())
}

fn subst_opt(ty: &Ty, dim: Option<u8>) -> Ty {
    match dim {
        Some(d) => subst(ty, d),
        None => ty.clone(),
    }
}

fn stub_class(cx: &Cx, out: &mut String, class: &Owned, here: &str) -> Result<()> {
    let ty = class.ty;
    out.push_str(&format!("class {}:\n", ty.name));
    docstring(out, 1, &summary(&ty.docs));
    let mut wrote = !ty.docs.is_empty();
    for f in class.fns.iter().filter(|f| is_constructor(f)) {
        let args = args(cx, f, Place::Static(ty), None)?;
        let params = stub_params(cx, &args, here);
        let mut all = vec!["self".to_string()];
        all.extend(params);
        out.push_str(&format!(
            "    def __init__({}) -> None: ...\n",
            all.join(", ")
        ));
        wrote = true;
    }
    for field in ty.fields.iter().filter(|f| f.public && crossable(&f.ty)) {
        out.push_str("    @property\n");
        out.push_str(&format!(
            "    def {}(self) -> {}:\n",
            py_name(&field.name),
            py_type(cx, &field.ty, here)
        ));
        if field.docs.is_empty() {
            out.push_str("        ...\n");
        } else {
            docstring(out, 2, &summary(&field.docs));
        }
        if field.ty.settable() {
            out.push_str(&format!(
                "    @{0}.setter\n    def {0}(self, value: {1}) -> None: ...\n",
                py_name(&field.name),
                py_type(cx, &field.ty, here)
            ));
        }
        wrote = true;
    }
    for f in &class.fns {
        let place = if f.self_kind.is_some() {
            Place::Method(ty)
        } else {
            Place::Static(ty)
        };
        let export = Export {
            name: f.name.clone(),
            variants: vec![(f, None)],
        };
        stub_fn(cx, out, 1, &export, place, here)?;
        wrote = true;
    }
    if cx.derives(&ty.path, "Deserialize") {
        out.push_str(&format!(
            "    @staticmethod\n    def from_dict(data: Any) -> {}:\n        \"\"\"Reads plain data into the class.\"\"\"\n",
            ty.name
        ));
        wrote = true;
    }
    if cx.derives(&ty.path, "Serialize") {
        out.push_str(
            "    def to_dict(self) -> Any:\n        \"\"\"Returns the value as plain data.\"\"\"\n",
        );
        wrote = true;
    }
    if !wrote {
        out.push_str("    ...\n");
    }
    out.push('\n');
    Ok(())
}

fn stub_holder(cx: &Cx, out: &mut String, holder: &Owned, here: &str) -> Result<()> {
    let ty = holder.ty;
    out.push_str(&format!("class {}:\n", ty.name));
    docstring(out, 1, &summary(&ty.docs));
    for f in &holder.fns {
        let export = Export {
            name: f.name.clone(),
            variants: vec![(f, None)],
        };
        stub_fn(cx, out, 1, &export, Place::Static(ty), here)?;
    }
    out.push('\n');
    Ok(())
}

fn stub_rng(cx: &Cx, out: &mut String, here: &str) -> Result<()> {
    out.push_str("class Rng:\n");
    out.push_str("    \"\"\"The seeded random stream, one class, passed wherever Rust takes a mutable stream.\"\"\"\n");
    let mut fns: Vec<&Function> = cx
        .manifest
        .functions
        .iter()
        .filter(|f| f.cross == Cross::Ok && f.owner.as_deref() == Some("core::Rng"))
        .collect();
    fns.sort_by_key(|f| f.name != "new");
    for f in &fns {
        let export = Export {
            name: f.name.clone(),
            variants: vec![(f, None)],
        };
        if f.name == "new" {
            let args = args(cx, f, Place::Free, None)?;
            let mut all = vec!["self".to_string()];
            all.extend(stub_params(cx, &args, here));
            out.push_str(&format!("    def __init__({}) -> None:\n", all.join(", ")));
            docstring(out, 2, &summary(&f.docs));
        } else if f.self_kind.is_some() {
            stub_fn(
                cx,
                out,
                1,
                &export,
                Place::Method(cx.ty("core::Rng")?),
                here,
            )?;
        }
    }
    out.push_str("    def choice(self, seq: Any) -> Any:\n        \"\"\"Draws one item of the sequence, the same draw as Rust's choice.\"\"\"\n");
    out.push_str("    def shuffle(self, seq: list[Any]) -> None:\n        \"\"\"Shuffles the list in place, the same permutation as Rust's shuffle.\"\"\"\n");
    out.push('\n');
    Ok(())
}

fn stub_file(cx: &Cx, node: &Node) -> Result<String> {
    let here = node.path.as_str();
    let mut out = String::new();
    let mut body = String::new();
    for c in &node.consts {
        body.push_str(&format!("{}: {}\n", c.name, py_type(cx, &c.ty, here)));
    }
    if !node.consts.is_empty() {
        body.push('\n');
    }
    if here == "core" {
        stub_rng(cx, &mut body, here)?;
    }
    for class in &node.classes {
        stub_class(cx, &mut body, class, here)?;
    }
    for holder in &node.holders {
        stub_holder(cx, &mut body, holder, here)?;
    }
    for export in &node.exports {
        stub_fn(cx, &mut body, 0, export, Place::Free, here)?;
        body.push('\n');
    }
    let mut packages: Vec<String> = Vec::new();
    for word in body.split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.')) {
        if let Some(rest) = word.strip_prefix("mrlypy.") {
            let module = rest.rsplit_once('.').map(|(m, _)| m).unwrap_or(rest);
            let pkg = format!("mrlypy.{module}");
            if !packages.contains(&pkg) {
                packages.push(pkg);
            }
        }
    }
    packages.sort();
    out.push_str("from typing import Any, Literal\n\nfrom numpy.typing import NDArray\n");
    for pkg in &packages {
        out.push_str(&format!("import {pkg}\n"));
    }
    if !node.children.is_empty() {
        let children: Vec<&str> = node.children.keys().map(String::as_str).collect();
        out.push_str(&format!("from . import {}\n", children.join(", ")));
    }
    out.push('\n');
    out.push_str(body.trim_end());
    out.push('\n');
    Ok(out)
}

fn init_file(node: &Node) -> String {
    let mut out = String::new();
    out.push_str(&format!("from {NATIVE}.{} import *\n", dotted(&node.path)));
    if !node.children.is_empty() {
        let children: Vec<&str> = node.children.keys().map(String::as_str).collect();
        out.push_str(&format!("from . import {}\n", children.join(", ")));
    }
    let mut names = node.item_names();
    names.extend(node.children.keys().cloned());
    let listed: Vec<String> = names.iter().map(|n| py_str(n)).collect();
    out.push_str(&format!("\n__all__ = [{}]\n", listed.join(", ")));
    out
}

fn python_files(cx: &Cx, root: &Node, python: &Path) -> Result<()> {
    let children: Vec<&str> = root.children.keys().map(String::as_str).collect();
    let listed: Vec<String> = children.iter().map(|n| py_str(n)).collect();
    let init = format!(
        "from mrlypy import _mrlypy\nfrom mrlypy import {}\n\n__version__ = _mrlypy.__version__\n__all__ = [{}]\n",
        children.join(", "),
        listed.join(", ")
    );
    save(&python.join("__init__.py"), &init)?;
    let stub = format!(
        "from . import {}\n\n__version__: str\n",
        children.join(", ")
    );
    save(&python.join("__init__.pyi"), &stub)?;
    let mut nodes = Vec::new();
    for child in root.children.values() {
        child.walk(&mut nodes);
    }
    for node in nodes {
        let dir = python.join(node.path.replace("::", "/"));
        save(&dir.join("__init__.py"), &init_file(node))?;
        save(&dir.join("__init__.pyi"), &stub_file(cx, node)?)?;
    }
    Ok(())
}

// TEST

fn test_file(cx: &Cx) -> String {
    let ok = cx
        .manifest
        .functions
        .iter()
        .filter(|f| f.cross == Cross::Ok)
        .count();
    format!(
        r#"import importlib
import json
import keyword
import pathlib

import mrlypy

MANIFEST = pathlib.Path(__file__).resolve().parents[3] / "bridge" / "manifest.json"


def load():
    return json.loads(MANIFEST.read_text())


def name_of(word):
    return word + "_" if keyword.iskeyword(word) else word


def locate(fn, kinds):
    module = importlib.import_module("mrlypy." + fn["module"].replace("::", "."))
    exported = name_of(fn["path"].rsplit("::", 1)[-1])
    owner = fn["owner"]
    if owner and kinds[owner] in ("class", "plain", "enum"):
        return getattr(getattr(module, owner.rsplit("::", 1)[-1]), exported)
    return getattr(module, exported)


def test_every_ok_function_is_callable_under_its_rust_doc():
    manifest = load()
    kinds = {{t["path"]: t["cross"]["kind"] for t in manifest["types"]}}
    count = 0
    for fn in manifest["functions"]:
        if fn["cross"]["status"] != "ok":
            continue
        target = locate(fn, kinds)
        assert callable(target), fn["path"]
        doc = target.__doc__ or ""
        if fn["docs"]:
            if fn["dims"]:
                assert fn["docs"][0] in doc, fn["path"]
            else:
                assert doc.startswith(fn["docs"][0]), fn["path"]
        count += 1
    assert count == {ok}
"#
    )
}
