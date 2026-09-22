use crate::model::*;
use crate::parse::{self, Files};
use crate::report;
use crate::resolve::{self, Built};
use crate::{cli, js, py, version};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

fn build(lib: &str) -> Built {
    let files: Files = [("lib.rs".to_string(), lib.to_string())]
        .into_iter()
        .collect();
    resolve::build(&files, "0.0.0").expect("the source builds")
}

fn manifest(lib: &str) -> Manifest {
    build(lib).manifest
}

fn function<'a>(m: &'a Manifest, path: &str) -> &'a Function {
    let paths: Vec<&str> = m.functions.iter().map(|f| f.path.as_str()).collect();
    m.functions
        .iter()
        .find(|f| f.path == path)
        .unwrap_or_else(|| panic!("no {path} in {paths:?}"))
}

fn reason(m: &Manifest, path: &str) -> String {
    match &function(m, path).cross {
        Cross::Skip { reason } => reason.clone(),
        other => panic!("{path} is {other:?}"),
    }
}

fn kind<'a>(m: &'a Manifest, path: &str) -> &'a TypeCross {
    let paths: Vec<&str> = m.types.iter().map(|t| t.path.as_str()).collect();
    &m.types
        .iter()
        .find(|t| t.path == path)
        .unwrap_or_else(|| panic!("no {path} in {paths:?}"))
        .cross
}

