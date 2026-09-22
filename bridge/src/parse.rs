use crate::model::{Result, SelfKind};
use quote::ToTokens;
use std::collections::BTreeMap;
use std::path::Path;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{braced, Attribute, Ident, LitStr, Token};

pub type Files = BTreeMap<String, String>;

pub struct Tree {
    pub modules: BTreeMap<String, Module>,
    pub macro_body_fns: usize,
}

pub struct Module {
    pub file: String,
    pub public: bool,
    pub docs: Vec<String>,
    pub children: Vec<String>,
    pub items: Vec<Item>,
    pub uses: Vec<Use>,
}

pub struct Use {
    pub public: bool,
    pub path: Vec<String>,
    pub name: String,
    pub glob: bool,
}

pub enum Item {
    Fn(Fn),
    Struct(Struct),
    Enum(Enum),
    Alias(Alias),
    Const(Const),
    Impl(Impl),
    Trait(Trait),
}

pub struct Trait {
    pub name: String,
    pub fns: Vec<Fn>,
}

pub struct Fn {
    pub name: String,
    pub line: usize,
    pub docs: Vec<String>,
    pub constant: bool,
    pub generated: bool,
    pub type_params: Vec<(String, String)>,
    pub const_params: Vec<String>,
    pub where_text: String,
    pub self_kind: Option<SelfKind>,
    pub params: Vec<(String, syn::Type)>,
    pub ret: Option<syn::Type>,
}

pub struct Struct {
    pub name: String,
    pub line: usize,
    pub docs: Vec<String>,
    pub derives: Vec<String>,
    pub serde: Vec<String>,
    pub const_generic: bool,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub public: bool,
    pub docs: Vec<String>,
    pub serde: Vec<String>,
    pub ty: syn::Type,
}

pub struct Enum {
    pub name: String,
    pub line: usize,
    pub docs: Vec<String>,
    pub derives: Vec<String>,
    pub serde: Vec<String>,
    pub named: bool,
    pub variants: Vec<Variant>,
}

pub struct Variant {
    pub name: String,
    pub word: Option<String>,
    pub docs: Vec<String>,
    pub serde: Vec<String>,
    pub fields: Vec<Field>,
}

pub struct Alias {
    pub name: String,
    pub line: usize,
    pub docs: Vec<String>,
    pub generic: bool,
    pub ty: syn::Type,
}

pub struct Const {
    pub name: String,
    pub line: usize,
    pub docs: Vec<String>,
    pub ty: syn::Type,
}

pub struct Impl {
    pub trait_path: Option<Vec<String>>,
    pub self_ty: syn::Type,
    pub const_params: Vec<String>,
    pub type_params: Vec<String>,
    pub fns: Vec<Fn>,
}

pub fn is_pub_fn_line(line: &str) -> bool {
    line.trim_start().starts_with("pub fn ")
}

pub fn load(src: &Path) -> Result<Files> {
    let mut files = Files::new();
    read_dir(src, src, &mut files)?;
    Ok(files)
}

fn read_dir(src: &Path, dir: &Path, files: &mut Files) -> Result<()> {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    entries.sort();
    for path in entries {
        let rel = path.strip_prefix(src).map_err(|e| e.to_string())?;
        if path.is_dir() {
            if rel == Path::new("bin") {
                continue;
            }
            read_dir(src, &path, files)?;
        } else if path.extension().is_some_and(|x| x == "rs") {
            let text =
                std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            files.insert(rel.to_string_lossy().replace('\\', "/"), text);
        }
    }
    Ok(())
}

pub fn parse(files: &Files) -> Result<Tree> {
    let mut walker = Walker {
        files,
        tree: Tree {
            modules: BTreeMap::new(),
            macro_body_fns: 0,
        },
    };
    walker.file(At {
        path: vec![],
        file: "lib.rs".into(),
        dir: String::new(),
        public: true,
        docs: vec![],
    })?;
    Ok(walker.tree)
}

struct Walker<'a> {
    files: &'a Files,
    tree: Tree,
}

struct At {
    path: Vec<String>,
    file: String,
    dir: String,
    public: bool,
    docs: Vec<String>,
}

