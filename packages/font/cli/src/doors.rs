use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "font.Glyph.height",
        signature: "(self: font.Glyph) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_font_glyph_height),
    },
    Function {
        name: "font.Glyph.new",
        signature: "(char: char, rows: [String]) -> font.Glyph",
        docs: "",
        uncallable: None,
        door: Some(door_font_glyph_new),
    },
    Function {
        name: "font.Glyph.width",
        signature: "(self: font.Glyph) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_font_glyph_width),
    },
    Function {
        name: "font.all",
        signature: "() -> [font.Glyph]",
        docs: "",
        uncallable: None,
        door: Some(door_font_all),
    },
    Function {
        name: "font.animate",
        signature: "(text: str, pad: usize) -> font.Anim",
        docs: "",
        uncallable: None,
        door: Some(door_font_animate),
    },
    Function {
        name: "font.cycle",
        signature: "(write: font.Anim, merge: [[usize]], hold: usize) -> font.Anim",
        docs: "",
        uncallable: None,
        door: Some(door_font_cycle),
    },
    Function {
        name: "font.digits",
        signature: "() -> [font.Glyph]",
        docs: "",
        uncallable: None,
        door: Some(door_font_digits),
    },
    Function {
        name: "font.draft",
        signature: "(rows: [String]) -> [[(usize, usize)]]",
        docs: "",
        uncallable: None,
        door: Some(door_font_draft),
    },
    Function {
        name: "font.extras",
        signature: "() -> [font.Glyph]",
        docs: "",
        uncallable: None,
        door: Some(door_font_extras),
    },
    Function {
        name: "font.floor",
        signature: "(rows: [String]) -> usize",
        docs: "",
        uncallable: None,
        door: Some(door_font_floor),
    },
    Function {
        name: "font.glyph",
        signature: "(c: char) -> font.Glyph?",
        docs: "",
        uncallable: None,
        door: Some(door_font_glyph),
    },
    Function {
        name: "font.lower",
        signature: "(rows: [str]) -> [String]",
        docs: "",
        uncallable: None,
        door: Some(door_font_lower),
    },
    Function {
        name: "font.lowers",
        signature: "() -> [font.Glyph]",
        docs: "",
        uncallable: None,
        door: Some(door_font_lowers),
    },
    Function {
        name: "font.map",
        signature: "() -> {char: [String]}",
        docs: "",
        uncallable: None,
        door: Some(door_font_map),
    },
    Function {
        name: "font.merge",
        signature: "(text: str, pad: usize) -> [[usize]]",
        docs: "",
        uncallable: None,
        door: Some(door_font_merge),
    },
    Function {
        name: "font.name_of",
        signature: "(c: char) -> String",
        docs: "",
        uncallable: None,
        door: Some(door_font_name_of),
    },
    Function {
        name: "font.path",
        signature: "(c: char) -> [(usize, usize)]",
        docs: "",
        uncallable: None,
        door: Some(door_font_path),
    },
    Function {
        name: "font.paths.penned",
        signature: "(c: char) -> [[(usize, usize)]]?",
        docs: "",
        uncallable: None,
        door: Some(door_font_paths_penned),
    },
    Function {
        name: "font.pens.all",
        signature: "() -> [(char, [String])]",
        docs: "",
        uncallable: None,
        door: Some(door_font_pens_all),
    },
    Function {
        name: "font.raster",
        signature: "(text: str) -> [[u8]]",
        docs: "",
        uncallable: None,
        door: Some(door_font_raster),
    },
    Function {
        name: "font.specials",
        signature: "() -> [font.Glyph]",
        docs: "",
        uncallable: None,
        door: Some(door_font_specials),
    },
    Function {
        name: "font.strokes",
        signature: "(c: char) -> [[(usize, usize)]]",
        docs: "",
        uncallable: None,
        door: Some(door_font_strokes),
    },
    Function {
        name: "font.supported",
        signature: "() -> [char]",
        docs: "",
        uncallable: None,
        door: Some(door_font_supported),
    },
    Function {
        name: "font.trim",
        signature: "(rows: [String]) -> [String]",
        docs: "",
        uncallable: None,
        door: Some(door_font_trim),
    },
    Function {
        name: "font.uppers",
        signature: "() -> [font.Glyph]",
        docs: "",
        uncallable: None,
        door: Some(door_font_uppers),
    },
];

fn door_font_glyph_height(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_font::Glyph = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::Glyph::height(&a0)))
}

fn door_font_glyph_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: char = take!(name, 0, &args[0]);
    let a1: Vec<String> = take!(name, 1, &args[1]);
    Ok(give!(mrly_font::Glyph::new(a0, a1)))
}

fn door_font_glyph_width(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_font::Glyph = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::Glyph::width(&a0)))
}

fn door_font_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::all()))
}

fn door_font_animate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_font::animate(a0.as_str(), a1)))
}

fn door_font_cycle(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_font::Anim = take!(name, 0, &args[0]);
    let a1: Vec<Vec<usize>> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_font::cycle(&a0, &a1, a2)))
}

fn door_font_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::digits()))
}

fn door_font_draft(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::draft(&a0)))
}

fn door_font_extras(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::extras()))
}

fn door_font_floor(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::floor(&a0)))
}

fn door_font_glyph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::glyph(a0)))
}

fn door_font_lower(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    let b0: Vec<&str> = a0.iter().map(|item| item.as_str()).collect();
    Ok(give!(mrly_font::lower(&b0)))
}

fn door_font_lowers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::lowers()))
}

fn door_font_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::map()))
}

fn door_font_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_font::merge(a0.as_str(), a1)))
}

fn door_font_name_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::name_of(a0)))
}

fn door_font_path(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::path(a0)))
}

fn door_font_paths_penned(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::paths::penned(a0)))
}

fn door_font_pens_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::pens::all()))
}

fn door_font_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::raster(a0.as_str())))
}

fn door_font_specials(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::specials()))
}

fn door_font_strokes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::strokes(a0)))
}

fn door_font_supported(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::supported()))
}

fn door_font_trim(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrly_font::trim(&a0)))
}

fn door_font_uppers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_font::uppers()))
}
