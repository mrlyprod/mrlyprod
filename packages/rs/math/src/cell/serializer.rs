use super::models::dtype_for;
use mrly_core::Json;
use mrly_core::error::{Error, Result, value_error};
use mrly_core::tensor::Tensor;
use serde::Deserialize;

pub fn parse(text: &str) -> Result<Json> {
    mrly_core::error::parse(text)
}

pub fn types_field(data: &Json) -> Result<&Json> {
    data.get("types")
        .ok_or_else(|| Error::Value("missing types.".to_string()))
}

pub fn byte_grid(value: &Json) -> Result<Vec<Vec<u8>>> {
    Ok(Vec::deserialize(value)?)
}

pub fn byte_cube(value: &Json) -> Result<Vec<Vec<Vec<u8>>>> {
    Ok(Vec::deserialize(value)?)
}

pub fn count_grid(value: &Json) -> Result<Vec<i64>> {
    let rows: Vec<Vec<u32>> = Vec::deserialize(value)?;
    Ok(rows.concat().into_iter().map(i64::from).collect())
}

pub fn count_cube(value: &Json) -> Result<Vec<i64>> {
    let planes: Vec<Vec<Vec<u32>>> = Vec::deserialize(value)?;
    Ok(planes
        .concat()
        .concat()
        .into_iter()
        .map(i64::from)
        .collect())
}

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

pub fn color_grid(value: &Json) -> Result<Vec<Vec<[u8; 4]>>> {
    Ok(Vec::deserialize(value)?)
}
