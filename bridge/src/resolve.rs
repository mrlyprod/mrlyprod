use crate::model::*;
use crate::parse::{self, Files, Item, Tree};
use quote::ToTokens;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

pub struct Built {
    pub manifest: Manifest,
    pub collisions: Vec<String>,
    pub macro_body_fns: usize,
}

type Key = (String, usize);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Target {
    Module(String),
    Item(String, usize),
    External(Vec<String>),
}

pub fn build(files: &Files, version: &str) -> Result<Built> {
    let tree = parse::parse(files)?;
    let reach = reach(&tree);
    let mut builder = Builder {
        tree: &tree,
        reach,
        cross: HashMap::new(),
        paths: HashMap::new(),
    };
    builder.classify_types()?;
    let manifest = builder.manifest(version);
    let collisions = collisions(&manifest);
    Ok(Built {
        manifest,
        collisions,
        macro_body_fns: tree.macro_body_fns,
    })
}

// LOOKUP

fn parent(key: &str) -> String {
    match key.rfind("::") {
        Some(i) => key[..i].to_string(),
        None => String::new(),
    }
}

fn join(module: &str, name: &str) -> String {
    if module.is_empty() {
        name.to_string()
    } else {
        format!("{module}::{name}")
    }
}

fn item_name(item: &Item) -> Option<&str> {
    match item {
        Item::Fn(f) => Some(&f.name),
        Item::Struct(s) => Some(&s.name),
        Item::Enum(e) => Some(&e.name),
        Item::Alias(a) => Some(&a.name),
        Item::Const(c) => Some(&c.name),
        Item::Trait(t) => Some(&t.name),
        Item::Impl(_) => None,
    }
}

type Stack = Vec<(String, String)>;

fn lookup(tree: &Tree, module: &str, name: &str, origin: &str, stack: &mut Stack) -> Vec<Target> {
    let mut out = vec![];
    let Some(m) = tree.modules.get(module) else {
        return out;
    };
    let key = (module.to_string(), name.to_string());
    if stack.len() > 32 || stack.contains(&key) {
        return out;
    }
    stack.push(key);
    let inside =
        module.is_empty() || origin == module || origin.starts_with(&format!("{module}::"));
    if m.children.iter().any(|c| c == name) {
        let child = join(module, name);
        if inside || tree.modules.get(&child).is_some_and(|c| c.public) {
            out.push(Target::Module(child));
        }
    }
    for (i, item) in m.items.iter().enumerate() {
        if item_name(item) == Some(name) {
            out.push(Target::Item(module.to_string(), i));
        }
    }
    for u in m
        .uses
        .iter()
        .filter(|u| !u.glob && u.name == name && (u.public || inside))
    {
        out.extend(resolve_path(tree, module, &u.path, stack));
    }
    for u in m.uses.iter().filter(|u| u.glob && (u.public || inside)) {
        for target in resolve_path(tree, module, &u.path, stack) {
            if let Target::Module(p) = target {
                out.extend(lookup(tree, &p, name, origin, stack));
            }
        }
    }
    stack.pop();
    out.dedup();
    out
}

fn resolve_path(tree: &Tree, origin: &str, segs: &[String], stack: &mut Stack) -> Vec<Target> {
    if segs.is_empty() {
        return vec![];
    }
    let mut rest = &segs[1..];
    let mut current = match segs[0].as_str() {
        "crate" => vec![Target::Module(String::new())],
        "self" => vec![Target::Module(origin.to_string())],
        "super" => {
            let mut at = parent(origin);
            while rest.first().map(String::as_str) == Some("super") {
                at = parent(&at);
                rest = &rest[1..];
            }
            vec![Target::Module(at)]
        }
        first => {
            let found = lookup(tree, origin, first, origin, stack);
            if found.is_empty() {
                return vec![Target::External(segs.to_vec())];
            }
            found
        }
    };
    for (i, seg) in rest.iter().enumerate() {
        let last = i + 1 == rest.len();
        let mut next = vec![];
        for target in current {
            match target {
                Target::Module(p) => next.extend(lookup(tree, &p, seg, origin, stack)),
                Target::External(mut e) => {
                    e.push(seg.clone());
                    next.push(Target::External(e));
                }
                Target::Item(..) => {}
            }
        }
        if !last {
            next.retain(|t| !matches!(t, Target::Item(..)));
        }
        current = next;
    }
    current
}

