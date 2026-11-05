use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "life.Boundary.all",
        signature: "() -> [life.Boundary; 2]",
        docs: "",
        uncallable: None,
        door: Some(door_life_boundary_all),
    },
    Function {
        name: "life.Boundary.wrap",
        signature: "(self: life.Boundary) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_life_boundary_wrap),
    },
    Function {
        name: "life.Config.budget",
        signature: "(self: life.Config) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_life_config_budget),
    },
    Function {
        name: "life.Config.counts",
        signature: "(self: life.Config) -> ([usize], [usize])",
        docs: "",
        uncallable: None,
        door: Some(door_life_config_counts),
    },
    Function {
        name: "life.Config.new",
        signature: "(mask: Cell2d, birth: life.Counts, survive: life.Counts) -> life.Config",
        docs: "",
        uncallable: None,
        door: Some(door_life_config_new),
    },
    Function {
        name: "life.Counts.drawn",
        signature: "(seq: life.Source, zeros: bool, ones: bool) -> life.Counts",
        docs: "",
        uncallable: None,
        door: Some(door_life_counts_drawn),
    },
    Function {
        name: "life.Counts.list",
        signature: "(counts: [usize]) -> life.Counts",
        docs: "",
        uncallable: None,
        door: Some(door_life_counts_list),
    },
    Function {
        name: "life.Counts.values",
        signature: "(self: life.Counts, budget: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_life_counts_values),
    },
    Function {
        name: "life.Fate.all",
        signature: "() -> [life.Fate; 4]",
        docs: "",
        uncallable: None,
        door: Some(door_life_fate_all),
    },
    Function {
        name: "life.Life.last",
        signature: "(self: life.Life) -> Cell2d?",
        docs: "",
        uncallable: None,
        door: Some(door_life_life_last),
    },
    Function {
        name: "life.Rule.boundary",
        signature: "(self: life.Rule) -> life.Boundary",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_boundary),
    },
    Function {
        name: "life.Rule.checked",
        signature: "(self: life.Rule) -> life.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_checked),
    },
    Function {
        name: "life.Rule.config",
        signature: "(self: life.Rule, mask: Cell2d) -> life.Config",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_config),
    },
    Function {
        name: "life.Rule.from_file",
        signature: "(text: str) -> life.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_from_file),
    },
    Function {
        name: "life.Rule.from_json",
        signature: "(text: str) -> life.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_from_json),
    },
    Function {
        name: "life.Rule.from_url",
        signature: "(text: str) -> life.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_from_url),
    },
    Function {
        name: "life.Rule.new",
        signature: "(birth: life.Counts, survive: life.Counts, wrap: bool) -> life.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_new),
    },
    Function {
        name: "life.Rule.of",
        signature: "(config: life.Config) -> life.Rule",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_of),
    },
    Function {
        name: "life.Rule.to_file",
        signature: "(self: life.Rule) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_to_file),
    },
    Function {
        name: "life.Rule.to_id",
        signature: "(self: life.Rule) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_to_id),
    },
    Function {
        name: "life.Rule.to_json",
        signature: "(self: life.Rule) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_to_json),
    },
    Function {
        name: "life.Rule.to_prose",
        signature: "(self: life.Rule) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_to_prose),
    },
    Function {
        name: "life.Rule.to_url",
        signature: "(self: life.Rule) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_to_url),
    },
    Function {
        name: "life.Source.all",
        signature: "() -> [life.Source; 23]",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_all),
    },
    Function {
        name: "life.Source.designs",
        signature: "() -> [life.Source; 17]",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_designs),
    },
    Function {
        name: "life.Source.is_random",
        signature: "(self: life.Source) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_is_random),
    },
    Function {
        name: "life.Source.name",
        signature: "(self: life.Source) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_name),
    },
    Function {
        name: "life.Source.numbers",
        signature: "() -> [life.Source; 6]",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_numbers),
    },
    Function {
        name: "life.Source.oeis",
        signature: "(self: life.Source) -> String?",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_oeis),
    },
    Function {
        name: "life.Source.parse",
        signature: "(name: str) -> life.Source",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_parse),
    },
    Function {
        name: "life.Source.read",
        signature: "(text: str) -> (life.Source, str)?",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_read),
    },
    Function {
        name: "life.affine",
        signature: "(rule: u8) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_life_affine),
    },
    Function {
        name: "life.animate",
        signature: "(seed: Cell2d, config: life.Config) -> life.Life",
        docs: "",
        uncallable: None,
        door: Some(door_life_animate),
    },
    Function {
        name: "life.churn",
        signature: "(grids: [Cell2d]) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_life_churn),
    },
    Function {
        name: "life.corner_bits",
        signature: "(rule: u8) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_life_corner_bits),
    },
    Function {
        name: "life.counts",
        signature: "(seq: life.Source, max_neighbors: usize, include_zeros: bool, include_ones: bool) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_life_counts),
    },
    Function {
        name: "life.crop",
        signature: "(grids: [Cell2d]) -> [Cell2d]",
        docs: "",
        uncallable: None,
        door: Some(door_life_crop),
    },
    Function {
        name: "life.cube_orbit",
        signature: "(rule: u8) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_life_cube_orbit),
    },
    Function {
        name: "life.design_mask",
        signature: "(dimension: usize, code: Code, number: usize, level: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_life_design_mask),
    },
    Function {
        name: "life.elementary.output",
        signature: "(rule: u8, l: u8, c: u8, r: u8) -> u8",
        docs: "",
        uncallable: None,
        door: Some(door_life_elementary_output),
    },
    Function {
        name: "life.entropy",
        signature: "(grid: Cell2d) -> i64",
        docs: "",
        uncallable: None,
        door: Some(door_life_entropy),
    },
    Function {
        name: "life.frames",
        signature: "(grids: [Cell2d], scale: usize) -> [Image]",
        docs: "",
        uncallable: None,
        door: Some(door_life_frames),
    },
    Function {
        name: "life.gasket",
        signature: "(rule: u8) -> String?",
        docs: "",
        uncallable: None,
        door: Some(door_life_gasket),
    },
    Function {
        name: "life.genus",
        signature: "(rule: u8) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_genus),
    },
    Function {
        name: "life.heatmap",
        signature: "(grids: [Cell2d], scale: usize) -> [Image]",
        docs: "",
        uncallable: None,
        door: Some(door_life_heatmap),
    },
    Function {
        name: "life.history",
        signature: "(row: [u8], rule: u8, steps: usize, wrap: bool) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_life_history),
    },
    Function {
        name: "life.lambda",
        signature: "(rule: u8) -> f64",
        docs: "",
        uncallable: None,
        door: Some(door_life_lambda),
    },
    Function {
        name: "life.lattice_index",
        signature: "(mask: Tensor) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_life_lattice_index),
    },
    Function {
        name: "life.mask_offsets",
        signature: "(mask: Tensor) -> [[i64]]",
        docs: "",
        uncallable: None,
        door: Some(door_life_mask_offsets),
    },
    Function {
        name: "life.moore",
        signature: "() -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_life_moore),
    },
    Function {
        name: "life.next_grid",
        signature: "(cell: Cell2d, birth: [usize], survive: [usize], mask: Tensor, boundary: life.Boundary) -> Cell2d",
        docs: "",
        uncallable: None,
        door: Some(door_life_next_grid),
    },
    Function {
        name: "life.npn_class",
        signature: "(rule: u8) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_life_npn_class),
    },
    Function {
        name: "life.outer_totalistic",
        signature: "(rule: u8) -> ([usize], [usize])?",
        docs: "",
        uncallable: None,
        door: Some(door_life_outer_totalistic),
    },
    Function {
        name: "life.popcount",
        signature: "(rule: u8) -> u32",
        docs: "",
        uncallable: None,
        door: Some(door_life_popcount),
    },
    Function {
        name: "life.recipe.Mode.all",
        signature: "() -> [life.recipe.Mode; 2]",
        docs: "",
        uncallable: None,
        door: Some(door_life_recipe_mode_all),
    },
    Function {
        name: "life.recipe.Run.background",
        signature: "(self: life.recipe.Run) -> u8",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.birth",
        signature: "(self: life.recipe.Run) -> [u32]",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.budget",
        signature: "(self: life.recipe.Run) -> u32",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.cap",
        signature: "(self: life.recipe.Run) -> u32",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.flips",
        signature: "(self: life.recipe.Run) -> bool",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.frame",
        signature: "(self: life.recipe.Run) -> Cell",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.generation",
        signature: "(self: life.recipe.Run) -> u32",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.mask",
        signature: "(self: life.recipe.Run) -> Tensor",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.new",
        signature: "(recipe: life.recipe.Recipe) -> life.recipe.Run",
        docs: "",
        uncallable: Some("life::recipe::Run has no Serialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.origin",
        signature: "(self: life.recipe.Run) -> [i64; 2]",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.population",
        signature: "(self: life.recipe.Run) -> u32",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.reach",
        signature: "(self: life.recipe.Run) -> u32",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.side",
        signature: "(self: life.recipe.Run) -> u32",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.step",
        signature: "(self: life.recipe.Run) -> life.recipe.Stop?",
        docs: "",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.stop",
        signature: "(self: life.recipe.Run) -> life.recipe.Stop?",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.Run.survive",
        signature: "(self: life.recipe.Run) -> [u32]",
        docs: "",
        uncallable: Some("life::recipe::Run has no Deserialize"),
        door: None,
    },
    Function {
        name: "life.recipe.grammar",
        signature: "() -> life.recipe.Grammar",
        docs: "",
        uncallable: None,
        door: Some(door_life_recipe_grammar),
    },
    Function {
        name: "life.recipe.name",
        signature: "(recipe: life.recipe.Recipe) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_recipe_name),
    },
    Function {
        name: "life.recipe.parse",
        signature: "(name: str) -> life.recipe.Recipe",
        docs: "",
        uncallable: None,
        door: Some(door_life_recipe_parse),
    },
    Function {
        name: "life.render.frame",
        signature: "(grid: Cell2d, scale: usize) -> Image",
        docs: "",
        uncallable: None,
        door: Some(door_life_render_frame),
    },
    Function {
        name: "life.reversible",
        signature: "(rule: u8) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_life_reversible),
    },
    Function {
        name: "life.rule_degree",
        signature: "(rule: u8) -> i32",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_degree),
    },
    Function {
        name: "life.rule_name",
        signature: "(rule: u8) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_life_rule_name),
    },
    Function {
        name: "life.single_seed",
        signature: "(rule: u8, steps: usize) -> Tensor",
        docs: "",
        uncallable: None,
        door: Some(door_life_single_seed),
    },
    Function {
        name: "life.source.sequence",
        signature: "(seq: life.Source, limit: usize) -> [usize]",
        docs: "",
        uncallable: None,
        door: Some(door_life_source_sequence),
    },
    Function {
        name: "life.step",
        signature: "(row: [u8], rule: u8, wrap: bool) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_life_step),
    },
    Function {
        name: "life.surjective",
        signature: "(rule: u8) -> bool",
        docs: "",
        uncallable: None,
        door: Some(door_life_surjective),
    },
    Function {
        name: "life.tessellate",
        signature: "(grids: [Cell2d], min_canvas: usize) -> [Cell2d]",
        docs: "",
        uncallable: None,
        door: Some(door_life_tessellate),
    },
    Function {
        name: "life.wolfram_class",
        signature: "(rule: u8) -> [u8]",
        docs: "",
        uncallable: None,
        door: Some(door_life_wolfram_class),
    },
];

