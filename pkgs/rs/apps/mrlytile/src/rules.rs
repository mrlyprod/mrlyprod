use mrlycore::paint::Paint;
use mrlycore::tile::{Design, Group, Source, Tile as Model};
use mrlycore::Json;
use mrlymath::two::tile as tile2d;

/// Builds the carpet tile the studio opens on.
pub fn carpet() -> Model {
    starter(Design::Carpet)
}

/// Builds one classic design as a two-level fractal of threes.
pub fn starter(design: Design) -> Model {
    let mut tile = Model::new(Group::Fractal);
    tile.sources = vec![Source::Classic(design)];
    tile.numbers = vec![3];
    tile.levels = vec![2];
    tile.rotations = vec![0];
    tile.anti = vec![false];
    tile.resize();
    tile
}

/// Checks the model laws and that the tile really builds at its declared size.
pub fn check_model(model: &Model) -> Result<(), &'static str> {
    model.check()?;
    match tile2d::build(model) {
        Ok(cell) if cell.width() == model.width && cell.height() == model.height => Ok(()),
        _ => Err("tile does not build"),
    }
}

/// Reads a saved tile and its paint back, refusing anything that would not build.
pub fn validate_saved(value: &Json) -> Result<(Model, Option<Paint>), &'static str> {
    if !value.is_object() {
        return Err("saved tile must be an object");
    }
    let model = Model::from_json(&value["tile"]).map_err(|_| "bad tile")?;
    check_model(&model)?;
    let coating = match &value["paint"] {
        Json::Null => None,
        given => Some(Paint::from_json(given).map_err(|_| "bad paint")?),
    };
    Ok((model, coating))
}
