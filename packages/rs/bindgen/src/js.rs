use crate::disk::save;
use crate::doors::{BlobText, Table};
use crate::model::{
    Const, Cross, Field, Function, Hand, Manifest, Result, SelfKind, Ty, Type, TypeCross, TypeKind,
    hand, hands, row,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

const ALL: &str = crate::config::ALL;

const TYPED: &[&str] = &[
    "u8", "u16", "u32", "i8", "i16", "i32", "f32", "f64", "usize", "u64", "i64",
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

const STATE: [&str; 2] = ["__state", "__restore"];

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

pub fn field_word(t: &Type, field: &Field) -> String {
    enum_word(t, &field.name, &field.serde)
}

pub fn write(out: &Path, manifest: &Manifest, name: &str, table: &Table) -> Result<()> {
    let unit = Unit::new(manifest, name);
    let dir = out.join("js").join(name);
    save(&dir.join("index.js"), &unit.facade(table))?;
    save(&dir.join("index.d.ts"), &unit.types_dts()?)?;
    save(
        &dir.join("home.js"),
        &format!("export {{ default }} from \"../blobs/{ALL}.js\";\n"),
    )?;
    save(
        &out.join("api").join(format!("{name}.json")),
        &api_json(name, table),
    )
}

pub fn blob(
    out: &Path,
    name: &str,
    crates: &[&str],
    made: &BlobText,
    shapes: &[Vec<String>],
) -> Result<()> {
    let mut text = String::from("import { blob } from \"@mrlyprod/loader\";\n\nconst shapes = [\n");
    for shape in shapes {
        let fields: Vec<String> = shape.iter().map(|one| format!("{one:?}: r()")).collect();
        writeln!(text, "  (r) => ({{ {} }}),", fields.join(", ")).ok();
    }
    let at = mrly_json::write(&made.at).map_err(|e| e.to_string())?;
    let rng = mrly_json::write(&made.rng).map_err(|e| e.to_string())?;
    writeln!(
        text,
        "];\n\nconst home = blob({name:?}, shapes, {at}, {rng});\n\nexport default home;\n\nconst fs = globalThis.process?.getBuiltinModule?.(\"node:fs\");\nif (fs) {{\n  try {{\n    home.initSync({{ module: fs.readFileSync(new URL(\"./{name}.wasm\", import.meta.url)) }});\n  }} catch {{}}\n}}"
    )
    .ok();
    save(&out.join("js/blobs").join(format!("{name}.js")), &text)?;
    let mut load =
        format!("import home from \"./{name}.js\";\nimport url from \"./{name}.wasm\";\n");
    for one in crates {
        writeln!(load, "import * as {one} from \"../{one}/index.js\";").ok();
    }
    writeln!(
        load,
        "\nexport {{ {} }};\nexport const ready = home.init({{ module_or_path: url }});",
        crates.join(", ")
    )
    .ok();
    save(&out.join("js/blobs").join(format!("{name}.load.ts")), &load)
}

pub fn units(out: &Path, names: &[String]) -> Result<()> {
    let mut text = String::from("export const LOAD = {\n");
    for name in names {
        writeln!(text, "  {name}: () => import(\"../blobs/{ALL}.load.ts\"),").ok();
    }
    text.push_str("};\n");
    save(&out.join("js/units/index.ts"), &text)
}

pub fn classes_used(manifest: &Manifest, name: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let inside = |path: &str| path.split("::").next() == Some(name);
    let mut note = |ty: &Ty| {
        ty.walk(&mut |one| {
            if let Ty::Class { path, .. } = one
                && !inside(path)
                && let Some(home) = path.split("::").next()
            {
                out.insert(home.to_string());
            }
        });
    };
    for f in manifest
        .functions
        .iter()
        .filter(|f| f.cross == Cross::Ok && inside(&f.module))
    {
        for p in &f.params {
            note(&p.ty);
        }
        note(&f.ret);
    }
    out
}

fn api_json(name: &str, table: &Table) -> String {
    let direct: mrly_json::Map = table
        .direct
        .iter()
        .map(|(key, info)| {
            (
                key.clone(),
                mrly_json::json!({
                    "at": info.id,
                    "bool": info.boolean,
                    "int": info.whole,
                    "fail": info.fails,
                    "self": info.owner,
                    "n": info.count,
                }),
            )
        })
        .collect();
    let classes: mrly_json::Map = table
        .classes
        .iter()
        .map(|(key, info)| {
            (
                key.clone(),
                mrly_json::json!({
                    "fields": info.fields,
                    "methods": info.methods,
                    "statics": info.statics,
                    "from": info.from,
                    "json": info.json,
                }),
            )
        })
        .collect();
    mrly_json::json!({
        "unit": name,
        "doors": table.names,
        "direct": direct,
        "classes": classes,
        "consts": table.consts,
    })
    .to_string()
}

pub fn field_skips(manifest: &Manifest) -> Vec<(String, String)> {
    let mut out = vec![];
    for t in manifest
        .types
        .iter()
        .filter(|t| t.cross == TypeCross::Class)
    {
        let home = t.path.split("::").next();
        for field in t.fields.iter().filter(|f| f.public) {
            let mut foreign = false;
            field.ty.walk(&mut |ty| {
                if let Ty::Class { path, .. } = ty {
                    foreign |= path.split("::").next() != home;
                }
            });
            if foreign {
                out.push((
                    format!("{}::{}", t.path, field.name),
                    "field type is a class of another unit".to_string(),
                ));
            }
        }
    }
    out
}

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
        for row in hands().iter().filter(|row| row.stream) {
            let class = row.ts.to_lowercase();
            let path = self.manifest.hand_type(&row.name).map(|t| t.path.as_str());
            for f in &self.manifest.functions {
                if path.is_some() && f.owner.as_deref() == path {
                    out.insert(format!("{class}_{}", f.name));
                }
            }
            for method in STATE {
                out.insert(format!("{class}_{method}"));
            }
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

    fn holds(&self, path: &str) -> bool {
        path.split("::").next() == Some(self.name)
    }

    fn rel<'p>(&self, path: &'p str) -> &'p str {
        match path.strip_prefix(self.name) {
            Some("") => "",
            Some(rest) => rest.strip_prefix("::").unwrap_or(path),
            None => path,
        }
    }

    fn local(&self, path: &str) -> String {
        path.replace("::", "_")
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

    fn hand_row(&self, t: &Type) -> Result<&'static Hand> {
        match &t.cross {
            TypeCross::Hand { name } => row(name),
            _ => Err(format!("{} is no hand type", t.path)),
        }
    }

    fn self_param(&self, f: &Function, owner: &Type) -> Result<Option<(String, Ty)>> {
        let Some(kind) = f.self_kind else {
            return Ok(None);
        };
        let row = self.hand_row(owner)?;
        let inner = Ty::Hand {
            name: row.name.clone(),
            dim: None,
        };
        let taken = f.params.iter().any(|p| p.name == row.word);
        let label = if taken {
            "self_".to_string()
        } else {
            row.word.clone()
        };
        let ty = match kind {
            SelfKind::Mut => Ty::Ref {
                mutable: true,
                lifetime: None,
                item: Box::new(inner),
            },
            _ if row.stream => {
                return Err(format!("{}: a {} self must be mutable", f.path, row.name));
            }
            _ => inner,
        };
        Ok(Some((label, ty)))
    }

    fn mentions(&self, f: &Function, owner: Owner, name: &str) -> bool {
        if f.params.iter().any(|p| p.name == name) {
            return true;
        }
        match owner {
            Owner::Hand(t) => {
                f.self_kind.is_some()
                    && self.hand_row(t).is_ok_and(|row| {
                        let label = row.word.as_str();
                        let taken = f.params.iter().any(|p| p.name == label);
                        (if taken { "self_" } else { label }) == name
                    })
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

    fn tree(&self, table: &Table) -> Tree {
        let door = |name: &str| table.names.iter().position(|one| one == name);
        let mut tree = Tree::default();
        let mut holders: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
        for f in self.functions() {
            let dotted = f.path.replace("::", ".");
            let Some(at) = door(&dotted) else { continue };
            match self.owner_of(f) {
                Owner::Free | Owner::Hand(_) => {
                    tree.insert(
                        self.rel(&f.module),
                        ts_leaf(&leaf(&f.path)),
                        format!("$d({at})"),
                    );
                }
                Owner::Class(_) => {}
                Owner::Holder(t) => {
                    let list = holders.entry(t.path.clone()).or_default();
                    if !list.iter().any(|(name, _)| *name == f.name) {
                        list.push((f.name.clone(), format!("$d({at})")));
                    }
                }
            }
        }
        for (path, list) in holders {
            let parts: Vec<String> = list
                .iter()
                .map(|(name, value)| format!("{name:?}: {value}"))
                .collect();
            tree.insert(
                self.rel(&parent(&path)),
                leaf(&path),
                format!("{{ {} }}", parts.join(", ")),
            );
        }
        for t in self.classes() {
            let dotted = t.path.replace("::", ".");
            if table.classes.contains_key(&dotted) {
                tree.insert(
                    self.rel(&parent(&t.path)),
                    t.name.clone(),
                    format!("$c({dotted:?})"),
                );
            }
        }
        for c in self.consts() {
            let dotted = c.path.replace("::", ".");
            if let Some(at) = door(&dotted) {
                tree.insert(
                    self.rel(&parent(&c.path)),
                    c.name.clone(),
                    format!("$k({at})"),
                );
            }
        }
        tree
    }

    fn facade(&self, table: &Table) -> String {
        let name = self.name;
        let mut out = String::new();
        writeln!(out, "import {{ unit as $unit }} from \"@mrlyprod/loader\";").ok();
        writeln!(out, "import $home from \"./home.js\";").ok();
        for one in classes_used(self.manifest, name) {
            writeln!(out, "import \"../{one}/index.js\";").ok();
        }
        writeln!(
            out,
            "\nconst $made = $unit($home, {});",
            api_json(name, table)
        )
        .ok();
        writeln!(
            out,
            "const {{ door: $d, constant: $k, klass: $c, init: $init, initSync: $initSync, Rng: $Rng }} = $made;\n"
        )
        .ok();
        writeln!(
            out,
            "export default $init;\nexport {{ $initSync as initSync, $Rng as Rng }};\n"
        )
        .ok();
        let tree = self.tree(table);
        for (leaf, export) in &tree.leaves {
            writeln!(out, "export const {leaf} = {export};").ok();
        }
        for (seg, child) in &tree.children {
            writeln!(out, "export const {seg} = {};", child.literal(1)).ok();
        }
        out
    }

    fn getter_ok(&self, ty: &Ty, owner: &str) -> bool {
        let home = owner.split("::").next();
        let mut ok = true;
        ty.walk(&mut |t| match t {
            Ty::Unknown { .. } | Ty::Opaque { .. } => ok = false,
            Ty::Class { path, .. } if !self.holds(path) || path.split("::").next() != home => {
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

    fn ts_ref(&self, path: &str) -> String {
        let dotted = self.rel(path).replace("::", ".");
        if self.holds(path) {
            dotted
        } else {
            format!("${dotted}")
        }
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
            Ty::Hand { name, .. } => match hand(name) {
                Some(row) if row.big => self.ts_direct(&Ty::Code, ret),
                Some(row) => row.ts.clone(),
                None => "unknown".into(),
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
            Ty::Hand { name, .. } => hand(name)
                .map_or("unknown", |row| row.ts_data.as_str())
                .into(),
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
                    if let Ok(Some((label, ty))) = self.self_param(f, t)
                        && !merged.iter().any(|(n, _, _)| *n == label)
                    {
                        merged.push((label, ty, false));
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
        for t in self.manifest.types.iter().filter(|t| self.holds(&t.path)) {
            queue.push(t.path.clone());
        }
        while let Some(path) = queue.pop() {
            if !out.insert(path.clone()) {
                continue;
            }
            if !self.holds(&path) {
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
                    writeln!(out, "{inner}static from(data: {data}): {name};").ok();
                }
                if derives(t, "Serialize") {
                    writeln!(out, "{inner}toJSON(): {data};").ok();
                }
                let methods: BTreeSet<&str> = groups.iter().map(|g| g[0].name.as_str()).collect();
                for field in t.fields.iter().filter(|f| f.public) {
                    if !self.getter_ok(&field.ty, &t.path) || methods.contains(field.name.as_str())
                    {
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
        let mut out = String::new();
        let referenced = self.referenced();
        let homes: BTreeSet<&str> = referenced
            .iter()
            .filter(|path| !self.holds(path))
            .filter_map(|path| path.split("::").next())
            .collect();
        for home in &homes {
            writeln!(out, "import type * as ${home} from \"../{home}/index\";").ok();
        }
        out.push_str(
            "export default function init(source: unknown): Promise<unknown>;\nexport function initSync(source: unknown): unknown;\n\n",
        );
        for row in hands().iter().filter(|row| !row.decl.is_empty()) {
            writeln!(out, "{}", row.decl).ok();
        }
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
        for path in &referenced {
            if !self.holds(path) {
                continue;
            }
            let Some(t) = self.ty(path) else { continue };
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
            out.push_str(&format!("{pad}{leaf}: {export},\n"));
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

fn is_wide(name: &str) -> bool {
    name == "u64" || name == "i64"
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
