use super::models::dtype_for;
use mrlycore::errors::{value_error, MrlyError, Result};
use mrlycore::tensor::Tensor;
use mrlycore::Json;

/// Parses JSON text into a value tree, or a parse error.
pub fn parse(text: &str) -> Result<Json> {
    mrlycore::json::parse(text)
}

/// Returns the types field of the data, or an error when it is missing.
pub fn types_field(data: &Json) -> Result<&Json> {
    data.get("types")
        .ok_or_else(|| MrlyError::Value("missing types.".to_string()))
}

fn items(value: &Json) -> Result<&Vec<Json>> {
    value
        .as_array()
        .ok_or_else(|| MrlyError::Value("expected an array.".to_string()))
}

fn byte(value: &Json) -> Result<u8> {
    value
        .as_i64()
        .and_then(|n| u8::try_from(n).ok())
        .ok_or_else(|| MrlyError::Value("expected a byte.".to_string()))
}

fn bytes(value: &Json) -> Result<Vec<u8>> {
    items(value)?.iter().map(byte).collect()
}

fn color(value: &Json) -> Result<[u8; 4]> {
    <[u8; 4]>::try_from(bytes(value)?)
        .map_err(|_| MrlyError::Value("expected four channels.".to_string()))
}

fn colors(value: &Json) -> Result<Vec<[u8; 4]>> {
    items(value)?.iter().map(color).collect()
}

/// Reads a nested JSON array into rows of bytes.
pub fn byte_grid(value: &Json) -> Result<Vec<Vec<u8>>> {
    items(value)?.iter().map(bytes).collect()
}

/// Reads a triply nested JSON array into layers of byte rows.
pub fn byte_cube(value: &Json) -> Result<Vec<Vec<Vec<u8>>>> {
    items(value)?.iter().map(byte_grid).collect()
}

fn count(value: &Json) -> Result<i64> {
    match value.as_i64() {
        Some(n) if n >= 0 => Ok(n),
        _ => Err(MrlyError::Value("expected a count.".to_string())),
    }
}

fn counts(value: &Json) -> Result<Vec<i64>> {
    items(value)?.iter().map(count).collect()
}

/// Reads a nested JSON array of counts into one flat run, however wide the counts read.
pub fn count_grid(value: &Json) -> Result<Vec<i64>> {
    Ok(items(value)?
        .iter()
        .map(counts)
        .collect::<Result<Vec<Vec<i64>>>>()?
        .concat())
}

/// Reads a triply nested JSON array of counts into one flat run, however wide the counts read.
pub fn count_cube(value: &Json) -> Result<Vec<i64>> {
    Ok(items(value)?
        .iter()
        .map(count_grid)
        .collect::<Result<Vec<Vec<i64>>>>()?
        .concat())
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
    items(value)?.iter().map(colors).collect()
}
