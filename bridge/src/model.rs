use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, String>;

pub const HAND: [&str; 7] = ["Tensor", "Cell", "CellNd", "Cell6d", "Color", "Code", "Rng"];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub krate: String,
    pub version: String,
    pub modules: Vec<Module>,
    pub types: Vec<Type>,
    pub consts: Vec<Const>,
    pub functions: Vec<Function>,
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
