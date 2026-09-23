use crate::model::{
    Const, Cross, Function, Manifest, Result, SelfKind, Ty, Type, TypeCross, TypeKind,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

const TYPED: &[&str] = &[
    "u8", "u16", "u32", "i8", "i16", "i32", "f32", "f64", "usize", "u64", "i64",
];

const NATIVE_ITEM: &[&str] = &[
    "u8", "u16", "u32", "i8", "i16", "i32", "f32", "f64", "usize",
];

const WORDS: &[&str] = &[
    "void",
    "new",
    "class",
    "function",
    "default",
    "delete",
    "in",
    "var",
    "let",
    "const",
    "export",
    "import",
    "switch",
    "case",
    "for",
    "if",
    "else",
    "this",
    "super",
    "with",
    "yield",
    "enum",
    "await",
    "typeof",
    "instanceof",
    "return",
    "try",
    "catch",
    "finally",
    "throw",
    "while",
    "do",
    "break",
    "continue",
    "debugger",
    "static",
    "interface",
    "package",
    "private",
    "protected",
    "public",
    "implements",
    "arguments",
    "eval",
    "null",
    "true",
    "false",
    "extends",
    "as",
    "crate",
    "extern",
    "fn",
    "impl",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "self",
    "struct",
    "trait",
    "type",
    "unsafe",
    "use",
    "where",
    "async",
    "dyn",
    "box",
    "macro",
    "gen",
];

const RNG_METHODS: &[&str] = &[
    "new",
    "unit",
    "below",
    "range",
    "boolean",
    "chance",
    "sample_indices",
    "choice",
    "shuffle",
    "__state",
    "__restore",
];

const JS_RESERVED: &[&str] = &[
    "new",
    "void",
    "class",
    "function",
    "default",
    "delete",
    "in",
    "var",
    "let",
    "const",
    "export",
    "import",
    "switch",
    "case",
    "for",
    "if",
    "else",
    "this",
    "super",
    "with",
    "yield",
    "enum",
    "await",
    "typeof",
    "instanceof",
    "return",
    "try",
    "catch",
    "finally",
    "throw",
    "while",
    "do",
    "break",
    "continue",
    "debugger",
    "static",
    "interface",
    "package",
    "private",
    "protected",
    "public",
    "implements",
    "null",
    "true",
    "false",
    "extends",
];

const HAND_RS: &str =
    "#[path = \"../../../hand/hand.rs\"]\nmod crossing;\n\npub use crossing::*;\n";

const ALLOWS: &str = "#![allow(non_camel_case_types, non_snake_case, clippy::too_many_arguments)]";

// FILES

pub fn write(manifest: &Manifest, root: &Path) -> Result<()> {
    let names =
        std::fs::read_to_string(root.join("bridge/units.txt")).map_err(|e| e.to_string())?;
    let names: Vec<&str> = names
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let js = root.join("pkgs/mrlyjs");
    for name in &names {
        let unit = Unit::new(manifest, name);
        let dir = js.join("units").join(name);
        save(
            &dir.join("Cargo.toml"),
            &cargo_toml(name, &manifest.version),
        )?;
        save(&dir.join("src/hand.rs"), HAND_RS)?;
        save(&dir.join("src/lib.rs"), &unit.lib_rs()?)?;
        save(&js.join(format!("{name}.js")), &unit.wrapper_js())?;
        save(&js.join(format!("{name}.d.ts")), &unit.types_dts()?)?;
    }
    save(&js.join("manifest.test.js"), MANIFEST_TEST)?;
    Ok(())
}

fn save(path: &Path, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    if std::fs::read_to_string(path).ok().as_deref() == Some(text) {
        return Ok(());
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

fn cargo_toml(name: &str, version: &str) -> String {
    let what = if name == "all" {
        "every module of mrlyrs".to_string()
    } else {
        format!("the {name} module of mrlyrs")
    };
    format!(
        "[package]\nname = \"mrlyjs_{name}\"\nversion = \"{version}\"\nedition.workspace = true\nlicense.workspace = true\nrepository.workspace = true\ndescription = \"The {name} wasm unit of mrlyjs: {what} in a browser.\"\npublish = false\n\n[lib]\ncrate-type = [\"cdylib\"]\n\n[dependencies]\nmrlyrs.workspace = true\nwasm-bindgen = \"0.2\"\nserde-wasm-bindgen = \"0.6\"\njs-sys = \"0.3\"\nserde = {{ version = \"1\", features = [\"derive\"] }}\nserde_json = \"1\"\n\n[package.metadata.wasm-pack.profile.release]\nwasm-opt = false\n"
    )
}

// UNIT

struct Unit<'a> {
    name: &'a str,
    manifest: &'a Manifest,
    types: BTreeMap<&'a str, &'a Type>,
    reserved: BTreeSet<String>,
}

#[derive(Clone, Copy)]
enum Owner<'a> {
    Free,
    Hand(&'a Type),
    Class(&'a Type),
    Holder(&'a Type),
}

#[derive(Clone, Copy)]
struct Cx<'a> {
    dim: Option<u8>,
    fname: &'a str,
}

struct ParamPlan {
    name: String,
    sig: String,
    pre: Vec<String>,
    arg: String,
    post: Vec<String>,
}

struct RetPlan {
    sig: String,
    done: String,
}

impl<'a> Unit<'a> {
    fn new(manifest: &'a Manifest, name: &'a str) -> Unit<'a> {
        let mut unit = Unit {
            name,
            manifest,
            types: manifest
                .types
                .iter()
                .map(|t| (t.path.as_str(), t))
                .collect(),
            reserved: BTreeSet::new(),
        };
        unit.reserved = unit.symbols();
        unit
    }

    fn symbols(&self) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        for method in RNG_METHODS {
            out.insert(format!("rng_{method}"));
        }
        for t in self.classes().into_iter().chain(self.holders()) {
            let lower = self.local(&t.path).to_lowercase();
            for f in self.functions() {
                if f.owner.as_deref() == Some(t.path.as_str()) {
                    out.insert(format!("{lower}_{}", f.name));
                }
            }
            for extra in ["from", "toJSON", "new"] {
                out.insert(format!("{lower}_{extra}"));
            }
        }
        out
    }

    fn export(&self, name: &str) -> String {
        let mut out = name.to_string();
        while self.reserved.contains(&out) {
            out.push('_');
        }
        out
    }

    fn holds(&self, path: &str) -> bool {
        self.name == "all" || path.split("::").next() == Some(self.name)
    }

    fn rel<'p>(&self, path: &'p str) -> &'p str {
        if self.name == "all" {
            return path;
        }
        match path.strip_prefix(self.name) {
            Some("") => "",
            Some(rest) => rest.strip_prefix("::").unwrap_or(path),
            None => path,
        }
    }

    fn local(&self, path: &str) -> String {
        self.rel(path).replace("::", "_")
    }

    fn ty(&self, path: &str) -> Option<&'a Type> {
        self.types.get(path).copied()
    }

    fn functions(&self) -> Vec<&'a Function> {
        self.manifest
            .functions
            .iter()
            .filter(|f| f.cross == Cross::Ok && self.holds(&f.module))
            .collect()
    }

    fn groups(&self) -> Vec<Vec<&'a Function>> {
        let mut out: Vec<Vec<&'a Function>> = vec![];
        for f in self.functions() {
            match out.last_mut() {
                Some(group) if group[0].path == f.path => group.push(f),
                _ => out.push(vec![f]),
            }
        }
        out
    }

    fn consts(&self) -> Vec<&'a Const> {
        self.manifest
            .consts
            .iter()
            .filter(|c| c.cross == Cross::Ok && self.holds(&c.path))
            .collect()
    }

    fn classes(&self) -> Vec<&'a Type> {
        self.manifest
            .types
            .iter()
            .filter(|t| t.cross == TypeCross::Class && self.holds(&t.path))
            .collect()
    }

    fn holders(&self) -> Vec<&'a Type> {
        let owners: BTreeSet<&str> = self
            .functions()
            .iter()
            .filter_map(|f| f.owner.as_deref())
            .collect();
        self.manifest
            .types
            .iter()
            .filter(|t| {
                matches!(t.cross, TypeCross::Plain | TypeCross::Enum { .. })
                    && self.holds(&t.path)
                    && owners.contains(t.path.as_str())
            })
            .collect()
    }

    fn owner_of(&self, f: &Function) -> Owner<'a> {
        let Some(path) = f.owner.as_deref() else {
            return Owner::Free;
        };
        match self.ty(path).map(|t| (t, &t.cross)) {
            Some((t, TypeCross::Hand { .. })) => Owner::Hand(t),
            Some((t, TypeCross::Class)) => Owner::Class(t),
            Some((t, TypeCross::Plain | TypeCross::Enum { .. })) => Owner::Holder(t),
            _ => Owner::Free,
        }
    }

    fn hand_name(&self, t: &Type) -> &'static str {
        match &t.cross {
            TypeCross::Hand { name } => match name.as_str() {
                "Tensor" => "Tensor",
                "Cell" => "Cell",
                "CellNd" => "CellNd",
                "Cell6d" => "Cell6d",
                "Color" => "Color",
                "Code" => "Code",
                _ => "Rng",
            },
            _ => "Rng",
        }
    }

    // RUST TYPES

    fn rust_ty(&self, ty: &Ty, cx: Cx) -> Result<String> {
        Ok(match ty {
            Ty::Unit => "()".into(),
            Ty::Scalar { name } => name.clone(),
            Ty::Str | Ty::String => "String".into(),
            Ty::U128 => "u128".into(),
            Ty::I128 => "i128".into(),
            Ty::Code => "mrlyrs::math::bang::Code".into(),
            Ty::Json => "mrlyrs::core::Json".into(),
            Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => {
                format!("Vec<{}>", self.rust_ty(item, cx)?)
            }
            Ty::Option { item } => format!("Option<{}>", self.rust_ty(item, cx)?),
            Ty::Tuple { items } => {
                let parts: Result<Vec<String>> =
                    items.iter().map(|t| self.rust_ty(t, cx)).collect();
                format!("({})", parts?.join(", "))
            }
            Ty::Array { item, len } => format!("[{}; {len}]", self.rust_ty(item, cx)?),
            Ty::Ref { item, .. } | Ty::Result { item } => self.rust_ty(item, cx)?,
            Ty::Map { key, value } => format!(
                "std::collections::HashMap<{}, {}>",
                self.rust_ty(key, cx)?,
                self.rust_ty(value, cx)?
            ),
            Ty::Hand { name, dim } => match name.as_str() {
                "Tensor" => "mrlyrs::core::Tensor".into(),
                "Cell" => "mrlyrs::core::Cell".into(),
                "CellNd" => match dim.or(cx.dim) {
                    Some(2) => "mrlyrs::math::two::Cell2d".into(),
                    Some(3) => "mrlyrs::math::three::Cell3d".into(),
                    _ => return Err(format!("{}: a cell without a dimension", cx.fname)),
                },
                "Cell6d" => "mrlyrs::math::six::Cell6d".into(),
                "Color" => "mrlyrs::core::Color".into(),
                "Code" => "mrlyrs::math::bang::Code".into(),
                "Rng" => "mrlyrs::core::Rng".into(),
                other => return Err(format!("{}: unknown hand type {other}", cx.fname)),
            },
            Ty::Plain { path } => {
                let generic = self.ty(path).is_some_and(|t| t.const_generic);
                if generic {
                    format!("mrlyrs::{path}<{}>", suffix_dim(cx.fname))
                } else {
                    format!("mrlyrs::{path}")
                }
            }
            Ty::Enum { path } | Ty::Class { path, .. } => format!("mrlyrs::{path}"),
            Ty::Opaque { path } => return Err(format!("{}: {path} cannot cross", cx.fname)),
            Ty::Unknown { text } => return Err(format!("{}: {text} cannot cross", cx.fname)),
        })
    }

    fn is_copy(&self, ty: &Ty) -> bool {
        match ty {
            Ty::Unit | Ty::Scalar { .. } | Ty::U128 | Ty::I128 | Ty::Code => true,
            Ty::Hand { name, .. } => name == "Color" || name == "Code",
            Ty::Tuple { items } => items.iter().all(|t| self.is_copy(t)),
            Ty::Array { item, .. } | Ty::Option { item } => self.is_copy(item),
            Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } => {
                self.ty(path).is_some_and(|t| derives(t, "Copy"))
            }
            _ => false,
        }
    }

    // CROSSINGS IN

    fn cross_in(&self, ty: &Ty, e: &str, cx: Cx, depth: usize) -> Result<String> {
        if pure_in(ty) {
            return Ok(format!("hand::from_js::<{}>({e})?", self.rust_ty(ty, cx)?));
        }
        let x = format!("x{depth}");
        Ok(match ty {
            Ty::Scalar { name } => match name.as_str() {
                "u64" => format!("hand::u64_from_js({e})?"),
                "i64" => format!("hand::i64_from_js({e})?"),
                "bool" => format!("hand::bool_from_js({e})?"),
                "char" => format!("hand::char_from_js({e})?"),
                other => format!("(hand::number_from_js({e})? as {other})"),
            },
            Ty::Str | Ty::String => format!("hand::string_from_js({e})?"),
            Ty::U128 => format!("hand::u128_from_js({e})?"),
            Ty::I128 => format!("hand::i128_from_js({e})?"),
            Ty::Code => format!("hand::code_from_js({e})?"),
            Ty::Json => format!("hand::json_from_js({e})?"),
            Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Set { item } => format!(
                "hand::list_from_js({e}, {})?",
                closure(&x, ok(self.cross_in(item, &x, cx, depth + 1)?))
            ),
            Ty::Option { item } => format!(
                "hand::option_from_js({e}, {})?",
                closure(&x, ok(self.cross_in(item, &x, cx, depth + 1)?))
            ),
            Ty::Tuple { items } => {
                let mut parts = vec![];
                for (i, t) in items.iter().enumerate() {
                    parts.push(self.cross_in(
                        t,
                        &format!("&hand::item({e}, {i})?"),
                        cx,
                        depth + 1,
                    )?);
                }
                format!("({})", parts.join(", "))
            }
            Ty::Array { item, len } => format!(
                "hand::array_from_js::<_, {len}>({e}, {})?",
                closure(&x, ok(self.cross_in(item, &x, cx, depth + 1)?))
            ),
            Ty::Ref { item, .. } | Ty::Result { item } => self.cross_in(item, e, cx, depth)?,
            Ty::Map { key, value } => format!(
                "hand::map_from_js::<{}, _>({e}, {})?",
                self.rust_ty(key, cx)?,
                closure(&x, ok(self.cross_in(value, &x, cx, depth + 1)?))
            ),
            Ty::Hand { name, dim } => match name.as_str() {
                "Tensor" => format!("hand::tensor_from_js({e})?"),
                "Cell" => format!("hand::cell_from_js({e})?"),
                "CellNd" => match dim.or(cx.dim) {
                    Some(2) => format!("hand::cell2d_from_js({e})?"),
                    Some(3) => format!("hand::cell3d_from_js({e})?"),
                    _ => return Err(format!("{}: a cell without a dimension", cx.fname)),
                },
                "Cell6d" => format!("hand::cell6d_from_js({e})?"),
                "Color" => format!("hand::color_from_js({e})?"),
                "Code" => format!("hand::code_from_js({e})?"),
                other => return Err(format!("{}: {other} cannot cross in by value", cx.fname)),
            },
            Ty::Class { .. } => format!(
                "hand::from_js::<{}>(&hand::plain({e})?)?",
                self.rust_ty(ty, cx)?
            ),
            Ty::Unit | Ty::Plain { .. } | Ty::Enum { .. } => {
                format!("hand::from_js::<{}>({e})?", self.rust_ty(ty, cx)?)
            }
            Ty::Opaque { path } => return Err(format!("{}: {path} cannot cross", cx.fname)),
            Ty::Unknown { text } => return Err(format!("{}: {text} cannot cross", cx.fname)),
        })
    }

    // CROSSINGS OUT

    fn cross_out(&self, ty: &Ty, e: &str, cx: Cx, depth: usize) -> Result<String> {
        if pure_out(ty) {
            return Ok(format!("hand::to_js({e})?"));
        }
        let x = format!("x{depth}");
        Ok(match ty {
            Ty::Unit => "JsValue::UNDEFINED".into(),
            Ty::Scalar { name } => match name.as_str() {
                "u64" | "i64" => format!("JsValue::from({})", deref(e)),
                "bool" => format!("JsValue::from_bool({})", deref(e)),
                "char" => format!("JsValue::from_str(&{}.to_string())", place(e)),
                _ => format!("JsValue::from_f64({} as f64)", deref(e)),
            },
            Ty::Str | Ty::String => format!("JsValue::from_str({e})"),
            Ty::U128 | Ty::I128 => format!("JsValue::from_str(&{}.to_string())", place(e)),
            Ty::Code => format!("JsValue::from_str(&hand::code_to_js({}))", deref(e)),
            Ty::Json => format!("hand::json_to_js({e})?"),
            Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Array { item, .. }
                if typed_scalar(item) =>
            {
                format!("hand::typed(&({})[..])", deref(e))
            }
            Ty::Vec { item }
            | Ty::Slice { item, .. }
            | Ty::Array { item, .. }
            | Ty::Set { item } => {
                format!(
                    "hand::list_to_js({e}, {})?",
                    closure(&x, ok(self.cross_out(item, &x, cx, depth + 1)?))
                )
            }
            Ty::Option { item } => match &**item {
                Ty::Ref { item: inner, .. } => format!(
                    "hand::option_to_js({}, {})?",
                    deref(e),
                    closure(&x, ok(self.cross_out(inner, &x, cx, depth + 1)?))
                ),
                _ => format!(
                    "hand::option_to_js({}.as_ref(), {})?",
                    place(e),
                    closure(&x, ok(self.cross_out(item, &x, cx, depth + 1)?))
                ),
            },
            Ty::Tuple { items } => {
                let mut parts = vec![];
                for (i, t) in items.iter().enumerate() {
                    parts.push(self.cross_out(t, &format!("&{}.{i}", place(e)), cx, depth + 1)?);
                }
                format!("hand::tuple_to_js(&[{}])", parts.join(", "))
            }
            Ty::Ref { item, .. } | Ty::Result { item } => {
                self.cross_out(item, &format!("({})", deref(e)), cx, depth)?
            }
            Ty::Map { value, .. } => format!(
                "hand::map_to_js({e}, {})?",
                closure(&x, ok(self.cross_out(value, &x, cx, depth + 1)?))
            ),
            Ty::Hand { name, dim } => match name.as_str() {
                "Tensor" => format!("hand::tensor_to_js({e})?"),
                "Cell" => format!("hand::cell_to_js({e})?"),
                "CellNd" => match dim.or(cx.dim) {
                    Some(2) => format!("hand::cell2d_to_js({e})?"),
                    Some(3) => format!("hand::cell3d_to_js({e})?"),
                    _ => return Err(format!("{}: a cell without a dimension", cx.fname)),
                },
                "Cell6d" => format!("hand::cell6d_to_js({e})?"),
                "Color" => format!("hand::color_to_js({})", deref(e)),
                "Code" => format!("JsValue::from_str(&hand::code_to_js({}))", deref(e)),
                _ => format!("JsValue::from(hand::Rng::wrap({}.clone()))", place(e)),
            },
            Ty::Class { path, .. } if self.holds(path) => {
                let inner = if self.is_copy(ty) {
                    deref(e)
                } else {
                    format!("{}.clone()", place(e))
                };
                format!("JsValue::from({} {{ inner: {inner} }})", self.local(path))
            }
            Ty::Class { .. } | Ty::Plain { .. } | Ty::Enum { .. } => format!("hand::to_js({e})?"),
            Ty::Opaque { path } => return Err(format!("{}: {path} cannot cross", cx.fname)),
            Ty::Unknown { text } => return Err(format!("{}: {text} cannot cross", cx.fname)),
        })
    }

    fn ret_plan(&self, ty: &Ty, cx: Cx) -> Result<RetPlan> {
        let plan = |sig: &str, done: String| RetPlan {
            sig: sig.to_string(),
            done,
        };
        Ok(match ty {
            Ty::Unit => plan("()", "Ok(())".into()),
            Ty::Result { item } => self.ret_plan(item, cx)?,
            Ty::Scalar { name } => plan(name, "Ok(value)".into()),
            Ty::String => plan("String", "Ok(value)".into()),
            Ty::Str => plan("String", "Ok(value.to_string())".into()),
            Ty::Vec { item } if typed_scalar(item) => plan(
                &format!("Vec<{}>", self.rust_ty(item, cx)?),
                "Ok(value)".into(),
            ),
            Ty::Slice { item, .. } | Ty::Array { item, .. } if typed_scalar(item) => plan(
                &format!("Vec<{}>", self.rust_ty(item, cx)?),
                "Ok(value.to_vec())".into(),
            ),
            Ty::Class { path, .. } if self.holds(path) => {
                let ident = self.local(path);
                plan(&ident, format!("Ok({ident} {{ inner: value }})"))
            }
            Ty::Option { item } => match &**item {
                Ty::Class { path, .. } if self.holds(path) => {
                    let ident = self.local(path);
                    plan(
                        &format!("Option<{ident}>"),
                        format!("Ok(value.map(|inner| {ident} {{ inner }}))"),
                    )
                }
                _ => plan("JsValue", ok(self.cross_out(ty, "&value", cx, 1)?)),
            },
            Ty::Hand { name, .. } if name == "Rng" => {
                plan("hand::Rng", "Ok(hand::Rng::wrap(value))".into())
            }
            Ty::Ref { item, .. } => match &**item {
                Ty::Slice { item: inner, .. } | Ty::Array { item: inner, .. }
                    if typed_scalar(inner) =>
                {
                    plan(
                        &format!("Vec<{}>", self.rust_ty(inner, cx)?),
                        "Ok(value.to_vec())".into(),
                    )
                }
                Ty::Str => plan("String", "Ok(value.to_string())".into()),
                _ => plan("JsValue", ok(self.cross_out(item, "value", cx, 1)?)),
            },
            Ty::Slice { .. } => plan("JsValue", ok(self.cross_out(ty, "value", cx, 1)?)),
            _ => plan("JsValue", ok(self.cross_out(ty, "&value", cx, 1)?)),
        })
    }

    // PARAMETERS

    fn plan_param(&self, name: &str, ty: &Ty, cx: Cx, extra: bool) -> Result<ParamPlan> {
        let n = ident(name);
        let js = |pre: Vec<String>, arg: String, post: Vec<String>| ParamPlan {
            name: n.clone(),
            sig: "JsValue".into(),
            pre,
            arg,
            post,
        };
        let native = |sig: String, arg: String| ParamPlan {
            name: n.clone(),
            sig,
            pre: vec![],
            arg,
            post: vec![],
        };
        let read = |inner: &Ty| -> Result<String> {
            let converted = self.cross_in(inner, &format!("&{n}"), cx, 1)?;
            Ok(if extra {
                let want = format!("a {}d cell wants {name}.", cx.dim.unwrap_or(0));
                format!("let {n} = if {n}.is_undefined() {{ return Err(hand::refuse(\"{want}\")); }} else {{ {converted} }};")
            } else {
                format!("let {n} = {converted};")
            })
        };
        let owned = |inner: &Ty, arg: String| -> Result<ParamPlan> {
            Ok(js(vec![read(inner)?], arg, vec![]))
        };
        if !extra {
            match ty {
                Ty::Scalar { name } if !is_wide(name) => {
                    return Ok(native(name.clone(), n.clone()))
                }
                Ty::Str => return Ok(native("&str".into(), n.clone())),
                Ty::String => return Ok(native("String".into(), n.clone())),
                Ty::Slice { item, .. } if native_item(item) => {
                    return Ok(native(format!("&[{}]", self.rust_ty(item, cx)?), n.clone()))
                }
                Ty::Vec { item } if native_item(item) => {
                    return Ok(native(
                        format!("Vec<{}>", self.rust_ty(item, cx)?),
                        n.clone(),
                    ))
                }
                Ty::Class { path, .. } if self.holds(path) => {
                    let taken = if self.is_copy(ty) {
                        format!("{n}.inner")
                    } else {
                        format!("{n}.inner.clone()")
                    };
                    return Ok(native(format!("&{}", self.local(path)), taken));
                }
                Ty::Ref { mutable, item, .. } => match &**item {
                    Ty::Class { path, .. } if self.holds(path) => {
                        let ident = self.local(path);
                        return Ok(if *mutable {
                            native(format!("&mut {ident}"), format!("&mut {n}.inner"))
                        } else {
                            native(format!("&{ident}"), format!("&{n}.inner"))
                        });
                    }
                    Ty::Hand { name, .. } if name == "Rng" && *mutable => {
                        return Ok(native("&mut hand::Rng".into(), format!("{n}.stream()")))
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(match ty {
            Ty::Ref {
                mutable: true,
                item,
                ..
            } => match &**item {
                Ty::Hand { name, .. } if name == "Tensor" || name == "Cell" => {
                    let back = if name == "Tensor" {
                        "tensor_into_js"
                    } else {
                        "cell_into_js"
                    };
                    let converted = self.cross_in(item, &format!("&{n}"), cx, 1)?;
                    js(
                        vec![format!("let mut {n}_value = {converted};")],
                        format!("&mut {n}_value"),
                        vec![format!("hand::{back}(&{n}, &{n}_value)?;")],
                    )
                }
                _ => {
                    return Err(format!(
                        "{}: {name} mutates a value that cannot cross",
                        cx.fname
                    ))
                }
            },
            Ty::Ref { item, .. } => owned(item, format!("&{n}"))?,
            Ty::Slice { item, .. } => match self.view(item, &n, cx)? {
                Some(seen) => {
                    let mut pre = vec![read(ty)?];
                    pre.push(seen);
                    js(pre, format!("&{n}_view"), vec![])
                }
                None => owned(ty, format!("&{n}"))?,
            },
            Ty::Option { item } => match &**item {
                Ty::Ref {
                    mutable: true,
                    item: inner,
                    ..
                } if is_rng(inner) => js(
                    vec![format!("let mut {n}_stream = hand::stream_from_js(&{n})?;")],
                    format!("{n}_stream.as_mut()"),
                    vec![format!(
                        "if let Some(stream) = &{n}_stream {{ hand::stream_to_js(&{n}, stream)?; }}"
                    )],
                ),
                Ty::Ref { item: inner, .. } if matches!(**inner, Ty::Slice { .. }) => {
                    owned(ty, format!("{n}.as_deref()"))?
                }
                Ty::Ref { .. } => owned(ty, format!("{n}.as_ref()"))?,
                Ty::Slice { .. } => owned(ty, format!("{n}.as_deref()"))?,
                _ => owned(ty, n.clone())?,
            },
            _ => owned(ty, n.clone())?,
        })
    }

    fn view(&self, item: &Ty, n: &str, cx: Cx) -> Result<Option<String>> {
        Ok(match item {
            Ty::Str => Some(format!(
                "let {n}_view: Vec<&str> = {n}.iter().map(String::as_str).collect();"
            )),
            Ty::Slice { item: inner, .. } => Some(format!(
                "let {n}_view: Vec<&[{}]> = {n}.iter().map(Vec::as_slice).collect();",
                self.rust_ty(inner, cx)?
            )),
            Ty::Tuple { items } if items.iter().any(|t| matches!(t, Ty::Str)) => {
                let names: Vec<String> = (0..items.len()).map(|i| format!("a{i}")).collect();
                let parts: Vec<String> = items
                    .iter()
                    .zip(&names)
                    .map(|(t, a)| match t {
                        Ty::Str => format!("{a}.as_str()"),
                        Ty::Scalar { .. } => format!("*{a}"),
                        _ => format!("{a}.clone()"),
                    })
                    .collect();
                let mut tys = vec![];
                for t in items {
                    tys.push(match t {
                        Ty::Str => "&str".to_string(),
                        other => self.rust_ty(other, cx)?,
                    });
                }
                Some(format!(
                    "let {n}_view: Vec<({})> = {n}.iter().map(|({})| ({})).collect();",
                    tys.join(", "),
                    names.join(", "),
                    parts.join(", ")
                ))
            }
            _ => None,
        })
    }

    fn self_param(&self, f: &Function, owner: &Type) -> Result<Option<(String, Ty)>> {
        let Some(kind) = f.self_kind else {
            return Ok(None);
        };
        let name = self.hand_name(owner);
        let inner = Ty::Hand {
            name: name.to_string(),
            dim: None,
        };
        let taken = f.params.iter().any(|p| p.name == hand_self(name));
        let label = if taken {
            "self_".to_string()
        } else {
            hand_self(name).to_string()
        };
        let ty = match (kind, name) {
            (SelfKind::Mut, _) => Ty::Ref {
                mutable: true,
                lifetime: None,
                item: Box::new(inner),
            },
            (_, "Rng") => return Err(format!("{}: an Rng self must be mutable", f.path)),
            _ => inner,
        };
        Ok(Some((label, ty)))
    }

    // FUNCTIONS

    fn rank_source(&self, params: &[(String, Ty, bool)]) -> Option<String> {
        fn cell(t: &Ty) -> bool {
            matches!(t, Ty::Hand { name, dim: None } if name == "CellNd")
        }
        fn shaped(t: &Ty) -> bool {
            matches!(t, Ty::Hand { name, .. } if name == "Tensor" || name == "Cell" || name == "CellNd")
        }
        for test in [cell, shaped] {
            for (name, ty, _) in params {
                let n = ident(name);
                match ty {
                    t if test(t) => return Some(format!("hand::cell_rank(&{n})?")),
                    Ty::Ref { item, .. } if test(item) => {
                        return Some(format!("hand::cell_rank(&{n})?"))
                    }
                    Ty::Slice { item, .. } | Ty::Vec { item } if test(item) => {
                        return Some(format!("hand::cell_rank(&hand::first(&{n})?)?"))
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn emit_group(&self, out: &mut String, group: &[&Function], owner: Owner) -> Result<()> {
        let head = group[0];
        let dispatch = group.len() > 1 || !head.dims.is_empty();
        let name = match owner {
            Owner::Free | Owner::Hand(_) => self.local(&head.path),
            _ => head.name.clone(),
        };
        let mut merged: Vec<(String, Ty, bool)> = vec![];
        for f in group {
            if let Owner::Hand(t) = owner {
                if let Some((label, ty)) = self.self_param(f, t)? {
                    if !merged.iter().any(|(n, _, _)| *n == label) {
                        merged.push((label, ty, false));
                    }
                }
            }
            if let Owner::Holder(t) = owner {
                if f.self_kind.is_some() {
                    let label = t.name.to_lowercase();
                    let label = if f.params.iter().any(|p| p.name == label) {
                        "self_".to_string()
                    } else {
                        label
                    };
                    if !merged.iter().any(|(n, _, _)| *n == label) {
                        merged.push((label, plain_ty(t), false));
                    }
                }
            }
            for p in &f.params {
                if !merged.iter().any(|(n, _, _)| *n == p.name) {
                    merged.push((p.name.clone(), p.ty.clone(), false));
                }
            }
        }
        let shared: Vec<String> = merged
            .iter()
            .filter(|(n, _, _)| group.iter().all(|f| self.mentions(f, owner, n)))
            .map(|(n, _, _)| n.clone())
            .collect();
        for slot in &mut merged {
            slot.2 = !shared.contains(&slot.0);
        }
        let cx = Cx {
            dim: head.dims.first().copied(),
            fname: &head.path,
        };
        let ret = self.ret_plan(&head.ret, cx)?;
        let mut sigs: Vec<String> = vec![];
        let mut plans: Vec<ParamPlan> = vec![];
        for (n, ty, extra) in &merged {
            let plan = self.plan_param(n, ty, cx, *extra)?;
            sigs.push(format!("{}: {}", plan.name, plan.sig));
            plans.push(plan);
        }
        if let Owner::Class(_) = owner {
            let receiver = match head.self_kind {
                Some(SelfKind::Mut) => Some("&mut self"),
                Some(_) => Some("&self"),
                None => None,
            };
            if let Some(receiver) = receiver {
                sigs.insert(0, receiver.to_string());
            }
        }
        let doc = doc_line(&head.docs);
        let indent = if matches!(owner, Owner::Class(_) | Owner::Holder(_)) {
            "    "
        } else {
            ""
        };
        let constructor =
            matches!(owner, Owner::Class(_)) && head.name == "new" && head.self_kind.is_none();
        let renamed = match owner {
            Owner::Holder(_) => head.name == "new" || head.name == "default",
            Owner::Class(_) => head.name == "default",
            Owner::Free | Owner::Hand(_) => false,
        };
        if !doc.is_empty() {
            writeln!(out, "{indent}/// {doc}").ok();
        }
        if constructor {
            writeln!(out, "{indent}#[wasm_bindgen(constructor)]").ok();
        } else if renamed {
            writeln!(out, "{indent}#[wasm_bindgen(js_name = \"{}\")]", head.name).ok();
        } else if indent.is_empty() {
            writeln!(out, "#[wasm_bindgen]").ok();
        }
        let rust_name = if renamed {
            format!("{}_", head.name)
        } else if indent.is_empty() {
            self.export(&name)
        } else {
            name.clone()
        };
        writeln!(
            out,
            "{indent}pub fn {rust_name}({}) -> Result<{}, JsValue> {{",
            sigs.join(", "),
            ret.sig
        )
        .ok();
        if dispatch {
            let rank = self
                .rank_source(&merged)
                .ok_or_else(|| format!("{}: no cell to dispatch on", head.path))?;
            writeln!(out, "{indent}    match {rank} {{").ok();
            let mut dims: Vec<u8> = group.iter().flat_map(|f| f.dims.iter().copied()).collect();
            dims.sort();
            dims.dedup();
            for dim in dims {
                let f = group
                    .iter()
                    .find(|f| f.dims.contains(&dim))
                    .ok_or_else(|| format!("{}: no entry for {dim}d", head.path))?;
                let cx = Cx {
                    dim: Some(dim),
                    fname: &head.path,
                };
                let ret = self.ret_plan(&f.ret, cx)?;
                let mut plans: Vec<ParamPlan> = vec![];
                for (n, ty, extra) in &merged {
                    if !self.mentions(f, owner, n) {
                        continue;
                    }
                    plans.push(self.plan_param(n, ty, cx, *extra)?);
                }
                writeln!(out, "{indent}        {dim} => {{").ok();
                out.push_str(&self.emit_body(
                    f,
                    owner,
                    &plans,
                    &ret,
                    cx,
                    &format!("{indent}            "),
                )?);
                writeln!(out, "{indent}        }}").ok();
            }
            writeln!(
                out,
                "{indent}        rank => Err(hand::refuse(&format!(\"a 2d or 3d cell was wanted, not {{rank}}d.\"))),"
            )
            .ok();
            writeln!(out, "{indent}    }}").ok();
        } else {
            out.push_str(&self.emit_body(
                head,
                owner,
                &plans,
                &ret,
                cx,
                &format!("{indent}    "),
            )?);
        }
        writeln!(out, "{indent}}}").ok();
        Ok(())
    }

    fn mentions(&self, f: &Function, owner: Owner, name: &str) -> bool {
        if f.params.iter().any(|p| p.name == name) {
            return true;
        }
        match owner {
            Owner::Hand(t) => {
                f.self_kind.is_some() && {
                    let label = hand_self(self.hand_name(t));
                    let taken = f.params.iter().any(|p| p.name == label);
                    (if taken { "self_" } else { label }) == name
                }
            }
            Owner::Holder(t) => {
                f.self_kind.is_some() && {
                    let label = t.name.to_lowercase();
                    let taken = f.params.iter().any(|p| p.name == label);
                    (if taken { "self_".to_string() } else { label }) == name
                }
            }
            _ => false,
        }
    }

    fn emit_body(
        &self,
        f: &Function,
        owner: Owner,
        plans: &[ParamPlan],
        ret: &RetPlan,
        cx: Cx,
        pad: &str,
    ) -> Result<String> {
        let mut out = String::new();
        for plan in plans {
            for line in &plan.pre {
                writeln!(out, "{pad}{line}").ok();
            }
        }
        let (receiver, rest): (Option<&ParamPlan>, &[ParamPlan]) = match owner {
            Owner::Hand(_) | Owner::Holder(_) if f.self_kind.is_some() => {
                (plans.first(), &plans[1..])
            }
            _ => (None, plans),
        };
        let args: Vec<String> = rest.iter().map(|p| p.arg.clone()).collect();
        let args = args.join(", ");
        let turbo = if f.dims.is_empty() {
            String::new()
        } else {
            format!("::<{}>", cx.dim.unwrap_or(2))
        };
        let call = match owner {
            Owner::Free => format!("mrlyrs::{}{turbo}({args})", f.path),
            Owner::Hand(t) => match receiver {
                Some(r) => {
                    let name = match r.post.is_empty() {
                        true => r.arg.trim_start_matches('&').to_string(),
                        false => format!("{}_value", r.name),
                    };
                    let name = if self.hand_name(t) == "Rng" {
                        r.arg.clone()
                    } else {
                        name
                    };
                    format!("{name}.{}({args})", f.name)
                }
                None => {
                    let base = self.rust_ty(
                        &Ty::Hand {
                            name: self.hand_name(t).into(),
                            dim: cx.dim,
                        },
                        cx,
                    )?;
                    let base = if self.hand_name(t) == "CellNd" {
                        format!(
                            "mrlyrs::math::cell::models::CellNd::<{}>",
                            cx.dim.unwrap_or(2)
                        )
                    } else {
                        base
                    };
                    format!("{base}::{}({args})", f.name)
                }
            },
            Owner::Class(t) => {
                let base = format!("mrlyrs::{}", t.path);
                let copied = self.ty(&t.path).is_some_and(|t| derives(t, "Copy"));
                let me = match f.self_kind {
                    Some(SelfKind::Value) if copied => "self.inner".to_string(),
                    Some(SelfKind::Value) => "self.inner.clone()".to_string(),
                    Some(SelfKind::Ref) => "&self.inner".to_string(),
                    Some(SelfKind::Mut) => "&mut self.inner".to_string(),
                    None => String::new(),
                };
                match (&f.via, f.self_kind) {
                    (Some(via), None) => format!("<{base} as mrlyrs::{via}>::{}({args})", f.name),
                    (Some(via), Some(_)) => {
                        let all = if args.is_empty() {
                            me
                        } else {
                            format!("{me}, {args}")
                        };
                        format!("<{base} as mrlyrs::{via}>::{}({all})", f.name)
                    }
                    (None, None) => format!("{base}::{}({args})", f.name),
                    (None, Some(SelfKind::Value)) => format!("{me}.{}({args})", f.name),
                    (None, Some(_)) => format!("self.inner.{}({args})", f.name),
                }
            }
            Owner::Holder(t) => match receiver {
                Some(r) => format!("{}.{}({args})", r.arg, f.name),
                None => format!("mrlyrs::{}::{}({args})", t.path, f.name),
            },
        };
        let fallible = matches!(f.ret, Ty::Result { .. });
        let unit = match &f.ret {
            Ty::Unit => true,
            Ty::Result { item } => matches!(**item, Ty::Unit),
            _ => false,
        };
        match (unit, fallible) {
            (true, true) => writeln!(out, "{pad}{call}.map_err(hand::throw)?;").ok(),
            (true, false) => writeln!(out, "{pad}{call};").ok(),
            (false, true) => writeln!(out, "{pad}let value = {call}.map_err(hand::throw)?;").ok(),
            (false, false) => writeln!(out, "{pad}let value = {call};").ok(),
        };
        for plan in plans {
            for line in &plan.post {
                writeln!(out, "{pad}{line}").ok();
            }
        }
        writeln!(out, "{pad}{}", ret.done).ok();
        Ok(out)
    }

    fn emit_const(&self, out: &mut String, c: &Const) -> Result<()> {
        let cx = Cx {
            dim: None,
            fname: &c.path,
        };
        let ret = self.ret_plan(&c.ty, cx)?;
        let doc = doc_line(&c.docs);
        if !doc.is_empty() {
            writeln!(out, "/// {doc}").ok();
        }
        let name = self.export(&self.local(&c.path));
        writeln!(out, "#[wasm_bindgen]").ok();
        writeln!(out, "pub fn {name}() -> Result<{}, JsValue> {{", ret.sig).ok();
        writeln!(out, "    let value = mrlyrs::{};", c.path).ok();
        writeln!(out, "    {}", ret.done).ok();
        writeln!(out, "}}").ok();
        Ok(())
    }

    fn getter_ok(&self, ty: &Ty) -> bool {
        let mut ok = true;
        ty.walk(&mut |t| match t {
            Ty::Unknown { .. } | Ty::Opaque { .. } => ok = false,
            Ty::Class { path, .. } if !self.holds(path) => ok = false,
            Ty::Ref {
                lifetime: Some(l),
                item,
                ..
            } if l == "'static" && !matches!(**item, Ty::Str) => ok = false,
            _ => {}
        });
        ok
    }

    fn emit_class(&self, out: &mut String, t: &Type, groups: &[Vec<&Function>]) -> Result<()> {
        let ident = self.local(&t.path);
        let cx = Cx {
            dim: None,
            fname: &t.path,
        };
        let doc = doc_line(&t.docs);
        if !doc.is_empty() {
            writeln!(out, "/// {doc}").ok();
        }
        writeln!(out, "#[wasm_bindgen]").ok();
        writeln!(
            out,
            "pub struct {ident} {{\n    inner: mrlyrs::{},\n}}\n",
            t.path
        )
        .ok();
        writeln!(out, "#[wasm_bindgen]\nimpl {ident} {{").ok();
        if derives(t, "Deserialize") {
            writeln!(out, "    /// Reads the {} from its plain data.", t.name).ok();
            writeln!(out, "    #[wasm_bindgen(js_name = \"from\")]").ok();
            writeln!(
                out,
                "    pub fn from_plain(data: JsValue) -> Result<{ident}, JsValue> {{"
            )
            .ok();
            writeln!(
                out,
                "        Ok({ident} {{ inner: hand::from_js(&data)? }})"
            )
            .ok();
            writeln!(out, "    }}").ok();
        }
        if derives(t, "Serialize") {
            writeln!(out, "    /// Writes the {} as plain data.", t.name).ok();
            writeln!(out, "    #[wasm_bindgen(js_name = \"toJSON\")]").ok();
            writeln!(
                out,
                "    pub fn to_plain(&self) -> Result<JsValue, JsValue> {{"
            )
            .ok();
            writeln!(out, "        hand::to_js(&self.inner)").ok();
            writeln!(out, "    }}").ok();
        }
        let methods: BTreeSet<&str> = groups.iter().map(|g| g[0].name.as_str()).collect();
        for field in t.fields.iter().filter(|f| f.public) {
            if !self.getter_ok(&field.ty) || methods.contains(field.name.as_str()) {
                continue;
            }
            let ret = self.ret_plan(&field.ty, cx)?;
            let doc = doc_line(&field.docs);
            if !doc.is_empty() {
                writeln!(out, "    /// {doc}").ok();
            }
            writeln!(out, "    #[wasm_bindgen(getter)]").ok();
            writeln!(
                out,
                "    pub fn {}(&self) -> Result<{}, JsValue> {{",
                field.name, ret.sig
            )
            .ok();
            let value = match field.ty {
                Ty::Ref { .. } => format!("self.inner.{}", field.name),
                _ if self.is_copy(&field.ty) => format!("self.inner.{}", field.name),
                _ => format!("self.inner.{}.clone()", field.name),
            };
            writeln!(out, "        let value = {value};").ok();
            writeln!(out, "        {}", ret.done).ok();
            writeln!(out, "    }}").ok();
            if field.ty.settable() {
                let plan = self.plan_param("value", &field.ty, cx, false)?;
                writeln!(out, "    #[wasm_bindgen(setter)]").ok();
                writeln!(
                    out,
                    "    pub fn set_{}(&mut self, {}: {}) -> Result<(), JsValue> {{",
                    field.name, plan.name, plan.sig
                )
                .ok();
                for line in &plan.pre {
                    writeln!(out, "        {line}").ok();
                }
                writeln!(out, "        self.inner.{} = {};", field.name, plan.arg).ok();
                for line in &plan.post {
                    writeln!(out, "        {line}").ok();
                }
                writeln!(out, "        Ok(())").ok();
                writeln!(out, "    }}").ok();
            }
        }
        for group in groups {
            self.emit_group(out, group, Owner::Class(t))?;
        }
        writeln!(out, "}}").ok();
        Ok(())
    }

    fn emit_holder(&self, out: &mut String, t: &Type, groups: &[Vec<&Function>]) -> Result<()> {
        let ident = self.local(&t.path);
        let doc = doc_line(&t.docs);
        if !doc.is_empty() {
            writeln!(out, "/// {doc}").ok();
        }
        writeln!(out, "#[wasm_bindgen]").ok();
        writeln!(out, "pub struct {ident} {{}}\n").ok();
        writeln!(out, "#[wasm_bindgen]\nimpl {ident} {{").ok();
        for group in groups {
            self.emit_group(out, group, Owner::Holder(t))?;
        }
        writeln!(out, "}}").ok();
        Ok(())
    }

    fn lib_rs(&self) -> Result<String> {
        let mut out = String::new();
        writeln!(
            out,
            "{ALLOWS}\n\nmod hand;\n\nuse wasm_bindgen::prelude::*;\n"
        )
        .ok();
        let groups = self.groups();
        let mut owned: BTreeMap<&str, Vec<Vec<&Function>>> = BTreeMap::new();
        for group in &groups {
            match self.owner_of(group[0]) {
                Owner::Free | Owner::Hand(_) => {
                    self.emit_group(&mut out, group, self.owner_of(group[0]))?;
                    out.push('\n');
                }
                Owner::Class(t) | Owner::Holder(t) => {
                    owned
                        .entry(t.path.as_str())
                        .or_default()
                        .push(group.clone());
                }
            }
        }
        for c in self.consts() {
            self.emit_const(&mut out, c)?;
            out.push('\n');
        }
        for t in self.classes() {
            let groups = owned.get(t.path.as_str()).cloned().unwrap_or_default();
            self.emit_class(&mut out, t, &groups)?;
            out.push('\n');
        }
        for t in self.holders() {
            let groups = owned.get(t.path.as_str()).cloned().unwrap_or_default();
            self.emit_holder(&mut out, t, &groups)?;
            out.push('\n');
        }
        while out.ends_with("\n\n") {
            out.pop();
        }
        Ok(out)
    }

    // WRAPPER

    fn tree(&self) -> Tree {
        let mut tree = Tree::default();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for f in self.functions() {
            let (home, leaf) = match self.owner_of(f) {
                Owner::Free | Owner::Hand(_) => (f.module.clone(), leaf(&f.path)),
                Owner::Class(t) | Owner::Holder(t) => (parent(&t.path), t.name.clone()),
            };
            let key = format!("{home}::{leaf}");
            if seen.insert(key) {
                let export = match self.owner_of(f) {
                    Owner::Free | Owner::Hand(_) => self.export(&self.local(&f.path)),
                    Owner::Class(t) | Owner::Holder(t) => self.local(&t.path),
                };
                if JS_RESERVED.contains(&leaf.as_str()) {
                    tree.insert(self.rel(&home), format!("{leaf}_"), export.clone());
                }
                tree.insert(self.rel(&home), leaf, export);
            }
        }
        for t in self.classes() {
            let key = format!("{}::{}", parent(&t.path), t.name);
            if seen.insert(key) {
                tree.insert(
                    self.rel(&parent(&t.path)),
                    t.name.clone(),
                    self.local(&t.path),
                );
            }
        }
        for c in self.consts() {
            let export = self.export(&self.local(&c.path));
            tree.insert(self.rel(&parent(&c.path)), c.name.clone(), export);
        }
        tree
    }

    fn wrapper_js(&self) -> String {
        let name = self.name;
        let mut out = String::new();
        writeln!(
            out,
            "import * as wasm from \"./pkg/{name}/mrlyjs_{name}.js\";\n"
        )
        .ok();
        writeln!(
            out,
            "export {{ default, initSync }} from \"./pkg/{name}/mrlyjs_{name}.js\";"
        )
        .ok();
        writeln!(out, "export const Rng = wasm.Rng;").ok();
        let tree = self.tree();
        for (leaf, export) in &tree.leaves {
            writeln!(out, "export const {leaf} = wasm.{export};").ok();
        }
        for (seg, child) in &tree.children {
            writeln!(out, "export const {seg} = {};", child.literal(1)).ok();
        }
        out
    }

    // TYPES

    fn ts_ref(&self, path: &str) -> String {
        self.rel(path).replace("::", ".")
    }

    fn ts_direct(&self, ty: &Ty, ret: bool) -> String {
        match ty {
            Ty::Unit => "void".into(),
            Ty::Scalar { name } => match name.as_str() {
                "bool" => "boolean".into(),
                "char" => "string".into(),
                "u64" | "i64" if ret => "bigint".into(),
                "u64" | "i64" => "number | bigint".into(),
                _ => "number".into(),
            },
            Ty::Str | Ty::String => "string".into(),
            Ty::U128 | Ty::I128 | Ty::Code if ret => "string".into(),
            Ty::U128 | Ty::I128 | Ty::Code => "string | number | bigint".into(),
            Ty::Json => "any".into(),
            Ty::Vec { item }
            | Ty::Slice { item, .. }
            | Ty::Array { item, .. }
            | Ty::Set { item } => match &**item {
                Ty::Scalar { name } if TYPED.contains(&name.as_str()) => {
                    if ret {
                        typed_array(name).into()
                    } else if is_wide(name) {
                        "ArrayLike<number | bigint>".into()
                    } else {
                        "ArrayLike<number>".into()
                    }
                }
                _ => format!("{}[]", self.ts_wrap(&self.ts_direct(item, ret))),
            },
            Ty::Option { item } => format!("{} | undefined", self.ts_direct(item, ret)),
            Ty::Tuple { items } => format!(
                "[{}]",
                items
                    .iter()
                    .map(|t| self.ts_direct(t, ret))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Ty::Ref { item, .. } | Ty::Result { item } => self.ts_direct(item, ret),
            Ty::Map { value, .. } => format!("Record<string, {}>", self.ts_direct(value, ret)),
            Ty::Hand { name, .. } => match name.as_str() {
                "Tensor" => "Tensor".into(),
                "Cell" | "CellNd" => "Cell".into(),
                "Cell6d" => "Cell6d".into(),
                "Color" => "Color".into(),
                "Code" if ret => "string".into(),
                "Code" => "string | number | bigint".into(),
                _ => "Rng".into(),
            },
            Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } => self.ts_ref(path),
            Ty::Opaque { .. } | Ty::Unknown { .. } => "unknown".into(),
        }
    }

    fn ts_wrap(&self, text: &str) -> String {
        if text.contains(" | ") {
            format!("({text})")
        } else {
            text.to_string()
        }
    }

    fn ts_serde(&self, ty: &Ty) -> String {
        match ty {
            Ty::Unit => "null".into(),
            Ty::Scalar { name } => match name.as_str() {
                "bool" => "boolean".into(),
                "char" => "string".into(),
                _ => "number".into(),
            },
            Ty::Str | Ty::String => "string".into(),
            Ty::U128 | Ty::I128 | Ty::Code => "bigint".into(),
            Ty::Json => "any".into(),
            Ty::Vec { item }
            | Ty::Slice { item, .. }
            | Ty::Array { item, .. }
            | Ty::Set { item } => {
                format!("{}[]", self.ts_wrap(&self.ts_serde(item)))
            }
            Ty::Option { item } => format!("{} | undefined", self.ts_serde(item)),
            Ty::Tuple { items } => format!(
                "[{}]",
                items
                    .iter()
                    .map(|t| self.ts_serde(t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Ty::Ref { item, .. } | Ty::Result { item } => self.ts_serde(item),
            Ty::Map { value, .. } => format!("Record<string, {}>", self.ts_serde(value)),
            Ty::Hand { name, .. } => match name.as_str() {
                "Tensor" => "TensorData".into(),
                "Cell" => "CellData".into(),
                "CellNd" => "{ cell: CellData }".into(),
                "Cell6d" => "Cell6dData".into(),
                "Color" => "ColorData".into(),
                "Code" => "bigint".into(),
                _ => "unknown".into(),
            },
            Ty::Plain { path } | Ty::Enum { path } => self.ts_ref(path),
            Ty::Class { path, .. } => format!("{}Data", self.ts_ref(path)),
            Ty::Opaque { .. } | Ty::Unknown { .. } => "string".into(),
        }
    }

    fn ts_params(&self, params: &[(String, Ty, bool)]) -> String {
        let last_required = params
            .iter()
            .rposition(|(_, ty, extra)| !matches!(ty, Ty::Option { .. }) && !extra)
            .map(|i| i + 1)
            .unwrap_or(0);
        params
            .iter()
            .enumerate()
            .map(|(i, (name, ty, extra))| {
                let n = ident(name);
                let inner = match ty {
                    Ty::Option { item } => item,
                    _ => ty,
                };
                if *extra || (i >= last_required && matches!(ty, Ty::Option { .. })) {
                    format!("{n}?: {}", self.ts_direct(inner, false))
                } else {
                    format!("{n}: {}", self.ts_direct(ty, false))
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn ts_function(&self, group: &[&Function], owner: Owner) -> (String, String, String) {
        let head = group[0];
        let mut merged: Vec<(String, Ty, bool)> = vec![];
        for f in group {
            match owner {
                Owner::Hand(t) => {
                    if let Ok(Some((label, ty))) = self.self_param(f, t) {
                        if !merged.iter().any(|(n, _, _)| *n == label) {
                            merged.push((label, ty, false));
                        }
                    }
                }
                Owner::Holder(t) if f.self_kind.is_some() => {
                    let label = t.name.to_lowercase();
                    let label = if f.params.iter().any(|p| p.name == label) {
                        "self_".to_string()
                    } else {
                        label
                    };
                    if !merged.iter().any(|(n, _, _)| *n == label) {
                        merged.push((label, plain_ty(t), false));
                    }
                }
                _ => {}
            }
            for p in &f.params {
                if !merged.iter().any(|(n, _, _)| *n == p.name) {
                    merged.push((p.name.clone(), p.ty.clone(), false));
                }
            }
        }
        for slot in &mut merged {
            slot.2 = !group.iter().all(|f| self.mentions(f, owner, &slot.0));
        }
        let ret = match &head.ret {
            Ty::Result { item } => self.ts_direct(item, true),
            other => self.ts_direct(other, true),
        };
        (doc_line(&head.docs), self.ts_params(&merged), ret)
    }

    fn referenced(&self) -> BTreeSet<String> {
        let mut out: BTreeSet<String> = BTreeSet::new();
        let mut queue: Vec<String> = vec![];
        let note = |ty: &Ty, queue: &mut Vec<String>| {
            ty.walk(&mut |t| {
                if let Ty::Plain { path } | Ty::Enum { path } | Ty::Class { path, .. } = t {
                    queue.push(path.clone());
                }
            })
        };
        for f in self.functions() {
            for p in &f.params {
                note(&p.ty, &mut queue);
            }
            note(&f.ret, &mut queue);
        }
        for c in self.consts() {
            note(&c.ty, &mut queue);
        }
        for t in self.classes() {
            queue.push(t.path.clone());
        }
        for t in self.holders() {
            queue.push(t.path.clone());
        }
        while let Some(path) = queue.pop() {
            if !out.insert(path.clone()) {
                continue;
            }
            let Some(t) = self.ty(&path) else { continue };
            for field in &t.fields {
                note(&field.ty, &mut queue);
            }
            for v in &t.variants {
                for field in &v.fields {
                    note(&field.ty, &mut queue);
                }
            }
        }
        out
    }

    fn ts_type_decl(&self, t: &Type, groups: &[Vec<&Function>], pad: &str) -> Result<String> {
        let mut out = String::new();
        let doc = doc_line(&t.docs);
        let name = &t.name;
        match &t.cross {
            TypeCross::Enum { .. } => {
                let words: Vec<String> = t
                    .variants
                    .iter()
                    .map(|v| enum_word(t, &v.name, &v.serde))
                    .collect();
                let union = words
                    .iter()
                    .map(|w| format!("{w:?}"))
                    .collect::<Vec<_>>()
                    .join(" | ");
                if !doc.is_empty() {
                    writeln!(out, "{pad}/** {doc} */").ok();
                }
                writeln!(out, "{pad}export type {name} = {union};").ok();
            }
            TypeCross::Plain if t.kind == TypeKind::Alias => {
                if let Some(alias) = &t.alias {
                    writeln!(out, "{pad}export type {name} = {};", self.ts_serde(alias)).ok();
                }
            }
            TypeCross::Plain if t.kind == TypeKind::Enum => {
                if !doc.is_empty() {
                    writeln!(out, "{pad}/** {doc} */").ok();
                }
                writeln!(out, "{pad}export type {name} = {};", self.ts_union(t)).ok();
            }
            TypeCross::Plain => {
                if !doc.is_empty() {
                    writeln!(out, "{pad}/** {doc} */").ok();
                }
                writeln!(out, "{pad}export interface {name} {{").ok();
                self.ts_fields(&mut out, t, &format!("{pad}    "), true);
                writeln!(out, "{pad}}}").ok();
            }
            TypeCross::Class => {
                let all_public = t.fields.iter().all(|f| f.public);
                let data = format!("{name}Data");
                if t.kind == TypeKind::Enum {
                    writeln!(out, "{pad}export type {data} = {};", self.ts_union(t)).ok();
                } else if all_public {
                    writeln!(out, "{pad}export interface {data} {{").ok();
                    self.ts_fields(&mut out, t, &format!("{pad}    "), false);
                    writeln!(out, "{pad}}}").ok();
                } else {
                    writeln!(out, "{pad}export type {data} = Record<string, unknown>;").ok();
                }
                if !doc.is_empty() {
                    writeln!(out, "{pad}/** {doc} */").ok();
                }
                writeln!(out, "{pad}export class {name} {{").ok();
                let inner = format!("{pad}    ");
                let constructor = groups
                    .iter()
                    .find(|g| g[0].name == "new" && g[0].self_kind.is_none());
                match constructor {
                    Some(g) => {
                        let (d, params, _) = self.ts_function(g, Owner::Class(t));
                        if !d.is_empty() {
                            writeln!(out, "{inner}/** {d} */").ok();
                        }
                        writeln!(out, "{inner}constructor({params});").ok();
                    }
                    None => {
                        writeln!(out, "{inner}private constructor();").ok();
                    }
                }
                writeln!(out, "{inner}free(): void;").ok();
                if derives(t, "Deserialize") {
                    writeln!(out, "{inner}/** Reads the {name} from its plain data. */").ok();
                    writeln!(out, "{inner}static from(data: {data}): {name};").ok();
                }
                if derives(t, "Serialize") {
                    writeln!(out, "{inner}/** Writes the {name} as plain data. */").ok();
                    writeln!(out, "{inner}toJSON(): {data};").ok();
                }
                let methods: BTreeSet<&str> = groups.iter().map(|g| g[0].name.as_str()).collect();
                for field in t.fields.iter().filter(|f| f.public) {
                    if !self.getter_ok(&field.ty) || methods.contains(field.name.as_str()) {
                        continue;
                    }
                    let d = doc_line(&field.docs);
                    if !d.is_empty() {
                        writeln!(out, "{inner}/** {d} */").ok();
                    }
                    let out_ty = self.ts_direct(&field.ty, true);
                    if field.ty.settable() {
                        writeln!(out, "{inner}get {}(): {out_ty};", field.name).ok();
                        writeln!(
                            out,
                            "{inner}set {}(value: {});",
                            field.name,
                            self.ts_direct(&field.ty, false)
                        )
                        .ok();
                    } else {
                        writeln!(out, "{inner}readonly {}: {out_ty};", ident(&field.name)).ok();
                    }
                }
                for g in groups {
                    let f = g[0];
                    if f.name == "new" && f.self_kind.is_none() {
                        continue;
                    }
                    let (d, params, ret) = self.ts_function(g, Owner::Class(t));
                    if !d.is_empty() {
                        writeln!(out, "{inner}/** {d} */").ok();
                    }
                    let prefix = if f.self_kind.is_none() { "static " } else { "" };
                    writeln!(out, "{inner}{prefix}{}({params}): {ret};", f.name).ok();
                }
                writeln!(out, "{pad}}}").ok();
            }
            TypeCross::Hand { .. } | TypeCross::Uncrossable { .. } => {}
        }
        if matches!(t.cross, TypeCross::Plain | TypeCross::Enum { .. }) && !groups.is_empty() {
            writeln!(out, "{pad}export const {name}: {{").ok();
            for g in groups {
                let f = g[0];
                let (d, params, ret) = self.ts_function(g, Owner::Holder(t));
                if !d.is_empty() {
                    writeln!(out, "{pad}    /** {d} */").ok();
                }
                let label = if f.name == "new" {
                    "\"new\"".to_string()
                } else {
                    f.name.clone()
                };
                writeln!(out, "{pad}    {label}({params}): {ret};").ok();
            }
            writeln!(out, "{pad}}};").ok();
        }
        Ok(out)
    }

    fn ts_union(&self, t: &Type) -> String {
        let untagged = t.serde.iter().any(|s| s == "untagged");
        let mut arms = vec![];
        for v in &t.variants {
            let word = enum_word(t, &v.name, &v.serde);
            let body = if v.fields.is_empty() {
                None
            } else if v.fields.iter().all(|f| f.name.parse::<usize>().is_ok()) {
                Some(if v.fields.len() == 1 {
                    self.ts_serde(&v.fields[0].ty)
                } else {
                    format!(
                        "[{}]",
                        v.fields
                            .iter()
                            .map(|f| self.ts_serde(&f.ty))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
            } else {
                Some(format!(
                    "{{ {} }}",
                    v.fields
                        .iter()
                        .map(|f| format!("{}: {}", f.name, self.ts_serde(&f.ty)))
                        .collect::<Vec<_>>()
                        .join("; ")
                ))
            };
            arms.push(match (untagged, body) {
                (true, None) => "null".to_string(),
                (false, None) => format!("{word:?}"),
                (true, Some(body)) => body,
                (false, Some(body)) => format!("{{ {word}: {body} }}"),
            });
        }
        arms.join(" | ")
    }

    fn ts_fields(&self, out: &mut String, t: &Type, pad: &str, all: bool) {
        for field in t
            .fields
            .iter()
            .filter(|f| (all || f.public) && !f.serde_skip)
        {
            let d = doc_line(&field.docs);
            if !d.is_empty() {
                writeln!(out, "{pad}/** {d} */").ok();
            }
            let (name, ty) = match &field.ty {
                Ty::Option { item } => (format!("{}?", ident(&field.name)), self.ts_serde(item)),
                other => (ident(&field.name), self.ts_serde(other)),
            };
            writeln!(out, "{pad}{name}: {ty};").ok();
        }
    }

    fn types_dts(&self) -> Result<String> {
        let name = self.name;
        let mut out = String::new();
        writeln!(
            out,
            "export {{ default, initSync }} from \"./pkg/{name}/mrlyjs_{name}.js\";\n"
        )
        .ok();
        let words = |path: &str| -> String {
            self.ty(path)
                .map(|t| {
                    t.variants
                        .iter()
                        .map(|v| enum_word(t, &v.name, &v.serde))
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default()
                .iter()
                .map(|w| format!("{w:?}"))
                .collect::<Vec<_>>()
                .join(" | ")
        };
        let projection = words("math::six::Projection");
        let orientation = words("math::six::Orientation");
        writeln!(out, "/** An rgba color as four bytes. */").ok();
        writeln!(out, "export type Color = [number, number, number, number];").ok();
        writeln!(
            out,
            "/** A tensor: its shape and its flat data as a typed array of its dtype. */"
        )
        .ok();
        writeln!(out, "export interface Tensor {{\n    shape: number[];\n    data: Uint8Array | Uint16Array | Uint32Array | Int32Array;\n}}").ok();
        writeln!(out, "/** A cell: the shape, the type bytes, and the flat rgba colors and the tags when present. */").ok();
        writeln!(out, "export interface Cell {{\n    shape: number[];\n    types: Uint8Array | Uint16Array | Uint32Array | Int32Array;\n    colors?: Uint8Array;\n    tags?: Uint8Array | Uint16Array | Uint32Array | Int32Array;\n}}").ok();
        writeln!(
            out,
            "/** A hex cell: a flat cell with its projection, orientation and start row. */"
        )
        .ok();
        writeln!(out, "export interface Cell6d {{\n    cell: Cell;\n    projection: {projection};\n    orientation: {orientation};\n    start: number;\n}}").ok();
        writeln!(out, "/** A color inside plain data, serde's form. */").ok();
        writeln!(out, "export interface ColorData {{\n    r: number;\n    g: number;\n    b: number;\n    a: number;\n}}").ok();
        writeln!(out, "/** A tensor inside plain data, serde's form. */").ok();
        writeln!(out, "export interface TensorData {{\n    shape: number[];\n    data: {{ U8: number[] }} | {{ U16: number[] }} | {{ U32: number[] }} | {{ I32: number[] }};\n}}").ok();
        writeln!(out, "/** A cell inside plain data, serde's form. */").ok();
        writeln!(out, "export interface CellData {{\n    types: TensorData;\n    colors?: number[][];\n    tags?: TensorData;\n}}").ok();
        writeln!(out, "/** A hex cell inside plain data, serde's form. */").ok();
        writeln!(out, "export interface Cell6dData {{\n    cell: {{ cell: CellData }};\n    projection: {projection};\n    orientation: {orientation};\n    start: number;\n}}").ok();
        writeln!(
            out,
            "/** A seeded random stream, opened from a number or a bigint seed. */"
        )
        .ok();
        writeln!(out, "export class Rng {{\n    constructor(seed: number | bigint | string);\n    free(): void;\n    /** Draws a float at or above zero and below one. */\n    unit(): number;\n    /** Draws an integer below n, or zero when n is zero. */\n    below(n: number): number;\n    /** Draws an integer between lo and hi inclusive, or lo when hi is not above lo. */\n    range(lo: number, hi: number): number;\n    /** Draws a fair coin flip. */\n    boolean(): boolean;\n    /** Returns true with probability p. */\n    chance(p: number): boolean;\n    /** Draws amount distinct indices below length, or every index when amount is larger. */\n    sample_indices(length: number, amount: number): Uint32Array;\n    /** Draws one item of the array, the same draw as Rust's choice. */\n    choice<T>(items: ArrayLike<T>): T;\n    /** Shuffles the array in place, the same permutation as Rust's shuffle. */\n    shuffle<T>(items: T[]): void;\n}}").ok();
        let mut tree = DeclTree::default();
        let groups = self.groups();
        let mut owned: BTreeMap<&str, Vec<Vec<&Function>>> = BTreeMap::new();
        for group in &groups {
            match self.owner_of(group[0]) {
                Owner::Free | Owner::Hand(_) => {
                    let (d, params, ret) = self.ts_function(group, self.owner_of(group[0]));
                    let mut text = String::new();
                    if !d.is_empty() {
                        text.push_str(&format!("/** {d} */\n"));
                    }
                    text.push_str(&format!(
                        "export function {}({params}): {ret};\n",
                        ts_leaf(&leaf(&group[0].path))
                    ));
                    tree.insert(self.rel(&group[0].module), text);
                }
                Owner::Class(t) | Owner::Holder(t) => {
                    owned
                        .entry(t.path.as_str())
                        .or_default()
                        .push(group.clone());
                }
            }
        }
        for c in self.consts() {
            let mut text = String::new();
            let d = doc_line(&c.docs);
            if !d.is_empty() {
                text.push_str(&format!("/** {d} */\n"));
            }
            text.push_str(&format!(
                "export function {}(): {};\n",
                c.name,
                self.ts_direct(&c.ty, true)
            ));
            tree.insert(self.rel(&parent(&c.path)), text);
        }
        for path in self.referenced() {
            let Some(t) = self.ty(&path) else { continue };
            if matches!(
                t.cross,
                TypeCross::Hand { .. } | TypeCross::Uncrossable { .. }
            ) {
                continue;
            }
            let groups = owned.get(path.as_str()).cloned().unwrap_or_default();
            let text = self.ts_type_decl(t, &groups, "")?;
            tree.insert(self.rel(&parent(&t.path)), text);
        }
        out.push_str(&tree.render(0));
        while out.ends_with("\n\n") {
            out.pop();
        }
        Ok(out)
    }
}

// TREES

#[derive(Default)]
struct Tree {
    leaves: BTreeMap<String, String>,
    children: BTreeMap<String, Tree>,
}

impl Tree {
    fn insert(&mut self, module: &str, leaf: String, export: String) {
        let mut node = self;
        for seg in module.split("::").filter(|s| !s.is_empty()) {
            node = node.children.entry(seg.to_string()).or_default();
        }
        node.leaves.insert(leaf, export);
    }

    fn literal(&self, depth: usize) -> String {
        let pad = "    ".repeat(depth);
        let mut out = String::from("{\n");
        for (leaf, export) in &self.leaves {
            out.push_str(&format!("{pad}{leaf}: wasm.{export},\n"));
        }
        for (seg, child) in &self.children {
            out.push_str(&format!("{pad}{seg}: {},\n", child.literal(depth + 1)));
        }
        out.push_str(&"    ".repeat(depth - 1));
        out.push('}');
        out
    }
}

#[derive(Default)]
struct DeclTree {
    texts: Vec<String>,
    children: BTreeMap<String, DeclTree>,
}

impl DeclTree {
    fn insert(&mut self, module: &str, text: String) {
        let mut node = self;
        for seg in module.split("::").filter(|s| !s.is_empty()) {
            node = node.children.entry(seg.to_string()).or_default();
        }
        node.texts.push(text);
    }

    fn render(&self, depth: usize) -> String {
        let pad = "    ".repeat(depth);
        let mut out = String::new();
        for text in &self.texts {
            for line in text.lines() {
                out.push_str(&pad);
                out.push_str(line);
                out.push('\n');
            }
        }
        for (seg, child) in &self.children {
            let keyword = if depth == 0 {
                "export declare namespace"
            } else {
                "export namespace"
            };
            out.push_str(&format!("{pad}{keyword} {seg} {{\n"));
            out.push_str(&child.render(depth + 1));
            out.push_str(&format!("{pad}}}\n"));
        }
        out
    }
}

// RULES

fn deref(e: &str) -> String {
    match e.strip_prefix('&') {
        Some(inner) => inner.to_string(),
        None => format!("*{e}"),
    }
}

fn place(e: &str) -> String {
    e.strip_prefix('&').unwrap_or(e).to_string()
}

fn closure(x: &str, body: String) -> String {
    let tail = format!("({x})");
    match body.strip_suffix(&tail) {
        Some(path)
            if !path.contains(x) && !path.contains(' ') || path.starts_with("hand::from_js::<") =>
        {
            path.to_string()
        }
        _ => format!("|{x}| {body}"),
    }
}

fn ok(expr: String) -> String {
    match expr.strip_suffix('?') {
        Some(inner) => inner.to_string(),
        None => format!("Ok({expr})"),
    }
}

fn is_wide(name: &str) -> bool {
    name == "u64" || name == "i64"
}

fn is_rng(ty: &Ty) -> bool {
    matches!(ty, Ty::Hand { name, .. } if name == "Rng")
}

fn typed_scalar(ty: &Ty) -> bool {
    matches!(ty, Ty::Scalar { name } if TYPED.contains(&name.as_str()))
}

fn native_item(ty: &Ty) -> bool {
    matches!(ty, Ty::Scalar { name } if NATIVE_ITEM.contains(&name.as_str()))
}

fn typed_array(name: &str) -> &'static str {
    match name {
        "u8" => "Uint8Array",
        "u16" => "Uint16Array",
        "u32" | "usize" => "Uint32Array",
        "i8" => "Int8Array",
        "i16" => "Int16Array",
        "i32" => "Int32Array",
        "f32" => "Float32Array",
        "f64" => "Float64Array",
        "u64" => "BigUint64Array",
        _ => "BigInt64Array",
    }
}

fn pure_in(ty: &Ty) -> bool {
    match ty {
        Ty::Unit | Ty::Str | Ty::String | Ty::Json | Ty::Plain { .. } | Ty::Enum { .. } => true,
        Ty::Scalar { name } => !is_wide(name),
        Ty::Vec { item }
        | Ty::Slice { item, .. }
        | Ty::Option { item }
        | Ty::Array { item, .. } => pure_in(item),
        Ty::Ref {
            mutable: false,
            item,
            ..
        } => pure_in(item),
        Ty::Tuple { items } => items.iter().all(pure_in),
        _ => false,
    }
}

fn pure_out(ty: &Ty) -> bool {
    match ty {
        Ty::Unit | Ty::Str | Ty::String | Ty::Json | Ty::Plain { .. } | Ty::Enum { .. } => true,
        Ty::Scalar { name } => !is_wide(name),
        Ty::Vec { item } | Ty::Slice { item, .. } | Ty::Array { item, .. } => {
            !typed_scalar(item) && pure_out(item)
        }
        Ty::Option { item } => pure_out(item),
        Ty::Tuple { items } => items.iter().all(pure_out),
        _ => false,
    }
}

fn plain_ty(t: &Type) -> Ty {
    match t.cross {
        TypeCross::Enum { .. } => Ty::Enum {
            path: t.path.clone(),
        },
        _ => Ty::Plain {
            path: t.path.clone(),
        },
    }
}

fn hand_self(name: &str) -> &'static str {
    match name {
        "Tensor" => "tensor",
        "Color" => "color",
        "Code" => "code",
        "Rng" => "rng",
        _ => "cell",
    }
}

fn suffix_dim(fname: &str) -> u8 {
    if fname.ends_with("_3d") || fname.ends_with("_6d") {
        3
    } else {
        2
    }
}

fn ident(name: &str) -> String {
    if WORDS.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

fn derives(t: &Type, what: &str) -> bool {
    t.derives.iter().any(|d| d == what)
}

fn doc_line(docs: &[String]) -> String {
    docs.first()
        .map(|d| d.trim().to_string())
        .unwrap_or_default()
}

fn ts_leaf(name: &str) -> String {
    if JS_RESERVED.contains(&name) {
        format!("{name}_")
    } else {
        name.to_string()
    }
}

fn leaf(path: &str) -> String {
    path.rsplit("::").next().unwrap_or(path).to_string()
}

fn parent(path: &str) -> String {
    match path.rfind("::") {
        Some(at) => path[..at].to_string(),
        None => String::new(),
    }
}

fn enum_word(t: &Type, variant: &str, serde: &[String]) -> String {
    for attr in serde {
        if let Some(rest) = attr.trim().strip_prefix("rename = ") {
            return rest.trim().trim_matches('"').to_string();
        }
    }
    for attr in &t.serde {
        if let Some(rest) = attr.trim().strip_prefix("rename_all = ") {
            let rule = rest.trim().trim_matches('"');
            return rename_all(variant, rule);
        }
    }
    variant.to_string()
}

fn rename_all(name: &str, rule: &str) -> String {
    let words: Vec<String> = {
        let mut out: Vec<String> = vec![];
        for c in name.chars() {
            if c.is_uppercase() || out.is_empty() {
                out.push(String::new());
            }
            out.last_mut().unwrap().push(c);
        }
        out.into_iter().map(|w| w.to_lowercase()).collect()
    };
    match rule {
        "lowercase" => name.to_lowercase(),
        "UPPERCASE" => name.to_uppercase(),
        "snake_case" => words.join("_"),
        "SCREAMING_SNAKE_CASE" => words.join("_").to_uppercase(),
        "kebab-case" => words.join("-"),
        "SCREAMING-KEBAB-CASE" => words.join("-").to_uppercase(),
        "camelCase" => {
            let mut c = name.chars();
            match c.next() {
                Some(first) => first.to_lowercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        }
        _ => name.to_string(),
    }
}

// TEST

const MANIFEST_TEST: &str = r#"import { expect, test } from "bun:test";

const manifest = await Bun.file(new URL("../../bridge/manifest.json", import.meta.url)).json();
const units = (await Bun.file(new URL("../../bridge/units.txt", import.meta.url)).text())
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean);
const pkg = await Bun.file(new URL("./package.json", import.meta.url)).json();
const kinds = new Map(manifest.types.map((t) => [t.path, t.cross.kind]));

function parent(path) {
    const at = path.lastIndexOf("::");
    return at < 0 ? "" : path.slice(0, at);
}

function relative(unit, path) {
    return unit === "all" ? path : path.slice(unit.length + 2);
}

function walk(mod, path) {
    let node = mod;
    for (const seg of path.split("::").filter(Boolean)) {
        if (node === undefined || node === null) return undefined;
        node = node[seg];
    }
    return node;
}

function holds(unit, path) {
    return unit === "all" || path.split("::")[0] === unit;
}

function check(mod, unit, f) {
    const kind = f.owner ? kinds.get(f.owner) : undefined;
    if (kind === "class") {
        const cls = walk(mod, relative(unit, f.owner));
        if (typeof cls !== "function") return false;
        if (f.self_kind === null && f.name === "new") return true;
        if (f.self_kind === null) return typeof cls[f.name] === "function";
        return typeof cls.prototype[f.name] === "function";
    }
    if (kind === "plain" || kind === "enum") {
        const holder = walk(mod, relative(unit, f.owner));
        return typeof holder === "function" && typeof holder[f.name] === "function";
    }
    return typeof walk(mod, relative(unit, f.path)) === "function";
}

for (const unit of units) {
    test(`${unit} exports every ok name`, async () => {
        expect(pkg.exports[unit === "all" ? "." : `./${unit}`]).toBeDefined();
        const mod = await import(`./${unit}.js`);
        const bytes = await Bun.file(new URL(`./pkg/${unit}/mrlyjs_${unit}_bg.wasm`, import.meta.url)).arrayBuffer();
        mod.initSync({ module: bytes });
        expect(typeof mod.Rng).toBe("function");
        const missing = [];
        const names = new Set();
        for (const f of manifest.functions) {
            if (f.cross.status !== "ok" || !holds(unit, f.module)) continue;
            names.add(f.path);
            if (!check(mod, unit, f)) missing.push(f.path);
        }
        for (const c of manifest.consts) {
            if (c.cross.status !== "ok" || !holds(unit, c.path)) continue;
            names.add(c.path);
            if (typeof walk(mod, relative(unit, c.path)) !== "function") missing.push(c.path);
        }
        for (const t of manifest.types) {
            if (t.cross.kind !== "class" || !holds(unit, t.path)) continue;
            names.add(t.path);
            if (typeof walk(mod, relative(unit, t.path)) !== "function") missing.push(t.path);
        }
        expect(missing).toEqual([]);
        console.log(`${unit}: ${names.size} names`);
    });
}
"#;
