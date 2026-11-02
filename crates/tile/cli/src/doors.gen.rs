use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "tile.Group.all",
        signature: "() -> [tile.Group; 5]",
        docs: "Returns every Group in canonical order.",
        uncallable: None,
        door: Some(door_tile_group_all),
    },
    Function {
        name: "tile.Parity.all",
        signature: "() -> [tile.Parity; 3]",
        docs: "Returns every Parity in canonical order.",
        uncallable: None,
        door: Some(door_tile_parity_all),
    },
    Function {
        name: "tile.Parity.keep",
        signature: "(self: tile.Parity, n: usize) -> bool",
        docs: "Returns true when the number passes the filter.",
        uncallable: None,
        door: Some(door_tile_parity_keep),
    },
    Function {
        name: "tile.Tile.check",
        signature: "(self: tile.Tile) -> null",
        docs: "Checks that the slots, numbers and sizes agree.\n\n# Errors\n\nErrs with a terse note for the first broken law: the slot count, a ragged slot list, a\nnumber, rotation, level, flip or factor out of range, or sizes the group does not imply.",
        uncallable: None,
        door: Some(door_tile_tile_check),
    },
    Function {
        name: "tile.Tile.degenerate",
        signature: "(self: tile.Tile) -> bool",
        docs: "Returns whether the recipe is a magic tile of one repeated source at one repeated number,\nthe shape a fractal tile of the same factor and level already draws.",
        uncallable: None,
        door: Some(door_tile_tile_degenerate),
    },
    Function {
        name: "tile.Tile.max_size",
        signature: "(self: tile.Tile) -> usize",
        docs: "Returns the larger of width and height.",
        uncallable: None,
        door: Some(door_tile_tile_max_size),
    },
    Function {
        name: "tile.Tile.new",
        signature: "(group: tile.Group) -> tile.Tile",
        docs: "Builds an empty tile in a group.",
        uncallable: None,
        door: Some(door_tile_tile_new),
    },
    Function {
        name: "tile.Tile.resize",
        signature: "(self: tile.Tile) -> null",
        docs: "Recomputes the factor and side length the group and numbers imply, zero when they overflow.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "tile.Tile.size",
        signature: "(self: tile.Tile, width: usize, height: usize) -> tile.Tile",
        docs: "Sets the tile's width and height.",
        uncallable: None,
        door: Some(door_tile_tile_size),
    },
    Function {
        name: "tile.background",
        signature: "(seed: u64, width: usize, height: usize) -> Image",
        docs: "Draws one seeded artwork and returns its image: a random flat tile under the default recipe\nconstraints and paint, repeated `width` across and `height` down at one pixel per cell.\n\nThe seed opens one stream, `variation::create` draws its own seed from it, and the paint and\nrender follow on the same stream, so one seed always paints the same background.\n\n# Errors\n\nErrs when the width or the height is zero, or when the artwork will not draw or render.",
        uncallable: None,
        door: Some(door_tile_background),
    },
    Function {
        name: "tile.build.Config2d.default",
        signature: "() -> tile.draw.ConfigNd",
        docs: "Returns the default Config2d.",
        uncallable: None,
        door: Some(door_tile_build_config2d_default),
    },
    Function {
        name: "tile.build.Config3d.default",
        signature: "() -> tile.draw.ConfigNd",
        docs: "Returns the default Config3d.",
        uncallable: None,
        door: Some(door_tile_build_config3d_default),
    },
    Function {
        name: "tile.build.build_2d",
        signature: "(tile: tile.Tile) -> Cell2d",
        docs: "Builds the flat cell the tile describes.\n\n# Errors\n\nErrs when the tile's slots are ragged, or when a source will not render at its size.",
        uncallable: None,
        door: Some(door_tile_build_build_2d),
    },
    Function {
        name: "tile.build.build_3d",
        signature: "(tile: tile.Tile) -> Cell3d",
        docs: "Builds the cube the tile describes.\n\n# Errors\n\nErrs when the tile's slots are ragged, or when a source is flat or will not render.",
        uncallable: None,
        door: Some(door_tile_build_build_3d),
    },
    Function {
        name: "tile.build.build_6d",
        signature: "(hex: tile.build.HexTile) -> Cell6d",
        docs: "Builds the tile's cube and flattens it through its projection.\n\n# Errors\n\nErrs when the cube will not build, or when the projection will not flatten it.",
        uncallable: None,
        door: Some(door_tile_build_build_6d),
    },
    Function {
        name: "tile.build.create_2d",
        signature: "(config: tile.draw.ConfigNd, rng: Rng(seed)) -> tile.Tile",
        docs: "Draws a random flat tile from the stream, rotations from the four quarter-turns.\n\n# Errors\n\nErrs when no allowed group fits the size constraints, or the catalog holds no source.",
        uncallable: None,
        door: Some(door_tile_build_create_2d),
    },
    Function {
        name: "tile.build.create_3d",
        signature: "(config: tile.draw.ConfigNd, rng: Rng(seed)) -> tile.Tile",
        docs: "Draws a cube tile from the config with cube orientations drawn from the stream.\n\n# Errors\n\nErrs when no allowed group fits the size constraints, or the catalog holds no source.",
        uncallable: None,
        door: Some(door_tile_build_create_3d),
    },
    Function {
        name: "tile.build.create_6d",
        signature: "(config: tile.draw.ConfigNd, rng: Rng(seed)) -> tile.build.HexTile",
        docs: "Draws a cube tile from the config under a projection drawn from the stream.\n\n# Errors\n\nErrs when no allowed group fits the size constraints, or the catalog holds no source.",
        uncallable: None,
        door: Some(door_tile_build_create_6d),
    },
    Function {
        name: "tile.build.random_tile_2d",
        signature: "(max_size: usize, rng: Rng(seed)) -> tile.Tile",
        docs: "Draws a random flat tile up to the given size under the default config.\n\n# Errors\n\nErrs when no group fits a tile inside the size, or the catalog holds no source.",
        uncallable: None,
        door: Some(door_tile_build_random_tile_2d),
    },
    Function {
        name: "tile.build.random_tile_3d",
        signature: "(max_size: usize, rng: Rng(seed)) -> tile.Tile",
        docs: "Draws a random cube tile up to the given size.\n\n# Errors\n\nErrs when no group fits a tile inside the size, or the catalog holds no source.",
        uncallable: None,
        door: Some(door_tile_build_random_tile_3d),
    },
    Function {
        name: "tile.build.random_tile_6d",
        signature: "(max_size: usize, rng: Rng(seed)) -> tile.build.HexTile",
        docs: "Draws a random cube tile up to the given size under a random projection.\n\n# Errors\n\nErrs when no group fits a tile inside the size, or the catalog holds no source.",
        uncallable: None,
        door: Some(door_tile_build_random_tile_6d),
    },
    Function {
        name: "tile.classic_code",
        signature: "(design: tile.recipe.Design) -> u128?",
        docs: "Returns the plane's bang code of a classic design, or None for one outside the plane.",
        uncallable: None,
        door: Some(door_tile_classic_code),
    },
    Function {
        name: "tile.classic_code_nd",
        signature: "(design: tile.recipe.Design, dimension: usize) -> u128?",
        docs: "Returns the bang code of a named design in a dimension, or None where it has no design.",
        uncallable: None,
        door: Some(door_tile_classic_code_nd),
    },
    Function {
        name: "tile.hex_key",
        signature: "(length: usize, rng: Rng(seed)) -> String",
        docs: "Draws a hex key of the given length from the stream.",
        uncallable: None,
        door: Some(door_tile_hex_key),
    },
    Function {
        name: "tile.name.Slots.default",
        signature: "() -> tile.name.Slots",
        docs: "Returns the default Slots.",
        uncallable: None,
        door: Some(door_tile_name_slots_default),
    },
    Function {
        name: "tile.name.Tile.checked",
        signature: "(self: tile.name.Tile) -> tile.name.Tile",
        docs: "Folds a decoded value to its canonical form, or an error for one outside the kind.",
        uncallable: None,
        door: Some(door_tile_name_tile_checked),
    },
    Function {
        name: "tile.name.Tile.from_file",
        signature: "(text: str) -> tile.name.Tile",
        docs: "Reads a filename back into the value, or an error.",
        uncallable: None,
        door: Some(door_tile_name_tile_from_file),
    },
    Function {
        name: "tile.name.Tile.from_json",
        signature: "(text: str) -> tile.name.Tile",
        docs: "Reads a JSON object into its canonical value, or an error naming the broken key.",
        uncallable: None,
        door: Some(door_tile_name_tile_from_json),
    },
    Function {
        name: "tile.name.Tile.from_url",
        signature: "(text: str) -> tile.name.Tile",
        docs: "Reads a path and query string back into the value, or an error.",
        uncallable: None,
        door: Some(door_tile_name_tile_from_url),
    },
    Function {
        name: "tile.name.Tile.of",
        signature: "(recipe: tile.Tile) -> tile.name.Tile",
        docs: "Folds a recipe to its name.\n\n# Errors\n\nErrs when a source carries no code, or when the recipe's slot lists are ragged.",
        uncallable: None,
        door: Some(door_tile_name_tile_of),
    },
    Function {
        name: "tile.name.Tile.recipe",
        signature: "(self: tile.name.Tile) -> tile.Tile",
        docs: "Builds the recipe the name folds, resized and checked.\n\n# Errors\n\nErrs when the keys do not say one group, a code is not in the plane, or the check fails.",
        uncallable: None,
        door: Some(door_tile_name_tile_recipe),
    },
    Function {
        name: "tile.name.Tile.to_file",
        signature: "(self: tile.name.Tile) -> String",
        docs: "Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_tile_name_tile_to_file),
    },
    Function {
        name: "tile.name.Tile.to_id",
        signature: "(self: tile.name.Tile) -> String",
        docs: "Prints the first eight hex digits of the sha256 of the canonical JSON.",
        uncallable: None,
        door: Some(door_tile_name_tile_to_id),
    },
    Function {
        name: "tile.name.Tile.to_json",
        signature: "(self: tile.name.Tile) -> String",
        docs: "Prints the canonical JSON object.",
        uncallable: None,
        door: Some(door_tile_name_tile_to_json),
    },
    Function {
        name: "tile.name.Tile.to_prose",
        signature: "(self: tile.name.Tile) -> String",
        docs: "Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_tile_name_tile_to_prose),
    },
    Function {
        name: "tile.name.Tile.to_url",
        signature: "(self: tile.name.Tile) -> String",
        docs: "Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_tile_name_tile_to_url),
    },
    Function {
        name: "tile.random_design",
        signature: "(rng: Rng(seed)) -> tile.recipe.Design",
        docs: "Draws one of the four flat classics from the stream: carpet, net, vertical tree or void.",
        uncallable: None,
        door: Some(door_tile_random_design),
    },
    Function {
        name: "tile.random_rotation",
        signature: "(design: tile.recipe.Design, rng: Rng(seed)) -> u8",
        docs: "Draws a design's turn from the stream: a tree turns 0 or 1, every other design 0 to 3.",
        uncallable: None,
        door: Some(door_tile_random_rotation),
    },
    Function {
        name: "tile.recipe.Design.all",
        signature: "() -> [tile.recipe.Design; 16]",
        docs: "Returns every Design in canonical order.",
        uncallable: None,
        door: Some(door_tile_recipe_design_all),
    },
    Function {
        name: "tile.recipe.classics",
        signature: "(dimension: usize) -> [tile.recipe.Design]",
        docs: "Returns the classic designs for a dimension.",
        uncallable: None,
        door: Some(door_tile_recipe_classics),
    },
    Function {
        name: "tile.recipe.generals",
        signature: "(min_size: usize, max_size: usize, parity: tile.Parity) -> [usize]",
        docs: "Returns every flat size in the range that passes the parity filter.",
        uncallable: None,
        door: Some(door_tile_recipe_generals),
    },
    Function {
        name: "tile.recipe.nestings",
        signature: "(min_size: usize, max_size: usize, parity: tile.Parity) -> [[usize]]",
        docs: "Returns every factor list of depth two and beyond whose product lands in the size range.",
        uncallable: None,
        door: Some(door_tile_recipe_nestings),
    },
    Function {
        name: "tile.recipe.powers",
        signature: "(min_size: usize, max_size: usize, parity: tile.Parity) -> [(usize, usize)]",
        docs: "Returns every factor and level whose power lands in the size range.",
        uncallable: None,
        door: Some(door_tile_recipe_powers),
    },
    Function {
        name: "tile.recipe.products",
        signature: "(min_size: usize, max_size: usize, count: usize, parity: tile.Parity) -> [[usize]]",
        docs: "Returns every count-long factor list whose product lands in the size range.",
        uncallable: None,
        door: Some(door_tile_recipe_products),
    },
    Function {
        name: "tile.recipe.size",
        signature: "(number: i64, level: i64) -> usize?",
        docs: "Returns the side a factor raised to a level makes, or None when no usize holds it.",
        uncallable: None,
        door: Some(door_tile_recipe_size),
    },
    Function {
        name: "tile.tree_mask",
        signature: "(n: usize) -> Tensor",
        docs: "Builds the mask of a mosaic tile: the two trees of the side, two where they cross.\n\n# Errors\n\nErrs when the side will not draw a tree.",
        uncallable: None,
        door: Some(door_tile_tree_mask),
    },
    Function {
        name: "tile.variation.Config.default",
        signature: "() -> tile.variation.Config",
        docs: "Returns the default Config.",
        uncallable: None,
        door: Some(door_tile_variation_config_default),
    },
    Function {
        name: "tile.variation.File.new",
        signature: "(width: usize, height: usize) -> tile.variation.File",
        docs: "Builds a file of the given repetition counts with no image.",
        uncallable: None,
        door: Some(door_tile_variation_file_new),
    },
    Function {
        name: "tile.variation.Variation.is_cover",
        signature: "(self: tile.variation.Variation) -> bool",
        docs: "Returns whether the edition paints the whole tiled canvas.",
        uncallable: None,
        door: Some(door_tile_variation_variation_is_cover),
    },
    Function {
        name: "tile.variation.Variation.is_prime",
        signature: "(self: tile.variation.Variation) -> bool",
        docs: "Returns whether the edition paints the base cell before tiling.",
        uncallable: None,
        door: Some(door_tile_variation_variation_is_prime),
    },
    Function {
        name: "tile.variation.create",
        signature: "(config: tile.variation.Config, rng: Rng(seed)) -> tile.variation.Variation",
        docs: "Draws a variation's seed from the stream, then the variation itself on that seed, with a\nmask when the edition is Neighbors.\n\n# Errors\n\nErrs when the tile config draws no tile, or when the Neighbors mask will not draw.",
        uncallable: None,
        door: Some(door_tile_variation_create),
    },
    Function {
        name: "tile.variation.generate",
        signature: "(variation: tile.variation.Variation, config: tile.variation.Config, rng: Rng(seed)) -> tile.variation.Variation",
        docs: "Builds the variation's base cell and draws its paint from the stream, painting the base\nunder a prime edition.\n\n# Errors\n\nErrs when the tile or its mask will not build, or when the paint will not lay down.",
        uncallable: None,
        door: Some(door_tile_variation_generate),
    },
    Function {
        name: "tile.variation.render",
        signature: "(variation: tile.variation.Variation, scale: usize, rng: Rng(seed)) -> tile.variation.Variation",
        docs: "Renders every file of the variation to an image at the given scale, scattering a Random edition\nfrom the stream.\n\n# Errors\n\nErrs when generate has not run, when a file will not tile, or on a scale below one.",
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