fn door_life_boundary_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_life::Boundary::all()))
}

fn door_life_boundary_wrap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Boundary = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Boundary::wrap(a0)))
}

fn door_life_config_budget(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Config = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Config::budget(&a0)))
}

fn door_life_config_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Config = take!(name, 0, &args[0]);
    match mrly_life::Config::counts(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_config_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly_life::Counts = take!(name, 1, &args[1]);
    let a2: mrly_life::Counts = take!(name, 2, &args[2]);
    Ok(give!(mrly_life::Config::new(a0, a1, a2)))
}

fn door_life_counts_drawn(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_life::Source = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_life::Counts::drawn(a0, a1, a2)))
}

fn door_life_counts_list(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Counts::list(a0)))
}

fn door_life_counts_values(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_life::Counts = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_life::Counts::values(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_fate_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_life::Fate::all()))
}

fn door_life_life_last(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Life = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Life::last(&a0)))
}

fn door_life_rule_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Rule::boundary(&a0)))
}

fn door_life_rule_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Rule = take!(name, 0, &args[0]);
    match <mrly_life::Rule as mrly_core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_config(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_life::Rule = take!(name, 0, &args[0]);
    let a1: mrly_math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    Ok(give!(mrly_life::Rule::config(&a0, a1)))
}

fn door_life_rule_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_life::Rule as mrly_core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_life::Rule as mrly_core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly_life::Rule as mrly_core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_life::Counts = take!(name, 0, &args[0]);
    let a1: mrly_life::Counts = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_life::Rule::new(a0, a1, a2)))
}

