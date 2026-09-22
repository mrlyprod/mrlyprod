#![allow(dead_code)]

use mrlyrs::core::cell::Cell;
use mrlyrs::core::colors::Color;
use mrlyrs::core::error::Error;
use mrlyrs::core::rng::Rng as Stream;
use mrlyrs::core::tensor::{Dtype, Tensor};
use mrlyrs::core::Json;
use mrlyrs::math::bang::Code;
use mrlyrs::math::cell::models::{Cell2d, Cell3d, CellNd};
use mrlyrs::math::six::{Cell6d, Orientation, Projection};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ERRORS

pub fn throw(error: Error) -> JsValue {
    js_sys::Error::new(&error.to_string()).into()
}

pub fn refuse(message: &str) -> JsValue {
    js_sys::Error::new(message).into()
}

// SERDE

fn serializer() -> serde_wasm_bindgen::Serializer {
    serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true)
}

pub fn to_js<T: Serialize + ?Sized>(value: &T) -> Result<JsValue, JsValue> {
    value
        .serialize(&serializer())
        .map_err(|error| refuse(&error.to_string()))
}

pub fn from_js<T: DeserializeOwned>(value: &JsValue) -> Result<T, JsValue> {
    serde_wasm_bindgen::from_value(value.clone()).map_err(|error| refuse(&error.to_string()))
}

pub fn json_to_js(value: &Json) -> Result<JsValue, JsValue> {
    to_js(value)
}

pub fn json_from_js(value: &JsValue) -> Result<Json, JsValue> {
    from_js(value)
}

pub fn plain(value: &JsValue) -> Result<JsValue, JsValue> {
    if !value.is_object() {
        return Ok(value.clone());
    }
    match field(value, "toJSON")?.dyn_ref::<js_sys::Function>() {
        Some(to_json) => to_json.call0(value),
        None => Ok(value.clone()),
    }
}

// FIELDS

fn field(value: &JsValue, name: &str) -> Result<JsValue, JsValue> {
    js_sys::Reflect::get(value, &JsValue::from_str(name))
}

fn put(object: &js_sys::Object, name: &str, value: JsValue) -> Result<(), JsValue> {
    js_sys::Reflect::set(object, &JsValue::from_str(name), &value)?;
    Ok(())
}

fn drop_field(object: &js_sys::Object, name: &str) -> Result<(), JsValue> {
    js_sys::Reflect::delete_property(object, &JsValue::from_str(name))?;
    Ok(())
}

fn object(value: &JsValue) -> Result<&js_sys::Object, JsValue> {
    value
        .dyn_ref::<js_sys::Object>()
        .ok_or_else(|| refuse("an object was wanted here."))
}

fn missing(value: &JsValue) -> bool {
    value.is_undefined() || value.is_null()
}

// SCALARS

pub fn number_from_js(value: &JsValue) -> Result<f64, JsValue> {
    value
        .as_f64()
        .ok_or_else(|| refuse("a number was wanted here."))
}

pub fn bool_from_js(value: &JsValue) -> Result<bool, JsValue> {
    value
        .as_bool()
        .ok_or_else(|| refuse("a boolean was wanted here."))
}

pub fn string_from_js(value: &JsValue) -> Result<String, JsValue> {
    value
        .as_string()
        .ok_or_else(|| refuse("a string was wanted here."))
}

pub fn char_from_js(value: &JsValue) -> Result<char, JsValue> {
    let text = string_from_js(value)?;
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(c), None) => Ok(c),
        _ => Err(refuse("a one-character string was wanted here.")),
    }
}

fn decimal(value: &JsValue) -> Option<String> {
    if let Some(text) = value.as_string() {
        return Some(text.trim().to_string());
    }
    if value.is_bigint() {
        return value
            .unchecked_ref::<js_sys::BigInt>()
            .to_string(10)
            .ok()
            .map(String::from);
    }
    let number = value.as_f64()?;
    if number.is_finite() && number.fract() == 0.0 {
        Some(format!("{number:.0}"))
    } else {
        None
    }
}

fn wide_from_js<T: FromStr>(value: &JsValue, name: &str) -> Result<T, JsValue> {
    decimal(value)
        .and_then(|text| text.parse().ok())
        .ok_or_else(|| refuse(&format!("{name} wants a decimal string, a whole number or a bigint.")))
}