impl Walker<'_> {
    fn file(&mut self, mut at: At) -> Result<()> {
        let file = at.file.clone();
        let text = self
            .files
            .get(&file)
            .ok_or_else(|| format!("missing file {file}"))?;
        let parsed =
            syn::parse_file(text).map_err(|e| format!("{file}:{}: {e}", e.span().start().line))?;
        at.docs.extend(docs_of(&parsed.attrs));
        self.module(at, text, &parsed.items)
    }

    fn module(&mut self, at: At, text: &str, items: &[syn::Item]) -> Result<()> {
        let At {
            path,
            file,
            dir,
            public,
            docs,
        } = at;
        let key = path.join("::");
        let mut module = Module {
            file: file.clone(),
            public,
            docs,
            children: vec![],
            items: vec![],
            uses: vec![],
        };
        let mut pending = vec![];
        for item in items {
            match item {
                syn::Item::Mod(m) if !is_test(&m.attrs) => {
                    let name = m.ident.to_string();
                    module.children.push(name.clone());
                    pending.push(m);
                }
                syn::Item::Use(u) if !is_test(&u.attrs) => {
                    let public = is_public(&u.vis);
                    flatten_use(&u.tree, vec![], public, &mut module.uses);
                }
                syn::Item::Fn(f) if !is_test(&f.attrs) && is_public(&f.vis) => {
                    module
                        .items
                        .push(Item::Fn(function(&f.sig, docs_of(&f.attrs), false)));
                }
                syn::Item::Struct(s) if !is_test(&s.attrs) && is_public(&s.vis) => {
                    module.items.push(Item::Struct(structure(s)));
                }
                syn::Item::Enum(e) if !is_test(&e.attrs) && is_public(&e.vis) => {
                    module.items.push(Item::Enum(enumeration(e)));
                }
                syn::Item::Type(t) if !is_test(&t.attrs) && is_public(&t.vis) => {
                    module.items.push(Item::Alias(Alias {
                        name: t.ident.to_string(),
                        line: t.ident.span().start().line,
                        docs: docs_of(&t.attrs),
                        generic: !t.generics.params.is_empty(),
                        ty: (*t.ty).clone(),
                    }));
                }
                syn::Item::Const(c) if !is_test(&c.attrs) && is_public(&c.vis) => {
                    module.items.push(Item::Const(Const {
                        name: c.ident.to_string(),
                        line: c.ident.span().start().line,
                        docs: docs_of(&c.attrs),
                        ty: (*c.ty).clone(),
                    }));
                }
                syn::Item::Impl(i) if !is_test(&i.attrs) => {
                    module.items.push(Item::Impl(implementation(i)));
                }
                syn::Item::Trait(t) if !is_test(&t.attrs) && is_public(&t.vis) => {
                    let fns = t
                        .items
                        .iter()
                        .filter_map(|item| match item {
                            syn::TraitItem::Fn(f) => {
                                Some(function(&f.sig, docs_of(&f.attrs), false))
                            }
                            _ => None,
                        })
                        .collect();
                    module.items.push(Item::Trait(Trait {
                        name: t.ident.to_string(),
                        fns,
                    }));
                }
                syn::Item::Macro(m) if m.mac.path.is_ident("named_enum") => {
                    let line = m.mac.path.span().start().line;
                    let named: NamedEnum = syn::parse2(m.mac.tokens.clone())
                        .map_err(|e| format!("{file}:{line}: named_enum! {e}"))?;
                    let (e, i) = named.items(line);
                    module.items.push(Item::Enum(e));
                    module.items.push(Item::Impl(i));
                }
                syn::Item::Macro(m) if m.mac.path.is_ident("macro_rules") => {
                    let start = m.span().start().line;
                    let end = m.span().end().line;
                    self.tree.macro_body_fns += text
                        .lines()
                        .skip(start - 1)
                        .take(end + 1 - start)
                        .filter(|l| is_pub_fn_line(l))
                        .count();
                }
                _ => {}
            }
        }
        self.tree.modules.insert(key, module);
        for m in pending {
            let name = m.ident.to_string();
            let mut child = path.clone();
            child.push(name.clone());
            let dir = if dir.is_empty() {
                name.clone()
            } else {
                format!("{dir}/{name}")
            };
            let public = is_public(&m.vis);
            let docs = docs_of(&m.attrs);
            match &m.content {
                Some((_, items)) => {
                    let at = At {
                        path: child,
                        file: file.clone(),
                        dir,
                        public,
                        docs,
                    };
                    self.module(at, text, items)?;
                }
                None => {
                    let flat = format!("{dir}.rs");
                    let nested = format!("{dir}/mod.rs");
                    let file = if self.files.contains_key(&flat) {
                        flat
                    } else {
                        nested
                    };
                    self.file(At {
                        path: child,
                        file,
                        dir,
                        public,
                        docs,
                    })?;
                }
            }
        }
        Ok(())
    }
}