fn lookup_from(tree: &Tree, module: &str, name: &str) -> Vec<Target> {
    lookup(tree, module, name, module, &mut Stack::new())
}

fn resolve_from(tree: &Tree, module: &str, segs: &[String]) -> Vec<Target> {
    resolve_path(tree, module, segs, &mut Stack::new())
}

// REACH

struct Reach {
    modules: HashMap<String, Vec<Vec<String>>>,
    items: HashMap<Key, Vec<Vec<String>>>,
}

fn public_names(tree: &Tree, module: &str) -> Vec<(String, Target)> {
    let mut out = vec![];
    let Some(m) = tree.modules.get(module) else {
        return out;
    };
    for (i, item) in m.items.iter().enumerate() {
        if let Some(name) = item_name(item) {
            out.push((name.to_string(), Target::Item(module.to_string(), i)));
        }
    }
    for c in &m.children {
        let key = join(module, c);
        if tree.modules.get(&key).is_some_and(|child| child.public) {
            out.push((c.clone(), Target::Module(key)));
        }
    }
    for u in m.uses.iter().filter(|u| u.public) {
        if u.glob {
            for target in resolve_from(tree, module, &u.path) {
                if let Target::Module(p) = target {
                    out.extend(public_names(tree, &p));
                }
            }
        } else {
            for target in resolve_from(tree, module, &u.path) {
                out.push((u.name.clone(), target));
            }
        }
    }
    out
}

fn reach(tree: &Tree) -> Reach {
    let mut reach = Reach {
        modules: HashMap::new(),
        items: HashMap::new(),
    };
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(String::new(), Vec::<String>::new())]);
    while let Some((module, path)) = queue.pop_front() {
        if path.len() > 12 || !seen.insert((module.clone(), path.clone())) {
            continue;
        }
        reach
            .modules
            .entry(module.clone())
            .or_default()
            .push(path.clone());
        for (name, target) in public_names(tree, &module) {
            let mut at = path.clone();
            at.push(name);
            match target {
                Target::Item(m, i) => reach.items.entry((m, i)).or_default().push(at),
                Target::Module(m) if tree.modules.get(&m).is_some_and(|x| x.public) => {
                    queue.push_back((m, at))
                }
                Target::Module(_) | Target::External(_) => {}
            }
        }
    }
    reach
}

fn shortest(candidates: &[Vec<String>], canonical: &str) -> Option<String> {
    candidates
        .iter()
        .map(|c| c.join("::"))
        .min_by_key(|p| (p.matches("::").count(), p != canonical, p.clone()))
}

// BUILDER

struct Scope<'a> {
    module: &'a str,
    generics: Vec<String>,
    self_ty: Option<Ty>,
}

struct Owner {
    path: Option<String>,
    module: String,
    cross: TypeCross,
    dim: Option<u8>,
    ty: Ty,
}

struct Builder<'a> {
    tree: &'a Tree,
    reach: Reach,
    cross: HashMap<Key, TypeCross>,
    paths: HashMap<Key, String>,
}