pub fn u128_to_js(value: u128) -> String {
    value.to_string()
}

pub fn u128_from_js(value: &JsValue) -> Result<u128, JsValue> {
    wide_from_js(value, "a u128")
}

pub fn i128_from_js(value: &JsValue) -> Result<i128, JsValue> {
    wide_from_js(value, "an i128")
}

pub fn u64_from_js(value: &JsValue) -> Result<u64, JsValue> {
    wide_from_js(value, "a u64")
}

pub fn i64_from_js(value: &JsValue) -> Result<i64, JsValue> {
    wide_from_js(value, "an i64")
}

pub fn code_to_js(code: Code) -> String {
    code.get().to_string()
}

pub fn code_from_js(value: &JsValue) -> Result<Code, JsValue> {
    Ok(Code::from(u128_from_js(value)?))
}

// LISTS

pub fn list_from_js<T>(
    value: &JsValue,
    item: impl Fn(&JsValue) -> Result<T, JsValue>,
) -> Result<Vec<T>, JsValue> {
    if let Some(array) = value.dyn_ref::<js_sys::Array>() {
        return array.iter().map(|x| item(&x)).collect();
    }
    match js_sys::try_iter(value)? {
        Some(iter) => iter.map(|x| item(&x?)).collect(),
        None => Err(refuse("a list was wanted here.")),
    }
}

pub fn list_to_js<T>(
    items: impl IntoIterator<Item = T>,
    item: impl Fn(T) -> Result<JsValue, JsValue>,
) -> Result<JsValue, JsValue> {
    let out = js_sys::Array::new();
    for x in items {
        out.push(&item(x)?);
    }
    Ok(out.into())
}

pub fn option_from_js<T>(
    value: &JsValue,
    item: impl Fn(&JsValue) -> Result<T, JsValue>,
) -> Result<Option<T>, JsValue> {
    if missing(value) {
        Ok(None)
    } else {
        item(value).map(Some)
    }
}

pub fn option_to_js<T>(
    value: Option<T>,
    item: impl Fn(T) -> Result<JsValue, JsValue>,
) -> Result<JsValue, JsValue> {
    match value {
        Some(x) => item(x),
        None => Ok(JsValue::UNDEFINED),
    }
}

pub fn item(value: &JsValue, index: u32) -> Result<JsValue, JsValue> {
    let array = value
        .dyn_ref::<js_sys::Array>()
        .ok_or_else(|| refuse("an array was wanted here."))?;
    if index >= array.length() {
        return Err(refuse(&format!("an array of at least {} was wanted here.", index + 1)));
    }
    Ok(array.get(index))
}

pub fn first(value: &JsValue) -> Result<JsValue, JsValue> {
    item(value, 0)
}

pub fn array_from_js<T, const N: usize>(
    value: &JsValue,
    item: impl Fn(&JsValue) -> Result<T, JsValue>,
) -> Result<[T; N], JsValue> {
    list_from_js(value, item)?
        .try_into()
        .map_err(|_| refuse(&format!("a list of {N} was wanted here.")))
}

pub fn tuple_to_js(items: &[JsValue]) -> JsValue {
    items.iter().collect::<js_sys::Array>().into()
}

pub fn map_from_js<K: FromStr + Eq + Hash, V>(
    value: &JsValue,
    each: impl Fn(&JsValue) -> Result<V, JsValue>,
) -> Result<HashMap<K, V>, JsValue> {
    let mut out = HashMap::new();
    for entry in js_sys::Object::entries(object(value)?).iter() {
        let key = string_from_js(&item(&entry, 0)?)?;
        let key = key
            .parse()
            .map_err(|_| refuse(&format!("the key {key:?} does not read.")))?;
        out.insert(key, each(&item(&entry, 1)?)?);
    }
    Ok(out)
}

pub fn map_to_js<'a, K: Display + 'a, V: 'a>(
    entries: impl IntoIterator<Item = (&'a K, &'a V)>,
    item: impl Fn(&V) -> Result<JsValue, JsValue>,
) -> Result<JsValue, JsValue> {
    let out = js_sys::Object::new();
    for (key, value) in entries {
        put(&out, &key.to_string(), item(value)?)?;
    }
    Ok(out.into())
}

pub trait Typed {
    fn typed(&self) -> JsValue;
}