fn flatten_use(tree: &syn::UseTree, prefix: Vec<String>, public: bool, out: &mut Vec<Use>) {
    match tree {
        syn::UseTree::Path(p) => {
            let mut next = prefix;
            next.push(p.ident.to_string());
            flatten_use(&p.tree, next, public, out);
        }
        syn::UseTree::Name(n) => {
            let mut path = prefix;
            let name = n.ident.to_string();
            if name == "self" {
                let name = path.last().cloned().unwrap_or_default();
                out.push(Use {
                    public,
                    path,
                    name,
                    glob: false,
                });
            } else {
                path.push(name.clone());
                out.push(Use {
                    public,
                    path,
                    name,
                    glob: false,
                });
            }
        }
        syn::UseTree::Rename(r) => {
            let mut path = prefix;
            path.push(r.ident.to_string());
            out.push(Use {
                public,
                path,
                name: r.rename.to_string(),
                glob: false,
            });
        }
        syn::UseTree::Glob(_) => out.push(Use {
            public,
            path: prefix,
            name: "*".into(),
            glob: true,
        }),
        syn::UseTree::Group(g) => {
            for item in &g.items {
                flatten_use(item, prefix.clone(), public, out);
            }
        }
    }
}

fn function(sig: &syn::Signature, docs: Vec<String>, generated: bool) -> Fn {
    let mut type_params = vec![];
    let mut const_params = vec![];
    for param in &sig.generics.params {
        match param {
            syn::GenericParam::Type(t) => {
                type_params.push((t.ident.to_string(), t.bounds.to_token_stream().to_string()))
            }
            syn::GenericParam::Const(c) => const_params.push(c.ident.to_string()),
            syn::GenericParam::Lifetime(_) => {}
        }
    }
    let mut self_kind = None;
    let mut params = vec![];
    for (i, arg) in sig.inputs.iter().enumerate() {
        match arg {
            syn::FnArg::Receiver(r) => {
                self_kind = Some(match (&r.reference, &r.mutability) {
                    (None, _) => SelfKind::Value,
                    (Some(_), None) => SelfKind::Ref,
                    (Some(_), Some(_)) => SelfKind::Mut,
                });
            }
            syn::FnArg::Typed(p) => {
                let name = match &*p.pat {
                    syn::Pat::Ident(id) => id.ident.to_string(),
                    _ => format!("arg{i}"),
                };
                params.push((name, (*p.ty).clone()));
            }
        }
    }
    Fn {
        name: sig.ident.to_string(),
        line: sig.ident.span().start().line,
        docs,
        constant: sig.constness.is_some(),
        generated,
        type_params,
        const_params,
        where_text: sig
            .generics
            .where_clause
            .as_ref()
            .map(|w| w.to_token_stream().to_string())
            .unwrap_or_default(),
        self_kind,
        params,
        ret: match &sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, t) => Some((**t).clone()),
        },
    }
}

fn fields_of(fields: &syn::Fields) -> Vec<Field> {
    fields
        .iter()
        .enumerate()
        .map(|(i, f)| Field {
            name: f
                .ident
                .as_ref()
                .map_or_else(|| i.to_string(), |id| id.to_string()),
            public: is_public(&f.vis),
            docs: docs_of(&f.attrs),
            serde: serde_of(&f.attrs),
            ty: f.ty.clone(),
        })
        .collect()
}

fn structure(s: &syn::ItemStruct) -> Struct {
    Struct {
        name: s.ident.to_string(),
        line: s.ident.span().start().line,
        docs: docs_of(&s.attrs),
        derives: derives_of(&s.attrs),
        serde: serde_of(&s.attrs),
        const_generic: s
            .generics
            .params
            .iter()
            .any(|p| matches!(p, syn::GenericParam::Const(_))),
        fields: fields_of(&s.fields),
    }
}

fn enumeration(e: &syn::ItemEnum) -> Enum {
    Enum {
        name: e.ident.to_string(),
        line: e.ident.span().start().line,
        docs: docs_of(&e.attrs),
        derives: derives_of(&e.attrs),
        serde: serde_of(&e.attrs),
        named: false,
        variants: e
            .variants
            .iter()
            .map(|v| Variant {
                name: v.ident.to_string(),
                word: None,
                docs: docs_of(&v.attrs),
                serde: serde_of(&v.attrs),
                fields: fields_of(&v.fields),
            })
            .collect(),
    }
}