fn door_life_rule_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Config = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Rule::of(&a0)))
}

fn door_life_rule_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Rule = take!(name, 0, &args[0]);
    match <mrly_life::Rule as mrly_core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Rule = take!(name, 0, &args[0]);
    Ok(give!(<mrly_life::Rule as mrly_core::name::Named>::to_id(
        &a0
    )))
}

fn door_life_rule_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Rule = take!(name, 0, &args[0]);
    Ok(give!(<mrly_life::Rule as mrly_core::name::Named>::to_json(
        &a0
    )))
}

fn door_life_rule_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Rule = take!(name, 0, &args[0]);
    match <mrly_life::Rule as mrly_core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Rule = take!(name, 0, &args[0]);
    match <mrly_life::Rule as mrly_core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_source_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_life::Source::all()))
}

fn door_life_source_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_life::Source::designs()))
}

fn door_life_source_is_random(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Source::is_random(a0)))
}

fn door_life_source_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Source::name(a0)))
}

fn door_life_source_numbers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_life::Source::numbers()))
}

fn door_life_source_oeis(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Source::oeis(a0)))
}

fn door_life_source_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_life::Source::parse(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_source_read(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::Source::read(a0.as_str())))
}

fn door_life_affine(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::affine(a0)))
}

fn door_life_animate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly_life::Config = take!(name, 1, &args[1]);
    match mrly_life::animate(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_churn(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::churn(&a0)))
}

