use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "tile.Group.all",
        signature: "() -> [tile.Group; 5]",
        docs: "",
        uncallable: None,
        door: Some(door_tile_group_all),
    },
    Function {
        name: "tile.Parity.all",
        signature: "() -> [tile.Parity; 3]",
        docs: "",
        uncallable: None,
        door: Some(door_tile_parity_all),
    },
    Function {
        name: "tile.Parity.keep",
        signature: "(self: tile.Parity, n: usize) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_tile_parity_keep),
    },
    Function {
        name: "tile.Tile.check",
        signature: "(self: tile.Tile) -> null",
        docs: "",
        uncallable: None,
        door: Some(door_tile_tile_check),
    },
    Function {
        name: "tile.Tile.degenerate",
        signature: "(self: tile.Tile) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_tile_tile_degenerate),
    },
    Function {
        name: "tile.Tile.max_size",
        signature: "(self: tile.Tile) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_tile_tile_max_size),
    },
    Function {
        name: "tile.Tile.new",
        signature: "(group: tile.Group) -> tile.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_tile_new),
    },
    Function {
        name: "tile.Tile.resize",
        signature: "(self: tile.Tile) -> null",
        docs: "",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "tile.Tile.size",
        signature: "(self: tile.Tile, width: usize, height: usize) -> tile.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_tile_size),
    },
    Function {
        name: "tile.background",
        signature: "(seed: u64, width: usize, height: usize) -> Image",
        docs: "",
        uncallable: None,
        door: Some(door_tile_background),
    },
    Function {
        name: "tile.build.Config2d.default",
        signature: "() -> tile.draw.ConfigNd",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_config2d_default),
    },
    Function {
        name: "tile.build.Config3d.default",
        signature: "() -> tile.draw.ConfigNd",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_config3d_default),
    },
    Function {
        name: "tile.build.build_2d",
        signature: "(tile: tile.Tile) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_build_2d),
    },
    Function {
        name: "tile.build.build_3d",
        signature: "(tile: tile.Tile) -> Cell3d",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_build_3d),
    },
    Function {
        name: "tile.build.build_6d",
        signature: "(hex: tile.build.HexTile) -> Cell6d",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_build_6d),
    },
    Function {
        name: "tile.build.create_2d",
        signature: "(config: tile.draw.ConfigNd, rng: Rng(seed)) -> tile.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_create_2d),
    },
    Function {
        name: "tile.build.create_3d",
        signature: "(config: tile.draw.ConfigNd, rng: Rng(seed)) -> tile.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_create_3d),
    },
    Function {
        name: "tile.build.create_6d",
        signature: "(config: tile.draw.ConfigNd, rng: Rng(seed)) -> tile.build.HexTile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_create_6d),
    },
    Function {
        name: "tile.build.random_tile_2d",
        signature: "(max_size: usize, rng: Rng(seed)) -> tile.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_random_tile_2d),
    },
    Function {
        name: "tile.build.random_tile_3d",
        signature: "(max_size: usize, rng: Rng(seed)) -> tile.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_random_tile_3d),
    },
    Function {
        name: "tile.build.random_tile_6d",
        signature: "(max_size: usize, rng: Rng(seed)) -> tile.build.HexTile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_build_random_tile_6d),
    },
    Function {
        name: "tile.classic_code",
        signature: "(design: tile.recipe.Design) -> u128?",
        docs: "",
        uncallable: None,
        door: Some(door_tile_classic_code),
    },
    Function {
        name: "tile.classic_code_nd",
        signature: "(design: tile.recipe.Design, dimension: usize) -> u128?",
        docs: "",
        uncallable: None,
        door: Some(door_tile_classic_code_nd),
    },
    Function {
        name: "tile.hex_key",
        signature: "(length: usize, rng: Rng(seed)) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_tile_hex_key),
    },
    Function {
        name: "tile.name.Slots.default",
        signature: "() -> tile.name.Slots",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_slots_default),
    },
    Function {
        name: "tile.name.Tile.checked",
        signature: "(self: tile.name.Tile) -> tile.name.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_checked),
    },
    Function {
        name: "tile.name.Tile.from_file",
        signature: "(text: str) -> tile.name.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_from_file),
    },
    Function {
        name: "tile.name.Tile.from_json",
        signature: "(text: str) -> tile.name.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_from_json),
    },
    Function {
        name: "tile.name.Tile.from_url",
        signature: "(text: str) -> tile.name.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_from_url),
    },
    Function {
        name: "tile.name.Tile.of",
        signature: "(recipe: tile.Tile) -> tile.name.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_of),
    },
    Function {
        name: "tile.name.Tile.recipe",
        signature: "(self: tile.name.Tile) -> tile.Tile",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_recipe),
    },
    Function {
        name: "tile.name.Tile.to_file",
        signature: "(self: tile.name.Tile) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_to_file),
    },
    Function {
        name: "tile.name.Tile.to_id",
        signature: "(self: tile.name.Tile) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_to_id),
    },
    Function {
        name: "tile.name.Tile.to_json",
        signature: "(self: tile.name.Tile) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_to_json),
    },
    Function {
        name: "tile.name.Tile.to_prose",
        signature: "(self: tile.name.Tile) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_to_prose),
    },
    Function {
        name: "tile.name.Tile.to_url",
        signature: "(self: tile.name.Tile) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_tile_name_tile_to_url),
    },
    Function {
        name: "tile.random_design",
        signature: "(rng: Rng(seed)) -> tile.recipe.Design",
        docs: "",
        uncallable: None,
        door: Some(door_tile_random_design),
    },
    Function {
        name: "tile.random_rotation",
        signature: "(design: tile.recipe.Design, rng: Rng(seed)) -> u8",
        docs: "",
        uncallable: None,
        door: Some(door_tile_random_rotation),
    },
    Function {
        name: "tile.recipe.Design.all",
        signature: "() -> [tile.recipe.Design; 16]",
        docs: "",
        uncallable: None,
        door: Some(door_tile_recipe_design_all),
    },
    Function {
        name: "tile.recipe.classics",
        signature: "(dimension: usize) -> [tile.recipe.Design]",
        docs: "",
        uncallable: None,
        door: Some(door_tile_recipe_classics),
    },
    Function {
        name: "tile.recipe.generals",
        signature: "(min_size: usize, max_size: usize, parity: tile.Parity) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_tile_recipe_generals),
    },
    Function {
        name: "tile.recipe.nestings",
        signature: "(min_size: usize, max_size: usize, parity: tile.Parity) -> [[usize]]",
        docs: "",
        uncallable: None,
        door: Some(door_tile_recipe_nestings),
    },
    Function {
        name: "tile.recipe.powers",
        signature: "(min_size: usize, max_size: usize, parity: tile.Parity) -> [(usize, usize)]",
        docs: "",
        uncallable: None,
        door: Some(door_tile_recipe_powers),
    },
    Function {
        name: "tile.recipe.products",
        signature: "(min_size: usize, max_size: usize, count: usize, parity: tile.Parity) -> [[usize]]",
        docs: "",
        uncallable: None,
        door: Some(door_tile_recipe_products),
    },
    Function {
        name: "tile.recipe.size",
        signature: "(number: i64, level: i64) -> usize?",
        docs: "",
        uncallable: None,
        door: Some(door_tile_recipe_size),
    },
    Function {
        name: "tile.tree_mask",
        signature: "(n: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_tile_tree_mask),
    },
    Function {
        name: "tile.variation.Config.default",
        signature: "() -> tile.variation.Config",
        docs: "",
        uncallable: None,
        door: Some(door_tile_variation_config_default),
    },
    Function {
        name: "tile.variation.File.new",
        signature: "(width: usize, height: usize) -> tile.variation.File",
        docs: "",
        uncallable: None,
        door: Some(door_tile_variation_file_new),
    },
    Function {
        name: "tile.variation.Variation.is_cover",
        signature: "(self: tile.variation.Variation) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_tile_variation_variation_is_cover),
    },
    Function {
        name: "tile.variation.Variation.is_prime",
        signature: "(self: tile.variation.Variation) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_tile_variation_variation_is_prime),
    },
    Function {
        name: "tile.variation.create",
        signature: "(config: tile.variation.Config, rng: Rng(seed)) -> tile.variation.Variation",
        docs: "",
        uncallable: None,
        door: Some(door_tile_variation_create),
    },
    Function {
        name: "tile.variation.generate",
        signature: "(variation: tile.variation.Variation, config: tile.variation.Config, rng: Rng(seed)) -> tile.variation.Variation",
        docs: "",
        uncallable: None,
        door: Some(door_tile_variation_generate),
    },
    Function {
        name: "tile.variation.render",
        signature: "(variation: tile.variation.Variation, scale: usize, rng: Rng(seed)) -> tile.variation.Variation",
        docs: "",
        uncallable: None,
        door: Some(door_tile_variation_render),
    },
];

