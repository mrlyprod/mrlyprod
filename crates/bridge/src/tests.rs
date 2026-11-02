use crate::model::*;
use crate::parse::{self, Files};
use crate::report;
use crate::resolve::{self, Built};
use crate::{cli, config, disk, js, leaf, py, version};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

fn doors(unit: &str) -> String {
    format!("crates/{unit}/cli/{}", cli::DOORS)
}

fn build(lib: &str) -> Built {
    let files: Files = [("lib.rs".to_string(), lib.to_string())]
        .into_iter()
        .collect();
    resolve::build(&files, "mrly-test", "0.0.0").expect("the source builds")
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

fn scratch(name: &str, lib: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("bridge-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let m = manifest(lib);
    let units: Vec<&str> = m
        .modules
        .iter()
        .map(|module| module.path.as_str())
        .filter(|path| !path.is_empty() && !path.contains("::"))
        .collect();
    for unit in &units {
        write_leaf(&m, &root, &leaf_of(unit, &units));
    }
    root
}

fn write_leaf(m: &Manifest, root: &Path, leaf: &leaf::Leaf) {
    cli::write_leaf(m, root, leaf).expect("the leaf cli writes");
    py::write_leaf(m, root, leaf).expect("the leaf python writes");
    js::write_leaf(m, root, leaf).expect("the leaf js writes");
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
    let src = format!(
        "{ERROR} use core::Result; pub fn f(a: Vec<u8>, b: &[u64], c: Option<usize>, d: (u8, u8), e: [u8; 4]) -> Result<Vec<u8>> {{ Ok(a) }}"
    );
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
    let m = manifest(
        "pub use serde_json::Value as Json; pub fn f(v: &Json, m: std::collections::HashMap<u8, Vec<u8>>) -> std::collections::BTreeSet<u8> { todo!() }",
    );
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
    let m = manifest(
        "pub mod core { pub mod tensor { pub struct Tensor { pub shape: Vec<usize> } impl Tensor { pub fn new(shape: Vec<usize>) -> Tensor { Tensor { shape } } pub fn size(&self) -> usize { 0 } } } pub use tensor::Tensor; }",
    );
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
    let m = manifest(
        "pub mod six { mod models { pub struct Cell6d { pub start: u8 } impl Cell6d { pub fn width(&self) -> usize { 0 } } } pub use models::Cell6d; }",
    );
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
fn an_image_crosses_by_hand_as_one_shaped_array_and_a_list_of_them() {
    let lib = "pub mod core { pub mod image { #[derive(Clone, Serialize, Deserialize)] pub struct Image { pub width: usize, pub height: usize, pub colors: Vec<[u8; 4]> } impl Image { pub fn scale(&self, scale: usize) -> Image { self.clone() } } } pub use image::Image; } pub mod life { pub fn frame(scale: usize) -> crate::core::Image { todo!() } pub fn frames(scale: usize) -> Vec<crate::core::Image> { vec![] } }";
    let m = manifest(lib);
    assert_eq!(
        kind(&m, "core::Image"),
        &TypeCross::Hand {
            name: "Image".into()
        }
    );
    let image = || Ty::Hand {
        name: "Image".into(),
        dim: None,
    };
    assert_eq!(function(&m, "life::frame").ret, image());
    assert_eq!(
        function(&m, "life::frames").ret,
        Ty::Vec {
            item: Box::new(image())
        }
    );
    let scale = function(&m, "core::image::scale");
    assert_eq!(
        (scale.self_kind, &scale.cross),
        (Some(SelfKind::Ref), &Cross::Ok)
    );
    let root = scratch("image", lib);
    let rust =
        read(&root, "crates/core/py/src/doors.rs") + &read(&root, "crates/life/py/src/doors.rs");
    assert!(rust.contains("image: PyImage,"));
    assert!(rust.contains("(PyImage(out)).into_bound_py_any(py)"));
    assert!(rust.contains(".map(PyImage)"));
    let stub = read(&root, "crates/life/py/python/mrly/life/__init__.pyi");
    assert!(stub.contains("def frame(scale: int) -> NDArray[Any]:"));
    assert!(stub.contains("def frames(scale: int) -> list[NDArray[Any]]:"));
    let wasm = read(&root, "crates/life/js/src/lib.rs");
    assert!(wasm.contains("hand::image_to_js(&value)"));
    assert!(wasm.contains("hand::list_to_js(&value, hand::image_to_js)"));
    let core = read(&root, "crates/core/js/src/lib.rs");
    assert!(core.contains("let image = hand::image_from_js(&image)?;"));
    let dts = read(&root, "crates/life/js/index.d.ts");
    assert!(dts.contains(
        "export interface Image {\n    shape: [number, number];\n    colors: Uint8Array<ArrayBuffer>;\n}"
    ));
    assert!(dts.contains("export function frame(scale: number): Image;"));
    assert!(dts.contains("export function frames(scale: number): Image[];"));
    let cli: String = read(&root, &doors("life")).split_whitespace().collect();
    assert!(cli.contains("name:\"life.frames\",signature:\"(scale:usize)->[Image]\""));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_boxed_self_reference_crosses_as_the_type_itself() {
    let lib = "pub mod shape { #[derive(Clone, Serialize, Deserialize)] pub enum Shape { Ball { radius: u8 }, Anti(Box<Shape>), Window { inner: Box<Shape>, low: Vec<u8> } } pub fn area(shape: &Shape) -> u8 { 0 } }";
    let m = manifest(lib);
    let shape = m
        .types
        .iter()
        .find(|t| t.path == "shape::Shape")
        .expect("the shape");
    let inner = || Ty::Plain {
        path: "shape::Shape".into(),
    };
    assert_eq!(shape.variants[1].fields[0].ty, inner());
    assert_eq!(shape.variants[2].fields[0].ty, inner());
    let root = scratch("boxed", lib);
    let dts = read(&root, "crates/shape/js/index.d.ts");
    assert!(dts.contains(
        "export type Shape = { Ball: { radius: number } } | { Anti: Shape } | { Window: { inner: Shape; low: number[] } };"
    ));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_serde_struct_without_self_methods_is_plain_and_keeps_its_constructors() {
    let m = manifest(
        "pub mod tile { #[derive(Serialize, Deserialize)] pub struct Tile { pub side: usize } impl Tile { pub fn new(side: usize) -> Tile { Tile { side } } } }",
    );
    assert_eq!(kind(&m, "tile::Tile"), &TypeCross::Plain);
    let f = function(&m, "tile::Tile::new");
    assert_eq!(
        (f.owner.as_deref(), f.module.as_str(), &f.cross),
        (Some("tile::Tile"), "tile", &Cross::Ok)
    );
}

#[test]
fn a_serde_skipped_field_makes_a_class() {
    let m = manifest(
        "pub mod tile { #[derive(Serialize, Deserialize)] pub struct File { pub width: usize, #[serde(default, skip_serializing_if = \"Vec::is_empty\")] pub tags: Vec<u8>, #[serde(skip)] pub image: Vec<u8> } #[derive(Serialize, Deserialize)] pub struct Tile { #[serde(default, skip_serializing_if = \"Option::is_none\")] pub side: Option<usize> } }",
    );
    assert_eq!(kind(&m, "tile::File"), &TypeCross::Class);
    assert_eq!(kind(&m, "tile::Tile"), &TypeCross::Plain);
    let file = m
        .types
        .iter()
        .find(|t| t.path == "tile::File")
        .expect("File is listed");
    let skips: Vec<bool> = file.fields.iter().map(|f| f.serde_skip).collect();
    assert_eq!(skips, [false, false, true]);
}

#[test]
fn a_struct_with_a_self_method_is_a_class() {
    let m = manifest(
        "pub mod life { #[derive(Serialize, Deserialize)] pub struct Life { pub time: usize } impl Life { pub fn step(&mut self) {} } }",
    );
    assert_eq!(kind(&m, "life::Life"), &TypeCross::Class);
    let f = function(&m, "life::Life::step");
    assert_eq!((f.self_kind, &f.cross), (Some(SelfKind::Mut), &Cross::Ok));
}

#[test]
fn a_fieldless_serde_enum_is_a_string_even_with_methods() {
    let m = manifest(
        "#[derive(Serialize, Deserialize)] pub enum Dtype { U8, U16 } impl Dtype { pub fn max(self) -> i64 { 0 } }",
    );
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
    let m = manifest(
        "pub mod life { named_enum! { #[derive(Serialize, Deserialize)] pub enum Fate { Dead => \"dead\", Alive => \"alive\", } } }",
    );
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
    let m = manifest(
        "pub mod cell { pub struct CellNd<const N: usize> { pub n: u8 } pub type Cell2d = CellNd<2>; pub fn fills<const N: usize>(cell: &CellNd<N>) -> usize { 0 } impl CellNd<2> { pub fn rotate(self) -> Cell2d { self } } }",
    );
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
    let m = manifest("pub fn table() -> &'static [u8] { &[] }");
    assert_eq!(reason(&m, "table"), "&'static return");
}

#[test]
fn a_static_str_return_is_copied_out() {
    let m = manifest("pub fn genus() -> &'static str { \"iso\" }");
    assert_eq!(function(&m, "genus").cross, Cross::Ok);
}

#[test]
fn a_lifetime_bearing_return_is_skipped() {
    let m = manifest("pub fn first<'a>(items: &'a [u8]) -> &'a u8 { &items[0] }");
    assert_eq!(reason(&m, "first"), "lifetime-bearing return");
}

#[test]
fn an_elided_borrow_return_is_copied_out() {
    let m = manifest(
        "pub struct Cell { pub types: Vec<u8> } impl Cell { pub fn shape(&self) -> &[u8] { &self.types } }",
    );
    assert_eq!(function(&m, "shape").cross, Cross::Ok);
}

#[test]
fn a_mutable_borrow_return_is_skipped() {
    let m = manifest(
        "pub struct Tensor { pub data: Vec<u8> } impl Tensor { pub fn bytes_mut(&mut self) -> &mut [u8] { &mut self.data } }",
    );
    assert_eq!(reason(&m, "bytes_mut"), "mutable borrow return");
}

#[test]
fn a_serde_helper_in_a_private_module_is_private() {
    let m = manifest(
        "mod counts { pub fn serialize<S: serde::Serializer>(counts: &u8, serializer: S) -> Result<S::Ok, S::Error> { todo!() } }",
    );
    assert_eq!(function(&m, "counts::serialize").cross, Cross::Private);
}

#[test]
fn an_error_constructor_is_skipped() {
    let src = format!("{ERROR} pub mod other {{}} impl core::error::Error {{}} pub mod core2 {{}}");
    let m = manifest(&format!(
        "pub mod core {{ pub mod error {{ pub enum Error {{ Value(String) }} pub type Result<T> = std::result::Result<T, Error>; pub fn value_error<T>(message: impl Into<String>) -> Result<T> {{ todo!() }} }} }} {}",
        &src[src.len()..]
    ));
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
    let m = manifest(
        "type Rotation = fn() -> u8; struct Point; pub fn create(rotation: Rotation) {} pub fn corners() -> Point { Point }",
    );
    assert_eq!(reason(&m, "create"), "rotation: unknown type Rotation");
    assert_eq!(reason(&m, "corners"), "returns unknown type Point");
}

#[test]
fn a_type_without_serde_or_methods_is_uncrossable() {
    let m = manifest("pub mod two { pub enum Shape { Square } pub fn image(shape: Shape) {} }");
    assert!(matches!(
        kind(&m, "two::Shape"),
        TypeCross::Uncrossable { .. }
    ));
    assert_eq!(
        reason(&m, "two::image"),
        "shape: uncrossable type two::Shape"
    );
}

#[test]
fn a_class_from_another_unit_is_skipped() {
    let m = manifest(
        "pub mod core { pub struct Colorizer { pub n: u8 } impl Colorizer { pub fn color(&self) -> u8 { 0 } } } pub mod math { pub fn render(c: &crate::core::Colorizer) -> u8 { 0 } }",
    );
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
    let m = manifest(
        "pub mod a { pub mod b { pub fn f() {} } pub use b::f; } pub mod cell { pub fn edges() {} } pub mod two { pub use crate::cell::edges; }",
    );
    assert_eq!(function(&m, "a::f").module, "a");
    assert_eq!(function(&m, "cell::edges").cross, Cross::Ok);
    assert!(m.functions.iter().all(|f| f.path != "two::edges"));
}

#[test]
fn re_exports_resolve_groups_renames_and_globs() {
    let m = manifest(
        "mod one { pub fn x() {} } mod two { pub fn y() {} } mod three { pub fn z() {} } pub mod g { pub use super::one::*; } pub mod h { pub use crate::two::y as ey; pub use crate::three::{z}; }",
    );
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
    let built = build(
        "pub mod core { pub struct Tensor { pub n: u8 } impl Tensor { pub fn new() -> Tensor { todo!() } } pub struct Cell { pub n: u8 } impl Cell { pub fn new() -> Cell { todo!() } } }",
    );
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
    let m = [manifest(
        "pub fn genus() -> &'static [u8] { &[] } pub fn fine() {}",
    )];
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
    let m = manifest(
        "pub mod font { pub const FPS: usize = 25; pub type Pen = (char, &'static [&'static str]); pub const UPPERS: &[Pen] = &[]; }",
    );
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
    let m = manifest(
        "pub mod name { pub trait Named: Sized { const KIND: &'static str; #[doc = \" Folds.\"] fn checked(self) -> Result<Self, u8>; fn to_json(&self) -> String { String::new() } fn from_json(text: &str) -> Result<Self, u8> { todo!() } } pub mod word { #[derive(Serialize, Deserialize)] pub struct Word { pub n: u8 } impl super::Named for Word { const KIND: &'static str = \"word\"; fn checked(self) -> Result<Self, u8> { Ok(self) } } } }",
    );
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
    let root = scratch(
        "setter",
        "pub mod life { #[derive(Clone, Copy, Serialize, Deserialize)] pub enum Boundary { Constant, Wrap } #[derive(Clone, Serialize, Deserialize)] pub struct Config { pub boundary: Boundary, pub padding: usize, pub name: &'static str } impl Config { pub fn budget(&self) -> usize { 0 } } }",
    );
    let rust = read(&root, "crates/life/py/src/doors.rs");
    assert!(rust.contains(
        "pub fn set_boundary(&mut self, value: PySerde<mrly_life::Boundary>) -> PyResult<()> {"
    ));
    assert!(rust.contains("self.0.padding = value;"));
    assert!(!rust.contains("set_name"));
    assert!(
        read(&root, "crates/life/py/python/mrly/life/__init__.pyi").contains("@boundary.setter")
    );
    let wasm = read(&root, "crates/life/js/src/lib.rs");
    assert!(
        wasm.contains("pub fn set_boundary(&mut self, value: JsValue) -> Result<(), JsValue> {")
    );
    assert!(wasm.contains("self.inner.padding = value;"));
    assert!(!wasm.contains("set_name"));
    let dts = read(&root, "crates/life/js/index.d.ts");
    assert!(dts.contains("set boundary(value: Boundary);"));
    assert!(dts.contains("readonly name: string;"));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn default_crosses_as_a_static_on_the_type_and_an_alias_fixes_n() {
    let lib = "pub mod tile { #[derive(Clone, Default, Serialize, Deserialize)] pub struct Paint { pub n: u8 } #[derive(Clone, Serialize, Deserialize)] pub struct Field { pub n: u8 } impl Field { pub fn size(&self) -> u8 { 0 } } impl Default for Field { fn default() -> Field { Field { n: 1 } } } #[derive(Clone, Serialize, Deserialize)] pub struct ConfigNd<const N: usize> { pub n: u8 } impl<const N: usize> Default for ConfigNd<N> { fn default() -> Self { ConfigNd { n: 0 } } } pub type Config2d = ConfigNd<2>; } pub mod core { #[derive(Clone, Default, Serialize, Deserialize)] pub struct Rng { s: u64 } }";
    let m = manifest(lib);
    let paint = function(&m, "tile::Paint::default");
    assert_eq!(
        (
            paint.source,
            paint.owner.as_deref(),
            paint.self_kind,
            &paint.cross
        ),
        (Source::Default, Some("tile::Paint"), None, &Cross::Ok)
    );
    assert_eq!(
        paint.ret,
        Ty::Plain {
            path: "tile::Paint".into()
        }
    );
    assert_eq!(
        function(&m, "tile::Field::default").ret,
        Ty::Class {
            path: "tile::Field".into(),
            dim: None
        }
    );
    assert_eq!(
        function(&m, "tile::Config2d::default").ret,
        Ty::Plain {
            path: "tile::ConfigNd".into()
        }
    );
    let defaults: Vec<&str> = m
        .functions
        .iter()
        .filter(|f| f.source == Source::Default)
        .map(|f| f.path.as_str())
        .collect();
    assert_eq!(
        defaults,
        [
            "tile::Config2d::default",
            "tile::Field::default",
            "tile::Paint::default"
        ]
    );
    let root = scratch("default", lib);
    assert!(
        read(&root, "crates/tile/py/python/mrly/tile/__init__.pyi")
            .contains("class Config2d:\n    @staticmethod\n    def default() -> dict[str, Any]:")
    );
    assert!(read(&root, "crates/tile/js/index.d.ts").contains("static default(): Field;"));
    let cli: String = read(&root, &doors("tile")).split_whitespace().collect();
    assert!(cli.contains("name:\"tile.Config2d.default\",signature:\"()->tile.ConfigNd\""));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn the_hand_rng_declares_choice_and_shuffle_in_both_bridges() {
    let root = scratch(
        "rng",
        "pub mod core { pub struct Rng { s: u64 } impl Rng { pub fn new(seed: u64) -> Rng { Rng { s: seed } } pub fn choice<'a, T>(&mut self, items: &'a [T]) -> &'a T { &items[0] } pub fn shuffle<T>(&mut self, seq: &mut [T]) {} } }",
    );
    let stub = read(&root, "crates/core/py/python/mrly/core/__init__.pyi");
    assert!(stub.contains("    def choice(self, seq: Any) -> Any:"));
    assert!(stub.contains("    def shuffle(self, seq: list[Any]) -> None:"));
    let dts = read(&root, "crates/core/js/index.d.ts");
    assert!(dts.contains("    choice<T>(items: ArrayLike<T>): T;"));
    assert!(dts.contains("    shuffle<T>(items: T[]): void;"));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_nested_type_without_serde_shuts_its_cli_door() {
    let lib = "pub mod num { #[derive(Clone)] pub struct Pair { pub n: u8 } impl Pair { pub fn size(&self) -> u8 { 0 } } pub fn pairs() -> Vec<Pair> { vec![] } }";
    let root = scratch("nested", lib);
    let cli: String = read(&root, &doors("num")).split_whitespace().collect();
    assert!(cli.contains(
        "signature:\"()->[num.Pair]\",docs:\"\",uncallable:Some(\"num::PairhasnoSerialize\"),door:None"
    ));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_watched_run_names_the_stale_files_and_writes_nothing() {
    let lib = "pub mod core { pub fn add(a: u8, b: u8) -> u8 { a + b } }";
    let root = scratch("check", lib);
    let m = manifest(lib);
    let (old, gone) = ("crates/core/py/src/doors.rs", "crates/core/js/index.js");
    std::fs::write(root.join(old), "stale").expect("a stale file");
    std::fs::remove_file(root.join(gone)).expect("a missing file");
    disk::watch();
    let leaf = leaf_of("core", &["core"]);
    py::write_leaf(&m, &root, &leaf).expect("the python bridge checks");
    js::write_leaf(&m, &root, &leaf).expect("the js bridge checks");
    assert_eq!(disk::stale(), [root.join(old), root.join(gone)]);
    assert_eq!(read(&root, old), "stale");
    assert!(!root.join(gone).exists());
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn generated_rust_is_saved_formatted() {
    let lib = "pub mod core { pub fn add(a: u8, b: u8) -> u8 { a + b } }";
    let root = scratch("fmt", lib);
    for file in [
        &*doors("core"),
        "crates/core/py/src/doors.rs",
        "crates/core/js/src/lib.rs",
    ] {
        let text = read(&root, file);
        assert_eq!(disk::rustfmt(&text).expect("rustfmt runs"), text, "{file}");
    }
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn every_copy_takes_the_manifest_version() {
    let lib = "pub mod core { pub fn f() {} }";
    let root = scratch("version", lib);
    let pins = "[workspace.package]\nversion = \"0.1.0\"\n\n[workspace.dependencies]\nmrly-core = { path = \"crates/core\", version = \"=0.1.0\" }\nmrly-num = { path = \"crates/num\", version = \"=0.1.0\" }\n";
    let other = "other = { version = \"=0.1.0\" }\n";
    let cargo = format!("{pins}{other}");
    std::fs::write(root.join("Cargo.toml"), cargo).expect("a workspace");
    let mut m = manifest(lib);
    m.version = "1.2.3".into();
    js::write_leaf(&m, &root, &leaf_of("core", &["core"])).expect("the js bridge writes");
    version::write(&root, &m.version).expect("the pins write");
    assert!(read(&root, "crates/core/js/Cargo.toml").contains("\nversion.workspace = true\n"));
    assert_eq!(
        read(&root, "Cargo.toml"),
        format!("{}{other}", pins.replace("=0.1.0", "=1.2.3"))
    );
    let package: Value =
        serde_json::from_str(&read(&root, "crates/core/js/package.json")).expect("a package");
    assert_eq!(package["version"], "1.2.3");
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

#[test]
fn the_bump_reads_each_leaf_manifest_at_the_tag() {
    let root = std::env::temp_dir().join(format!("bridge-bump-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let leaves = [(
        "core".to_string(),
        PathBuf::from("crates/core/manifest.json"),
    )];
    let file = root.join(&leaves[0].1);
    std::fs::create_dir_all(file.parent().expect("a folder")).expect("a scratch root");
    let git = |args: &[&str]| {
        let done = std::process::Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(["-c", "user.name=bridge"])
            .args(["-c", "user.email=bridge@localhost"])
            .args(["-c", "commit.gpgsign=false"])
            .args(args)
            .output()
            .expect("git runs");
        assert!(done.status.success(), "git {args:?}");
    };
    let write = |version: &str, ret: &str| {
        let text = json!({"version": version, "functions": [entry("a", ret, "ok")]});
        std::fs::write(&file, text.to_string()).expect("a manifest");
    };
    git(&["init", "-q"]);
    write("0.1.0", "u8");
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "one"]);
    git(&["tag", "v0.1.0"]);
    write("0.1.1", "u8");
    assert_eq!(version::bump(&root, &leaves), Ok(()));
    write("0.1.1", "u16");
    let refused = version::bump(&root, &leaves).expect_err("a break under a patch");
    assert!(refused.contains("needs 0.2.0"), "{refused}");
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_tag_before_the_split_reads_the_bridge_manifest_cut_to_the_unit() {
    let root = std::env::temp_dir().join(format!("bridge-split-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let git = |args: &[&str]| {
        let done = std::process::Command::new("git")
            .arg("-C")
            .arg(&root)
            .args(["-c", "user.name=bridge"])
            .args(["-c", "user.email=bridge@localhost"])
            .args(["-c", "commit.gpgsign=false"])
            .args(args)
            .output()
            .expect("git runs");
        assert!(done.status.success(), "git {args:?}");
    };
    let at = |module: &str, path: &str, ret: &str| {
        let mut f = entry(path, ret, "ok");
        f["module"] = json!(module);
        f
    };
    let save = |path: &str, text: Value| {
        let file = root.join(path);
        std::fs::create_dir_all(file.parent().expect("a folder")).expect("a folder");
        std::fs::write(file, text.to_string()).expect("a manifest");
    };
    save(
        "crates/bridge/manifest.json",
        json!({"version": "0.1.0", "functions": [at("core", "core::a", "u8"), at("num", "num::b", "u8")]}),
    );
    git(&["init", "-q"]);
    git(&["add", "-A"]);
    git(&["commit", "-q", "-m", "one"]);
    git(&["tag", "v0.1.0"]);
    let leaves = [(
        "core".to_string(),
        PathBuf::from("crates/core/manifest.json"),
    )];
    let core = |ret: &str| json!({"version": "0.1.1", "functions": [at("core", "core::a", ret)]});
    save("crates/core/manifest.json", core("u8"));
    assert_eq!(version::bump(&root, &leaves), Ok(()));
    save("crates/core/manifest.json", core("u16"));
    let refused = version::bump(&root, &leaves).expect_err("a break under a patch");
    assert!(refused.contains("needs 0.2.0"), "{refused}");
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_crate_without_a_unit_is_refused() {
    let root = std::env::temp_dir().join(format!("bridge-unit-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for dir in ["crates/bridge", "crates/whole"] {
        std::fs::create_dir_all(root.join(dir)).expect("a folder");
    }
    std::fs::write(root.join("Cargo.lock"), "").expect("a lock");
    std::fs::write(root.join("crates/bridge/bridge.toml"), "crates = \"..\"\n").expect("a config");
    std::fs::write(root.join("crates/whole/bridge.toml"), "").expect("a config");
    let refused = config::load(&root.join("crates/bridge"))
        .err()
        .expect("a crate without a unit");
    assert!(refused.ends_with("a crate without a unit"), "{refused}");
    std::fs::remove_dir_all(root).ok();
}

fn crate_files() -> Files {
    let config = config::load(&config::home()).expect("the bridge config loads");
    let units: Vec<String> = config.mounts.iter().map(|m| m.unit.clone()).collect();
    parse::load_crates(&config.mounts, &units).expect("the crates load")
}

#[test]
fn the_crate_parses_the_same_twice() {
    let files = crate_files();
    let a = resolve::build(&files, "mrly-test", "0")
        .expect("first")
        .manifest;
    let b = resolve::build(&files, "mrly-test", "0")
        .expect("second")
        .manifest;
    assert_eq!(a, b);
    assert_eq!(
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&b).unwrap()
    );
}

#[test]
fn function_entries_match_the_pub_fn_grep() {
    let files = crate_files();
    let built = resolve::build(&files, "mrly-test", "0").expect("the crate builds");
    let grep: usize = files
        .values()
        .map(|t| {
            let lines: Vec<&str> = t.lines().collect();
            (0..lines.len())
                .filter(|&i| parse::is_pub_fn_line(lines[i]))
                .filter(|&i| i == 0 || lines[i - 1].trim() != "#[doc(hidden)]")
                .count()
        })
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
    let text = files
        .values()
        .find(|text| text.contains("pub trait Named"))
        .expect("the trait");
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
        written
            + parse::parse(&files)
                .expect("the crates parse")
                .macro_body_fns,
        grep,
        "every grep line is a written pub fn or sits inside macro_rules"
    );
    assert_eq!(generated, 2 * named, "named_enum! writes all and name");
    assert_eq!(
        fns.len(),
        written + constant + generated + traits + defaults
    );
    assert!(built.collisions.is_empty(), "{:?}", built.collisions);
}

fn mounted(files: &[(&str, &str)]) -> Manifest {
    let files: Files = files
        .iter()
        .map(|(key, text)| (key.to_string(), text.to_string()))
        .collect();
    resolve::build(&files, "mrly-test", "0.0.0")
        .expect("the crates build")
        .manifest
}

#[test]
fn a_crate_path_inside_a_mount_starts_at_that_crate() {
    let m = mounted(&[
        ("lib.rs", "pub use mrly_core as core;"),
        ("core/lib.rs", "pub mod tensor; pub use tensor::Shape;"),
        (
            "core/tensor.rs",
            "#[derive(Serialize, Deserialize)] pub struct Shape { pub n: u8 } pub fn make() -> crate::tensor::Shape { Shape { n: 0 } }",
        ),
    ]);
    assert_eq!(kind(&m, "core::Shape"), &TypeCross::Plain);
    assert_eq!(function(&m, "core::tensor::make").cross, Cross::Ok);
}

#[test]
fn an_extern_crate_path_resolves_to_its_mount() {
    let m = mounted(&[
        (
            "lib.rs",
            "pub use mrly_core as core; pub use mrly_life as life;",
        ),
        (
            "core/lib.rs",
            "#[derive(Serialize, Deserialize)] pub struct Rule { pub n: u8 }",
        ),
        (
            "life/lib.rs",
            "pub fn rule() -> mrly_core::Rule { mrly_core::Rule { n: 3 } }",
        ),
    ]);
    assert_eq!(function(&m, "life::rule").cross, Cross::Ok);
}

#[test]
fn a_doc_hidden_item_is_not_public() {
    let m = manifest(
        "pub mod a { #[doc(hidden)] pub fn secret() {} pub fn open() {} #[doc(hidden)] pub use self::open as also; pub fn hidden() {} }",
    );
    let paths: Vec<&str> = m.functions.iter().map(|f| f.path.as_str()).collect();
    assert!(
        paths.contains(&"a::open") && paths.contains(&"a::hidden"),
        "{paths:?}"
    );
    assert!(
        !paths.contains(&"a::secret") && !paths.contains(&"a::also"),
        "{paths:?}"
    );
}

#[test]
fn a_class_field_of_another_units_class_is_a_js_skip() {
    let m = mounted(&[
        (
            "lib.rs",
            "pub use mrly_core as core; pub use mrly_tile as tile;",
        ),
        (
            "core/lib.rs",
            "pub struct Paint { n: u8 } impl Paint { pub fn n(&self) -> u8 { self.n } }",
        ),
        (
            "tile/lib.rs",
            "pub struct Art { pub paint: Option<mrly_core::Paint>, s: u8 } impl Art { pub fn s(&self) -> u8 { self.s } }",
        ),
    ]);
    assert_eq!(
        js::field_skips(&m),
        vec![(
            "tile::Art::paint".to_string(),
            "field type is a class of another unit".to_string()
        )]
    );
}

#[test]
fn no_two_crossing_types_print_the_same_name() {
    let m = resolve::build(&crate_files(), "mrly-test", "0")
        .expect("the crate builds")
        .manifest;
    let mut seen: std::collections::BTreeMap<String, String> = Default::default();
    let mut clashes = vec![];
    for f in &m.functions {
        let dim = match f.dims.as_slice() {
            [one] => Some(*one),
            _ => None,
        };
        for ty in f.params.iter().map(|p| &p.ty).chain([&f.ret]) {
            ty.walk(&mut |part| {
                let held = match part {
                    Ty::Hand { name, dim: own } => format!("hand {name} {:?}", own.or(dim)),
                    Ty::Plain { path }
                    | Ty::Enum { path }
                    | Ty::Class { path, .. }
                    | Ty::Opaque { path } => path.clone(),
                    _ => return,
                };
                let name = cli::show(part, dim);
                match seen.get(&name) {
                    Some(other) if *other != held => {
                        clashes.push(format!("{name}: {other} and {held}"))
                    }
                    Some(_) => {}
                    None => {
                        seen.insert(name, held);
                    }
                }
            });
        }
    }
    clashes.sort();
    clashes.dedup();
    assert!(clashes.is_empty(), "{clashes:#?}");
}

#[test]
fn doc_links_cross_as_plain_text() {
    assert_eq!(
        parse::plain(
            "See [`Tile::new`](crate::recipe::Tile::new), [`animate`](crate::animate()) and [`Rule`]."
        ),
        "See `Tile::new`, `animate` and `Rule`."
    );
    assert_eq!(
        parse::plain("A [site](https://mrly.net) and a list [1, 2] stay."),
        "A [site](https://mrly.net) and a list [1, 2] stay."
    );
}

#[test]
fn a_module_takes_its_docs_from_its_own_file_first() {
    let m = mounted(&[
        ("lib.rs", "/// The facade line.\npub use mrly_core as core;"),
        (
            "core/lib.rs",
            "//! The crate line.\n/// The outer line.\npub mod tensor;\npub mod ramp;",
        ),
        ("core/tensor.rs", "//! The inner line.\npub fn f() {}"),
        ("core/ramp.rs", "pub fn g() {}"),
    ]);
    let docs = |path: &str| {
        m.modules
            .iter()
            .find(|module| module.path == path)
            .map(|module| module.docs.clone())
            .unwrap_or_else(|| panic!("no module {path}"))
    };
    assert_eq!(docs("core"), ["The crate line."]);
    assert_eq!(docs("core::tensor"), ["The inner line."]);
    assert_eq!(docs("core::ramp"), Vec::<String>::new());
}

#[test]
fn the_bridge_marks_every_file_it_writes_as_generated_and_keeps_other_lines() {
    let root = scratch("made", "pub mod core { pub fn f() {} }");
    let before = "*.sh text eol=lf\ngone.js linguist-generated=true\n";
    let made = disk::attributes(&root, before);
    assert!(made.starts_with("*.sh text eol=lf\n"));
    assert!(!made.contains("gone.js"));
    for file in [
        &*doors("core"),
        "crates/core/js/index.js",
        "crates/core/js/package.json",
        "crates/core/py/src/doors.rs",
    ] {
        assert!(
            made.contains(&format!("\n{file} linguist-generated=true\n")),
            "{file}"
        );
    }
    assert!(!made.contains("\nCargo.toml "));
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn an_orphan_is_a_listed_file_the_run_did_not_write_and_nothing_else() {
    let root = scratch("gone", "pub mod core { pub fn f() {} }");
    let js = root.join("crates/core/js");
    for file in ["gone.d.ts", "gone.js", "hand.js", "hand.d.ts"] {
        std::fs::write(js.join(file), "").expect("a file");
    }
    let before = [
        "crates/core/js/index.js",
        "crates/core/js/gone.d.ts",
        "crates/core/js/gone.js",
        "crates/core/js/missing.js",
        "crates/core/js/*.d.ts",
        "../crates/core/js/hand.js",
    ]
    .map(|path| format!("{path} linguist-generated=true\n"))
    .concat();
    assert_eq!(
        disk::orphans(&root, &before),
        [js.join("gone.d.ts"), js.join("gone.js")]
    );
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_stub_name_that_shadows_a_builtin_type_qualifies_the_builtin() {
    let root = scratch(
        "shadow",
        "pub mod core { pub fn bytes(data: Vec<u8>) -> Vec<u8> { data } pub fn sum(data: Vec<u8>) -> u8 { 0 } }",
    );
    let stub = read(&root, "crates/core/py/python/mrly/core/__init__.pyi");
    assert!(stub.starts_with("import builtins\n"), "{stub}");
    assert!(
        stub.contains("def bytes(data: builtins.bytes) -> builtins.bytes:"),
        "{stub}"
    );
    assert!(
        stub.contains("def sum(data: builtins.bytes) -> int:"),
        "{stub}"
    );
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn every_hand_row_names_a_hand_type_of_the_crate() {
    let m = resolve::build(&crate_files(), "mrly-test", "0")
        .expect("the crate builds")
        .manifest;
    for row in hands() {
        for rust in &row.rust {
            let path = rust.strip_prefix("mrly::").expect("a mrly path");
            assert!(
                m.types.iter().any(|t| t.path == path
                    && matches!(&t.cross, TypeCross::Hand { name } if *name == row.name)),
                "{} names {rust}, not a hand type of the crate",
                row.name
            );
        }
    }
    for t in &m.types {
        if let TypeCross::Hand { name } = &t.cross {
            assert!(hand(name).is_some(), "{} has no HAND row", t.path);
        }
    }
}

#[test]
fn a_bridge_toml_holds_strings_lists_and_hand_rows_and_nothing_else() {
    use crate::toml::{Value, parse};
    let doc = parse(
        "unit = \"core\"\n\n[[hand]]\nname = \"A \\\"b\\\"\"\njs = [\"a\", \"b\"]\nrust = []\n",
    )
    .expect("a bridge.toml");
    assert_eq!(doc.top["unit"], Value::Text("core".into()));
    assert_eq!(doc.arrays.len(), 1);
    assert_eq!(doc.arrays[0].0, "hand");
    assert_eq!(doc.arrays[0].1["name"], Value::Text("A \"b\"".into()));
    assert_eq!(
        doc.arrays[0].1["js"],
        Value::List(vec!["a".into(), "b".into()])
    );
    assert_eq!(doc.arrays[0].1["rust"], Value::List(vec![]));
    assert!(parse("unit = core\n").is_err());
    assert!(parse("unit = \"a\"\nunit = \"b\"\n").is_err());
}

// LEAVES

fn leaf_of(unit: &str, seen: &[&str]) -> leaf::Leaf {
    let dir = |unit: &str, end: &str| PathBuf::from(format!("crates/{unit}/{end}"));
    leaf::Leaf {
        unit: unit.into(),
        manifest: dir(unit, "manifest.json"),
        js: dir(unit, "js"),
        py: dir(unit, "py"),
        cli: dir(unit, "cli"),
        seen: seen.iter().map(|s| s.to_string()).collect(),
        pys: vec![],
        keys: vec![],
        hand: "core".into(),
        base: "core".into(),
        hands: leaf::Hands {
            js: dir("core", "js/hand"),
            py: dir("core", "py/hand"),
            cli: dir("core", "cli/hand"),
        },
    }
}

fn scratch_leaf(name: &str, lib: &str, leaf: &leaf::Leaf) -> PathBuf {
    let root = std::env::temp_dir().join(format!("bridge-leaf-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    write_leaf(&manifest(lib), &root, leaf);
    root
}

const STREAM: &str = "pub mod core { pub mod rng { #[derive(Clone, Serialize, Deserialize)] pub struct Rng { s: u64 } impl Rng { pub fn unit(&mut self) -> f64 { 0.0 } } } pub use rng::Rng; pub fn coin(rng: &mut Rng) -> bool { true } } pub mod math { pub fn noise(n: usize, rng: &mut crate::core::Rng) -> u8 { 0 } }";

#[test]
fn a_leaf_calls_its_units_crates_by_name() {
    let root = scratch_leaf("names", STREAM, &leaf_of("math", &["core", "math"]));
    let js = read(&root, "crates/math/js/src/lib.rs");
    assert!(js.contains("mrly_math::noise(n, rng.stream())"), "{js}");
    assert!(js.contains("use mrly_core_js_hand as hand;"), "{js}");
    let cli = read(&root, "crates/math/cli/src/doors.gen.rs");
    assert!(cli.contains("mrly_core::Rng::new(seed("), "{cli}");
    for text in [js, cli, read(&root, "crates/math/py/src/doors.rs")] {
        assert!(!text.contains("mrly::"), "{text}");
    }
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn outside_core_a_python_stream_crosses_by_its_state() {
    let math = scratch_leaf("streams", STREAM, &leaf_of("math", &["core", "math"]));
    let doors = read(&math, "crates/math/py/src/doors.rs");
    assert!(doors.contains("rng: &Bound<'_, PyAny>"), "{doors}");
    assert!(doors.contains("crate::hand::rng_from_py(rng)?"), "{doors}");
    assert!(
        doors.contains("crate::hand::rng_write_back(rng, &rng_owned)?"),
        "{doors}"
    );
    let core = scratch_leaf("own-stream", STREAM, &leaf_of("core", &["core"]));
    let doors = read(&core, "crates/core/py/src/doors.rs");
    assert!(doors.contains("rng: &mut PyRng"), "{doors}");
    assert!(doors.contains("m.add_class::<PyRng>()?;"), "{doors}");
    std::fs::remove_dir_all(math).ok();
    std::fs::remove_dir_all(core).ok();
}

#[test]
fn a_leaf_python_package_is_a_portion_of_the_mrly_namespace() {
    let root = scratch_leaf("namespace", STREAM, &leaf_of("math", &["core", "math"]));
    let init = read(&root, "crates/math/py/python/mrly/math/__init__.py");
    assert!(
        init.starts_with("from mrly.math._math.math import *\n"),
        "{init}"
    );
    assert!(!root.join("crates/math/py/python/mrly/__init__.py").exists());
    assert!(!root.join("crates/math/py/python/mrly/core").exists());
    let lib = read(&root, "crates/math/py/src/lib.rs");
    assert!(lib.contains("fn _math("), "{lib}");
    let project = read(&root, "crates/math/py/pyproject.toml");
    assert!(
        project.contains("module-name = \"mrly.math._math\""),
        "{project}"
    );
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_leaf_leaves_out_a_class_field_of_another_unit_in_python() {
    let lib = "pub mod core { pub struct Paint { pub n: u8 } impl Paint { pub fn ink(&self) -> u8 { 0 } } } pub mod tile { pub struct Variation { pub paint: crate::core::Paint, pub size: u8 } impl Variation { pub fn grow(&self) -> u8 { 0 } } }";
    let root = scratch_leaf("field", lib, &leaf_of("tile", &["core", "tile"]));
    let doors = read(&root, "crates/tile/py/src/doors.rs");
    assert!(doors.contains("#[pyo3(name = \"size\")]"), "{doors}");
    assert!(!doors.contains("#[pyo3(name = \"paint\")]"), "{doors}");
    std::fs::remove_dir_all(root).ok();
}

#[test]
fn a_leaf_homes_an_item_in_its_own_unit_when_the_re_exporter_is_out_of_reach() {
    let config = config::load(&config::home()).expect("the bridge config loads");
    let leaves = leaf::leaves(&config.mounts).expect("the leaves load");
    let math = leaves
        .iter()
        .find(|l| l.unit == "math")
        .expect("a math leaf");
    assert_eq!(math.seen, ["core", "math", "num"]);
    let m = leaf::manifest(&config.mounts, math, "0")
        .expect("the math leaf builds")
        .manifest;
    assert!(
        m.types
            .iter()
            .any(|t| t.path == "math::bang::catalog::Catalog")
    );
    assert!(m.types.iter().all(|t| !t.path.starts_with("tile::")));
}

#[test]
fn every_leaf_writes_the_same_twice() {
    let config = config::load(&config::home()).expect("the bridge config loads");
    let root = std::env::temp_dir().join(format!("bridge-leaves-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let built = leaf::build(&config.mounts, "0").expect("the leaves build");
    leaf::write(&built, &root).expect("the leaves write");
    disk::watch();
    leaf::write(&built, &root).expect("the leaves write again");
    assert_eq!(disk::stale(), Vec::<PathBuf>::new());
    std::fs::remove_dir_all(root).ok();
}