fn implementation(i: &syn::ItemImpl) -> Impl {
    let mut const_params = vec![];
    let mut type_params = vec![];
    for param in &i.generics.params {
        match param {
            syn::GenericParam::Const(c) => const_params.push(c.ident.to_string()),
            syn::GenericParam::Type(t) => type_params.push(t.ident.to_string()),
            syn::GenericParam::Lifetime(_) => {}
        }
    }
    let fns = i
        .items
        .iter()
        .filter_map(|item| match item {
            syn::ImplItem::Fn(f) if is_public(&f.vis) && !is_test(&f.attrs) => {
                Some(function(&f.sig, docs_of(&f.attrs), false))
            }
            _ => None,
        })
        .collect();
    let trait_path = i
        .trait_
        .as_ref()
        .map(|(_, path, _)| path.segments.iter().map(|s| s.ident.to_string()).collect());
    Impl {
        trait_path,
        self_ty: (*i.self_ty).clone(),
        const_params,
        type_params,
        fns,
    }
}

struct NamedEnum {
    attrs: Vec<Attribute>,
    name: Ident,
    variants: Vec<(Vec<Attribute>, Ident, String)>,
}

impl Parse for NamedEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        input.parse::<Token![pub]>()?;
        input.parse::<Token![enum]>()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let mut variants = vec![];
        while !content.is_empty() {
            let vattrs = content.call(Attribute::parse_outer)?;
            let variant: Ident = content.parse()?;
            content.parse::<Token![=>]>()?;
            let word: LitStr = content.parse()?;
            variants.push((vattrs, variant, word.value()));
            if content.is_empty() {
                break;
            }
            content.parse::<Token![,]>()?;
        }
        Ok(NamedEnum {
            attrs,
            name,
            variants,
        })
    }
}

impl NamedEnum {
    fn items(&self, line: usize) -> (Enum, Impl) {
        let name = self.name.to_string();
        let count = self.variants.len();
        let enumeration = Enum {
            name: name.clone(),
            line,
            docs: docs_of(&self.attrs),
            derives: derives_of(&self.attrs),
            serde: serde_of(&self.attrs),
            named: true,
            variants: self
                .variants
                .iter()
                .map(|(attrs, ident, word)| Variant {
                    name: ident.to_string(),
                    word: Some(word.clone()),
                    docs: docs_of(attrs),
                    serde: serde_of(attrs),
                    fields: vec![],
                })
                .collect(),
        };
        let all: syn::Signature = syn::parse_str(&format!("const fn all() -> [{name}; {count}]"))
            .expect("the all signature parses");
        let word: syn::Signature =
            syn::parse_str("fn name(&self) -> &'static str").expect("the name signature parses");
        let mut all = function(
            &all,
            vec![format!("Returns every {name} in canonical order.")],
            true,
        );
        let mut word = function(
            &word,
            vec![format!("Returns the {name}'s display name.")],
            true,
        );
        all.line = line;
        word.line = line;
        let self_ty: syn::Type = syn::parse_str(&name).expect("the enum name parses");
        (
            enumeration,
            Impl {
                trait_path: None,
                self_ty,
                const_params: vec![],
                type_params: vec![],
                fns: vec![all, word],
            },
        )
    }
}

fn is_public(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

fn is_test(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|a| a.path().is_ident("cfg") && a.meta.to_token_stream().to_string().contains("test"))
}

pub fn docs_of(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .filter_map(|a| match &a.meta {
            syn::Meta::NameValue(nv) => match &nv.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) => Some(s.value()),
                _ => None,
            },
            _ => None,
        })
        .map(|s| s.strip_prefix(' ').unwrap_or(&s).to_string())
        .collect()
}

fn derives_of(attrs: &[Attribute]) -> Vec<String> {
    let mut out = vec![];
    for attr in attrs.iter().filter(|a| a.path().is_ident("derive")) {
        if let Ok(paths) =
            attr.parse_args_with(Punctuated::<syn::Path, Token![,]>::parse_terminated)
        {
            out.extend(
                paths
                    .iter()
                    .filter_map(|p| p.segments.last().map(|s| s.ident.to_string())),
            );
        }
    }
    out
}

fn serde_of(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("serde"))
        .filter_map(|a| a.meta.require_list().ok().map(|l| l.tokens.to_string()))
        .collect()
}