impl Builder<'_> {
    fn module(&self, key: &str) -> &parse::Module {
        &self.tree.modules[key]
    }

    fn public_path(&self, key: &Key, name: &str) -> Option<String> {
        let canonical = join(&key.0, name);
        self.reach
            .items
            .get(key)
            .and_then(|c| shortest(c, &canonical))
    }

    fn module_path(&self, key: &str) -> Option<String> {
        self.reach.modules.get(key).and_then(|c| shortest(c, key))
    }

    fn nearest_public_module(&self, key: &str) -> String {
        let mut at = key.to_string();
        loop {
            if let Some(p) = self.module_path(&at) {
                return p;
            }
            if at.is_empty() {
                return at;
            }
            at = parent(&at);
        }
    }

    fn impl_owner(&self, module: &str, imp: &parse::Impl) -> Option<(Key, Option<u8>)> {
        let syn::Type::Path(p) = &imp.self_ty else {
            return None;
        };
        let segs: Vec<String> = p
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        let dim = p.path.segments.last().and_then(|s| dim_of(&s.arguments));
        let targets = if segs.len() == 1 {
            lookup_from(self.tree, module, &segs[0])
        } else {
            resolve_from(self.tree, module, &segs)
        };
        targets.into_iter().find_map(|t| match t {
            Target::Item(m, i)
                if matches!(self.module(&m).items[i], Item::Struct(_) | Item::Enum(_)) =>
            {
                Some(((m, i), dim))
            }
            _ => None,
        })
    }

    fn trait_of(&self, module: &str, path: &[String]) -> Option<(Key, &parse::Trait)> {
        let targets = if path.len() == 1 {
            lookup_from(self.tree, module, &path[0])
        } else {
            resolve_from(self.tree, module, path)
        };
        targets.into_iter().find_map(|t| match t {
            Target::Item(m, i) => match &self.module(&m).items[i] {
                Item::Trait(tr) => Some(((m, i), tr)),
                _ => None,
            },
            _ => None,
        })
    }

    fn classify_types(&mut self) -> Result<()> {
        let mut with_methods = HashSet::new();
        let mut hand_seen: HashMap<String, usize> = HashMap::new();
        for (mk, m) in &self.tree.modules {
            for item in &m.items {
                let Item::Impl(imp) = item else { continue };
                let Some((key, _)) = self.impl_owner(mk, imp) else {
                    continue;
                };
                let fns = match &imp.trait_path {
                    Some(path) => match self.trait_of(mk, path) {
                        Some((_, tr)) => &tr.fns,
                        None => continue,
                    },
                    None => &imp.fns,
                };
                if fns.iter().any(|f| f.self_kind.is_some() && !f.generated) {
                    with_methods.insert(key);
                }
            }
        }
        for (mk, m) in &self.tree.modules {
            for (i, item) in m.items.iter().enumerate() {
                let key = (mk.clone(), i);
                let (name, cross) = match item {
                    Item::Struct(s) => {
                        let cross = if HAND.contains(&s.name.as_str()) {
                            *hand_seen.entry(s.name.clone()).or_default() += 1;
                            TypeCross::Hand {
                                name: s.name.clone(),
                            }
                        } else if with_methods.contains(&key) || s.fields.iter().any(|f| f.serde_skip) {
                            TypeCross::Class
                        } else if serde_both(&s.derives) {
                            TypeCross::Plain
                        } else {
                            TypeCross::Uncrossable {
                                reason: "no serde derives and no methods".into(),
                            }
                        };
                        (&s.name, cross)
                    }
                    Item::Enum(e) => {
                        let unit = e.variants.iter().all(|v| v.fields.is_empty());
                        let cross = if unit && serde_both(&e.derives) {
                            TypeCross::Enum {
                                named: e.named,
                                words: e.variants.iter().filter_map(|v| v.word.clone()).collect(),
                            }
                        } else if with_methods.contains(&key)
                            || e.variants.iter().flat_map(|v| &v.fields).any(|f| f.serde_skip)
                        {
                            TypeCross::Class
                        } else if serde_both(&e.derives) {
                            TypeCross::Plain
                        } else {
                            TypeCross::Uncrossable {
                                reason: "no serde derives and no methods".into(),
                            }
                        };
                        (&e.name, cross)
                    }
                    Item::Trait(_)
                    | Item::Fn(_)
                    | Item::Alias(_)
                    | Item::Const(_)
                    | Item::Impl(_) => continue,
                };
                if let Some(p) = self.public_path(&key, name) {
                    self.paths.insert(key.clone(), p);
                }
                self.cross.insert(key, cross);
            }
        }
        for (name, count) in hand_seen {
            if count > 1 {
                return Err(format!("hand type {name} is defined {count} times"));
            }
        }
        Ok(())
    }

    // TYPES

    fn ty(&self, scope: &Scope, t: &syn::Type) -> Ty {
        let text = t.to_token_stream().to_string();
        match t {
            syn::Type::Path(p) => self.path_ty(scope, p, &text),
            syn::Type::Reference(r) => {
                let lifetime = r.lifetime.as_ref().map(|l| format!("'{}", l.ident));
                let mutable = r.mutability.is_some();
                match (&*r.elem, &lifetime) {
                    (syn::Type::Slice(s), None) => Ty::Slice {
                        mutable,
                        item: Box::new(self.ty(scope, &s.elem)),
                    },
                    (syn::Type::Path(p), None)
                        if !mutable && p.qself.is_none() && p.path.is_ident("str") =>
                    {
                        Ty::Str
                    }
                    _ => Ty::Ref {
                        mutable,
                        lifetime,
                        item: Box::new(self.ty(scope, &r.elem)),
                    },
                }
            }
            syn::Type::Slice(s) => Ty::Slice {
                mutable: false,
                item: Box::new(self.ty(scope, &s.elem)),
            },
            syn::Type::Array(a) => match &a.len {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(n),
                    ..
                }) => Ty::Array {
                    item: Box::new(self.ty(scope, &a.elem)),
                    len: n.base10_parse().unwrap_or(0),
                },
                _ => Ty::Unknown { text },
            },
            syn::Type::Tuple(t) if t.elems.is_empty() => Ty::Unit,
            syn::Type::Tuple(t) => Ty::Tuple {
                items: t.elems.iter().map(|e| self.ty(scope, e)).collect(),
            },
            syn::Type::Paren(p) => self.ty(scope, &p.elem),
            syn::Type::ImplTrait(_) => Ty::Unknown {
                text: format!("impl Trait argument {text}"),
            },
            _ => Ty::Unknown { text },
        }
    }

    fn path_ty(&self, scope: &Scope, p: &syn::TypePath, text: &str) -> Ty {
        let unknown = || Ty::Unknown {
            text: text.to_string(),
        };
        if p.qself.is_some() {
            return unknown();
        }
        let segs: Vec<String> = p
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        let last = p.path.segments.last().expect("a path has a segment");
        let args: Vec<&syn::Type> = match &last.arguments {
            syn::PathArguments::AngleBracketed(a) => a
                .args
                .iter()
                .filter_map(|g| match g {
                    syn::GenericArgument::Type(t) => Some(t),
                    _ => None,
                })
                .collect(),
            _ => vec![],
        };
        let arg = |i: usize| args.get(i).map(|t| Box::new(self.ty(scope, t)));
        if segs.len() == 1 {
            let name = segs[0].as_str();
            if name == "Self" {
                return scope.self_ty.clone().unwrap_or_else(unknown);
            }
            if scope.generics.iter().any(|g| g == name) {
                return Ty::Unknown {
                    text: format!("generic {name}"),
                };
            }
            match name {
                "bool" | "u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32" | "i64"
                | "f32" | "f64" | "char" => {
                    return Ty::Scalar {
                        name: name.to_string(),
                    }
                }
                "u128" => return Ty::U128,
                "i128" => return Ty::I128,
                "String" => return Ty::String,
                "str" => return Ty::Str,
                "Vec" => return arg(0).map_or_else(unknown, |item| Ty::Vec { item }),
                "Option" => return arg(0).map_or_else(unknown, |item| Ty::Option { item }),
                "Result" => {
                    let crate_error = args.len() == 1
                        || (args.len() == 2
                            && matches!(self.ty(scope, args[1]), Ty::Opaque { path } if path.rsplit("::").next() == Some("Error")));
                    return match (crate_error, arg(0)) {
                        (true, Some(item)) => Ty::Result { item },
                        _ => unknown(),
                    };
                }
                _ => {}
            }
        }
        let targets = if segs.len() == 1 {
            lookup_from(self.tree, scope.module, &segs[0])
        } else {
            resolve_from(self.tree, scope.module, &segs)
        };
        for target in targets {
            match target {
                Target::Item(m, i) => return self.item_ty(&(m, i), dim_of(&last.arguments), text),
                Target::External(e) => {
                    return match e.join("::").as_str() {
                        "serde_json::Value" | "serde_json::Map" => Ty::Json,
                        "std::collections::HashMap" | "std::collections::BTreeMap"
                            if args.len() == 2 =>
                        {
                            Ty::Map {
                                key: arg(0).expect("a key"),
                                value: arg(1).expect("a value"),
                            }
                        }
                        "std::collections::BTreeSet" if args.len() == 1 => Ty::Set {
                            item: arg(0).expect("an item"),
                        },
                        _ => unknown(),
                    }
                }
                Target::Module(_) => {}
            }
        }
        if segs.len() == 1 {
            return Ty::Unknown {
                text: format!("unknown type {text}"),
            };
        }
        unknown()
    }

    fn item_ty(&self, key: &Key, dim: Option<u8>, text: &str) -> Ty {
        let item = &self.module(&key.0).items[key.1];
        if let Item::Alias(a) = item {
            if a.generic {
                return Ty::Unknown {
                    text: format!("generic alias {text}"),
                };
            }
            let scope = Scope {
                module: &key.0,
                generics: vec![],
                self_ty: None,
            };
            return self.ty(&scope, &a.ty);
        }
        let Some(cross) = self.cross.get(key) else {
            return Ty::Unknown {
                text: text.to_string(),
            };
        };
        let name = item_name(item).unwrap_or_default();
        let Some(path) = self.paths.get(key).cloned() else {
            return Ty::Unknown {
                text: format!("private type {name}"),
            };
        };
        match cross {
            TypeCross::Hand { name } if name == "Code" => Ty::Code,
            TypeCross::Hand { name } => Ty::Hand {
                name: name.clone(),
                dim,
            },
            TypeCross::Class => Ty::Class { path, dim },
            TypeCross::Plain => Ty::Plain { path },
            TypeCross::Enum { .. } => Ty::Enum { path },
            TypeCross::Uncrossable { .. } => Ty::Opaque { path },
        }
    }

    fn owner(&self, module: &str, imp: &parse::Impl) -> Option<Owner> {
        let (key, dim) = self.impl_owner(module, imp)?;
        let cross = self.cross[&key].clone();
        let path = self.paths.get(&key).cloned();
        let ty = self.item_ty(&key, dim, "Self");
        let target = match cross {
            TypeCross::Hand { .. } => self.nearest_public_module(&key.0),
            _ => path.as_deref().map(parent).unwrap_or_else(|| key.0.clone()),
        };
        Some(Owner {
            path,
            module: target,
            cross,
            dim,
            ty,
        })
    }

    // ENTRIES

    fn manifest(&self, version: &str) -> Manifest {
        let mut types = vec![];
        let mut consts = vec![];
        let mut functions = vec![];
        for (mk, m) in &self.tree.modules {
            let scope = Scope {
                module: mk,
                generics: vec![],
                self_ty: None,
            };
            for (i, item) in m.items.iter().enumerate() {
                let key = (mk.clone(), i);
                match item {
                    Item::Struct(s) => {
                        let Some(path) = self.paths.get(&key) else {
                            continue;
                        };
                        types.push(Type {
                            path: path.clone(),
                            name: s.name.clone(),
                            defined_at: format!("{}:{}", m.file, s.line),
                            docs: s.docs.clone(),
                            kind: TypeKind::Struct,
                            cross: self.cross[&key].clone(),
                            derives: s.derives.clone(),
                            serde: s.serde.clone(),
                            const_generic: s.const_generic,
                            fields: self.fields(&scope, &s.fields),
                            variants: vec![],
                            alias: None,
                        });
                    }
                    Item::Enum(e) => {
                        let Some(path) = self.paths.get(&key) else {
                            continue;
                        };
                        types.push(Type {
                            path: path.clone(),
                            name: e.name.clone(),
                            defined_at: format!("{}:{}", m.file, e.line),
                            docs: e.docs.clone(),
                            kind: TypeKind::Enum,
                            cross: self.cross[&key].clone(),
                            derives: e.derives.clone(),
                            serde: e.serde.clone(),
                            const_generic: false,
                            fields: vec![],
                            variants: e
                                .variants
                                .iter()
                                .map(|v| Variant {
                                    name: v.name.clone(),
                                    word: v.word.clone(),
                                    docs: v.docs.clone(),
                                    serde: v.serde.clone(),
                                    fields: self.fields(&scope, &v.fields),
                                })
                                .collect(),
                            alias: None,
                        });
                    }
                    Item::Alias(a) => {
                        let Some(path) = self.public_path(&key, &a.name) else {
                            continue;
                        };
                        let ty = self.item_ty(&key, None, &a.name);
                        let cross = match &ty {
                            Ty::Hand { name, .. } => TypeCross::Hand { name: name.clone() },
                            Ty::Class { .. } => TypeCross::Class,
                            Ty::Enum { .. } => TypeCross::Enum {
                                named: false,
                                words: vec![],
                            },
                            _ => match uncrossable(&ty) {
                                Some(reason) => TypeCross::Uncrossable { reason },
                                None => TypeCross::Plain,
                            },
                        };
                        types.push(Type {
                            path,
                            name: a.name.clone(),
                            defined_at: format!("{}:{}", m.file, a.line),
                            docs: a.docs.clone(),
                            kind: TypeKind::Alias,
                            cross,
                            derives: vec![],
                            serde: vec![],
                            const_generic: false,
                            fields: vec![],
                            variants: vec![],
                            alias: Some(ty),
                        });
                    }
                    Item::Const(c) => {
                        let Some(path) = self.public_path(&key, &c.name) else {
                            continue;
                        };
                        let ty = self.ty(&scope, &c.ty);
                        let cross = match uncrossable(&ty)
                            .or_else(|| borrow(&ty).map(|b| format!("{b} const")))
                        {
                            Some(reason) => Cross::Skip { reason },
                            None => Cross::Ok,
                        };
                        consts.push(Const {
                            path,
                            name: c.name.clone(),
                            defined_at: format!("{}:{}", m.file, c.line),
                            docs: c.docs.clone(),
                            ty,
                            cross,
                        });
                    }
                    Item::Fn(f) => {
                        let path = self.public_path(&key, &f.name);
                        functions.push(self.function(
                            mk,
                            &m.file,
                            f,
                            path,
                            None,
                            join(mk, &f.name),
                        ));
                    }
                    Item::Trait(_) => {}
                    Item::Impl(imp) if imp.trait_path.is_some() => {
                        let path = imp.trait_path.as_deref().unwrap_or_default();
                        let Some((tkey, tr)) = self.trait_of(mk, path) else {
                            continue;
                        };
                        let Some(via) = self.public_path(&tkey, &tr.name) else {
                            continue;
                        };
                        let Some(owner) = self.owner(mk, imp) else {
                            continue;
                        };
                        let Some(owner_path) = owner.path.clone() else {
                            continue;
                        };
                        let file = self.module(&tkey.0).file.clone();
                        for f in &tr.fns {
                            let path = match owner.cross {
                                TypeCross::Hand { .. } => join(&owner.module, &f.name),
                                _ => format!("{owner_path}::{}", f.name),
                            };
                            let mut entry = self.function(
                                &tkey.0,
                                &file,
                                f,
                                Some(path.clone()),
                                Some((&owner, imp)),
                                path,
                            );
                            entry.module = owner.module.clone();
                            entry.source = Source::Trait;
                            entry.via = Some(via.clone());
                            functions.push(entry);
                        }
                    }
                    Item::Impl(imp) => {
                        let Some(owner) = self.owner(mk, imp) else {
                            let text = imp.self_ty.to_token_stream().to_string().replace(' ', "");
                            for f in &imp.fns {
                                let fallback = format!("{}::{}", join(mk, &text), f.name);
                                functions.push(self.function(mk, &m.file, f, None, None, fallback));
                            }
                            continue;
                        };
                        for f in &imp.fns {
                            let path = owner.path.as_ref().map(|p| match owner.cross {
                                TypeCross::Hand { .. } => join(&owner.module, &f.name),
                                _ => format!("{p}::{}", f.name),
                            });
                            let fallback = format!(
                                "{}::{}",
                                owner.path.clone().unwrap_or_else(|| join(mk, "?")),
                                f.name
                            );
                            let mut entry =
                                self.function(mk, &m.file, f, path, Some((&owner, imp)), fallback);
                            if entry.cross != Cross::Private {
                                entry.module = owner.module.clone();
                            }
                            functions.push(entry);
                        }
                    }
                }
            }
        }
        types.sort_by(|a, b| a.path.cmp(&b.path));
        consts.sort_by(|a, b| a.path.cmp(&b.path));
        functions.sort_by(|a, b| {
            (&a.path, &a.dims, &a.defined_at).cmp(&(&b.path, &b.dims, &b.defined_at))
        });
        let mut modules: Vec<Module> = self
            .tree
            .modules
            .iter()
            .filter_map(|(mk, m)| {
                let path = self.module_path(mk)?;
                let items = functions
                    .iter()
                    .filter(|f| f.cross != Cross::Private && f.module == path)
                    .count()
                    + types.iter().filter(|t| parent(&t.path) == path).count()
                    + consts.iter().filter(|c| parent(&c.path) == path).count();
                Some(Module {
                    path,
                    file: m.file.clone(),
                    docs: m.docs.clone(),
                    items,
                })
            })
            .collect();
        modules.sort_by(|a, b| a.path.cmp(&b.path));
        modules.dedup_by(|a, b| a.path == b.path);
        Manifest {
            krate: "mrlyrs".into(),
            version: version.into(),
            modules,
            types,
            consts,
            functions,
        }
    }

    fn fields(&self, scope: &Scope, fields: &[parse::Field]) -> Vec<Field> {
        fields
            .iter()
            .map(|f| Field {
                name: f.name.clone(),
                public: f.public,
                docs: f.docs.clone(),
                serde: f.serde.clone(),
                serde_skip: f.serde_skip,
                ty: self.ty(scope, &f.ty),
            })
            .collect()
    }

    fn function(
        &self,
        module: &str,
        file: &str,
        f: &parse::Fn,
        path: Option<String>,
        owner: Option<(&Owner, &parse::Impl)>,
        fallback: String,
    ) -> Function {
        let mut generics: Vec<String> = f.type_params.iter().map(|(n, _)| n.clone()).collect();
        let mut dims = vec![];
        if let Some((o, imp)) = owner {
            generics.extend(imp.type_params.iter().cloned());
            match o.dim {
                Some(d) => dims.push(d),
                None if !imp.const_params.is_empty() => dims.extend([2, 3]),
                None => {}
            }
        }
        if dims.is_empty() && !f.const_params.is_empty() {
            dims.extend([2, 3]);
        }
        let scope = Scope {
            module,
            generics,
            self_ty: owner.map(|(o, _)| o.ty.clone()),
        };
        let params: Vec<Param> = f
            .params
            .iter()
            .map(|(name, ty)| Param {
                name: name.clone(),
                ty: self.ty(&scope, ty),
            })
            .collect();
        let ret = f.ret.as_ref().map_or(Ty::Unit, |t| self.ty(&scope, t));
        let private = path.is_none();
        let path = path.unwrap_or(fallback);
        let mut entry = Function {
            path: path.clone(),
            name: f.name.clone(),
            module: parent(&path),
            defined_at: format!("{file}:{}", f.line),
            docs: f.docs.clone(),
            owner: owner.and_then(|(o, _)| o.path.clone()),
            self_kind: f.self_kind,
            params,
            ret,
            dims,
            via: None,
            source: if f.generated {
                Source::NamedEnum
            } else {
                Source::Written
            },
            constant: f.constant,
            cross: Cross::Private,
        };
        if !private {
            entry.cross = self.classify(&entry, f);
        }
        entry
    }

    fn classify(&self, entry: &Function, f: &parse::Fn) -> Cross {
        let skip = |reason: String| Cross::Skip { reason };
        if let Some((name, bounds)) = f.type_params.first() {
            if bounds.contains("Fn") || f.where_text.contains("Fn") {
                return skip(format!("closure parameter {name}"));
            }
            if entry.module == "core::error" {
                return skip("error constructor".into());
            }
            return skip(format!("generic over {name}"));
        }
        for p in &entry.params {
            if let Some(reason) = uncrossable(&p.ty) {
                return skip(format!("{}: {reason}", p.name));
            }
            if let Ty::Ref {
                mutable: true,
                item,
                ..
            } = &p.ty
            {
                if !matches!(**item, Ty::Hand { .. } | Ty::Class { .. }) {
                    return skip(format!("{}: mutates plain data in place", p.name));
                }
            }
            if let Ty::Slice { mutable: true, .. } = &p.ty {
                return skip(format!("{}: mutable slice argument", p.name));
            }
        }
        if let Some(reason) = uncrossable(&entry.ret) {
            return skip(format!("returns {reason}"));
        }
        if let Some(borrow) = borrow(&entry.ret) {
            return skip(format!("{borrow} return"));
        }
        let unit = entry
            .module
            .split("::")
            .next()
            .unwrap_or_default()
            .to_string();
        let mut foreign = None;
        let mut see = |t: &Ty| {
            if let Ty::Class { path, .. } = t {
                let home = path.split("::").next().unwrap_or_default();
                if home != unit && foreign.is_none() {
                    foreign = Some(format!("class {path} lives in unit {home}"));
                }
            }
        };
        entry.params.iter().for_each(|p| p.ty.walk(&mut see));
        entry.ret.walk(&mut see);
        if let Some(reason) = foreign {
            return skip(reason);
        }
        Cross::Ok
    }
}