fn door_tile_group_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_tile::Group::all()))
}

fn door_tile_parity_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_tile::Parity::all()))
}

fn door_tile_parity_keep(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_tile::Parity = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_tile::Parity::keep(a0, a1)))
}

fn door_tile_tile_check(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::Tile = take!(name, 0, &args[0]);
    match mrly_tile::Tile::check(&a0) {
        Ok(()) => Ok(Value::Null),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_tile_degenerate(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::Tile = take!(name, 0, &args[0]);
    Ok(give!(mrly_tile::Tile::degenerate(&a0)))
}

fn door_tile_tile_max_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::Tile = take!(name, 0, &args[0]);
    Ok(give!(mrly_tile::Tile::max_size(&a0)))
}

fn door_tile_tile_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::Group = take!(name, 0, &args[0]);
    Ok(give!(mrly_tile::Tile::new(a0)))
}

fn door_tile_tile_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_tile::Tile = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_tile::Tile::size(a0, a1, a2)))
}

fn door_tile_background(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_tile::background(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_config2d_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_tile::build::Config2d::default()))
}

fn door_tile_build_config3d_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_tile::build::Config3d::default()))
}

fn door_tile_build_build_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::Tile = take!(name, 0, &args[0]);
    match mrly_tile::build::build_2d(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_build_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::Tile = take!(name, 0, &args[0]);
    match mrly_tile::build::build_3d(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_build_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::build::HexTile = take!(name, 0, &args[0]);
    match mrly_tile::build::build_6d(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_create_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    match mrly_tile::build::create_2d(&a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_create_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    match mrly_tile::build::create_3d(&a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_create_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    match mrly_tile::build::create_6d(&a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_random_tile_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    match mrly_tile::build::random_tile_2d(a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_random_tile_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    match mrly_tile::build::random_tile_3d(a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_random_tile_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    match mrly_tile::build::random_tile_6d(a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_classic_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::recipe::Design = take!(name, 0, &args[0]);
    Ok(match mrly_tile::classic_code(a0) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_tile_classic_code_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_tile::recipe::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(match mrly_tile::classic_code_nd(a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_tile_hex_key(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrly_tile::hex_key(a0, &mut a1)))
}

fn door_tile_name_slots_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_tile::name::Slots::default()))
}

fn door_tile_name_tile_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::name::Tile = take!(name, 0, &args[0]);
    match <mrly_tile::name::Tile as mrly_core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_tile::name::Tile as mrly_core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_tile::name::Tile as mrly_core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_tile::name::Tile as mrly_core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::Tile = take!(name, 0, &args[0]);
    match mrly_tile::name::Tile::of(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_recipe(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::name::Tile = take!(name, 0, &args[0]);
    match mrly_tile::name::Tile::recipe(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::name::Tile = take!(name, 0, &args[0]);
    match <mrly_tile::name::Tile as mrly_core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::name::Tile = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_tile::name::Tile as mrly_core::name::Named>::to_id(&a0)
    ))
}

fn door_tile_name_tile_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::name::Tile = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly_tile::name::Tile as mrly_core::name::Named>::to_json(&a0)
    ))
}

fn door_tile_name_tile_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::name::Tile = take!(name, 0, &args[0]);
    match <mrly_tile::name::Tile as mrly_core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::name::Tile = take!(name, 0, &args[0]);
    match <mrly_tile::name::Tile as mrly_core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_random_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrly_core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrly_tile::random_design(&mut a0)))
}

fn door_tile_random_rotation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_tile::recipe::Design = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrly_tile::random_rotation(a0, &mut a1)))
}