macro_rules! typed_arrays {
    ($($scalar:ty => $array:ident),* $(,)?) => {
        $(
            impl Typed for [$scalar] {
                fn typed(&self) -> JsValue {
                    js_sys::$array::from(self).into()
                }
            }
        )*
    };
}

typed_arrays! {
    u8 => Uint8Array,
    u16 => Uint16Array,
    u32 => Uint32Array,
    i8 => Int8Array,
    i16 => Int16Array,
    i32 => Int32Array,
    f32 => Float32Array,
    f64 => Float64Array,
    u64 => BigUint64Array,
    i64 => BigInt64Array,
}

impl Typed for [usize] {
    fn typed(&self) -> JsValue {
        let narrow: Vec<u32> = self.iter().map(|&n| n as u32).collect();
        js_sys::Uint32Array::from(&narrow[..]).into()
    }
}

pub fn typed<T: Typed + ?Sized>(items: &T) -> JsValue {
    items.typed()
}

// SHAPES

pub fn shape_to_js(shape: &[usize]) -> JsValue {
    shape
        .iter()
        .map(|&length| JsValue::from_f64(length as f64))
        .collect::<js_sys::Array>()
        .into()
}

pub fn shape_from_js(value: &JsValue) -> Result<Vec<usize>, JsValue> {
    let array = value
        .dyn_ref::<js_sys::Array>()
        .ok_or_else(|| refuse("a shape wants an array of lengths."))?;
    array
        .iter()
        .map(|length| {
            length
                .as_f64()
                .filter(|n| n.is_finite() && *n >= 0.0)
                .map(|n| n as usize)
                .ok_or_else(|| refuse("a shape wants whole lengths at or above zero."))
        })
        .collect()
}

// BUFFERS

pub fn bytes_to_js(bytes: &[u8]) -> JsValue {
    js_sys::Uint8Array::from(bytes).into()
}

pub fn bytes_from_js(value: &JsValue) -> Result<Vec<u8>, JsValue> {
    if let Some(array) = value.dyn_ref::<js_sys::Uint8Array>() {
        return Ok(array.to_vec());
    }
    if let Some(array) = value.dyn_ref::<js_sys::Uint8ClampedArray>() {
        return Ok(array.to_vec());
    }
    if let Some(array) = value.dyn_ref::<js_sys::Array>() {
        return array
            .iter()
            .map(|item| {
                item.as_f64()
                    .filter(|n| (0.0..=255.0).contains(n))
                    .map(|n| n as u8)
                    .ok_or_else(|| refuse("a byte array wants numbers from zero to 255."))
            })
            .collect();
    }
    Err(refuse("a Uint8Array or an array of bytes was wanted here."))
}

fn data_to_js(tensor: &Tensor) -> Result<JsValue, JsValue> {
    let data = match tensor.dtype() {
        Dtype::U8 => js_sys::Uint8Array::from(tensor.bytes().map_err(throw)?).into(),
        Dtype::U16 => js_sys::Uint16Array::from(tensor.u16s().map_err(throw)?).into(),
        Dtype::U32 => js_sys::Uint32Array::from(tensor.u32s().map_err(throw)?).into(),
        Dtype::I32 => js_sys::Int32Array::from(tensor.i32s().map_err(throw)?).into(),
    };
    Ok(data)
}

fn data_from_js(value: &JsValue, shape: Vec<usize>) -> Result<Tensor, JsValue> {
    if let Some(array) = value.dyn_ref::<js_sys::Uint16Array>() {
        return Tensor::u16(array.to_vec(), shape).map_err(throw);
    }
    if let Some(array) = value.dyn_ref::<js_sys::Uint32Array>() {
        return Tensor::u32(array.to_vec(), shape).map_err(throw);
    }
    if let Some(array) = value.dyn_ref::<js_sys::Int32Array>() {
        return Tensor::i32(array.to_vec(), shape).map_err(throw);
    }
    Tensor::u8(bytes_from_js(value)?, shape).map_err(throw)
}

// TENSORS

fn fill_tensor(out: &js_sys::Object, tensor: &Tensor) -> Result<(), JsValue> {
    put(out, "shape", shape_to_js(&tensor.shape))?;
    put(out, "data", data_to_js(tensor)?)
}

