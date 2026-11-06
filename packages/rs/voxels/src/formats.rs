use mrly_error::{Result, value_error};
use mrly_pixels::Grid;
use mrly_tensor::Tensor;
use mrly_three::Cell3d;

pub fn cell(grid: &Grid) -> Result<Cell3d> {
    let (wide, deep, tall) = (grid.width, grid.height, grid.frames.len());
    if wide == 0 || deep == 0 || tall == 0 {
        return value_error("a voxel grid needs at least one cell.");
    }
    let mut types = vec![0u8; wide * deep * tall];
    let mut colors = vec![[0u8; 4]; types.len()];
    for (y, layer) in grid.frames.iter().enumerate() {
        let inks = grid.colors(y);
        for z in 0..deep {
            for x in 0..wide {
                let to = (x * tall + y) * deep + z;
                types[to] = layer[z * wide + x];
                colors[to] = inks[z * wide + x];
            }
        }
    }
    let mut cell = Cell3d::new(Tensor::of(types, vec![wide, tall, deep])?)?;
    cell.cell.colors = Some(colors);
    Ok(cell)
}

pub fn obj(cell: &Cell3d) -> String {
    mrly_three::to_obj(cell)
}