fn borrow(ty: &Ty) -> Option<&'static str> {
    if ty.any(&|t| matches!(t, Ty::Ref { lifetime: Some(l), .. } if l == "'static")) {
        Some("&'static")
    } else if ty.any(&|t| {
        matches!(
            t,
            Ty::Ref {
                lifetime: Some(_),
                ..
            }
        )
    }) {
        Some("lifetime-bearing")
    } else if ty.any(&|t| {
        matches!(
            t,
            Ty::Ref { mutable: true, .. } | Ty::Slice { mutable: true, .. }
        )
    }) {
        Some("mutable borrow")
    } else {
        None
    }
}

fn dim_of(args: &syn::PathArguments) -> Option<u8> {
    let syn::PathArguments::AngleBracketed(a) = args else {
        return None;
    };
    match a.args.first()? {
        syn::GenericArgument::Const(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(n),
            ..
        })) => n.base10_parse().ok(),
        _ => None,
    }
}

fn serde_both(derives: &[String]) -> bool {
    derives.iter().any(|d| d == "Serialize") && derives.iter().any(|d| d == "Deserialize")
}

pub fn uncrossable(ty: &Ty) -> Option<String> {
    let mut reason = None;
    ty.walk(&mut |t| {
        if reason.is_some() {
            return;
        }
        reason = match t {
            Ty::Unknown { text } => Some(text.clone()),
            Ty::Opaque { path } => Some(format!("uncrossable type {path}")),
            Ty::Unit => None,
            Ty::Map { key, .. } if !matches!(**key, Ty::Scalar { .. } | Ty::String | Ty::Str) => {
                Some("map keyed by a non-scalar".into())
            }
            _ => None,
        };
        if let Ty::Unknown { text } = t {
            if text.contains("Iterator") {
                reason = Some("iterator return".into());
            }
        }
    });
    reason
}

