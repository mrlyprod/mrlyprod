use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hand {
    pub name: String,
    pub rust: Vec<String>,
    pub js: Vec<String>,
    pub word: String,
    pub ts: String,
    pub ts_data: String,
    pub py: Vec<String>,
    pub py_into: String,
    pub pyi: String,
    pub stream: bool,
    pub big: bool,
    pub rank: bool,
    pub write_back: Option<Back>,
    pub cli: Option<(String, String)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Back {
    Ref,
    Value,
}

impl Hand {
    pub fn generic(&self) -> bool {
        self.rust.len() > 1
    }
}

static HANDS: OnceLock<Vec<Hand>> = OnceLock::new();

pub fn install(hands: Vec<Hand>) {
    HANDS.get_or_init(|| hands);
}

pub fn hands() -> &'static [Hand] {
    HANDS.get().map_or(&[], Vec::as_slice)
}

pub fn hand(name: &str) -> Option<&'static Hand> {
    hands().iter().find(|hand| hand.name == name)
}

pub fn row(name: &str) -> Result<&'static Hand> {
    hand(name).ok_or_else(|| format!("no [[hand]] row names {name}"))
}

pub fn big() -> Result<&'static Hand> {
    hands()
        .iter()
        .find(|hand| hand.big)
        .ok_or_else(|| "no [[hand]] row is big".to_string())
}

pub fn row_of(ty: &Ty) -> Option<&'static Hand> {
    match ty {
        Ty::Hand { name, .. } => hand(name),
        _ => None,
    }
}

pub fn stream(ty: &Ty) -> bool {
    row_of(ty).is_some_and(|row| row.stream)
}

pub fn ranked(ty: &Ty) -> bool {
    row_of(ty).is_some_and(|row| row.rank)
}

pub fn undecided(ty: &Ty) -> bool {
    matches!(ty, Ty::Hand { dim: None, .. }) && row_of(ty).is_some_and(Hand::generic)
}

pub fn rust(path: &str) -> String {
    format!("mrly_{path}")
}

pub fn by_dim(list: &'static [String], dim: Option<u8>) -> Option<&'static str> {
    match list {
        [one] => Some(one),
        many => many
            .get(usize::from(dim?.checked_sub(2)?))
            .map(String::as_str),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub krate: String,
    pub version: String,
    pub modules: Vec<Module>,
    pub types: Vec<Type>,
    pub consts: Vec<Const>,
    pub functions: Vec<Function>,
}

impl Manifest {
    pub fn hand_type(&self, name: &str) -> Option<&Type> {
        self.types.iter().find(|t| {
            t.kind != TypeKind::Alias
                && matches!(&t.cross, TypeCross::Hand { name: n } if n == name)
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Module {
    pub path: String,
    pub file: String,
    pub docs: Vec<String>,
    pub items: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Type {
    pub path: String,
    pub name: String,
    pub defined_at: String,
    pub docs: Vec<String>,
    pub kind: TypeKind,
    pub cross: TypeCross,
    pub derives: Vec<String>,
    pub serde: Vec<String>,
    pub const_generic: bool,
    pub fields: Vec<Field>,
    pub variants: Vec<Variant>,
    pub alias: Option<Ty>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeKind {
    Struct,
    Enum,
    Alias,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TypeCross {
    Hand { name: String },
    Class,
    Plain,
    Enum { named: bool, words: Vec<String> },
    Uncrossable { reason: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub public: bool,
    pub docs: Vec<String>,
    pub serde: Vec<String>,
    pub serde_skip: bool,
    pub ty: Ty,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variant {
    pub name: String,
    pub word: Option<String>,
    pub docs: Vec<String>,
    pub serde: Vec<String>,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Const {
    pub path: String,
    pub name: String,
    pub defined_at: String,
    pub docs: Vec<String>,
    pub ty: Ty,
    pub cross: Cross,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    pub path: String,
    pub name: String,
    pub module: String,
    pub defined_at: String,
    pub docs: Vec<String>,
    pub owner: Option<String>,
    pub self_kind: Option<SelfKind>,
    pub params: Vec<Param>,
    pub ret: Ty,
    pub dims: Vec<u8>,
    #[serde(rename = "trait")]
    pub via: Option<String>,
    pub source: Source,
    pub constant: bool,
    pub cross: Cross,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelfKind {
    Value,
    Ref,
    Mut,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub ty: Ty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    Written,
    NamedEnum,
    Trait,
    Default,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Cross {
    Ok,
    Skip { reason: String },
    Private,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum Ty {
    Unit,
    Scalar {
        name: String,
    },
    Str,
    String,
    U128,
    I128,
    Code,
    Json,
    Vec {
        item: Box<Ty>,
    },
    Slice {
        mutable: bool,
        item: Box<Ty>,
    },
    Option {
        item: Box<Ty>,
    },
    Tuple {
        items: Vec<Ty>,
    },
    Array {
        item: Box<Ty>,
        len: usize,
    },
    Ref {
        mutable: bool,
        lifetime: Option<String>,
        item: Box<Ty>,
    },
    Result {
        item: Box<Ty>,
    },
    Map {
        key: Box<Ty>,
        value: Box<Ty>,
    },
    Set {
        item: Box<Ty>,
    },
    Hand {
        name: String,
        dim: Option<u8>,
    },
    Plain {
        path: String,
    },
    Enum {
        path: String,
    },
    Class {
        path: String,
        dim: Option<u8>,
    },
    Opaque {
        path: String,
    },
    Unknown {
        text: String,
    },
}

impl Ty {
    pub fn walk(&self, seen: &mut dyn FnMut(&Ty)) {
        seen(self);
        match self {
            Ty::Vec { item }
            | Ty::Slice { item, .. }
            | Ty::Option { item }
            | Ty::Array { item, .. }
            | Ty::Ref { item, .. }
            | Ty::Set { item }
            | Ty::Result { item } => item.walk(seen),
            Ty::Tuple { items } => items.iter().for_each(|t| t.walk(seen)),
            Ty::Map { key, value } => {
                key.walk(seen);
                value.walk(seen);
            }
            _ => {}
        }
    }
    pub fn settable(&self) -> bool {
        !matches!(self, Ty::Ref { .. })
    }
    pub fn any(&self, test: &dyn Fn(&Ty) -> bool) -> bool {
        let mut hit = false;
        self.walk(&mut |t| hit |= test(t));
        hit
    }
}