pub fn tensor_to_js(tensor: &Tensor) -> Result<JsValue, JsValue> {
    let out = js_sys::Object::new();
    fill_tensor(&out, tensor)?;
    Ok(out.into())
}

pub fn tensor_into_js(value: &JsValue, tensor: &Tensor) -> Result<(), JsValue> {
    fill_tensor(object(value)?, tensor)
}

pub fn tensor_from_js(value: &JsValue) -> Result<Tensor, JsValue> {
    let shape = shape_from_js(&field(value, "shape")?)?;
    data_from_js(&field(value, "data")?, shape)
}

// CELLS

fn fill_cell(out: &js_sys::Object, cell: &Cell) -> Result<(), JsValue> {
    put(out, "shape", shape_to_js(&cell.types.shape))?;
    put(out, "types", data_to_js(&cell.types)?)?;
    match &cell.colors {
        Some(colors) => {
            let flat: Vec<u8> = colors.iter().flatten().copied().collect();
            put(out, "colors", bytes_to_js(&flat))?;
        }
        None => drop_field(out, "colors")?,
    }
    match &cell.tags {
        Some(tags) => put(out, "tags", data_to_js(tags)?),
        None => drop_field(out, "tags"),
    }
}

pub fn cell_to_js(cell: &Cell) -> Result<JsValue, JsValue> {
    let out = js_sys::Object::new();
    fill_cell(&out, cell)?;
    Ok(out.into())
}

pub fn cell_into_js(value: &JsValue, cell: &Cell) -> Result<(), JsValue> {
    fill_cell(object(value)?, cell)
}

pub fn cell_from_js(value: &JsValue) -> Result<Cell, JsValue> {
    let shape = shape_from_js(&field(value, "shape")?)?;
    let types = data_from_js(&field(value, "types")?, shape.clone())?;
    let painted = field(value, "colors")?;
    let colors = if missing(&painted) {
        None
    } else {
        let flat = bytes_from_js(&painted)?;
        if flat.len() != types.size() * 4 {
            return Err(refuse("cell colors want four bytes a cell."));
        }
        Some(
            flat.chunks_exact(4)
                .map(|rgba| [rgba[0], rgba[1], rgba[2], rgba[3]])
                .collect(),
        )
    };
    let tagged = field(value, "tags")?;
    let tags = if missing(&tagged) {
        None
    } else {
        Some(data_from_js(&tagged, shape)?)
    };
    Ok(Cell {
        types,
        colors,
        tags,
    })
}

pub fn cell_rank(value: &JsValue) -> Result<usize, JsValue> {
    Ok(shape_from_js(&field(value, "shape")?)?.len())
}

fn cell_nd_from_js<const N: usize>(value: &JsValue) -> Result<CellNd<N>, JsValue> {
    let cell = cell_from_js(value)?;
    if cell.types.shape.len() != N {
        return Err(refuse(&format!(
            "a {N}d cell wants a shape of {N} lengths."
        )));
    }
    Ok(CellNd { cell })
}

pub fn cell2d_to_js(cell: &Cell2d) -> Result<JsValue, JsValue> {
    cell_to_js(&cell.cell)
}

pub fn cell2d_from_js(value: &JsValue) -> Result<Cell2d, JsValue> {
    cell_nd_from_js(value)
}

pub fn cell3d_to_js(cell: &Cell3d) -> Result<JsValue, JsValue> {
    cell_to_js(&cell.cell)
}

pub fn cell3d_from_js(value: &JsValue) -> Result<Cell3d, JsValue> {
    cell_nd_from_js(value)
}

pub fn cell6d_to_js(hex: &Cell6d) -> Result<JsValue, JsValue> {
    let out = js_sys::Object::new();
    put(&out, "cell", cell2d_to_js(&hex.cell)?)?;
    put(&out, "projection", to_js(&hex.projection)?)?;
    put(&out, "orientation", to_js(&hex.orientation)?)?;
    put(&out, "start", JsValue::from_f64(hex.start as f64))?;
    Ok(out.into())
}

pub fn cell6d_from_js(value: &JsValue) -> Result<Cell6d, JsValue> {
    let cell = cell2d_from_js(&field(value, "cell")?)?;
    let projection: Projection = from_js(&field(value, "projection")?)?;
    let orientation: Orientation = from_js(&field(value, "orientation")?)?;
    let start = number_from_js(&field(value, "start")?)? as u8;
    Ok(Cell6d::new(cell, projection, orientation, start))
}

