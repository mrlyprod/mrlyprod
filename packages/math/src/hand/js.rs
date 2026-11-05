use crate::bang::Code;
use crate::cell::models::{Cell2d, Cell3d, CellNd};
use crate::six::{Cell6d, Orientation, Projection};
pub use mrly_core::hand::js::*;
use wasm_bindgen::prelude::*;

pub fn code_to_js(code: Code) -> String {
    code.get().to_string()
}

pub fn code_from_js(value: &JsValue) -> Result<Code, JsValue> {
    Ok(Code::from(u128_from_js(value)?))
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