fn door_tile_recipe_design_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_tile::recipe::Design::all()))
}

fn door_tile_recipe_classics(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_tile::recipe::classics(a0)))
}

fn door_tile_recipe_generals(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly_tile::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrly_tile::recipe::generals(a0, a1, a2)))
}

fn door_tile_recipe_nestings(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly_tile::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrly_tile::recipe::nestings(a0, a1, a2)))
}

fn door_tile_recipe_powers(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly_tile::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrly_tile::recipe::powers(a0, a1, a2)))
}

fn door_tile_recipe_products(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrly_tile::Parity = take!(name, 3, &args[3]);
    Ok(give!(mrly_tile::recipe::products(a0, a1, a2, a3)))
}

fn door_tile_recipe_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_tile::recipe::size(a0, a1)))
}

fn door_tile_tree_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly_tile::tree_mask(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_variation_config_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_tile::variation::Config::default()))
}

fn door_tile_variation_file_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_tile::variation::File::new(a0, a1)))
}

fn door_tile_variation_variation_is_cover(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::variation::Variation = take!(name, 0, &args[0]);
    Ok(give!(mrly_tile::variation::Variation::is_cover(&a0)))
}

fn door_tile_variation_variation_is_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_tile::variation::Variation = take!(name, 0, &args[0]);
    Ok(give!(mrly_tile::variation::Variation::is_prime(&a0)))
}

fn door_tile_variation_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_tile::variation::Config = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    match mrly_tile::variation::create(&a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_variation_generate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_tile::variation::Variation = take!(name, 0, &args[0]);
    let a1: mrly_tile::variation::Config = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    match mrly_tile::variation::generate(a0, &a1, &mut a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_variation_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_tile::variation::Variation = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    match mrly_tile::variation::render(a0, a1, &mut a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}