// COLORS

pub fn color_to_js(color: Color) -> JsValue {
    [color.r, color.g, color.b, color.a]
        .iter()
        .map(|&channel| JsValue::from_f64(channel as f64))
        .collect::<js_sys::Array>()
        .into()
}

pub fn color_from_js(value: &JsValue) -> Result<Color, JsValue> {
    let channels = bytes_from_js(value)?;
    match channels.len() {
        3 => Ok(Color::rgb(channels[0], channels[1], channels[2])),
        4 => Ok(Color::rgba(
            channels[0],
            channels[1],
            channels[2],
            channels[3],
        )),
        _ => Err(refuse("a color wants three or four bytes.")),
    }
}

pub fn colors_to_js(colors: &[Color]) -> JsValue {
    colors
        .iter()
        .map(|&color| color_to_js(color))
        .collect::<js_sys::Array>()
        .into()
}

pub fn colors_from_js(value: &JsValue) -> Result<Vec<Color>, JsValue> {
    let array = value
        .dyn_ref::<js_sys::Array>()
        .ok_or_else(|| refuse("a palette wants an array of colors."))?;
    array.iter().map(|color| color_from_js(&color)).collect()
}

// CHANCE

/// A seeded random stream, opened from a number or a bigint seed.
#[wasm_bindgen]
pub struct Rng {
    stream: Stream,
}

#[wasm_bindgen]
impl Rng {
    /// Opens the stream on the seed.
    #[wasm_bindgen(constructor)]
    pub fn new(seed: JsValue) -> Result<Rng, JsValue> {
        Ok(Rng {
            stream: Stream::new(u64_from_js(&seed)?),
        })
    }
    /// Draws a float at or above zero and below one.
    pub fn unit(&mut self) -> f64 {
        self.stream.unit()
    }
    /// Draws an integer below n, or zero when n is zero.
    pub fn below(&mut self, n: usize) -> usize {
        self.stream.below(n)
    }
    /// Draws an integer between lo and hi inclusive, or lo when hi is not above lo.
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        self.stream.range(lo as i64, hi as i64) as f64
    }
    /// Draws a fair coin flip.
    pub fn boolean(&mut self) -> bool {
        self.stream.boolean()
    }
    /// Returns true with probability p.
    pub fn chance(&mut self, p: f64) -> bool {
        self.stream.chance(p)
    }
    /// Draws amount distinct indices below length, or every index when amount is larger.
    pub fn sample_indices(&mut self, length: usize, amount: usize) -> Vec<u32> {
        self.stream
            .sample_indices(length, amount)
            .into_iter()
            .map(|index| index as u32)
            .collect()
    }
    /// Prints the stream state, the way an optional stream crosses in.
    #[wasm_bindgen(js_name = "__state")]
    pub fn state(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.stream).map_err(|error| refuse(&error.to_string()))
    }
    /// Reads a stream state back, the way an optional stream crosses out.
    #[wasm_bindgen(js_name = "__restore")]
    pub fn restore(&mut self, state: &str) -> Result<(), JsValue> {
        self.stream = serde_json::from_str(state).map_err(|error| refuse(&error.to_string()))?;
        Ok(())
    }
}

impl Rng {
    pub fn wrap(stream: Stream) -> Rng {
        Rng { stream }
    }
    pub fn stream(&mut self) -> &mut Stream {
        &mut self.stream
    }
}

fn method(value: &JsValue, name: &str) -> Result<js_sys::Function, JsValue> {
    field(value, name)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| refuse("an Rng was wanted here."))
}

pub fn stream_from_js(value: &JsValue) -> Result<Option<Stream>, JsValue> {
    if missing(value) {
        return Ok(None);
    }
    let state = method(value, "__state")?.call0(value)?;
    serde_json::from_str(&string_from_js(&state)?)
        .map(Some)
        .map_err(|error| refuse(&error.to_string()))
}

pub fn stream_to_js(value: &JsValue, stream: &Stream) -> Result<(), JsValue> {
    let state = serde_json::to_string(stream).map_err(|error| refuse(&error.to_string()))?;
    method(value, "__restore")?.call1(value, &JsValue::from_str(&state))?;
    Ok(())
}
