use super::{CEILING, MIN};
use crate::core::paint::Paint;
use crate::core::tile::{Design, Group, Source, Tile as Model};
use crate::math::two::tile as tile2d;
use serde_json::Value as Json;

pub fn carpet() -> Model {
    let mut tile = Model::new(Group::Fractal).size(9, 9);
    tile.sources = vec![Source::Classic(Design::Carpet)];
    tile.numbers = vec![3];
    tile.levels = vec![2];
    tile.rotations = vec![0];
    tile.anti = vec![false];
    tile.factor = 3;
    tile
}

pub fn resize(tile: &mut Model) {
    if matches!(tile.group, Group::General | Group::Fractal | Group::Magic) {
        tile.factor = tile.numbers[0];
    }
    let size = match tile.group {
        Group::General => tile.numbers[0],
        Group::Fractal => tile.numbers[0].pow(tile.levels[0] as u32),
        Group::Magic => tile.numbers.iter().product(),
        Group::Special | Group::Mosaic => tile.factor * tile.numbers[0],
    };
    tile.width = size;
    tile.height = size;
}

pub fn check_model(model: &Model) -> Result<(), &'static str> {
    let slots = model.sources.len();
    let wanted = match model.group {
        Group::Mosaic => slots == 3,
        Group::Magic => (2..=6).contains(&slots),
        _ => slots == 1,
    };
    if !wanted {
        return Err("wrong slot count");
    }
    if model.numbers.len() != slots
        || model.levels.len() != slots
        || model.rotations.len() != slots
        || model.anti.len() != slots
    {
        return Err("ragged slots");
    }
    if model.numbers.iter().any(|&n| !(MIN..=CEILING).contains(&n)) {
        return Err("numbers are 2 to 64");
    }
    if model.rotations.iter().any(|&r| r > 3) {
        return Err("rotation is 0 to 3");
    }
    if model.flip && model.group != Group::Special {
        return Err("flip is special only");
    }
    if model.group == Group::Fractal {
        if !(1..=6).contains(&model.levels[0]) {
            return Err("level is 1 to 6");
        }
    } else if model.levels.iter().any(|&l| l != 1) {
        return Err("level is fractal only");
    }
    if matches!(model.group, Group::Special | Group::Mosaic)
        && !(MIN..=CEILING).contains(&model.factor)
    {
        return Err("factor is 2 to 64");
    }
    if model.group == Group::Mosaic && model.numbers.iter().any(|&n| n != model.numbers[0]) {
        return Err("mosaic shares one number");
    }
    let mut probe = model.clone();
    resize(&mut probe);
    if probe.width != model.width || probe.height != model.height || probe.factor != model.factor {
        return Err("sizes disagree");
    }
    if !(MIN..=CEILING).contains(&model.max_size()) {
        return Err("size is 2 to 64");
    }
    match tile2d::build(model) {
        Ok(cell) if cell.width() == model.width && cell.height() == model.height => Ok(()),
        _ => Err("tile does not build"),
    }
}

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