fn collisions(m: &Manifest) -> Vec<String> {
    let mut out = vec![];
    let mut free: BTreeMap<(String, String), Vec<&Function>> = BTreeMap::new();
    for f in m.functions.iter().filter(|f| f.cross != Cross::Private) {
        free.entry((f.module.clone(), f.path.clone()))
            .or_default()
            .push(f);
    }
    for ((_, path), entries) in free {
        if entries.len() < 2 {
            continue;
        }
        let dims: Vec<&u8> = entries.iter().flat_map(|f| f.dims.iter()).collect();
        let distinct: HashSet<&u8> = dims.iter().copied().collect();
        if !dims.is_empty()
            && dims.len() == distinct.len()
            && entries.iter().all(|f| !f.dims.is_empty())
        {
            continue;
        }
        let at: Vec<&str> = entries.iter().map(|f| f.defined_at.as_str()).collect();
        out.push(format!("collision: {path} at {}", at.join(" and ")));
    }
    let homes: HashSet<&str> = m
        .modules
        .iter()
        .filter(|x| x.items > 0)
        .map(|x| x.path.as_str())
        .collect();
    for f in m.functions.iter().filter(|f| f.cross != Cross::Private) {
        if homes.contains(f.path.as_str()) {
            out.push(format!(
                "collision: module {} and fn {} at {}",
                f.path, f.path, f.defined_at
            ));
        }
    }
    for t in &m.types {
        if homes.contains(t.path.as_str()) {
            out.push(format!(
                "collision: module {} and type {} at {}",
                t.path, t.path, t.defined_at
            ));
        }
    }
    out.sort();
    out.dedup();
    out
}