fn scratch(name: &str, lib: &str, units: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("bridge-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("bridge")).expect("a scratch root");
    std::fs::write(root.join("bridge/units.txt"), units).expect("units.txt");
    let m = manifest(lib);
    cli::write(&m, &root).expect("the cli writes");
    py::write(&m, &root).expect("the python bridge writes");
    js::write(&m, &root).expect("the js bridge writes");
    root
}

fn read(root: &Path, file: &str) -> String {
    std::fs::read_to_string(root.join(file)).unwrap_or_else(|_| panic!("no {file}"))
}

const ERROR: &str = "pub mod core { pub mod error { pub enum Error { Value(String) } pub type Result<T> = std::result::Result<T, Error>; } pub use error::{Error, Result}; }";

#[test]
fn scalars_and_strings_cross() {
    let m = manifest(
        "pub fn f(a: bool, b: u8, c: f64, d: String, e: &str, g: char, h: f32, i: i8) -> u64 { 0 }",
    );
    let f = function(&m, "f");
    assert_eq!(f.cross, Cross::Ok);
    assert_eq!(f.params[3].ty, Ty::String);
    assert_eq!(f.params[4].ty, Ty::Str);
    assert_eq!(
        f.params[5].ty,
        Ty::Scalar {
            name: "char".into()
        }
    );
    assert_eq!(f.ret, Ty::Scalar { name: "u64".into() });
}

#[test]
fn wide_integers_and_code_cross() {
    let m = manifest(
        "pub struct Code(pub(crate) u128); pub fn f(a: u128, b: i128, c: Code) -> Code { c }",
    );
    let f = function(&m, "f");
    assert_eq!(f.cross, Cross::Ok);
    assert_eq!(f.params[0].ty, Ty::U128);
    assert_eq!(f.params[1].ty, Ty::I128);
    assert_eq!(f.ret, Ty::Code);
}

#[test]
fn containers_and_results_cross() {
    let src = format!("{ERROR} use core::Result; pub fn f(a: Vec<u8>, b: &[u64], c: Option<usize>, d: (u8, u8), e: [u8; 4]) -> Result<Vec<u8>> {{ Ok(a) }}");
    let m = manifest(&src);
    let f = function(&m, "f");
    assert_eq!(f.cross, Cross::Ok);
    let u8 = || Box::new(Ty::Scalar { name: "u8".into() });
    assert_eq!(f.params[0].ty, Ty::Vec { item: u8() });
    assert_eq!(
        f.params[1].ty,
        Ty::Slice {
            mutable: false,
            item: Box::new(Ty::Scalar { name: "u64".into() })
        }
    );
    assert_eq!(
        f.params[2].ty,
        Ty::Option {
            item: Box::new(Ty::Scalar {
                name: "usize".into()
            })
        }
    );
    assert_eq!(
        f.params[3].ty,
        Ty::Tuple {
            items: vec![*u8(), *u8()]
        }
    );
    assert_eq!(f.params[4].ty, Ty::Array { item: u8(), len: 4 });
    assert_eq!(
        f.ret,
        Ty::Result {
            item: Box::new(Ty::Vec { item: u8() })
        }
    );
}

#[test]
fn json_maps_and_sets_cross() {
    let m = manifest("pub use serde_json::Value as Json; pub fn f(v: &Json, m: std::collections::HashMap<u8, Vec<u8>>) -> std::collections::BTreeSet<u8> { todo!() }");
    let f = function(&m, "f");
    assert_eq!(f.cross, Cross::Ok);
    assert_eq!(
        f.params[0].ty,
        Ty::Ref {
            mutable: false,
            lifetime: None,
            item: Box::new(Ty::Json)
        }
    );
    assert!(matches!(f.params[1].ty, Ty::Map { .. }));
    assert!(matches!(f.ret, Ty::Set { .. }));
}

#[test]
fn hand_methods_export_as_free_functions_self_first() {
    let m = manifest("pub mod core { pub mod tensor { pub struct Tensor { pub shape: Vec<usize> } impl Tensor { pub fn new(shape: Vec<usize>) -> Tensor { Tensor { shape } } pub fn size(&self) -> usize { 0 } } } pub use tensor::Tensor; }");
    assert_eq!(
        kind(&m, "core::Tensor"),
        &TypeCross::Hand {
            name: "Tensor".into()
        }
    );
    let new = function(&m, "core::tensor::new");
    assert_eq!(
        (new.owner.as_deref(), new.self_kind, new.module.as_str()),
        (Some("core::Tensor"), None, "core::tensor")
    );
    let size = function(&m, "core::tensor::size");
    assert_eq!(
        (size.self_kind, &size.cross),
        (Some(SelfKind::Ref), &Cross::Ok)
    );
}

#[test]
fn a_hand_type_in_a_private_module_exports_at_the_nearest_public_module() {
    let m = manifest("pub mod six { mod models { pub struct Cell6d { pub start: u8 } impl Cell6d { pub fn width(&self) -> usize { 0 } } } pub use models::Cell6d; }");
    let f = function(&m, "six::width");
    assert_eq!((f.module.as_str(), &f.cross), ("six", &Cross::Ok));
}

#[test]
fn rng_crosses_by_hand_and_is_passed_mutably() {
    let m = manifest("pub struct Rng { s: u64 } pub fn draw(rng: &mut Rng) -> u64 { 0 }");
    assert_eq!(kind(&m, "Rng"), &TypeCross::Hand { name: "Rng".into() });
    let f = function(&m, "draw");
    assert_eq!(f.cross, Cross::Ok);
    assert_eq!(
        f.params[0].ty,
        Ty::Ref {
            mutable: true,
            lifetime: None,
            item: Box::new(Ty::Hand {
                name: "Rng".into(),
                dim: None
            })
        }
    );
}

#[test]
fn a_serde_struct_without_self_methods_is_plain_and_keeps_its_constructors() {
    let m = manifest("pub mod gen { #[derive(Serialize, Deserialize)] pub struct Tile { pub side: usize } impl Tile { pub fn new(side: usize) -> Tile { Tile { side } } } }");
    assert_eq!(kind(&m, "gen::Tile"), &TypeCross::Plain);
    let f = function(&m, "gen::Tile::new");
    assert_eq!(
        (f.owner.as_deref(), f.module.as_str(), &f.cross),
        (Some("gen::Tile"), "gen", &Cross::Ok)
    );
}

#[test]
fn a_serde_skipped_field_makes_a_class() {
    let m = manifest("pub mod gen { #[derive(Serialize, Deserialize)] pub struct File { pub width: usize, #[serde(default, skip_serializing_if = \"Vec::is_empty\")] pub tags: Vec<u8>, #[serde(skip)] pub png: Vec<u8> } #[derive(Serialize, Deserialize)] pub struct Tile { #[serde(default, skip_serializing_if = \"Option::is_none\")] pub side: Option<usize> } }");
    assert_eq!(kind(&m, "gen::File"), &TypeCross::Class);
    assert_eq!(kind(&m, "gen::Tile"), &TypeCross::Plain);
    let file = m.types.iter().find(|t| t.path == "gen::File").expect("File is listed");
    let skips: Vec<bool> = file.fields.iter().map(|f| f.serde_skip).collect();
    assert_eq!(skips, [false, false, true]);
}

#[test]
fn a_struct_with_a_self_method_is_a_class() {
    let m = manifest("pub mod life { #[derive(Serialize, Deserialize)] pub struct Life { pub time: usize } impl Life { pub fn step(&mut self) {} } }");
    assert_eq!(kind(&m, "life::Life"), &TypeCross::Class);
    let f = function(&m, "life::Life::step");
    assert_eq!((f.self_kind, &f.cross), (Some(SelfKind::Mut), &Cross::Ok));
}

#[test]
fn a_fieldless_serde_enum_is_a_string_even_with_methods() {
    let m = manifest("#[derive(Serialize, Deserialize)] pub enum Dtype { U8, U16 } impl Dtype { pub fn max(self) -> i64 { 0 } }");
    assert_eq!(
        kind(&m, "Dtype"),
        &TypeCross::Enum {
            named: false,
            words: vec![]
        }
    );
    let f = function(&m, "Dtype::max");
    assert_eq!((f.self_kind, &f.cross), (Some(SelfKind::Value), &Cross::Ok));
}

#[test]
fn a_named_enum_records_its_words_all_and_name() {
    let m = manifest("pub mod life { named_enum! { #[derive(Serialize, Deserialize)] pub enum Fate { Dead => \"dead\", Alive => \"alive\", } } }");
    assert_eq!(
        kind(&m, "life::Fate"),
        &TypeCross::Enum {
            named: true,
            words: vec!["dead".into(), "alive".into()]
        }
    );
    let all = function(&m, "life::Fate::all");
    assert_eq!(
        (all.source, all.constant, &all.cross),
        (Source::NamedEnum, true, &Cross::Ok)
    );
    assert_eq!(
        all.ret,
        Ty::Array {
            item: Box::new(Ty::Enum {
                path: "life::Fate".into()
            }),
            len: 2
        }
    );
    let name = function(&m, "life::Fate::name");
    assert_eq!(
        (name.source, &name.cross),
        (
            Source::NamedEnum,
            &Cross::Skip {
                reason: "&'static return".into()
            }
        )
    );
}

#[test]
fn const_generic_functions_dispatch_on_two_and_three() {
    let m = manifest("pub mod cell { pub struct CellNd<const N: usize> { pub n: u8 } pub type Cell2d = CellNd<2>; pub fn fills<const N: usize>(cell: &CellNd<N>) -> usize { 0 } impl CellNd<2> { pub fn rotate(self) -> Cell2d { self } } }");
    let fills = function(&m, "cell::fills");
    assert_eq!(
        (fills.dims.as_slice(), &fills.cross),
        ([2, 3].as_slice(), &Cross::Ok)
    );
    let rotate = function(&m, "cell::rotate");
    assert_eq!(rotate.dims, vec![2]);
    assert_eq!(
        rotate.ret,
        Ty::Hand {
            name: "CellNd".into(),
            dim: Some(2)
        }
    );
    let alias = m
        .types
        .iter()
        .find(|t| t.path == "cell::Cell2d")
        .expect("the alias");
    assert_eq!(
        (alias.kind, &alias.alias),
        (
            TypeKind::Alias,
            &Some(Ty::Hand {
                name: "CellNd".into(),
                dim: Some(2)
            })
        )
    );
}

#[test]
fn a_trait_generic_is_skipped() {
    let m = manifest("pub fn uniform<T: PartialEq>(items: &[T]) -> bool { true }");
    assert_eq!(reason(&m, "uniform"), "generic over T");
}

#[test]
fn a_closure_parameter_is_skipped() {
    let m = manifest("pub fn render<F>(rule: F) -> u8 where F: Fn(&[u8]) -> bool { 0 }");
    assert_eq!(reason(&m, "render"), "closure parameter F");
}

#[test]
fn a_static_return_is_skipped() {
    let m = manifest("pub fn genus() -> &'static str { \"iso\" }");
    assert_eq!(reason(&m, "genus"), "&'static return");
}

#[test]
fn a_lifetime_bearing_return_is_skipped() {
    let m = manifest("pub fn first<'a>(items: &'a [u8]) -> &'a u8 { &items[0] }");
    assert_eq!(reason(&m, "first"), "lifetime-bearing return");
}

#[test]
fn an_elided_borrow_return_is_copied_out() {
    let m = manifest("pub struct Cell { pub types: Vec<u8> } impl Cell { pub fn shape(&self) -> &[u8] { &self.types } }");
    assert_eq!(function(&m, "shape").cross, Cross::Ok);
}

#[test]
fn a_mutable_borrow_return_is_skipped() {
    let m = manifest("pub struct Tensor { pub data: Vec<u8> } impl Tensor { pub fn bytes_mut(&mut self) -> &mut [u8] { &mut self.data } }");
    assert_eq!(reason(&m, "bytes_mut"), "mutable borrow return");
}

#[test]
fn a_serde_helper_in_a_private_module_is_private() {
    let m = manifest("mod counts { pub fn serialize<S: serde::Serializer>(counts: &u8, serializer: S) -> Result<S::Ok, S::Error> { todo!() } }");
    assert_eq!(function(&m, "counts::serialize").cross, Cross::Private);
}

#[test]
fn an_error_constructor_is_skipped() {
    let src = format!("{ERROR} pub mod other {{}} impl core::error::Error {{}} pub mod core2 {{}}");
    let m = manifest(&format!("pub mod core {{ pub mod error {{ pub enum Error {{ Value(String) }} pub type Result<T> = std::result::Result<T, Error>; pub fn value_error<T>(message: impl Into<String>) -> Result<T> {{ todo!() }} }} }} {}", &src[src.len()..]));
    assert_eq!(reason(&m, "core::error::value_error"), "error constructor");
}

#[test]
fn an_impl_trait_argument_is_skipped() {
    let m = manifest("pub fn new(birth: impl Into<u8>) -> u8 { 0 }");
    assert!(reason(&m, "new").starts_with("birth: impl Trait argument"));
}

#[test]
fn an_iterator_return_is_skipped() {
    let m = manifest("pub fn each() -> impl Iterator<Item = u8> { std::iter::empty() }");
    assert_eq!(reason(&m, "each"), "returns iterator return");
}

#[test]
fn a_private_type_in_a_signature_is_skipped() {
    let m = manifest("type Rotation = fn() -> u8; struct Point; pub fn create(rotation: Rotation) {} pub fn corners() -> Point { Point }");
    assert_eq!(reason(&m, "create"), "rotation: unknown type Rotation");
    assert_eq!(reason(&m, "corners"), "returns unknown type Point");
}

#[test]
fn a_type_without_serde_or_methods_is_uncrossable() {
    let m = manifest("pub mod two { pub enum Shape { Square } pub fn png(shape: Shape) {} }");
    assert!(matches!(
        kind(&m, "two::Shape"),
        TypeCross::Uncrossable { .. }
    ));
    assert_eq!(reason(&m, "two::png"), "shape: uncrossable type two::Shape");
}

#[test]
fn a_class_from_another_unit_is_skipped() {
    let m = manifest("pub mod core { pub struct Colorizer { pub n: u8 } impl Colorizer { pub fn color(&self) -> u8 { 0 } } } pub mod math { pub fn render(c: &crate::core::Colorizer) -> u8 { 0 } }");
    assert_eq!(
        reason(&m, "math::render"),
        "class core::Colorizer lives in unit core"
    );
}

#[test]
fn mutating_plain_data_in_place_is_skipped() {
    let m = manifest("pub fn push(row: &mut String) {} pub fn fft(re: &mut [f64]) {}");
    assert_eq!(reason(&m, "push"), "row: mutates plain data in place");
    assert_eq!(reason(&m, "fft"), "re: mutable slice argument");
}

#[test]
fn the_shortest_public_path_wins_and_ties_go_to_the_definition() {
    let m = manifest("pub mod a { pub mod b { pub fn f() {} } pub use b::f; } pub mod cell { pub fn edges() {} } pub mod two { pub use crate::cell::edges; }");
    assert_eq!(function(&m, "a::f").module, "a");
    assert_eq!(function(&m, "cell::edges").cross, Cross::Ok);
    assert!(m.functions.iter().all(|f| f.path != "two::edges"));
}

#[test]
fn re_exports_resolve_groups_renames_and_globs() {
    let m = manifest("mod one { pub fn x() {} } mod two { pub fn y() {} } mod three { pub fn z() {} } pub mod g { pub use super::one::*; } pub mod h { pub use crate::two::y as ey; pub use crate::three::{z}; }");
    for path in ["g::x", "h::ey", "h::z"] {
        assert_eq!(function(&m, path).cross, Cross::Ok, "{path}");
    }
}

#[test]
fn an_item_reachable_only_through_a_private_module_is_private() {
    let m = manifest(
        "mod hidden { pub fn f() {} pub struct S; } pub mod shown { mod deep { pub fn g() {} } }",
    );
    assert_eq!(function(&m, "hidden::f").cross, Cross::Private);
    assert_eq!(function(&m, "shown::deep::g").cross, Cross::Private);
    assert!(m.types.is_empty());
}

#[test]
fn a_name_collision_fails_the_run() {
    let built = build("pub mod core { pub struct Tensor { pub n: u8 } impl Tensor { pub fn new() -> Tensor { todo!() } } pub struct Cell { pub n: u8 } impl Cell { pub fn new() -> Cell { todo!() } } }");
    assert_eq!(
        built.collisions,
        vec!["collision: core::new at lib.rs:1 and lib.rs:1"]
    );
    let built =
        build("pub mod census { pub fn census() {} pub fn extra() {} } pub use census::census;");
    assert_eq!(
        built.collisions,
        vec!["collision: module census and fn census at lib.rs:1"]
    );
}

#[test]
fn skip_txt_must_name_every_skipped_function_and_nothing_else() {
    let m = manifest("pub fn genus() -> &'static str { \"iso\" } pub fn fine() {}");
    assert_eq!(report::skip_lines(&m), vec!["genus &'static return"]);
    assert_eq!(
        report::check_skip(&m, ""),
        vec!["skip.txt is missing genus"]
    );
    assert_eq!(
        report::check_skip(&m, "fine a stale line\n"),
        vec!["skip.txt is missing genus", "skip.txt names nothing: fine"]
    );
    assert!(report::check_skip(&m, "genus a decision\n").is_empty());
}

#[test]
fn consts_are_listed_with_their_type() {
    let m = manifest("pub mod font { pub const FPS: usize = 25; pub type Pen = (char, &'static [&'static str]); pub const UPPERS: &[Pen] = &[]; }");
    let fps = m
        .consts
        .iter()
        .find(|c| c.path == "font::FPS")
        .expect("fps");
    assert_eq!(
        (&fps.ty, &fps.cross),
        (
            &Ty::Scalar {
                name: "usize".into()
            },
            &Cross::Ok
        )
    );
    let uppers = m
        .consts
        .iter()
        .find(|c| c.path == "font::UPPERS")
        .expect("uppers");
    assert_eq!(
        uppers.cross,
        Cross::Skip {
            reason: "&'static const".into()
        }
    );
}

#[test]
fn a_public_trait_adds_its_methods_to_every_implementing_type() {
    let m = manifest("pub mod name { pub trait Named: Sized { const KIND: &'static str; #[doc = \" Folds.\"] fn checked(self) -> Result<Self, u8>; fn to_json(&self) -> String { String::new() } fn from_json(text: &str) -> Result<Self, u8> { todo!() } } pub mod word { #[derive(Serialize, Deserialize)] pub struct Word { pub n: u8 } impl super::Named for Word { const KIND: &'static str = \"word\"; fn checked(self) -> Result<Self, u8> { Ok(self) } } } }");
    assert_eq!(kind(&m, "name::word::Word"), &TypeCross::Class);
    let checked = function(&m, "name::word::Word::checked");
    assert_eq!(
        (checked.self_kind, checked.source, checked.via.as_deref()),
        (Some(SelfKind::Value), Source::Trait, Some("name::Named"))
    );
    assert_eq!(checked.docs, vec!["Folds."]);
    assert!(checked.defined_at.starts_with("lib.rs:"));
    let from = function(&m, "name::word::Word::from_json");
    assert_eq!(
        (from.self_kind, from.owner.as_deref()),
        (None, Some("name::word::Word"))
    );
    assert!(
        matches!(from.cross, Cross::Skip { .. }),
        "Result<Self, u8> is not the crate Result"
    );
    assert_eq!(function(&m, "name::word::Word::to_json").ret, Ty::String);
    assert!(m.functions.iter().all(|f| f.name != "KIND"));
}

#[test]
fn a_public_class_field_gets_a_setter_unless_it_holds_a_borrow() {
    let root = scratch("setter", "pub mod life { #[derive(Clone, Copy, Serialize, Deserialize)] pub enum Boundary { Constant, Wrap } #[derive(Clone, Serialize, Deserialize)] pub struct Config { pub boundary: Boundary, pub padding: usize, pub name: &'static str } impl Config { pub fn budget(&self) -> usize { 0 } } }", "life\n");
    let rust = read(&root, "pkgs/mrlypy/src/gen.rs");
    assert!(rust.contains("pub fn set_boundary(&mut self, value: PySerde<mrlyrs::life::Boundary>) -> PyResult<()> {"));
    assert!(rust.contains("self.0.padding = value;"));
    assert!(!rust.contains("set_name"));
    assert!(read(&root, "pkgs/mrlypy/python/mrlypy/life/__init__.pyi").contains("@boundary.setter"));
    let wasm = read(&root, "pkgs/mrlyjs/units/life/src/lib.rs");
    assert!(wasm.contains("pub fn set_boundary(&mut self, value: JsValue) -> Result<(), JsValue> {"));
    assert!(wasm.contains("self.inner.padding = value;"));
    assert!(!wasm.contains("set_name"));
    let dts = read(&root, "pkgs/mrlyjs/life.d.ts");
    assert!(dts.contains("set boundary(value: Boundary);"));
    assert!(dts.contains("readonly name: string;"));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn default_crosses_as_a_static_on_the_type_and_an_alias_fixes_n() {
    let lib = "pub mod gen { #[derive(Clone, Default, Serialize, Deserialize)] pub struct Paint { pub n: u8 } #[derive(Clone, Serialize, Deserialize)] pub struct Field { pub n: u8 } impl Field { pub fn size(&self) -> u8 { 0 } } impl Default for Field { fn default() -> Field { Field { n: 1 } } } #[derive(Clone, Serialize, Deserialize)] pub struct ConfigNd<const N: usize> { pub n: u8 } impl<const N: usize> Default for ConfigNd<N> { fn default() -> Self { ConfigNd { n: 0 } } } pub type Config2d = ConfigNd<2>; } pub mod core { #[derive(Clone, Default, Serialize, Deserialize)] pub struct Rng { s: u64 } }";
    let m = manifest(lib);
    let paint = function(&m, "gen::Paint::default");
    assert_eq!(
        (paint.source, paint.owner.as_deref(), paint.self_kind, &paint.cross),
        (Source::Default, Some("gen::Paint"), None, &Cross::Ok)
    );
    assert_eq!(paint.ret, Ty::Plain { path: "gen::Paint".into() });
    assert_eq!(
        function(&m, "gen::Field::default").ret,
        Ty::Class { path: "gen::Field".into(), dim: None }
    );
    assert_eq!(
        function(&m, "gen::Config2d::default").ret,
        Ty::Plain { path: "gen::ConfigNd".into() }
    );
    let defaults: Vec<&str> = m
        .functions
        .iter()
        .filter(|f| f.source == Source::Default)
        .map(|f| f.path.as_str())
        .collect();
    assert_eq!(defaults, ["gen::Config2d::default", "gen::Field::default", "gen::Paint::default"]);
    let root = scratch("default", lib, "gen\n");
    assert!(read(&root, "pkgs/mrlypy/python/mrlypy/gen/__init__.pyi").contains("class Config2d:\n    @staticmethod\n    def default() -> dict[str, Any]:"));
    assert!(read(&root, "pkgs/mrlyjs/gen.d.ts").contains("static default(): Field;"));
    assert!(read(&root, "pkgs/mrlyrs/src/bin/mrly.rs").contains("(\"gen.Config2d.default\", \"() -> gen.ConfigNd\""));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn the_hand_rng_declares_choice_and_shuffle_in_both_bridges() {
    let root = scratch("rng", "pub mod core { pub struct Rng { s: u64 } impl Rng { pub fn new(seed: u64) -> Rng { Rng { s: seed } } pub fn choice<'a, T>(&mut self, items: &'a [T]) -> &'a T { &items[0] } pub fn shuffle<T>(&mut self, seq: &mut [T]) {} } }", "core\n");
    let stub = read(&root, "pkgs/mrlypy/python/mrlypy/core/__init__.pyi");
    assert!(stub.contains("    def choice(self, seq: Any) -> Any:"));
    assert!(stub.contains("    def shuffle(self, seq: list[Any]) -> None:"));
    let dts = read(&root, "pkgs/mrlyjs/core.d.ts");
    assert!(dts.contains("    choice<T>(items: ArrayLike<T>): T;"));
    assert!(dts.contains("    shuffle<T>(items: T[]): void;"));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn every_copy_takes_the_manifest_version() {
    let lib = "pub mod core { pub fn f() {} }";
    let root = scratch("version", lib, "core\n");
    let py = "[package]\nname = \"mrlypy\"\nversion = \"0.1.0\"\n\n[dependencies]\npyo3 = { version = \"0.29\" }\n";
    let js = "{\n  \"name\": \"mrlyjs\",\n  \"version\": \"0.1.0\",\n  \"type\": \"module\"\n}\n";
    std::fs::write(root.join("pkgs/mrlypy/Cargo.toml"), py).expect("a crate");
    std::fs::write(root.join("pkgs/mrlyjs/package.json"), js).expect("a package");
    let mut m = manifest(lib);
    m.version = "1.2.3".into();
    js::write(&m, &root).expect("the js bridge writes");
    version::write(&m, &root).expect("the stamps write");
    assert!(read(&root, "pkgs/mrlyjs/units/core/Cargo.toml").contains("\nversion = \"1.2.3\"\n"));
    assert_eq!(
        read(&root, "pkgs/mrlypy/Cargo.toml"),
        py.replace("0.1.0", "1.2.3")
    );
    assert_eq!(
        read(&root, "pkgs/mrlyjs/package.json"),
        js.replace("0.1.0", "1.2.3")
    );
    std::fs::remove_dir_all(root).ok();
}

fn entry(path: &str, ret: &str, status: &str) -> Value {
    json!({"path": path, "self_kind": null, "params": [], "ret": {"t": "scalar", "name": ret}, "dims": [], "cross": {"status": status}})
}

#[test]
fn a_removed_or_changed_path_breaks_and_an_addition_does_not() {
    let old = [
        entry("a", "u8", "ok"),
        entry("b", "u8", "ok"),
        entry("c", "u8", "ok"),
        entry("d", "u8", "ok"),
        entry("p", "u8", "private"),
    ];
    let new = [
        entry("a", "u8", "ok"),
        entry("b", "u16", "ok"),
        entry("d", "u8", "skip"),
        entry("e", "u8", "ok"),
    ];
    let diff = version::diff(&json!({"functions": old}), &json!({"functions": new}));
    assert_eq!(diff.removed, ["c", "d"]);
    assert_eq!(diff.changed, ["b"]);
    assert_eq!(diff.added, 1);
    let grown = [&old[..4], &[entry("e", "u8", "ok")]].concat();
    let diff = version::diff(&json!({"functions": old}), &json!({"functions": grown}));
    assert!(diff.removed.is_empty() && diff.changed.is_empty());
    assert_eq!(diff.added, 1);
}

#[test]
fn a_break_needs_the_minor_under_one_and_the_major_from_one() {
    assert_eq!(version::need((0, 2, 3), true), (0, 3, 0));
    assert_eq!(version::need((0, 2, 3), false), (0, 2, 4));
    assert_eq!(version::need((1, 2, 3), true), (2, 0, 0));
    assert!(version::semver("0.10.0") > version::semver("0.9.9"));
    assert_eq!(version::semver("0.2"), None);
}

fn crate_files() -> Files {
    parse::load(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../pkgs/mrlyrs/src"
    )))
    .expect("the crate loads")
}

#[test]
fn the_crate_parses_the_same_twice() {
    let files = crate_files();
    let a = resolve::build(&files, "0").expect("first").manifest;
    let b = resolve::build(&files, "0").expect("second").manifest;
    assert_eq!(a, b);
    assert_eq!(
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&b).unwrap()
    );
}

#[test]
fn function_entries_match_the_pub_fn_grep() {
    let files = crate_files();
    let built = resolve::build(&files, "0").expect("the crate builds");
    let grep: usize = files
        .values()
        .map(|t| t.lines().filter(|l| parse::is_pub_fn_line(l)).count())
        .sum();
    let fns = &built.manifest.functions;
    let written = fns
        .iter()
        .filter(|f| f.source == Source::Written && !f.constant)
        .count();
    let constant = fns
        .iter()
        .filter(|f| f.source == Source::Written && f.constant)
        .count();
    let generated = fns.iter().filter(|f| f.source == Source::NamedEnum).count();
    let traits = fns.iter().filter(|f| f.source == Source::Trait).count();
    let defaults = fns.iter().filter(|f| f.source == Source::Default).count();
    let text = &files["math/name/mod.rs"];
    let start = text.find("pub trait Named").expect("the trait");
    let end = start + text[start..].find("\n}\n").expect("its end");
    let trait_fns = text[start..end]
        .lines()
        .filter(|l| l.starts_with("    fn "))
        .count();
    let impls: usize = files
        .values()
        .map(|t| t.matches("impl Named for ").count())
        .sum();
    assert_eq!(
        traits,
        impls * trait_fns,
        "every impl Named carries every trait fn"
    );
    let named = built
        .manifest
        .types
        .iter()
        .filter(|t| matches!(t.cross, TypeCross::Enum { named: true, .. }))
        .count();
    assert_eq!(
        written + built.macro_body_fns,
        grep,
        "every grep line is a written pub fn or sits inside macro_rules"
    );
    assert_eq!(generated, 2 * named, "named_enum! writes all and name");
    assert_eq!(fns.len(), written + constant + generated + traits + defaults);
    assert!(built.collisions.is_empty(), "{:?}", built.collisions);
}
