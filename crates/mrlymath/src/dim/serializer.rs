use super::models::dtype_for;
use mrlycore::errors::{value_error, MrlyError, Result};
use mrlycore::tensor::Tensor;
use mrlycore::Json;
use serde::Deserialize;

/// Parses JSON text into a value tree, or a parse error.
pub fn parse(text: &str) -> Result<Json> {
    mrlycore::json::parse(text)
}

/// Returns the types field of the data, or an error when it is missing.
pub fn types_field(data: &Json) -> Result<&Json> {
    data.get("types")
        .ok_or_else(|| MrlyError::Value("missing types.".to_string()))
}

/// Reads a nested JSON array into rows of bytes.
pub fn byte_grid(value: &Json) -> Result<Vec<Vec<u8>>> {
    Ok(Vec::deserialize(value)?)
}

/// Reads a triply nested JSON array into layers of byte rows.
pub fn byte_cube(value: &Json) -> Result<Vec<Vec<Vec<u8>>>> {
    Ok(Vec::deserialize(value)?)
}

/// Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
pub fn count_grid(value: &Json) -> Result<Vec<i64>> {
    let rows: Vec<Vec<u32>> = Vec::deserialize(value)?;
    Ok(rows.concat().into_iter().map(i64::from).collect())
}

/// Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits.
pub fn count_cube(value: &Json) -> Result<Vec<i64>> {
    let planes: Vec<Vec<Vec<u32>>> = Vec::deserialize(value)?;
    Ok(planes
        .concat()
        .concat()
        .into_iter()
        .map(i64::from)
        .collect())
}

/// Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them.
pub fn tag_layer(counts: &[i64], shape: Vec<usize>) -> Result<Tensor> {
    if counts.len() != shape.iter().product::<usize>() {
        return value_error("tags must match the cell's shape.");
    }
    let peak = counts.iter().copied().max().unwrap_or(0);
    let mut tags = Tensor::typed(shape, dtype_for(peak));
    for (flat, &value) in counts.iter().enumerate() {
        tags.put(flat, value);
    }
    Ok(tags)
}

/// Reads a nested JSON array into rows of four-channel colors.
pub fn color_grid(value: &Json) -> Result<Vec<Vec<[u8; 4]>>> {
    Ok(Vec::deserialize(value)?)
}