fn door_life_corner_bits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::corner_bits(a0)))
}

fn door_life_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_life::Source = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrly_life::counts(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    match mrly_life::crop(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_cube_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::cube_orbit(a0)))
}

fn door_life_design_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly_math::bang::Code = mrly_math::bang::Code::from(big(name, 1, &args[1])?);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly_life::design_mask(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_elementary_output(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(give!(mrly_life::elementary::output(a0, a1, a2, a3)))
}

fn door_life_entropy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::entropy(&a0)))
}

fn door_life_frames(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_life::frames(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::gasket(a0)))
}

fn door_life_genus(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::genus(a0)))
}

fn door_life_heatmap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_life::heatmap(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_history(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrly_life::history(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_lambda(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::lambda(a0)))
}

fn door_life_lattice_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::lattice_index(&a0)))
}

fn door_life_mask_offsets(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::mask_offsets(&a0)))
}

fn door_life_moore(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly_life::moore() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_next_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    let a3: mrly_core::Tensor = take!(name, 3, &args[3]);
    let a4: mrly_life::Boundary = take!(name, 4, &args[4]);
    match mrly_life::next_grid(&a0, &a1, &a2, &a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_npn_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::npn_class(a0)))
}

fn door_life_outer_totalistic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::outer_totalistic(a0)))
}

fn door_life_popcount(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::popcount(a0)))
}

fn door_life_recipe_mode_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_life::recipe::Mode::all()))
}

fn door_life_recipe_grammar(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_life::recipe::grammar()))
}

fn door_life_recipe_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_life::recipe::Recipe = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::recipe::name(&a0)))
}

fn door_life_recipe_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_life::recipe::parse(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_render_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_life::render::frame(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_reversible(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::reversible(a0)))
}

fn door_life_rule_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::rule_degree(a0)))
}

fn door_life_rule_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    match mrly_life::rule_name(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_single_seed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_life::single_seed(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_source_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_life::Source = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_life::source::sequence(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_step(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_life::step(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_surjective(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::surjective(a0)))
}

fn door_life_tessellate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly_math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_life::tessellate(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_wolfram_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly_life::wolfram_class(a0)))
}
