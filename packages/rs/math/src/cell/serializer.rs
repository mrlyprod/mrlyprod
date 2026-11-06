use super::models::dtype_for;
use mrly_error::{Error, Result, value_error};
use mrly_json::Json;
use mrly_tensor::Tensor;

pub fn parse(text: &str) -> Result<Json> {
    mrly_json::parse(text)
}

pub fn types_field(data: &Json) -> Result<&Json> {
    data.get("types")
        .ok_or_else(|| Error::Value("missing types.".to_string()))
}

pub fn byte_grid(value: &Json) -> Result<Vec<Vec<u8>>> {
    mrly_json::take(value)
}

pub fn byte_cube(value: &Json) -> Result<Vec<Vec<Vec<u8>>>> {
    mrly_json::take(value)
}

pub fn count_grid(value: &Json) -> Result<Vec<i64>> {
    let rows: Vec<Vec<u32>> = mrly_json::take(value)?;
    Ok(rows.concat().into_iter().map(i64::from).collect())
}

pub fn count_cube(value: &Json) -> Result<Vec<i64>> {
    let planes: Vec<Vec<Vec<u32>>> = mrly_json::take(value)?;
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
    mrly_json::take(value)
}
