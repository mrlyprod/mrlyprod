use serde_json::Value;
use std::process::ExitCode;

// PLUMBING

enum Fail {
    Usage(String),
    Error(String),
}

type Done = std::result::Result<Value, Fail>;

type Call = fn(&str, &[Value]) -> Done;

type Door = (&'static str, &'static str, Option<Call>);

const USAGE: &str = "usage: mrly list | mrly <module.fn> '[json, args]'";

macro_rules! take {
    ($name:expr, $at:expr, $value:expr) => {
        match serde_json::from_value(Value::clone($value)) {
            Ok(value) => value,
            Err(error) => return Err(bad($name, $at, &error.to_string())),
        }
    };
}

macro_rules! give {
    ($value:expr) => {
        match serde_json::to_value($value) {
            Ok(value) => value,
            Err(error) => return Err(Fail::Error(error.to_string())),
        }
    };
}

fn usage(what: String) -> Fail {
    Fail::Usage(format!("mrly: {what}; {USAGE}"))
}

fn bad(name: &str, at: usize, why: &str) -> Fail {
    usage(format!("{name} argument {at}: {why}"))
}

fn count(name: &str, args: &[Value], want: usize) -> std::result::Result<(), Fail> {
    if args.len() == want {
        Ok(())
    } else {
        Err(usage(format!(
            "{name} takes {want} arguments, got {}",
            args.len()
        )))
    }
}

fn slot<'a>(name: &str, args: &'a [Value], at: usize) -> std::result::Result<&'a Value, Fail> {
    args.get(at)
        .ok_or_else(|| usage(format!("{name} wants an argument {at}")))
}

fn big(name: &str, at: usize, value: &Value) -> std::result::Result<u128, Fail> {
    let read = match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_u64().map(u128::from),
    };
    match read {
        Some(number) => Ok(number),
        None => Err(bad(name, at, "a u128 is a decimal string")),
    }
}

fn small(name: &str, at: usize, value: &Value) -> std::result::Result<i128, Fail> {
    let read = match value.as_str() {
        Some(text) => text.parse().ok(),
        None => value.as_i64().map(i128::from),
    };
    match read {
        Some(number) => Ok(number),
        None => Err(bad(name, at, "an i128 is a decimal string")),
    }
}

fn seed(name: &str, at: usize, value: &Value) -> std::result::Result<u64, Fail> {
    match value.as_u64() {
        Some(number) => Ok(number),
        None => Err(bad(name, at, "an Rng is a seed number")),
    }
}

fn color(name: &str, at: usize, value: &Value) -> std::result::Result<mrlyrs::core::Color, Fail> {
    let bytes: [u8; 4] = match serde_json::from_value(value.clone()) {
        Ok(bytes) => bytes,
        Err(_) => return Err(bad(name, at, "a Color is four bytes")),
    };
    Ok(mrlyrs::core::Color {
        r: bytes[0],
        g: bytes[1],
        b: bytes[2],
        a: bytes[3],
    })
}

fn tint(color: mrlyrs::core::Color) -> Value {
    Value::Array(vec![
        color.r.into(),
        color.g.into(),
        color.b.into(),
        color.a.into(),
    ])
}

fn items<'a>(name: &str, at: usize, value: &'a Value) -> std::result::Result<&'a [Value], Fail> {
    match value.as_array() {
        Some(list) => Ok(list),
        None => Err(bad(name, at, "a JSON array")),
    }
}

fn parts<'a>(
    name: &str,
    at: usize,
    value: &'a Value,
    want: usize,
) -> std::result::Result<&'a [Value], Fail> {
    match value.as_array() {
        Some(list) if list.len() == want => Ok(list),
        _ => Err(bad(name, at, &format!("a JSON array of {want}"))),
    }
}

fn fields<'a>(
    name: &str,
    at: usize,
    value: &'a Value,
) -> std::result::Result<&'a serde_json::Map<String, Value>, Fail> {
    match value.as_object() {
        Some(map) => Ok(map),
        None => Err(bad(name, at, "a JSON object")),
    }
}

fn axes(value: &Value) -> Option<usize> {
    Some(value.get("shape")?.as_array()?.len())
}

fn walls(value: &Value) -> Option<usize> {
    axes(value.get("cell")?.get("types")?)
}

fn tensor_rank(name: &str, args: &[Value], at: usize) -> std::result::Result<usize, Fail> {
    match axes(slot(name, args, at)?) {
        Some(rank) => Ok(rank),
        None => Err(bad(name, at, "a Tensor with a shape")),
    }
}

fn cell_rank(name: &str, args: &[Value], at: usize) -> std::result::Result<usize, Fail> {
    match walls(slot(name, args, at)?) {
        Some(rank) => Ok(rank),
        None => Err(bad(name, at, "a cell")),
    }
}

fn cells_rank(name: &str, args: &[Value], at: usize) -> std::result::Result<usize, Fail> {
    let value = slot(name, args, at)?;
    match value.as_array().and_then(|list| list.first()).and_then(walls) {
        Some(rank) => Ok(rank),
        None => Err(bad(name, at, "a list of cells")),
    }
}

fn main() -> ExitCode {
    let words: Vec<String> = std::env::args().skip(1).collect();
    match run(&words) {
        Ok(()) => ExitCode::SUCCESS,
        Err(Fail::Error(message)) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
        Err(Fail::Usage(message)) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}

fn run(words: &[String]) -> std::result::Result<(), Fail> {
    match words {
        [one] if one == "list" => {
            for (name, sig, _) in DOORS {
                println!("{name}{sig}");
            }
            Ok(())
        }
        [name, text] => {
            let at = DOORS
                .binary_search_by_key(&name.as_str(), |door| door.0)
                .map_err(|_| usage(format!("no door named {name:?}")))?;
            let call = DOORS[at]
                .2
                .ok_or_else(|| usage(format!("{name} is not callable from the shell")))?;
            let args = match serde_json::from_str::<Value>(text) {
                Ok(Value::Array(args)) => args,
                Ok(_) => return Err(usage(format!("{name} wants a JSON array of arguments"))),
                Err(error) => return Err(usage(format!("{name} arguments: {error}"))),
            };
            println!("{}", call(name, &args)?);
            Ok(())
        }
        _ => Err(usage("say list or a door".to_string())),
    }
}

// DOORS

static DOORS: &[Door] = &[
    ("core.Colorizer.diverge", "() -> core.Colorizer", Some(door_core_colorizer_diverge)),
    ("core.Colorizer.fire", "() -> core.Colorizer", Some(door_core_colorizer_fire)),
    ("core.Colorizer.gradient_bins", "(background: Color, colors: [Color], shades: usize) -> core.Colorizer", Some(door_core_colorizer_gradient_bins)),
    ("core.Colorizer.heat", "() -> core.Colorizer", Some(door_core_colorizer_heat)),
    ("core.Dtype.max", "(self: core.Dtype) -> i64", Some(door_core_dtype_max)),
    ("core.Image.colors", "(self: core.Image) -> [[u8; 4]]", Some(door_core_image_colors)),
    ("core.Image.from_pixels", "(width: usize, height: usize, pixels: [[u8; 4]]) -> core.Image", Some(door_core_image_from_pixels)),
    ("core.Image.new", "(width: usize, height: usize, rows: [[usize]], palette: [Color]) -> core.Image", Some(door_core_image_new)),
    ("core.Image.png", "(self: core.Image, scale: usize) -> [u8]", Some(door_core_image_png)),
    ("core.Image.resample", "(self: core.Image, width: usize, height: usize, filter: core.Filter) -> core.Image", Some(door_core_image_resample)),
    ("core.cell.anti", "(self: Cell) -> Cell", Some(door_core_cell_anti)),
    ("core.cell.binarize", "(self: Cell, threshold: u8) -> Cell", Some(door_core_cell_binarize)),
    ("core.cell.binarize_otsu", "(self: Cell) -> Cell", Some(door_core_cell_binarize_otsu)),
    ("core.cell.blur", "(self: Cell, mask: Tensor, wrap: bool) -> Cell", Some(door_core_cell_blur)),
    ("core.cell.color_at", "(self: Cell, flat: usize) -> [u8; 4]", Some(door_core_cell_color_at)),
    ("core.cell.combine", "(self: Cell, other: Cell) -> Cell", Some(door_core_cell_combine)),
    ("core.cell.fractal", "(self: Cell, level: usize) -> Cell", Some(door_core_cell_fractal)),
    ("core.cell.invert", "(self: Cell) -> Cell", Some(door_core_cell_invert)),
    ("core.cell.layers", "(self: Cell, dtype: core.Dtype) -> Cell", Some(door_core_cell_layers)),
    ("core.cell.magic", "(cells: [Cell]) -> Cell", Some(door_core_cell_magic)),
    ("core.cell.mapping", "() -> {u8: [Color]}", Some(door_core_cell_mapping)),
    ("core.cell.merge", "(cells: [Cell], reps: [usize]) -> Cell", Some(door_core_cell_merge)),
    ("core.cell.moore", "(dimension: usize) -> Tensor", Some(door_core_cell_moore)),
    ("core.cell.mosaic", "(mask: Tensor, cells: [Cell]) -> Cell", Some(door_core_cell_mosaic)),
    ("core.cell.neighbors", "(self: Cell, mask: Tensor, target: u8, wrap: bool, dtype: core.Dtype) -> Cell", Some(door_core_cell_neighbors)),
    ("core.cell.new", "(types: Tensor) -> Cell", Some(door_core_cell_new)),
    ("core.cell.pad", "(self: Cell, count: usize, value: u8) -> Cell", Some(door_core_cell_pad)),
    ("core.cell.paint", "(self: Cell, mapping: {u8: [Color]}, mode: core.Mode, rng: Rng(seed)?) -> Cell", Some(door_core_cell_paint)),
    ("core.cell.perforate", "(self: Cell, mask: Tensor, value: u8) -> Cell", Some(door_core_cell_perforate)),
    ("core.cell.remap", "(cell: Cell, map: [usize], shape: [usize]) -> Cell", Some(door_core_cell_remap)),
    ("core.cell.rgba", "(self: Cell) -> [u8]", Some(door_core_cell_rgba)),
    ("core.cell.rot90_map", "(shape: [usize], k: usize, axes: (usize, usize)) -> [usize]", Some(door_core_cell_rot90_map)),
    ("core.cell.rotate", "(self: Cell, k: usize, axes: (usize, usize)) -> Cell", Some(door_core_cell_rotate)),
    ("core.cell.shape", "(self: Cell) -> [usize]", Some(door_core_cell_shape)),
    ("core.cell.size", "(self: Cell) -> usize", Some(door_core_cell_size)),
    ("core.cell.tile", "(self: Cell, reps: [usize]) -> Cell", Some(door_core_cell_tile)),
    ("core.cell.tile_map", "(shape: [usize], reps: [usize]) -> [usize]", Some(door_core_cell_tile_map)),
    ("core.codec.gif", "(frames: [[u8]], palette: [[u8; 4]], width: usize, height: usize, scale: usize, delay: usize) -> [u8]", Some(door_core_codec_gif)),
    ("core.codec.png", "(colors: [[u8; 4]], width: usize, height: usize, scale: usize) -> [u8]", Some(door_core_codec_png)),
    ("core.colors.Theme.hues", "(self: core.colors.Theme) -> [Color; 13]", Some(door_core_colors_theme_hues)),
    ("core.colors.Theme.inks", "(self: core.colors.Theme) -> [Color; 6]", Some(door_core_colors_theme_inks)),
    ("core.colors.alpha", "(self: Color, level: u8) -> Color", Some(door_core_colors_alpha)),
    ("core.colors.board", "(dark: bool) -> [u8; 4]", Some(door_core_colors_board)),
    ("core.colors.css", "(self: Color) -> String", Some(door_core_colors_css)),
    ("core.colors.from_hex", "(hex: str) -> Color", Some(door_core_colors_from_hex)),
    ("core.colors.gradient", "(colors: [Color], steps: usize) -> [Color]", Some(door_core_colors_gradient)),
    ("core.colors.ink", "(dark: bool) -> [u8; 4]", Some(door_core_colors_ink)),
    ("core.colors.invert", "(self: Color) -> Color", Some(door_core_colors_invert)),
    ("core.colors.lightness", "(self: Color, level: u8) -> Color", Some(door_core_colors_lightness)),
    ("core.colors.luma_types", "(pixels: [[u8; 4]], width: usize, height: usize, level: u8) -> Tensor", Some(door_core_colors_luma_types)),
    ("core.colors.mix", "(color_1: Color, color_2: Color, ratio: f64) -> Color", Some(door_core_colors_mix)),
    ("core.colors.named", "(name: str) -> Color", Some(door_core_colors_named)),
    ("core.colors.random", "(alpha: bool, rng: Rng(seed)) -> Color", Some(door_core_colors_random)),
    ("core.colors.rgb", "(r: u8, g: u8, b: u8) -> Color", Some(door_core_colors_rgb)),
    ("core.colors.rgba", "(r: u8, g: u8, b: u8, a: u8) -> Color", Some(door_core_colors_rgba)),
    ("core.colors.shades", "(hue: Color) -> [Color; 3]", Some(door_core_colors_shades)),
    ("core.colors.snap", "(pixels: [[u8; 4]], palette: [Color]) -> [[u8; 4]]", Some(door_core_colors_snap)),
    ("core.colors.to_hex", "(self: Color) -> String", Some(door_core_colors_to_hex)),
    ("core.error.parse", "(text: str) -> Json", Some(door_core_error_parse)),
    ("core.hex_fit", "(pixels: [[u8; 4]], width: usize, height: usize, vertical: bool, filter: core.Filter) -> (usize, usize, [[u8; 4]])", Some(door_core_hex_fit)),
    ("core.hex_size", "(width: usize, height: usize, vertical: bool) -> (usize, usize)", Some(door_core_hex_size)),
    ("core.image.blur", "(pixels: [[u8; 4]], width: usize, height: usize, radius: usize) -> [[u8; 4]]", Some(door_core_image_blur)),
    ("core.paint.Edition.all", "() -> [core.paint.Edition; 7]", Some(door_core_paint_edition_all)),
    ("core.paint.Edition.mode", "(self: core.paint.Edition) -> core.Mode?", Some(door_core_paint_edition_mode)),
    ("core.paint.Ink.all", "() -> [core.paint.Ink; 15]", Some(door_core_paint_ink_all)),
    ("core.paint.Ink.color", "(self: core.paint.Ink) -> Color", Some(door_core_paint_ink_color)),
    ("core.paint.Paint.is_simple", "(self: core.paint.Paint) -> bool", Some(door_core_paint_paint_is_simple)),
    ("core.paint.Paint.new", "(edition: core.paint.Edition) -> core.paint.Paint", Some(door_core_paint_paint_new)),
    ("core.paint.Scheme.all", "() -> [core.paint.Scheme; 2]", Some(door_core_paint_scheme_all)),
    ("core.paint.Target.all", "() -> [core.paint.Target; 2]", Some(door_core_paint_target_all)),
    ("core.paint.apply", "(paint: core.paint.Paint, cell: Cell, rng: Rng(seed)) -> null # uncallable: mutates its argument in place", None),
    ("core.paint.coat", "(cell: Cell, paint: core.paint.Paint, mask: Tensor?, rng: Rng(seed)) -> null # uncallable: mutates its argument in place", None),
    ("core.paint.paint", "(cell: Cell, config: core.paint.Config, mask: Tensor?, rng: Rng(seed)) -> core.paint.Paint # uncallable: mutates its argument in place", None),
    ("core.paint.prime", "(paint: core.paint.Paint, cell: Cell, mask: Tensor?, rng: Rng(seed)) -> core.paint.Paint # uncallable: mutates its argument in place", None),
    ("core.paint.random_edition", "(editions: [core.paint.Edition]?, rng: Rng(seed)) -> core.paint.Edition", Some(door_core_paint_random_edition)),
    ("core.paint.reroll", "(paint: core.paint.Paint, rng: Rng(seed)) -> core.paint.Paint", Some(door_core_paint_reroll)),
    ("core.paint.setup", "(paint: core.paint.Paint, config: core.paint.Config, rng: Rng(seed)) -> core.paint.Paint", Some(door_core_paint_setup)),
    ("core.paint.tag", "(cell: Cell, edition: core.paint.Edition, target: core.paint.Target, mask: Tensor?) -> usize # uncallable: mutates its argument in place", None),
    ("core.ramp.color", "(colorizer: core.Colorizer, value: usize, max: usize) -> Color", Some(door_core_ramp_color)),
    ("core.ramp.colors", "(colorizer: core.Colorizer, values: [usize], max: usize) -> [[u8; 4]]", Some(door_core_ramp_colors)),
    ("core.resample", "(pixels: [[u8; 4]], width: usize, height: usize, out_w: usize, out_h: usize, filter: core.Filter) -> [[u8; 4]]", Some(door_core_resample)),
    ("core.rng.below", "(self: Rng(seed), n: usize) -> usize", Some(door_core_rng_below)),
    ("core.rng.boolean", "(self: Rng(seed)) -> bool", Some(door_core_rng_boolean)),
    ("core.rng.chance", "(self: Rng(seed), p: f64) -> bool", Some(door_core_rng_chance)),
    ("core.rng.new", "(seed: u64) -> Rng", Some(door_core_rng_new)),
    ("core.rng.range", "(self: Rng(seed), lo: i64, hi: i64) -> i64", Some(door_core_rng_range)),
    ("core.rng.sample_indices", "(self: Rng(seed), length: usize, amount: usize) -> [usize]", Some(door_core_rng_sample_indices)),
    ("core.rng.unit", "(self: Rng(seed)) -> f64", Some(door_core_rng_unit)),
    ("core.tensor.at", "(self: Tensor, flat: usize) -> i64", Some(door_core_tensor_at)),
    ("core.tensor.binarize", "(self: Tensor, threshold: u8) -> Tensor", Some(door_core_tensor_binarize)),
    ("core.tensor.binarize_otsu", "(self: Tensor) -> Tensor", Some(door_core_tensor_binarize_otsu)),
    ("core.tensor.blur", "(self: Tensor, mask: Tensor, wrap: bool) -> Tensor", Some(door_core_tensor_blur)),
    ("core.tensor.bytes", "(self: Tensor) -> [u8]", Some(door_core_tensor_bytes)),
    ("core.tensor.count", "(self: Tensor, value: u8) -> usize", Some(door_core_tensor_count)),
    ("core.tensor.dtype", "(self: Tensor) -> core.Dtype", Some(door_core_tensor_dtype)),
    ("core.tensor.exposed", "(self: Tensor) -> u128", Some(door_core_tensor_exposed)),
    ("core.tensor.filled", "(shape: [usize], value: i64, dtype: core.Dtype) -> Tensor", Some(door_core_tensor_filled)),
    ("core.tensor.flip", "(self: Tensor, axis: usize) -> Tensor", Some(door_core_tensor_flip)),
    ("core.tensor.fractal", "(self: Tensor, level: usize) -> Tensor", Some(door_core_tensor_fractal)),
    ("core.tensor.full", "(shape: [usize], value: u8) -> Tensor", Some(door_core_tensor_full)),
    ("core.tensor.get", "(self: Tensor, multi: [usize]) -> u8", Some(door_core_tensor_get)),
    ("core.tensor.i32", "(data: [i32], shape: [usize]) -> Tensor", Some(door_core_tensor_i32)),
    ("core.tensor.i32s", "(self: Tensor) -> [i32]", Some(door_core_tensor_i32s)),
    ("core.tensor.index", "(self: Tensor, multi: [usize]) -> usize", Some(door_core_tensor_index)),
    ("core.tensor.invert", "(self: Tensor) -> Tensor", Some(door_core_tensor_invert)),
    ("core.tensor.kron", "(self: Tensor, other: Tensor) -> Tensor", Some(door_core_tensor_kron)),
    ("core.tensor.layers", "(self: Tensor, dtype: core.Dtype) -> Tensor", Some(door_core_tensor_layers)),
    ("core.tensor.neighbors", "(self: Tensor, mask: Tensor, target: u8, wrap: bool, dtype: core.Dtype) -> Tensor", Some(door_core_tensor_neighbors)),
    ("core.tensor.new", "(shape: [usize]) -> Tensor", Some(door_core_tensor_new)),
    ("core.tensor.of", "(data: [u8], shape: [usize]) -> Tensor", Some(door_core_tensor_of)),
    ("core.tensor.otsu_threshold", "(self: Tensor) -> u8", Some(door_core_tensor_otsu_threshold)),
    ("core.tensor.pad", "(self: Tensor, count: usize, value: u8) -> Tensor", Some(door_core_tensor_pad)),
    ("core.tensor.perforate", "(self: Tensor, mask: Tensor, value: u8) -> Tensor", Some(door_core_tensor_perforate)),
    ("core.tensor.put", "(self: Tensor, flat: usize, value: i64) -> null # uncallable: mutates its argument in place", None),
    ("core.tensor.rot90", "(self: Tensor, k: usize, axes: (usize, usize)) -> Tensor", Some(door_core_tensor_rot90)),
    ("core.tensor.set", "(self: Tensor, multi: [usize], value: u8) -> null # uncallable: mutates its argument in place", None),
    ("core.tensor.size", "(self: Tensor) -> usize", Some(door_core_tensor_size)),
    ("core.tensor.slice", "(self: Tensor, axis: usize, index: usize) -> Tensor", Some(door_core_tensor_slice)),
    ("core.tensor.sum", "(self: Tensor) -> u64", Some(door_core_tensor_sum)),
    ("core.tensor.tile", "(self: Tensor, reps: [usize]) -> Tensor", Some(door_core_tensor_tile)),
    ("core.tensor.transpose", "(self: Tensor, a: usize, b: usize) -> Tensor", Some(door_core_tensor_transpose)),
    ("core.tensor.typed", "(shape: [usize], dtype: core.Dtype) -> Tensor", Some(door_core_tensor_typed)),
    ("core.tensor.u16", "(data: [u16], shape: [usize]) -> Tensor", Some(door_core_tensor_u16)),
    ("core.tensor.u16s", "(self: Tensor) -> [u16]", Some(door_core_tensor_u16s)),
    ("core.tensor.u32", "(data: [u32], shape: [usize]) -> Tensor", Some(door_core_tensor_u32)),
    ("core.tensor.u32s", "(self: Tensor) -> [u32]", Some(door_core_tensor_u32s)),
    ("core.tensor.u8", "(data: [u8], shape: [usize]) -> Tensor", Some(door_core_tensor_u8)),
    ("core.unpng", "(bytes: [u8]) -> (usize, usize, [[u8; 4]])", Some(door_core_unpng)),
    ("font.Glyph.height", "(self: font.Glyph) -> usize", Some(door_font_glyph_height)),
    ("font.Glyph.new", "(char: char, rows: [String]) -> font.Glyph", Some(door_font_glyph_new)),
    ("font.Glyph.width", "(self: font.Glyph) -> usize", Some(door_font_glyph_width)),
    ("font.all", "() -> [font.Glyph]", Some(door_font_all)),
    ("font.animate", "(text: str, pad: usize) -> font.Anim", Some(door_font_animate)),
    ("font.cycle", "(write: font.Anim, merge: [[usize]], hold: usize) -> font.Anim", Some(door_font_cycle)),
    ("font.digits", "() -> [font.Glyph]", Some(door_font_digits)),
    ("font.draft", "(rows: [String]) -> [[(usize, usize)]]", Some(door_font_draft)),
    ("font.extras", "() -> [font.Glyph]", Some(door_font_extras)),
    ("font.floor", "(rows: [String]) -> usize", Some(door_font_floor)),
    ("font.glyph", "(c: char) -> font.Glyph?", Some(door_font_glyph)),
    ("font.lower", "(rows: [str]) -> [String]", Some(door_font_lower)),
    ("font.lowers", "() -> [font.Glyph]", Some(door_font_lowers)),
    ("font.map", "() -> {char: [String]}", Some(door_font_map)),
    ("font.merge", "(text: str, pad: usize) -> [[usize]]", Some(door_font_merge)),
    ("font.name_of", "(c: char) -> String", Some(door_font_name_of)),
    ("font.path", "(c: char) -> [(usize, usize)]", Some(door_font_path)),
    ("font.paths.penned", "(c: char) -> [[(usize, usize)]]?", Some(door_font_paths_penned)),
    ("font.pens.all", "() -> [(char, [String])]", Some(door_font_pens_all)),
    ("font.raster", "(text: str) -> [[u8]]", Some(door_font_raster)),
    ("font.specials", "() -> [font.Glyph]", Some(door_font_specials)),
    ("font.strokes", "(c: char) -> [[(usize, usize)]]", Some(door_font_strokes)),
    ("font.supported", "() -> [char]", Some(door_font_supported)),
    ("font.trim", "(rows: [String]) -> [String]", Some(door_font_trim)),
    ("font.uppers", "() -> [font.Glyph]", Some(door_font_uppers)),
    ("gen.Group.all", "() -> [gen.Group; 5]", Some(door_gen_group_all)),
    ("gen.Parity.all", "() -> [gen.Parity; 3]", Some(door_gen_parity_all)),
    ("gen.Parity.keep", "(self: gen.Parity, n: usize) -> bool", Some(door_gen_parity_keep)),
    ("gen.Tile.check", "(self: gen.Tile) -> null", Some(door_gen_tile_check)),
    ("gen.Tile.degenerate", "(self: gen.Tile) -> bool", Some(door_gen_tile_degenerate)),
    ("gen.Tile.max_size", "(self: gen.Tile) -> usize", Some(door_gen_tile_max_size)),
    ("gen.Tile.new", "(group: gen.Group) -> gen.Tile", Some(door_gen_tile_new)),
    ("gen.Tile.resize", "(self: gen.Tile) -> null # uncallable: mutates its argument in place", None),
    ("gen.Tile.size", "(self: gen.Tile, width: usize, height: usize) -> gen.Tile", Some(door_gen_tile_size)),
    ("gen.background", "(seed: u64, width: usize, height: usize) -> [u8]", Some(door_gen_background)),
    ("gen.build.build_2d", "(tile: gen.Tile) -> Cell2d", Some(door_gen_build_build_2d)),
    ("gen.build.build_3d", "(tile: gen.Tile) -> Cell3d", Some(door_gen_build_build_3d)),
    ("gen.build.build_6d", "(hex: gen.build.HexTile) -> Cell6d", Some(door_gen_build_build_6d)),
    ("gen.build.create_2d", "(config: gen.draw.ConfigNd, rng: Rng(seed)) -> gen.Tile", Some(door_gen_build_create_2d)),
    ("gen.build.create_3d", "(config: gen.draw.ConfigNd, rng: Rng(seed)) -> gen.Tile", Some(door_gen_build_create_3d)),
    ("gen.build.create_6d", "(config: gen.draw.ConfigNd, rng: Rng(seed)) -> gen.build.HexTile", Some(door_gen_build_create_6d)),
    ("gen.build.random_tile_2d", "(max_size: usize, rng: Rng(seed)) -> gen.Tile", Some(door_gen_build_random_tile_2d)),
    ("gen.build.random_tile_3d", "(max_size: usize, rng: Rng(seed)) -> gen.Tile", Some(door_gen_build_random_tile_3d)),
    ("gen.build.random_tile_6d", "(max_size: usize, rng: Rng(seed)) -> gen.build.HexTile", Some(door_gen_build_random_tile_6d)),
    ("gen.classic_code", "(design: gen.recipe.Design) -> u128?", Some(door_gen_classic_code)),
    ("gen.classic_code_nd", "(design: gen.recipe.Design, dimension: usize) -> u128?", Some(door_gen_classic_code_nd)),
    ("gen.hex_key", "(length: usize, rng: Rng(seed)) -> String", Some(door_gen_hex_key)),
    ("gen.name.Tile.checked", "(self: gen.name.Tile) -> gen.name.Tile", Some(door_gen_name_tile_checked)),
    ("gen.name.Tile.from_file", "(text: str) -> gen.name.Tile", Some(door_gen_name_tile_from_file)),
    ("gen.name.Tile.from_json", "(text: str) -> gen.name.Tile", Some(door_gen_name_tile_from_json)),
    ("gen.name.Tile.from_url", "(text: str) -> gen.name.Tile", Some(door_gen_name_tile_from_url)),
    ("gen.name.Tile.of", "(recipe: gen.Tile) -> gen.name.Tile", Some(door_gen_name_tile_of)),
    ("gen.name.Tile.recipe", "(self: gen.name.Tile) -> gen.Tile", Some(door_gen_name_tile_recipe)),
    ("gen.name.Tile.to_file", "(self: gen.name.Tile) -> String", Some(door_gen_name_tile_to_file)),
    ("gen.name.Tile.to_id", "(self: gen.name.Tile) -> String", Some(door_gen_name_tile_to_id)),
    ("gen.name.Tile.to_json", "(self: gen.name.Tile) -> String", Some(door_gen_name_tile_to_json)),
    ("gen.name.Tile.to_mrly", "(self: gen.name.Tile) -> String", Some(door_gen_name_tile_to_mrly)),
    ("gen.name.Tile.to_url", "(self: gen.name.Tile) -> String", Some(door_gen_name_tile_to_url)),
    ("gen.random_design", "(rng: Rng(seed)) -> gen.recipe.Design", Some(door_gen_random_design)),
    ("gen.random_rotation", "(design: gen.recipe.Design, rng: Rng(seed)) -> u8", Some(door_gen_random_rotation)),
    ("gen.recipe.Design.all", "() -> [gen.recipe.Design; 16]", Some(door_gen_recipe_design_all)),
    ("gen.recipe.classics", "(dimension: usize) -> [gen.recipe.Design]", Some(door_gen_recipe_classics)),
    ("gen.recipe.generals", "(min_size: usize, max_size: usize, parity: gen.Parity) -> [usize]", Some(door_gen_recipe_generals)),
    ("gen.recipe.nestings", "(min_size: usize, max_size: usize, parity: gen.Parity) -> [[usize]]", Some(door_gen_recipe_nestings)),
    ("gen.recipe.powers", "(min_size: usize, max_size: usize, parity: gen.Parity) -> [(usize, usize)]", Some(door_gen_recipe_powers)),
    ("gen.recipe.products", "(min_size: usize, max_size: usize, count: usize, parity: gen.Parity) -> [[usize]]", Some(door_gen_recipe_products)),
    ("gen.recipe.size", "(number: i64, level: i64) -> usize?", Some(door_gen_recipe_size)),
    ("gen.tree_mask", "(n: usize) -> Tensor", Some(door_gen_tree_mask)),
    ("gen.variation.File.new", "(width: usize, height: usize) -> gen.variation.File", Some(door_gen_variation_file_new)),
    ("gen.variation.Variation.is_cover", "(self: gen.variation.Variation) -> bool", Some(door_gen_variation_variation_is_cover)),
    ("gen.variation.Variation.is_prime", "(self: gen.variation.Variation) -> bool", Some(door_gen_variation_variation_is_prime)),
    ("gen.variation.create", "(config: gen.variation.Config, rng: Rng(seed)) -> gen.variation.Variation", Some(door_gen_variation_create)),
    ("gen.variation.generate", "(variation: gen.variation.Variation, config: gen.variation.Config, rng: Rng(seed)) -> gen.variation.Variation", Some(door_gen_variation_generate)),
    ("gen.variation.render", "(variation: gen.variation.Variation, scale: usize, rng: Rng(seed)) -> gen.variation.Variation", Some(door_gen_variation_render)),
    ("life.Boundary.all", "() -> [life.Boundary; 2]", Some(door_life_boundary_all)),
    ("life.Boundary.wrap", "(self: life.Boundary) -> bool", Some(door_life_boundary_wrap)),
    ("life.Config.budget", "(self: life.Config) -> usize", Some(door_life_config_budget)),
    ("life.Config.counts", "(self: life.Config) -> ([usize], [usize])", Some(door_life_config_counts)),
    ("life.Config.new", "(mask: Cell2d, birth: life.Counts, survive: life.Counts) -> life.Config", Some(door_life_config_new)),
    ("life.Counts.drawn", "(seq: life.Source, zeros: bool, ones: bool) -> life.Counts", Some(door_life_counts_drawn)),
    ("life.Counts.list", "(counts: [usize]) -> life.Counts", Some(door_life_counts_list)),
    ("life.Counts.values", "(self: life.Counts, budget: usize) -> [usize]", Some(door_life_counts_values)),
    ("life.Fate.all", "() -> [life.Fate; 4]", Some(door_life_fate_all)),
    ("life.Life.last", "(self: life.Life) -> Cell2d?", Some(door_life_life_last)),
    ("life.Rule.boundary", "(self: life.Rule) -> life.Boundary", Some(door_life_rule_boundary)),
    ("life.Rule.checked", "(self: life.Rule) -> life.Rule", Some(door_life_rule_checked)),
    ("life.Rule.config", "(self: life.Rule, mask: Cell2d) -> life.Config", Some(door_life_rule_config)),
    ("life.Rule.from_file", "(text: str) -> life.Rule", Some(door_life_rule_from_file)),
    ("life.Rule.from_json", "(text: str) -> life.Rule", Some(door_life_rule_from_json)),
    ("life.Rule.from_url", "(text: str) -> life.Rule", Some(door_life_rule_from_url)),
    ("life.Rule.new", "(birth: life.Counts, survive: life.Counts, wrap: bool) -> life.Rule", Some(door_life_rule_new)),
    ("life.Rule.of", "(config: life.Config) -> life.Rule", Some(door_life_rule_of)),
    ("life.Rule.to_file", "(self: life.Rule) -> String", Some(door_life_rule_to_file)),
    ("life.Rule.to_id", "(self: life.Rule) -> String", Some(door_life_rule_to_id)),
    ("life.Rule.to_json", "(self: life.Rule) -> String", Some(door_life_rule_to_json)),
    ("life.Rule.to_mrly", "(self: life.Rule) -> String", Some(door_life_rule_to_mrly)),
    ("life.Rule.to_url", "(self: life.Rule) -> String", Some(door_life_rule_to_url)),
    ("life.Source.all", "() -> [life.Source; 23]", Some(door_life_source_all)),
    ("life.Source.designs", "() -> [life.Source; 17]", Some(door_life_source_designs)),
    ("life.Source.is_random", "(self: life.Source) -> bool", Some(door_life_source_is_random)),
    ("life.Source.name", "(self: life.Source) -> String", Some(door_life_source_name)),
    ("life.Source.numbers", "() -> [life.Source; 6]", Some(door_life_source_numbers)),
    ("life.Source.oeis", "(self: life.Source) -> String?", Some(door_life_source_oeis)),
    ("life.Source.parse", "(name: str) -> life.Source", Some(door_life_source_parse)),
    ("life.Source.read", "(text: str) -> (life.Source, str)?", Some(door_life_source_read)),
    ("life.affine", "(rule: u8) -> bool", Some(door_life_affine)),
    ("life.animate", "(seed: Cell2d, config: life.Config) -> life.Life", Some(door_life_animate)),
    ("life.churn", "(grids: [Cell2d]) -> f64", Some(door_life_churn)),
    ("life.corner_bits", "(rule: u8) -> [u8]", Some(door_life_corner_bits)),
    ("life.counts", "(seq: life.Source, max_neighbors: usize, include_zeros: bool, include_ones: bool) -> [usize]", Some(door_life_counts)),
    ("life.crop", "(grids: [Cell2d]) -> [Cell2d]", Some(door_life_crop)),
    ("life.cube_orbit", "(rule: u8) -> [u8]", Some(door_life_cube_orbit)),
    ("life.design_mask", "(dimension: usize, code: Code, number: usize, level: usize) -> Tensor", Some(door_life_design_mask)),
    ("life.elementary.output", "(rule: u8, l: u8, c: u8, r: u8) -> u8", Some(door_life_elementary_output)),
    ("life.entropy", "(grid: Cell2d) -> i64", Some(door_life_entropy)),
    ("life.frames", "(grids: [Cell2d], scale: usize) -> [[u8]]", Some(door_life_frames)),
    ("life.gasket", "(rule: u8) -> String?", Some(door_life_gasket)),
    ("life.genus", "(rule: u8) -> String", Some(door_life_genus)),
    ("life.heatmap", "(grids: [Cell2d], scale: usize) -> [[u8]]", Some(door_life_heatmap)),
    ("life.history", "(row: [u8], rule: u8, steps: usize, wrap: bool) -> Tensor", Some(door_life_history)),
    ("life.lambda", "(rule: u8) -> f64", Some(door_life_lambda)),
    ("life.lattice_index", "(mask: Tensor) -> usize", Some(door_life_lattice_index)),
    ("life.mask_offsets", "(mask: Tensor) -> [[i64]]", Some(door_life_mask_offsets)),
    ("life.moore", "() -> Cell2d", Some(door_life_moore)),
    ("life.movie", "(grids: [Cell2d], scale: usize, delay: usize) -> [u8]", Some(door_life_movie)),
    ("life.next_grid", "(cell: Cell2d, birth: [usize], survive: [usize], mask: Tensor, boundary: life.Boundary) -> Cell2d", Some(door_life_next_grid)),
    ("life.npn_class", "(rule: u8) -> [u8]", Some(door_life_npn_class)),
    ("life.outer_totalistic", "(rule: u8) -> ([usize], [usize])?", Some(door_life_outer_totalistic)),
    ("life.popcount", "(rule: u8) -> u32", Some(door_life_popcount)),
    ("life.render.frame", "(grid: Cell2d, scale: usize) -> [u8]", Some(door_life_render_frame)),
    ("life.reversible", "(rule: u8) -> bool", Some(door_life_reversible)),
    ("life.rule_degree", "(rule: u8) -> i32", Some(door_life_rule_degree)),
    ("life.rule_name", "(rule: u8) -> String", Some(door_life_rule_name)),
    ("life.single_seed", "(rule: u8, steps: usize) -> Tensor", Some(door_life_single_seed)),
    ("life.source.sequence", "(seq: life.Source, limit: usize) -> [usize]", Some(door_life_source_sequence)),
    ("life.step", "(row: [u8], rule: u8, wrap: bool) -> [u8]", Some(door_life_step)),
    ("life.surjective", "(rule: u8) -> bool", Some(door_life_surjective)),
    ("life.tessellate", "(grids: [Cell2d], min_canvas: usize) -> [Cell2d]", Some(door_life_tessellate)),
    ("life.wolfram_class", "(rule: u8) -> [u8]", Some(door_life_wolfram_class)),
    ("math.atoms.carpet_2d", "(n: usize) -> Tensor", Some(door_math_atoms_carpet_2d)),
    ("math.atoms.carpet_3d", "(n: usize) -> Tensor", Some(door_math_atoms_carpet_3d)),
    ("math.atoms.carpet_nd", "(n: usize, rank: usize) -> Tensor", Some(door_math_atoms_carpet_nd)),
    ("math.atoms.dust_2d", "(n: usize) -> Tensor", Some(door_math_atoms_dust_2d)),
    ("math.atoms.dust_3d", "(n: usize) -> Tensor", Some(door_math_atoms_dust_3d)),
    ("math.atoms.dust_nd", "(n: usize, rank: usize) -> Tensor", Some(door_math_atoms_dust_nd)),
    ("math.atoms.hline_2d", "(n: usize) -> Tensor", Some(door_math_atoms_hline_2d)),
    ("math.atoms.htree_2d", "(n: usize) -> Tensor", Some(door_math_atoms_htree_2d)),
    ("math.atoms.line_nd", "(n: usize, rank: usize, axis: usize) -> Tensor", Some(door_math_atoms_line_nd)),
    ("math.atoms.net_2d", "(n: usize) -> Tensor", Some(door_math_atoms_net_2d)),
    ("math.atoms.net_3d", "(n: usize) -> Tensor", Some(door_math_atoms_net_3d)),
    ("math.atoms.net_nd", "(n: usize, rank: usize) -> Tensor", Some(door_math_atoms_net_nd)),
    ("math.atoms.noise_2d", "(n: usize, density: f64, rng: Rng(seed)) -> Tensor", Some(door_math_atoms_noise_2d)),
    ("math.atoms.noise_3d", "(n: usize, density: f64, rng: Rng(seed)) -> Tensor", Some(door_math_atoms_noise_3d)),
    ("math.atoms.ones_2d", "(n: usize) -> Tensor", Some(door_math_atoms_ones_2d)),
    ("math.atoms.ones_3d", "(n: usize) -> Tensor", Some(door_math_atoms_ones_3d)),
    ("math.atoms.point_2d", "(n: usize) -> Tensor", Some(door_math_atoms_point_2d)),
    ("math.atoms.point_3d", "(n: usize) -> Tensor", Some(door_math_atoms_point_3d)),
    ("math.atoms.point_nd", "(n: usize, rank: usize) -> Tensor", Some(door_math_atoms_point_nd)),
    ("math.atoms.star_2d", "(n: usize) -> Tensor", Some(door_math_atoms_star_2d)),
    ("math.atoms.star_3d", "(n: usize) -> Tensor", Some(door_math_atoms_star_3d)),
    ("math.atoms.star_nd", "(n: usize, rank: usize) -> Tensor", Some(door_math_atoms_star_nd)),
    ("math.atoms.tree_nd", "(n: usize, rank: usize, axis: usize) -> Tensor", Some(door_math_atoms_tree_nd)),
    ("math.atoms.vline_2d", "(n: usize) -> Tensor", Some(door_math_atoms_vline_2d)),
    ("math.atoms.void_2d", "(n: usize) -> Tensor", Some(door_math_atoms_void_2d)),
    ("math.atoms.void_3d", "(n: usize) -> Tensor", Some(door_math_atoms_void_3d)),
    ("math.atoms.void_nd", "(n: usize, rank: usize) -> Tensor", Some(door_math_atoms_void_nd)),
    ("math.atoms.vtree_2d", "(n: usize) -> Tensor", Some(door_math_atoms_vtree_2d)),
    ("math.atoms.xline_3d", "(n: usize) -> Tensor", Some(door_math_atoms_xline_3d)),
    ("math.atoms.xtree_3d", "(n: usize) -> Tensor", Some(door_math_atoms_xtree_3d)),
    ("math.atoms.yline_3d", "(n: usize) -> Tensor", Some(door_math_atoms_yline_3d)),
    ("math.atoms.ytree_3d", "(n: usize) -> Tensor", Some(door_math_atoms_ytree_3d)),
    ("math.atoms.zeros_2d", "(n: usize) -> Tensor", Some(door_math_atoms_zeros_2d)),
    ("math.atoms.zeros_3d", "(n: usize) -> Tensor", Some(door_math_atoms_zeros_3d)),
    ("math.atoms.zline_3d", "(n: usize) -> Tensor", Some(door_math_atoms_zline_3d)),
    ("math.atoms.ztree_3d", "(n: usize) -> Tensor", Some(door_math_atoms_ztree_3d)),
    ("math.bang.Design.anf", "(self: math.bang.Design) -> String", Some(door_math_bang_design_anf)),
    ("math.bang.Design.degree", "(self: math.bang.Design) -> i32", Some(door_math_bang_design_degree)),
    ("math.bang.Design.name", "(self: math.bang.Design) -> String", Some(door_math_bang_design_name)),
    ("math.bang.Design.rule", "(self: math.bang.Design) -> [[u8]]", Some(door_math_bang_design_rule)),
    ("math.bang.MagicLayer.new", "(design: math.name.Bang, number: usize) -> math.bang.MagicLayer", Some(door_math_bang_magiclayer_new)),
    ("math.bang.Universe.all", "(self: math.bang.Universe) -> [math.bang.Design]", Some(door_math_bang_universe_all)),
    ("math.bang.Universe.canonical", "(self: math.bang.Universe) -> [math.bang.Design]", Some(door_math_bang_universe_canonical)),
    ("math.bang.Universe.design", "(self: math.bang.Universe, code: Code) -> math.bang.Design", Some(door_math_bang_universe_design)),
    ("math.bang.Universe.distinct", "(self: math.bang.Universe) -> usize", Some(door_math_bang_universe_distinct)),
    ("math.bang.Universe.new", "(dimension: usize) -> math.bang.Universe", Some(door_math_bang_universe_new)),
    ("math.bang.bang", "(dimension: usize) -> math.bang.Universe", Some(door_math_bang_bang)),
    ("math.bang.baseq.axis_maps", "(base: usize) -> [[usize]]", Some(door_math_bang_baseq_axis_maps)),
    ("math.bang.baseq.bracelets", "(max_base: usize) -> [u128]", Some(door_math_bang_baseq_bracelets)),
    ("math.bang.baseq.canonical", "(group: [[usize]], code: Code) -> Code", Some(door_math_bang_baseq_canonical)),
    ("math.bang.baseq.carry", "(element: [usize], code: Code) -> Code", Some(door_math_bang_baseq_carry)),
    ("math.bang.baseq.class_sequence", "(max_dimension: usize) -> [u128]", Some(door_math_bang_baseq_class_sequence)),
    ("math.bang.baseq.classes", "(dimension: usize) -> u128", Some(door_math_bang_baseq_classes)),
    ("math.bang.baseq.distinct_designs", "(base: usize, dimension: usize) -> u128", Some(door_math_bang_baseq_distinct_designs)),
    ("math.bang.baseq.even_fill_is_balanced", "(number: usize, dimension: usize, popcount: u128) -> u128", Some(door_math_bang_baseq_even_fill_is_balanced)),
    ("math.bang.baseq.fill_from_corners", "(filled: [[u8]], number: usize, dimension: usize) -> u128", Some(door_math_bang_baseq_fill_from_corners)),
    ("math.bang.baseq.group", "(base: usize, dimension: usize) -> [[usize]]", Some(door_math_bang_baseq_group)),
    ("math.bang.baseq.group_order", "(base: usize, dimension: usize) -> u128", Some(door_math_bang_baseq_group_order)),
    ("math.bang.baseq.orbit", "(group: [[usize]], code: Code) -> [Code]", Some(door_math_bang_baseq_orbit)),
    ("math.bang.baseq.predicted_group_order", "(base: usize, dimension: usize) -> u128", Some(door_math_bang_baseq_predicted_group_order)),
    ("math.bang.baseq.representatives", "(base: usize, dimension: usize) -> [(Code, usize)]", Some(door_math_bang_baseq_representatives)),
    ("math.bang.baseq.sequence", "(base: usize, max_dimension: usize) -> [u128]", Some(door_math_bang_baseq_sequence)),
    ("math.bang.baseq.total_designs", "(base: usize, dimension: usize) -> u128", Some(door_math_bang_baseq_total_designs)),
    ("math.bang.catalog.antis", "(dimension: usize) -> [gen.recipe.Design]", Some(door_math_bang_catalog_antis)),
    ("math.bang.code.get", "(self: Code) -> u128", Some(door_math_bang_code_get)),
    ("math.bang.code_to_corners", "(code: Code, dimension: usize, base: usize) -> [[u8]]", Some(door_math_bang_code_to_corners)),
    ("math.bang.corners", "(dimension: usize) -> [[u8]]", Some(door_math_bang_corners)),
    ("math.bang.corners_to_code", "(filled: [[u8]], dimension: usize, base: usize) -> Code", Some(door_math_bang_corners_to_code)),
    ("math.bang.factory.create", "(code: Code, number: usize, dimension: usize, base: usize, level: usize) -> Tensor", Some(door_math_bang_factory_create)),
    ("math.bang.factory.create_from_corners", "(filled: [[u8]], number: usize, dimension: usize, base: usize, level: usize) -> Tensor", Some(door_math_bang_factory_create_from_corners)),
    ("math.bang.factory.create_named", "(spec: str, number: usize, level: usize) -> Tensor", Some(door_math_bang_factory_create_named)),
    ("math.bang.factory.residue_corners", "(dimension: usize, base: usize) -> [[u8]]", Some(door_math_bang_factory_residue_corners)),
    ("math.bang.factory.total_codes", "(dimension: usize, base: usize) -> Code", Some(door_math_bang_factory_total_codes)),
    ("math.bang.levels_code", "(dimension: usize, base: usize, levels: [usize]) -> Code", Some(door_math_bang_levels_code)),
    ("math.bang.magic", "(layers: [math.bang.MagicLayer]) -> Tensor", Some(door_math_bang_magic)),
    ("math.bang.magic_named", "(layers: [(str, usize)]) -> Tensor", Some(door_math_bang_magic_named)),
    ("math.bang.sources", "(catalog: gen.recipe.Catalog, dimension: usize) -> [gen.recipe.Source]", Some(door_math_bang_sources)),
    ("math.bang.symmetries", "(dimension: usize) -> [([usize], [u8])]", Some(door_math_bang_symmetries)),
    ("math.bang.total_exposure", "(code: Code, dimension: usize) -> bool", Some(door_math_bang_total_exposure)),
    ("math.bang.touches_every_corner", "(code: Code, dimension: usize) -> bool", Some(door_math_bang_touches_every_corner)),
    ("math.bang.universe.anf", "(code: Code, dimension: usize) -> [u8]", Some(door_math_bang_universe_anf)),
    ("math.bang.universe.anf_string", "(code: Code, dimension: usize) -> String", Some(door_math_bang_universe_anf_string)),
    ("math.bang.universe.apply", "(element: ([usize], [u8]), corner: [u8]) -> [u8]", Some(door_math_bang_universe_apply)),
    ("math.bang.universe.corner_index", "(corner: [u8]) -> usize", Some(door_math_bang_universe_corner_index)),
    ("math.bang.universe.degree", "(code: Code, dimension: usize) -> i32", Some(door_math_bang_universe_degree)),
    ("math.bang.universe.orbit", "(code: Code, dimension: usize) -> [Code]", Some(door_math_bang_universe_orbit)),
    ("math.bang.universe.permutations", "(n: usize) -> [[usize]]", Some(door_math_bang_universe_permutations)),
    ("math.bang.universe_codes", "(dimension: usize) -> [u128]", Some(door_math_bang_universe_codes)),
    ("math.bang.word.Schedule.all", "() -> [math.bang.word.Schedule; 3]", Some(door_math_bang_word_schedule_all)),
    ("math.bang.word.Schedule.frequencies", "(self: math.bang.word.Schedule) -> (f64, f64)", Some(door_math_bang_word_schedule_frequencies)),
    ("math.bang.word.Schedule.place", "(self: math.bang.word.Schedule, index: usize) -> usize", Some(door_math_bang_word_schedule_place)),
    ("math.bang.word.components", "(layers: [math.bang.MagicLayer]) -> u128", Some(door_math_bang_word_components)),
    ("math.bang.word.constant_functional", "(layers: [math.bang.MagicLayer]) -> f64", Some(door_math_bang_word_constant_functional)),
    ("math.bang.word.dimension", "(layers: [math.bang.MagicLayer]) -> f64", Some(door_math_bang_word_dimension)),
    ("math.bang.word.fill", "(layers: [math.bang.MagicLayer]) -> u128", Some(door_math_bang_word_fill)),
    ("math.bang.word.fills", "(layers: [math.bang.MagicLayer]) -> [u128]", Some(door_math_bang_word_fills)),
    ("math.bang.word.letter", "(layer: math.bang.MagicLayer) -> math.bang.word.Letter", Some(door_math_bang_word_letter)),
    ("math.bang.word.native", "(layers: [math.bang.MagicLayer]) -> bool", Some(door_math_bang_word_native)),
    ("math.bang.word.period", "(layers: [math.bang.MagicLayer]) -> usize", Some(door_math_bang_word_period)),
    ("math.bang.word.prefixes", "(layers: [math.bang.MagicLayer]) -> [math.bang.word.Counts]", Some(door_math_bang_word_prefixes)),
    ("math.bang.word.rates", "(layers: [math.bang.MagicLayer]) -> [(f64, f64)]", Some(door_math_bang_word_rates)),
    ("math.bang.word.side", "(layers: [math.bang.MagicLayer]) -> u128", Some(door_math_bang_word_side)),
    ("math.bang.word.spell", "(schedule: math.bang.word.Schedule, pair: (math.bang.MagicLayer, math.bang.MagicLayer), length: usize) -> [math.bang.MagicLayer]", Some(door_math_bang_word_spell)),
    ("math.bang.word.staircase", "(depth: usize) -> [math.bang.MagicLayer]", Some(door_math_bang_word_staircase)),
    ("math.bang.word.thue_morse", "(index: usize) -> usize", Some(door_math_bang_word_thue_morse)),
    ("math.cell.census.edges", "(cell: Cell) -> usize", Some(door_math_cell_census_edges)),
    ("math.cell.census.exposure", "(cell: Cell) -> u128", Some(door_math_cell_census_exposure)),
    ("math.cell.census.fills", "(cell: Cell) -> usize", Some(door_math_cell_census_fills)),
    ("math.cell.census.vertices", "(cell: Cell) -> usize", Some(door_math_cell_census_vertices)),
    ("math.cell.census.voids", "(cell: Cell) -> usize", Some(door_math_cell_census_voids)),
    ("math.cell.geometry.merge_reps", "(cells: [Cell], reps: [usize]) -> Cell", Some(door_math_cell_geometry_merge_reps)),
    ("math.cell.geometry.perforate", "(mask: Tensor, cell: Cell, value: u8) -> Cell", Some(door_math_cell_geometry_perforate)),
    ("math.cell.grow", "(pattern: Tensor, level: usize) -> Cell", Some(door_math_cell_grow)),
    ("math.cell.models.anti", "(self: Cell) -> Cell", Some(door_math_cell_models_anti)),
    ("math.cell.models.binarize", "(self: Cell, threshold: u8) -> Cell", Some(door_math_cell_models_binarize)),
    ("math.cell.models.binarize_otsu", "(self: Cell) -> Cell", Some(door_math_cell_models_binarize_otsu)),
    ("math.cell.models.blur", "(self: Cell, mask: Tensor, wrap: bool) -> Cell", Some(door_math_cell_models_blur)),
    ("math.cell.models.combine", "(self: Cell, other: Cell) -> Cell", Some(door_math_cell_models_combine)),
    ("math.cell.models.counting_dtype", "(mask: Tensor) -> core.Dtype", Some(door_math_cell_models_counting_dtype)),
    ("math.cell.models.depth", "(self: Cell3d) -> usize", Some(door_math_cell_models_depth)),
    ("math.cell.models.dtype_for", "(peak: i64) -> core.Dtype", Some(door_math_cell_models_dtype_for)),
    ("math.cell.models.fractal", "(self: Cell, level: usize) -> Cell", Some(door_math_cell_models_fractal)),
    ("math.cell.models.height", "(self: Cell) -> usize", Some(door_math_cell_models_height)),
    ("math.cell.models.invert", "(self: Cell) -> Cell", Some(door_math_cell_models_invert)),
    ("math.cell.models.layers", "(self: Cell) -> Cell", Some(door_math_cell_models_layers)),
    ("math.cell.models.neighbors", "(self: Cell, mask: Tensor, target: u8, wrap: bool) -> Cell", Some(door_math_cell_models_neighbors)),
    ("math.cell.models.new", "(types: Tensor) -> Cell", Some(door_math_cell_models_new)),
    ("math.cell.models.orient", "(self: Cell3d, index: usize) -> Cell3d", Some(door_math_cell_models_orient)),
    ("math.cell.models.pad", "(self: Cell, count: usize, value: u8) -> Cell", Some(door_math_cell_models_pad)),
    ("math.cell.models.paint", "(self: Cell, mapping: {u8: [Color]}, mode: core.Mode, rng: Rng(seed)?) -> Cell", Some(door_math_cell_models_paint)),
    ("math.cell.models.perforate", "(self: Cell, mask: Tensor, value: u8) -> Cell", Some(door_math_cell_models_perforate)),
    ("math.cell.models.rotate", "(self: Cell2d, k: usize) -> Cell2d | (self: Cell3d, k: usize, axes: (usize, usize)) -> Cell3d", Some(door_math_cell_models_rotate)),
    ("math.cell.models.tile", "(self: Cell2d, width: usize, height: usize) -> Cell2d | (self: Cell3d, width: usize, height: usize, depth: usize) -> Cell3d", Some(door_math_cell_models_tile)),
    ("math.cell.models.types", "(self: Cell) -> Tensor", Some(door_math_cell_models_types)),
    ("math.cell.models.width", "(self: Cell) -> usize", Some(door_math_cell_models_width)),
    ("math.cell.paint", "(cell: Cell, custom: {u8: [Color]}?, mode: core.Mode?, rng: Rng(seed)?) -> Cell", Some(door_math_cell_paint)),
    ("math.cell.serializer.byte_cube", "(value: Json) -> [[[u8]]]", Some(door_math_cell_serializer_byte_cube)),
    ("math.cell.serializer.byte_grid", "(value: Json) -> [[u8]]", Some(door_math_cell_serializer_byte_grid)),
    ("math.cell.serializer.color_grid", "(value: Json) -> [[[u8; 4]]]", Some(door_math_cell_serializer_color_grid)),
    ("math.cell.serializer.count_cube", "(value: Json) -> [i64]", Some(door_math_cell_serializer_count_cube)),
    ("math.cell.serializer.count_grid", "(value: Json) -> [i64]", Some(door_math_cell_serializer_count_grid)),
    ("math.cell.serializer.parse", "(text: str) -> Json", Some(door_math_cell_serializer_parse)),
    ("math.cell.serializer.tag_layer", "(counts: [i64], shape: [usize]) -> Tensor", Some(door_math_cell_serializer_tag_layer)),
    ("math.cell.serializer.types_field", "(data: Json) -> Json", Some(door_math_cell_serializer_types_field)),
    ("math.counts.Exposure.at", "(self: math.counts.Exposure, level: u32) -> u128?", Some(door_math_counts_exposure_at)),
    ("math.counts.Exposure.from_corners", "(filled: [[u8]], number: usize, dimension: usize, base: usize) -> math.counts.Exposure", Some(door_math_counts_exposure_from_corners)),
    ("math.counts.Exposure.of_tile", "(tile: Tensor) -> math.counts.Exposure", Some(door_math_counts_exposure_of_tile)),
    ("math.counts.Exposure.recurrence", "(self: math.counts.Exposure) -> [i128]", Some(door_math_counts_exposure_recurrence)),
    ("math.counts.centered_hexagonal", "(m: usize) -> u128", Some(door_math_counts_centered_hexagonal)),
    ("math.counts.cut_fills", "(code: Code, number: usize, level: u32) -> u128", Some(door_math_counts_cut_fills)),
    ("math.counts.cut_voids", "(code: Code, number: usize, level: u32) -> u128", Some(door_math_counts_cut_voids)),
    ("math.counts.dimension", "(code: Code, number: usize, base_dimension: usize, base: usize) -> f64", Some(door_math_counts_dimension)),
    ("math.counts.edges_of_tile", "(tile: Tensor, level: u32) -> u128?", Some(door_math_counts_edges_of_tile)),
    ("math.counts.exposure", "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128", Some(door_math_counts_exposure)),
    ("math.counts.exposure_of_tile", "(tile: Tensor, level: u32) -> u128?", Some(door_math_counts_exposure_of_tile_2)),
    ("math.counts.exposure_recurrence", "(tile: Tensor) -> [i128]", Some(door_math_counts_exposure_recurrence_2)),
    ("math.counts.fill", "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128", Some(door_math_counts_fill)),
    ("math.counts.fill_from_corners", "(filled: [[u8]], number: usize, _dimension: usize, level: u32, base: usize) -> u128", Some(door_math_counts_fill_from_corners)),
    ("math.counts.grid", "(number: usize, dimension: usize, level: u32) -> u128", Some(door_math_counts_grid)),
    ("math.counts.ladder.cap", "(base: usize) -> usize", Some(door_math_counts_ladder_cap)),
    ("math.counts.ladder.carry_matrix", "(base: usize, dimension: usize) -> [[i128]]", Some(door_math_counts_ladder_carry_matrix)),
    ("math.counts.ladder.characteristic", "(rows: [[i128]]) -> [i128]", Some(door_math_counts_ladder_characteristic)),
    ("math.counts.ladder.determinant", "(rows: [[i128]]) -> i128", Some(door_math_counts_ladder_determinant)),
    ("math.counts.ladder.digit_polynomial", "(base: usize, dimension: usize) -> [i128]", Some(door_math_counts_ladder_digit_polynomial)),
    ("math.counts.ladder.even_block", "(base: usize, dimension: usize) -> [[i128]]", Some(door_math_counts_ladder_even_block)),
    ("math.counts.ladder.fill", "(base: usize, dimension: usize) -> i128", Some(door_math_counts_ladder_fill)),
    ("math.counts.ladder.ladder", "(base: usize, dimension: usize, levels: usize) -> [i128]", Some(door_math_counts_ladder_ladder)),
    ("math.counts.ladder.perron", "(rows: [[i128]]) -> f64", Some(door_math_counts_ladder_perron)),
    ("math.counts.ladder.sign", "(base: usize, dimension: usize) -> i32", Some(door_math_counts_ladder_sign)),
    ("math.counts.ladder.spectral_ratio", "(base: usize, dimension: usize) -> f64?", Some(door_math_counts_ladder_spectral_ratio)),
    ("math.counts.ladder.trace", "(rows: [[i128]]) -> i128", Some(door_math_counts_ladder_trace)),
    ("math.counts.limit", "(code: Code, dimension: usize, level: u32, base: usize) -> (u128, u128)", Some(door_math_counts_limit)),
    ("math.counts.pairs", "(tile: Tensor) -> [(u128, u128)]", Some(door_math_counts_pairs)),
    ("math.counts.positions", "(residue: usize, number: usize, base: usize) -> u128", Some(door_math_counts_positions)),
    ("math.counts.pro_fills", "(code: Code, number: usize, level: u32) -> u128", Some(door_math_counts_pro_fills)),
    ("math.counts.pro_voids", "(code: Code, number: usize, level: u32) -> u128", Some(door_math_counts_pro_voids)),
    ("math.counts.profile_of_tile", "(tile: Tensor, level: u32) -> [u128]", Some(door_math_counts_profile_of_tile)),
    ("math.counts.ratio", "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> f64", Some(door_math_counts_ratio)),
    ("math.counts.rational", "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> (u128, u128)", Some(door_math_counts_rational)),
    ("math.counts.six.grid_triangles", "(number: usize, level: u32) -> u128", Some(door_math_counts_six_grid_triangles)),
    ("math.counts.six.solid_slice_boundary", "(number: usize) -> u128", Some(door_math_counts_six_solid_slice_boundary)),
    ("math.counts.six.solid_slice_core_edges", "(number: usize) -> u128", Some(door_math_counts_six_solid_slice_core_edges)),
    ("math.counts.six.solid_slice_core_nodes", "(number: usize) -> u128", Some(door_math_counts_six_solid_slice_core_nodes)),
    ("math.counts.six.solid_slice_edges", "(number: usize) -> u128", Some(door_math_counts_six_solid_slice_edges)),
    ("math.counts.six.solid_slice_interior", "(number: usize) -> u128", Some(door_math_counts_six_solid_slice_interior)),
    ("math.counts.six.solid_slice_triangles", "(number: usize) -> u128", Some(door_math_counts_six_solid_slice_triangles)),
    ("math.counts.six.solid_slice_vertices", "(number: usize) -> u128", Some(door_math_counts_six_solid_slice_vertices)),
    ("math.counts.surface", "(code: Code, number: usize, level: u32, base: usize) -> u128", Some(door_math_counts_surface)),
    ("math.counts.void", "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128", Some(door_math_counts_void)),
    ("math.graph.Layout.energy", "(self: math.graph.Layout) -> f64 # uncallable: math::graph::Layout has no Deserialize", None),
    ("math.graph.Layout.from_network", "(network: math.graph.Network, seed: u64) -> math.graph.Layout # uncallable: math::graph::Layout has no Serialize", None),
    ("math.graph.Layout.ideal", "(self: math.graph.Layout) -> f64 # uncallable: math::graph::Layout has no Deserialize", None),
    ("math.graph.Layout.moved", "(self: math.graph.Layout) -> f64 # uncallable: math::graph::Layout has no Deserialize", None),
    ("math.graph.Layout.new", "(positions: [f64], branches: [(usize, usize)], dim: usize, seed: u64) -> math.graph.Layout # uncallable: math::graph::Layout has no Serialize", None),
    ("math.graph.Layout.nodes", "(self: math.graph.Layout) -> usize # uncallable: math::graph::Layout has no Deserialize", None),
    ("math.graph.Layout.positions", "(self: math.graph.Layout) -> [f64] # uncallable: math::graph::Layout has no Deserialize", None),
    ("math.graph.Layout.step", "(self: math.graph.Layout, ticks: usize) -> f64 # uncallable: mutates its argument in place", None),
    ("math.graph.Layout.temperature", "(self: math.graph.Layout) -> f64 # uncallable: math::graph::Layout has no Deserialize", None),
    ("math.graph.Layout.ticks", "(self: math.graph.Layout) -> usize # uncallable: math::graph::Layout has no Deserialize", None),
    ("math.graph.Network.add_branch", "(self: math.graph.Network, parent: usize, child: usize, radius: f64) -> null # uncallable: mutates its argument in place", None),
    ("math.graph.Network.add_node", "(self: math.graph.Network, position: [f64]) -> usize # uncallable: mutates its argument in place", None),
    ("math.graph.Network.adjacency", "(self: math.graph.Network) -> {usize: [usize]}", Some(door_math_graph_network_adjacency)),
    ("math.graph.Network.degree", "(self: math.graph.Network) -> [usize]", Some(door_math_graph_network_degree)),
    ("math.graph.Network.new", "(dim: usize) -> math.graph.Network", Some(door_math_graph_network_new)),
    ("math.graph.census", "(network: math.graph.Network) -> math.graph.Census", Some(door_math_graph_census)),
    ("math.graph.components", "(network: math.graph.Network) -> usize", Some(door_math_graph_components)),
    ("math.graph.core_graph", "(grid: Tensor) -> math.graph.Network", Some(door_math_graph_core_graph)),
    ("math.graph.edge_graph", "(grid: Tensor) -> math.graph.Network", Some(door_math_graph_edge_graph)),
    ("math.graph.fractal_dimension", "(network: math.graph.Network, samples: usize) -> f64", Some(door_math_graph_fractal_dimension)),
    ("math.graph.junctions", "(network: math.graph.Network) -> usize", Some(door_math_graph_junctions)),
    ("math.graph.largest_component", "(network: math.graph.Network) -> math.graph.Network", Some(door_math_graph_largest_component)),
    ("math.graph.roles", "(network: math.graph.Network) -> [math.graph.Role]", Some(door_math_graph_roles)),
    ("math.graph.tips", "(network: math.graph.Network) -> usize", Some(door_math_graph_tips)),
    ("math.graph.total_length", "(network: math.graph.Network) -> f64", Some(door_math_graph_total_length)),
    ("math.graph.tunnel_graph", "(grid: Tensor) -> math.graph.Network", Some(door_math_graph_tunnel_graph)),
    ("math.moire.Field.as_f64", "(self: math.moire.Field) -> [f64]", Some(door_math_moire_field_as_f64)),
    ("math.moire.Field.from_data", "(data: [f32], size: usize) -> math.moire.Field", Some(door_math_moire_field_from_data)),
    ("math.moire.Field.max", "(self: math.moire.Field) -> f32", Some(door_math_moire_field_max)),
    ("math.moire.Field.mean", "(self: math.moire.Field) -> f64", Some(door_math_moire_field_mean)),
    ("math.moire.Field.min", "(self: math.moire.Field) -> f32", Some(door_math_moire_field_min)),
    ("math.moire.Field.new", "(size: usize) -> math.moire.Field", Some(door_math_moire_field_new)),
    ("math.moire.Field.normalized", "(self: math.moire.Field, symmetric: bool) -> [f32]", Some(door_math_moire_field_normalized)),
    ("math.moire.Layer.new", "(spec: math.moire.Spec, number: usize) -> math.moire.Layer", Some(door_math_moire_layer_new)),
    ("math.moire.Preset.carpet", "(limit: usize) -> math.moire.Preset", Some(door_math_moire_preset_carpet)),
    ("math.moire.Preset.field", "(self: math.moire.Preset, size: usize) -> math.moire.Field # uncallable: math::moire::Preset has no Deserialize", None),
    ("math.moire.Preset.heatmap", "(limit: usize) -> math.moire.Preset", Some(door_math_moire_preset_heatmap)),
    ("math.moire.Preset.hive", "(limit: usize) -> math.moire.Preset", Some(door_math_moire_preset_hive)),
    ("math.moire.Preset.weave", "(limit: usize) -> math.moire.Preset", Some(door_math_moire_preset_weave)),
    ("math.moire.Spec.new", "(code: u128, base: usize, dimension: usize) -> math.moire.Spec", Some(door_math_moire_spec_new)),
    ("math.moire.Volume.at", "(self: math.moire.Volume, x: usize, y: usize, z: usize) -> f32", Some(door_math_moire_volume_at)),
    ("math.moire.Volume.count", "(self: math.moire.Volume, level: f32) -> usize", Some(door_math_moire_volume_count)),
    ("math.moire.Volume.from_data", "(data: [f32], size: usize) -> math.moire.Volume", Some(door_math_moire_volume_from_data)),
    ("math.moire.Volume.max", "(self: math.moire.Volume) -> f32", Some(door_math_moire_volume_max)),
    ("math.moire.Volume.min", "(self: math.moire.Volume) -> f32", Some(door_math_moire_volume_min)),
    ("math.moire.Volume.new", "(size: usize) -> math.moire.Volume", Some(door_math_moire_volume_new)),
    ("math.moire.Volume.plane", "(self: math.moire.Volume, frame: math.moire.Frame, out: usize) -> ([f32], [u8])", Some(door_math_moire_volume_plane)),
    ("math.moire.Volume.sample", "(self: math.moire.Volume, p: [f64; 3]) -> f32?", Some(door_math_moire_volume_sample)),
    ("math.moire.Volume.solid", "(self: math.moire.Volume, level: f32) -> Tensor", Some(door_math_moire_volume_solid)),
    ("math.moire.all", "(limit: usize) -> [math.moire.Preset]", Some(door_math_moire_all)),
    ("math.moire.frame", "(normal: [f64; 3], offset: f64) -> math.moire.Frame", Some(door_math_moire_frame)),
    ("math.moire.layer", "(params: math.moire.Layer) -> [bool]", Some(door_math_moire_layer)),
    ("math.moire.named", "(name: str, limit: usize) -> math.moire.Preset", Some(door_math_moire_named)),
    ("math.moire.pairs.correlation", "(m: usize, n: usize) -> f64", Some(door_math_moire_pairs_correlation)),
    ("math.moire.pairs.sampled", "(m: usize, n: usize) -> f64", Some(door_math_moire_pairs_sampled)),
    ("math.moire.pairs.witness", "(scale: usize) -> math.moire.pairs.Witness", Some(door_math_moire_pairs_witness)),
    ("math.moire.render", "(field: math.moire.Field, colorizer: core.Colorizer, levels: usize, symmetric: bool, invert: bool, scale: usize) -> [u8]", Some(door_math_moire_render)),
    ("math.moire.sample.axes", "(size: usize, lattice: math.moire.Lattice, row: usize) -> ([f64], [f64])", Some(door_math_moire_sample_axes)),
    ("math.moire.sample.membership", "(code: u128, base: usize, dimension: usize) -> [bool]", Some(door_math_moire_sample_membership)),
    ("math.moire.sample.pack", "(residues: [usize], base: usize) -> usize", Some(door_math_moire_sample_pack)),
    ("math.moire.stack", "(spec: math.moire.Spec, numbers: [usize], combine: math.moire.Combine, level: usize, lattice: math.moire.Lattice, size: usize, slices: [f64]) -> math.moire.Field", Some(door_math_moire_stack)),
    ("math.moire.stack_codes", "(specs: [math.moire.Spec], number: usize, level: usize, lattice: math.moire.Lattice, size: usize, slices: [f64]) -> math.moire.Field", Some(door_math_moire_stack_codes)),
    ("math.moire.volume", "(spec: math.moire.Spec, numbers: [usize], combine: math.moire.Combine, level: usize, size: usize) -> math.moire.Volume", Some(door_math_moire_volume)),
    ("math.name.Bang.cells", "(self: math.name.Bang) -> u32", Some(door_math_name_bang_cells)),
    ("math.name.Bang.checked", "(self: math.name.Bang) -> math.name.Bang", Some(door_math_name_bang_checked)),
    ("math.name.Bang.from_file", "(text: str) -> math.name.Bang", Some(door_math_name_bang_from_file)),
    ("math.name.Bang.from_json", "(text: str) -> math.name.Bang", Some(door_math_name_bang_from_json)),
    ("math.name.Bang.from_url", "(text: str) -> math.name.Bang", Some(door_math_name_bang_from_url)),
    ("math.name.Bang.new", "(code: u128, dim: usize, base: usize) -> math.name.Bang", Some(door_math_name_bang_new)),
    ("math.name.Bang.to_file", "(self: math.name.Bang) -> String", Some(door_math_name_bang_to_file)),
    ("math.name.Bang.to_id", "(self: math.name.Bang) -> String", Some(door_math_name_bang_to_id)),
    ("math.name.Bang.to_json", "(self: math.name.Bang) -> String", Some(door_math_name_bang_to_json)),
    ("math.name.Bang.to_mrly", "(self: math.name.Bang) -> String", Some(door_math_name_bang_to_mrly)),
    ("math.name.Bang.to_url", "(self: math.name.Bang) -> String", Some(door_math_name_bang_to_url)),
    ("math.name.Lattice.is_square", "(self: math.name.Lattice) -> bool", Some(door_math_name_lattice_is_square)),
    ("math.name.Lattice.units", "(self: math.name.Lattice) -> usize", Some(door_math_name_lattice_units)),
    ("math.name.Sequence.checked", "(self: math.name.Sequence) -> math.name.Sequence", Some(door_math_name_sequence_checked)),
    ("math.name.Sequence.design", "(self: math.name.Sequence) -> math.name.Bang", Some(door_math_name_sequence_design)),
    ("math.name.Sequence.from_file", "(text: str) -> math.name.Sequence", Some(door_math_name_sequence_from_file)),
    ("math.name.Sequence.from_json", "(text: str) -> math.name.Sequence", Some(door_math_name_sequence_from_json)),
    ("math.name.Sequence.from_url", "(text: str) -> math.name.Sequence", Some(door_math_name_sequence_from_url)),
    ("math.name.Sequence.new", "(code: u128, dim: usize, base: usize, measure: str, axis: str) -> math.name.Sequence", Some(door_math_name_sequence_new)),
    ("math.name.Sequence.to_file", "(self: math.name.Sequence) -> String", Some(door_math_name_sequence_to_file)),
    ("math.name.Sequence.to_id", "(self: math.name.Sequence) -> String", Some(door_math_name_sequence_to_id)),
    ("math.name.Sequence.to_json", "(self: math.name.Sequence) -> String", Some(door_math_name_sequence_to_json)),
    ("math.name.Sequence.to_mrly", "(self: math.name.Sequence) -> String", Some(door_math_name_sequence_to_mrly)),
    ("math.name.Sequence.to_url", "(self: math.name.Sequence) -> String", Some(door_math_name_sequence_to_url)),
    ("math.name.Word.bases", "(self: math.name.Word) -> [usize]", Some(door_math_name_word_bases)),
    ("math.name.Word.checked", "(self: math.name.Word) -> math.name.Word", Some(door_math_name_word_checked)),
    ("math.name.Word.from_file", "(text: str) -> math.name.Word", Some(door_math_name_word_from_file)),
    ("math.name.Word.from_json", "(text: str) -> math.name.Word", Some(door_math_name_word_from_json)),
    ("math.name.Word.from_url", "(text: str) -> math.name.Word", Some(door_math_name_word_from_url)),
    ("math.name.Word.letters", "(self: math.name.Word) -> [math.name.Bang]", Some(door_math_name_word_letters)),
    ("math.name.Word.new", "(dim: usize, letters: [(u128, usize)]) -> math.name.Word", Some(door_math_name_word_new)),
    ("math.name.Word.to_file", "(self: math.name.Word) -> String", Some(door_math_name_word_to_file)),
    ("math.name.Word.to_id", "(self: math.name.Word) -> String", Some(door_math_name_word_to_id)),
    ("math.name.Word.to_json", "(self: math.name.Word) -> String", Some(door_math_name_word_to_json)),
    ("math.name.Word.to_mrly", "(self: math.name.Word) -> String", Some(door_math_name_word_to_mrly)),
    ("math.name.Word.to_url", "(self: math.name.Word) -> String", Some(door_math_name_word_to_url)),
    ("math.press.Press.add", "(self: math.press.Press, number: u128, weight: i128) -> null # uncallable: mutates its argument in place", None),
    ("math.press.Press.new", "(dimension: usize, base: usize) -> math.press.Press", Some(door_math_press_press_new)),
    ("math.press.Press.total", "(self: math.press.Press, code: Code) -> i128", Some(door_math_press_press_total)),
    ("math.press.Press.totals", "(self: math.press.Press) -> [i128]", Some(door_math_press_press_totals)),
    ("math.press.containing", "(number: u128, dimension: usize, base: usize) -> u128", Some(door_math_press_containing)),
    ("math.press.coordinates", "(number: u128, dimension: usize, base: usize) -> [u128]", Some(door_math_press_coordinates)),
    ("math.press.count_below", "(code: Code, dimension: usize, base: usize, limit: u128) -> u128", Some(door_math_press_count_below)),
    ("math.press.distinct", "(number: u128, dimension: usize, base: usize) -> u32", Some(door_math_press_distinct)),
    ("math.press.interleave", "(coords: [u128], base: usize) -> u128", Some(door_math_press_interleave)),
    ("math.press.layer_table", "(layer: math.bang.MagicLayer) -> [bool]", Some(door_math_press_layer_table)),
    ("math.press.member", "(code: Code, number: u128, dimension: usize, base: usize) -> bool", Some(door_math_press_member)),
    ("math.press.members", "(code: Code, dimension: usize, base: usize, count: usize) -> [u128]", Some(door_math_press_members)),
    ("math.press.profile", "(code: Code, dimension: usize, base: usize, level: usize) -> [u128]", Some(door_math_press_profile)),
    ("math.press.usage", "(number: u128, dimension: usize, base: usize) -> Code", Some(door_math_press_usage)),
    ("math.press.word_count", "(layers: [math.bang.MagicLayer]) -> u128", Some(door_math_press_word_count)),
    ("math.press.word_member", "(layers: [math.bang.MagicLayer], number: u128) -> bool", Some(door_math_press_word_member)),
    ("math.press.word_members", "(layers: [math.bang.MagicLayer]) -> [u128]", Some(door_math_press_word_members)),
    ("math.press.word_profile", "(layers: [math.bang.MagicLayer]) -> [u128]", Some(door_math_press_word_profile)),
    ("math.roulette.Nodes.pair", "(self: math.roulette.Nodes, i: usize, j: usize) -> usize", Some(door_math_roulette_nodes_pair)),
    ("math.roulette.Nodes.paired", "(self: math.roulette.Nodes) -> usize", Some(door_math_roulette_nodes_paired)),
    ("math.roulette.Nodes.selved", "(self: math.roulette.Nodes) -> usize", Some(door_math_roulette_nodes_selved)),
    ("math.roulette.Nodes.total", "(self: math.roulette.Nodes) -> usize", Some(door_math_roulette_nodes_total)),
    ("math.roulette.nodes", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], samples: usize, tol: f64) -> math.roulette.Nodes", Some(door_math_roulette_nodes)),
    ("math.roulette.side", "(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> i32", Some(door_math_roulette_side)),
    ("math.roulette.spread", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> [math.spirograph.Pencil]", Some(door_math_roulette_spread)),
    ("math.rules.render", "(filled: [[u8]], number: usize, dimension: usize, base: usize) -> Tensor", Some(door_math_rules_render)),
    ("math.rules.tree_axes", "(dimension: usize, free_axis: usize) -> [usize]", Some(door_math_rules_tree_axes)),
    ("math.shape.Frac.minus", "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac", Some(door_math_shape_frac_minus)),
    ("math.shape.Frac.new", "(num: i64, den: i64) -> math.shape.Frac", Some(door_math_shape_frac_new)),
    ("math.shape.Frac.plus", "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac", Some(door_math_shape_frac_plus)),
    ("math.shape.Frac.times", "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac", Some(door_math_shape_frac_times)),
    ("math.shape.Frac.whole", "(num: i64) -> math.shape.Frac", Some(door_math_shape_frac_whole)),
    ("math.shape.Region.flip", "(self: math.shape.Region) -> math.shape.Region", Some(door_math_shape_region_flip)),
    ("math.shape.census", "(shape: math.shape.Shape, types: Tensor) -> math.shape.ShapeCensus", Some(door_math_shape_census)),
    ("math.shape.classify", "(shape: math.shape.Shape, side: usize, index: [usize]) -> math.shape.Region", Some(door_math_shape_classify)),
    ("math.shape.crop", "(types: Tensor, shape: math.shape.Shape, keep_cut: bool) -> Tensor", Some(door_math_shape_crop)),
    ("math.shape.crossing_shell", "(radius: u64, number: u64, level: u32) -> [(u64, u64)]", Some(door_math_shape_crossing_shell)),
    ("math.shape.crossing_tree", "(radius: u64, number: u64, keep: [bool]) -> math.shape.Shell", Some(door_math_shape_crossing_tree)),
    ("math.shape.named", "(name: str, dimension: usize, radius: math.shape.Frac) -> math.shape.Shape", Some(door_math_shape_named)),
    ("math.shape.radial_census", "(types: Tensor, centre: [i64], r_max: u64) -> [math.shape.RadialCounts]", Some(door_math_shape_radial_census)),
    ("math.shape.refine", "(types: Tensor, shape: math.shape.Shape, base: usize, extra: usize, keep_cut: bool) -> Tensor", Some(door_math_shape_refine)),
    ("math.shape.regions", "(shape: math.shape.Shape, dims: [usize]) -> Tensor", Some(door_math_shape_regions)),
    ("math.shape.shapes", "(dimension: usize) -> [String]", Some(door_math_shape_shapes)),
    ("math.six.anti", "(self: Cell6d) -> Cell6d", Some(door_math_six_anti)),
    ("math.six.binarize", "(self: Cell6d, threshold: u8) -> Cell6d", Some(door_math_six_binarize)),
    ("math.six.binarize_otsu", "(self: Cell6d) -> Cell6d", Some(door_math_six_binarize_otsu)),
    ("math.six.blank", "(radius: usize, orient: math.six.Orientation, fill: u8, void: u8) -> Cell2d", Some(door_math_six_blank)),
    ("math.six.blur", "(self: Cell6d, mask: Tensor, wrap: bool) -> Cell6d", Some(door_math_six_blur)),
    ("math.six.census", "(cell: Cell6d, include_grid: bool) -> math.six.Census", Some(door_math_six_census)),
    ("math.six.components", "(cell: Cell6d) -> usize", Some(door_math_six_components)),
    ("math.six.cut", "(cell: Cell3d) -> Cell6d", Some(door_math_six_cut)),
    ("math.six.cut_design", "(code: Code, number: usize, level: usize, base: usize) -> Cell6d", Some(door_math_six_cut_design)),
    ("math.six.east", "(x: i64, y: i64) -> [(i64, i64); 3]", Some(door_math_six_east)),
    ("math.six.euler", "(cell: Cell6d, include_grid: bool) -> i64", Some(door_math_six_euler)),
    ("math.six.fills", "(cell: Cell6d) -> usize", Some(door_math_six_fills)),
    ("math.six.fills_only", "(cell: Cell6d) -> math.six.Census", Some(door_math_six_fills_only)),
    ("math.six.framed", "(cell: Cell6d) -> Cell6d", Some(door_math_six_framed)),
    ("math.six.from_json", "(text: str) -> Cell6d", Some(door_math_six_from_json)),
    ("math.six.giant", "(cell: Cell6d) -> usize", Some(door_math_six_giant)),
    ("math.six.giant_network", "(cell: Cell6d) -> math.graph.Network", Some(door_math_six_giant_network)),
    ("math.six.height", "(self: Cell6d) -> usize", Some(door_math_six_height)),
    ("math.six.holes", "(cell: Cell6d) -> usize", Some(door_math_six_holes)),
    ("math.six.is_cube", "(cell: Cell3d) -> bool", Some(door_math_six_is_cube)),
    ("math.six.is_hex", "(cell: Cell2d) -> bool", Some(door_math_six_is_hex)),
    ("math.six.iso", "(cell: Cell3d) -> Cell6d", Some(door_math_six_iso)),
    ("math.six.iso_design", "(code: Code, number: usize, level: usize, base: usize) -> Cell6d", Some(door_math_six_iso_design)),
    ("math.six.new", "(cell: Cell2d, projection: math.six.Projection, orientation: math.six.Orientation, start: u8) -> Cell6d", Some(door_math_six_new)),
    ("math.six.north", "(x: i64, y: i64) -> [(i64, i64); 3]", Some(door_math_six_north)),
    ("math.six.orientation", "(width: usize, height: usize) -> math.six.Orientation", Some(door_math_six_orientation)),
    ("math.six.pad", "(cell: Cell6d, k: usize, value: u8) -> Cell6d", Some(door_math_six_pad)),
    ("math.six.paint", "(cell: Cell6d, custom: {u8: [Color]}?, mode: core.Mode?, rng: Rng(seed)?) -> Cell6d", Some(door_math_six_paint)),
    ("math.six.perforate", "(self: Cell6d, mask: Tensor, value: u8) -> Cell6d", Some(door_math_six_perforate)),
    ("math.six.png", "(cell: Cell6d, scale: usize, outline: Color?, width: usize) -> [u8]", Some(door_math_six_png)),
    ("math.six.pro", "(cell: Cell3d) -> Cell6d", Some(door_math_six_pro)),
    ("math.six.pro_design", "(code: Code, number: usize, level: usize, base: usize) -> Cell6d", Some(door_math_six_pro_design)),
    ("math.six.radial", "(cell: Cell6d, radius: usize) -> Cell2d", Some(door_math_six_radial)),
    ("math.six.radial_crop", "(cell: Cell2d, radius: usize, size: (usize, usize)) -> Cell2d", Some(door_math_six_radial_crop)),
    ("math.six.radial_mask", "(radius: usize, orient: math.six.Orientation) -> Tensor", Some(door_math_six_radial_mask)),
    ("math.six.raster", "(cell: Cell6d, size: usize) -> [f32]", Some(door_math_six_raster)),
    ("math.six.rect_png", "(cell: Cell6d, scale: usize, start: usize?) -> [u8]", Some(door_math_six_rect_png)),
    ("math.six.rect_svg", "(cell: Cell6d, scale: usize, start: usize?) -> String", Some(door_math_six_rect_svg)),
    ("math.six.rim_holes", "(cell: Cell6d) -> usize", Some(door_math_six_rim_holes)),
    ("math.six.skin", "(cell: Cell6d) -> Cell6d", Some(door_math_six_skin)),
    ("math.six.slice_core_graph", "(cell: Cell6d) -> math.graph.Network", Some(door_math_six_slice_core_graph)),
    ("math.six.slice_dual_graph", "(cell: Cell6d) -> math.graph.Network", Some(door_math_six_slice_dual_graph)),
    ("math.six.slice_edge_graph", "(cell: Cell6d, value: u8?) -> math.graph.Network", Some(door_math_six_slice_edge_graph)),
    ("math.six.slice_tunnel_graph", "(cell: Cell6d) -> math.graph.Network", Some(door_math_six_slice_tunnel_graph)),
    ("math.six.south", "(x: i64, y: i64) -> [(i64, i64); 3]", Some(door_math_six_south)),
    ("math.six.spectral_exponent", "(cell: Cell6d, window: f64) -> f64", Some(door_math_six_spectral_exponent)),
    ("math.six.star.Branch.constant", "(self: math.six.star.Branch) -> f64", Some(door_math_six_star_branch_constant)),
    ("math.six.star.Branch.name", "(self: math.six.star.Branch) -> String", Some(door_math_six_star_branch_name)),
    ("math.six.star.Branch.of", "(layers: usize) -> math.six.star.Branch", Some(door_math_six_star_branch_of)),
    ("math.six.star.Branch.residual", "(self: math.six.star.Branch) -> f64?", Some(door_math_six_star_branch_residual)),
    ("math.six.star.Share.reduced", "(self: math.six.star.Share) -> (i64, i64)", Some(door_math_six_star_share_reduced)),
    ("math.six.star.Share.value", "(self: math.six.star.Share) -> f64", Some(door_math_six_star_share_value)),
    ("math.six.star.Star.arm", "(self: math.six.star.Star, number: usize, half: usize) -> math.six.star.Share", Some(door_math_six_star_star_arm)),
    ("math.six.star.Star.cell", "(self: math.six.star.Star, number: usize, x: i64, z: i64) -> bool?", Some(door_math_six_star_star_cell)),
    ("math.six.star.Star.excesses", "(self: math.six.star.Star, layers: usize, half: usize) -> [f64]", Some(door_math_six_star_star_excesses)),
    ("math.six.star.Star.hexagon", "(self: math.six.star.Star, number: usize) -> math.six.star.Share", Some(door_math_six_star_star_hexagon)),
    ("math.six.star.Star.new", "(code: u128) -> math.six.star.Star", Some(door_math_six_star_star_new)),
    ("math.six.star.arm_law", "(number: usize) -> math.six.star.Share", Some(door_math_six_star_arm_law)),
    ("math.six.star.chi8", "(number: usize) -> i64", Some(door_math_six_star_chi8)),
    ("math.six.star.constant", "() -> f64", Some(door_math_six_star_constant)),
    ("math.six.star.decay", "(excesses: [f64], layers: usize) -> math.six.star.Decay", Some(door_math_six_star_decay)),
    ("math.six.star.width_law", "(half: usize) -> f64", Some(door_math_six_star_width_law)),
    ("math.six.svg", "(cell: Cell6d, scale: usize, outline: Color?, width: usize, start: usize?) -> String", Some(door_math_six_svg)),
    ("math.six.tessellate", "(cell: Cell6d, mask: Tensor) -> Cell2d", Some(door_math_six_tessellate)),
    ("math.six.tile", "(cell: Cell6d, width: usize, height: usize) -> Cell2d", Some(door_math_six_tile)),
    ("math.six.tile_cell", "(cell: Cell6d, width: usize, height: usize, crop: bool) -> Cell6d", Some(door_math_six_tile_cell)),
    ("math.six.tile_crop", "(cell: Cell2d, size: (usize, usize)) -> Cell2d", Some(door_math_six_tile_crop)),
    ("math.six.tile_step", "(size: (usize, usize)) -> (usize, usize)", Some(door_math_six_tile_step)),
    ("math.six.to_json", "(cell: Cell6d) -> String", Some(door_math_six_to_json)),
    ("math.six.triangles", "(cell: Cell6d, start: usize?) -> [([(i64, i64); 3], [u8; 4])]", Some(door_math_six_triangles)),
    ("math.six.west", "(x: i64, y: i64) -> [(i64, i64); 3]", Some(door_math_six_west)),
    ("math.six.width", "(self: Cell6d) -> usize", Some(door_math_six_width)),
    ("math.spectrum.clusters", "(eigenvalues: [f64], tolerance: f64) -> [(f64, usize)]", Some(door_math_spectrum_clusters)),
    ("math.spectrum.laplacian", "(network: math.graph.Network, normalised: bool) -> [[f64]]", Some(door_math_spectrum_laplacian)),
    ("math.spectrum.laplacian_spectrum", "(network: math.graph.Network, normalised: bool) -> [f64]", Some(door_math_spectrum_laplacian_spectrum)),
    ("math.spectrum.multiplicity", "(eigenvalues: [f64], value: f64, tolerance: f64) -> usize", Some(door_math_spectrum_multiplicity)),
    ("math.spectrum.spectral_exponent", "(eigenvalues: [f64], window: f64) -> f64?", Some(door_math_spectrum_spectral_exponent)),
    ("math.spectrum.spectral_fit", "(eigenvalues: [f64], window: f64) -> (f64, f64, usize)?", Some(door_math_spectrum_spectral_fit)),
    ("math.spectrum.spectral_points", "(eigenvalues: [f64]) -> [(f64, f64)]", Some(door_math_spectrum_spectral_points)),
    ("math.spectrum.symmetric_eigenvalues", "(matrix: [[f64]]) -> [f64]", Some(door_math_spectrum_symmetric_eigenvalues)),
    ("math.spin.Blend.fold", "(self: math.spin.Blend, values: [f32]) -> f32", Some(door_math_spin_blend_fold)),
    ("math.spin.Blend.named", "(name: str) -> math.spin.Blend?", Some(door_math_spin_blend_named)),
    ("math.spin.arcs", "(data: [f32], size: usize, radius: f64) -> [(f64, f64, f32)]", Some(door_math_spin_arcs)),
    ("math.spin.harmonics", "(data: [f32], size: usize, rings: usize, orders: usize) -> [f64]", Some(door_math_spin_harmonics)),
    ("math.spin.mass", "(profile: [f32], size: usize) -> f64", Some(door_math_spin_mass)),
    ("math.spin.mass_within", "(profile: [f32], size: usize, radius: f64) -> f64", Some(door_math_spin_mass_within)),
    ("math.spin.petals", "(copies: usize, order: usize) -> usize", Some(door_math_spin_petals)),
    ("math.spin.profile", "(data: [f32], size: usize, steps: usize) -> [f32]", Some(door_math_spin_profile)),
    ("math.spin.radial", "(data: [f32], size: usize, out: usize, copies: usize, step: f64, blend: math.spin.Blend, samples: usize) -> [f32]", Some(door_math_spin_radial)),
    ("math.spin.reach", "(size: usize) -> f64", Some(door_math_spin_reach)),
    ("math.spin.ring", "(data: [f32], size: usize, radius: f64) -> f64", Some(door_math_spin_ring)),
    ("math.spin.turns", "(power: [f64]) -> usize", Some(door_math_spin_turns)),
    ("math.spin.wheel", "(profile: [f32], size: usize) -> [f32]", Some(door_math_spin_wheel)),
    ("math.spirograph.cell", "(width: usize, height: usize, reach: f64) -> f64", Some(door_math_spirograph_cell)),
    ("math.spirograph.cover", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool, samples: usize, side: usize) -> math.spirograph.Cover", Some(door_math_spirograph_cover)),
    ("math.spirograph.disc", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil]) -> math.spirograph.Disc", Some(door_math_spirograph_disc)),
    ("math.spirograph.distinct", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> usize", Some(door_math_spirograph_distinct)),
    ("math.spirograph.frame", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil]) -> [f64; 4]", Some(door_math_spirograph_frame)),
    ("math.spirograph.nodes", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> u64?", Some(door_math_spirograph_nodes)),
    ("math.spirograph.pencils", "(types: [u8], width: usize, height: usize, mode: str, reach: f64, jitter: f64, seed: u32) -> [math.spirograph.Pencil]", Some(door_math_spirograph_pencils)),
    ("math.spirograph.point", "(track: math.spirograph.Track, pencil: math.spirograph.Pencil, s: f64) -> (f64, f64)", Some(door_math_spirograph_point)),
    ("math.spirograph.pose", "(track: math.spirograph.Track, s: f64) -> (f64, f64)", Some(door_math_spirograph_pose)),
    ("math.spirograph.representatives", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> [usize]", Some(door_math_spirograph_representatives)),
    ("math.spirograph.seats", "(pencils: [math.spirograph.Pencil]) -> math.spirograph.Seats", Some(door_math_spirograph_seats)),
    ("math.spirograph.signed_area", "(track: math.spirograph.Track, pencil: math.spirograph.Pencil) -> f64?", Some(door_math_spirograph_signed_area)),
    ("math.spirograph.trace", "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], samples: usize) -> [f32]", Some(door_math_spirograph_trace)),
    ("math.spirograph.track", "(kind: str, ring: usize, wheel: usize, sides: usize, laps: usize) -> math.spirograph.Track", Some(door_math_spirograph_track)),
    ("math.spirograph.turn", "(track: math.spirograph.Track, s: f64) -> f64", Some(door_math_spirograph_turn)),
    ("math.three.Vec3.cross", "(self: math.three.Vec3, o: math.three.Vec3) -> math.three.Vec3", Some(door_math_three_vec3_cross)),
    ("math.three.Vec3.dot", "(self: math.three.Vec3, o: math.three.Vec3) -> f32", Some(door_math_three_vec3_dot)),
    ("math.three.Vec3.new", "(x: f32, y: f32, z: f32) -> math.three.Vec3", Some(door_math_three_vec3_new)),
    ("math.three.Vec3.scale", "(self: math.three.Vec3, s: f32) -> math.three.Vec3", Some(door_math_three_vec3_scale)),
    ("math.three.carpet", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_carpet)),
    ("math.three.census", "(cell: Cell3d) -> math.three.Census", Some(door_math_three_census)),
    ("math.three.core_graph", "(cell: Cell) -> math.graph.Network", Some(door_math_three_core_graph)),
    ("math.three.create", "(code: Code, number: usize, level: usize, base: usize) -> Cell3d", Some(door_math_three_create)),
    ("math.three.diagonal_slice", "(code: Code, number: usize, level: usize, base: usize, height: usize) -> [[u32; 3]]", Some(door_math_three_diagonal_slice)),
    ("math.three.diagonal_svg", "(code: Code, number: usize, level: usize, base: usize, heights: [usize], scale: usize) -> String", Some(door_math_three_diagonal_svg)),
    ("math.three.dust", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_dust)),
    ("math.three.edge_graph", "(cell: Cell) -> math.graph.Network", Some(door_math_three_edge_graph)),
    ("math.three.euler", "(cell: Cell3d) -> i64", Some(door_math_three_euler)),
    ("math.three.extrude", "(cell: Cell2d, axis: usize, depth: usize) -> Cell3d", Some(door_math_three_extrude)),
    ("math.three.extrude_cube", "(cell: Cell3d, depth: usize) -> Cell3d", Some(door_math_three_extrude_cube)),
    ("math.three.faces", "(cell: Cell3d) -> usize", Some(door_math_three_faces)),
    ("math.three.fills", "(cell: Cell3d) -> usize", Some(door_math_three_fills)),
    ("math.three.from_corners", "(corners: [[u8]], number: usize, level: usize, base: usize) -> Cell3d", Some(door_math_three_from_corners)),
    ("math.three.from_json", "(text: str) -> Cell3d", Some(door_math_three_from_json)),
    ("math.three.from_strings", "(data: [[String]]) -> Cell3d", Some(door_math_three_from_strings)),
    ("math.three.hidden", "(cell: Cell3d) -> u128", Some(door_math_three_hidden)),
    ("math.three.level_set", "(number: usize, levels: [usize], level: usize, base: usize) -> Cell3d", Some(door_math_three_level_set)),
    ("math.three.magic", "(cells: [Cell]) -> Cell", Some(door_math_three_magic)),
    ("math.three.manhattan_layers", "(cell: Cell3d) -> Cell3d", Some(door_math_three_manhattan_layers)),
    ("math.three.merge", "(cells: [Cell3d], width: usize, height: usize, depth: usize) -> Cell3d", Some(door_math_three_merge)),
    ("math.three.mosaic", "(mask: Tensor, cells: [Cell]) -> Cell", Some(door_math_three_mosaic)),
    ("math.three.named", "(design: gen.recipe.Design, number: usize, level: usize) -> Cell3d", Some(door_math_three_named)),
    ("math.three.net", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_net)),
    ("math.three.noise", "(number: usize, level: usize, density: f64, rng: Rng(seed)) -> Cell3d", Some(door_math_three_noise)),
    ("math.three.ones", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_ones)),
    ("math.three.orientations", "() -> [(usize, usize, usize)]", Some(door_math_three_orientations)),
    ("math.three.point", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_point)),
    ("math.three.profile", "(code: Code, number: usize, level: usize, base: usize) -> [u128]", Some(door_math_three_profile)),
    ("math.three.project", "(point: [u32; 3]) -> (f64, f64)", Some(door_math_three_project)),
    ("math.three.quads", "(cell: Cell3d) -> [math.three.Quad]", Some(door_math_three_quads)),
    ("math.three.shadow", "(point: [u32; 3]) -> (i64, i64)", Some(door_math_three_shadow)),
    ("math.three.slice", "(cell: Cell3d, axis: usize, index: usize) -> Cell2d", Some(door_math_three_slice)),
    ("math.three.special", "(mask: Tensor, cell: Cell3d) -> Cell3d", Some(door_math_three_special)),
    ("math.three.star", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_star)),
    ("math.three.support", "(counts: [u128]) -> (usize, usize)?", Some(door_math_three_support)),
    ("math.three.surface", "(cell: Cell3d) -> u128", Some(door_math_three_surface)),
    ("math.three.text", "(cell: Cell3d, glyphs: {u8: String}?) -> [String]", Some(door_math_three_text)),
    ("math.three.to_json", "(cell: Cell3d) -> String", Some(door_math_three_to_json)),
    ("math.three.to_obj", "(cell: Cell3d) -> String", Some(door_math_three_to_obj)),
    ("math.three.to_strings", "(cell: Cell3d) -> [[String]]", Some(door_math_three_to_strings)),
    ("math.three.tunnel_graph", "(cell: Cell) -> math.graph.Network", Some(door_math_three_tunnel_graph)),
    ("math.three.void", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_void)),
    ("math.three.voids", "(cell: Cell3d) -> usize", Some(door_math_three_voids)),
    ("math.three.volume", "(cell: Cell3d) -> usize", Some(door_math_three_volume)),
    ("math.three.wires", "(cell: Cell3d) -> [[math.three.Vec3; 2]]", Some(door_math_three_wires)),
    ("math.three.xline", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_xline)),
    ("math.three.xtree", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_xtree)),
    ("math.three.yline", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_yline)),
    ("math.three.ytree", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_ytree)),
    ("math.three.zeros", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_zeros)),
    ("math.three.zline", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_zline)),
    ("math.three.ztree", "(number: usize, level: usize) -> Cell3d", Some(door_math_three_ztree)),
    ("math.tourbillon.eyes", "(qmax: usize) -> [math.tourbillon.Eye]", Some(door_math_tourbillon_eyes)),
    ("math.tourbillon.field", "(top: usize, size: usize, schedule: str, increment: f64, set: str, weights: str, mode: str, blend: str, seed: u32) -> [f32]", Some(door_math_tourbillon_field)),
    ("math.tourbillon.layers", "(top: usize, schedule: str, increment: f64, set: str, weights: str, seed: u32) -> [math.tourbillon.Layer]", Some(door_math_tourbillon_layers)),
    ("math.tourbillon.period", "(increment: f64) -> usize?", Some(door_math_tourbillon_period)),
    ("math.tourbillon.sharing", "(list: [math.tourbillon.Layer]) -> (usize, usize)", Some(door_math_tourbillon_sharing)),
    ("math.tourbillon.stack", "(list: [math.tourbillon.Layer], size: usize, mode: str, blend: math.spin.Blend) -> [f32]", Some(door_math_tourbillon_stack)),
    ("math.tourbillon.stats", "(field: [f32], size: usize, top: usize, schedule: str, increment: f64, set: str, weights: str, blend: str, seed: u32) -> math.tourbillon.Stats", Some(door_math_tourbillon_stats)),
    ("math.two.capacity", "(cell: Cell2d) -> usize", Some(door_math_two_capacity)),
    ("math.two.carpet", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_carpet)),
    ("math.two.census", "(cell: Cell2d) -> math.two.Census", Some(door_math_two_census)),
    ("math.two.create", "(code: Code, number: usize, level: usize, rotation: usize, base: usize) -> Cell2d", Some(door_math_two_create)),
    ("math.two.dust", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_dust)),
    ("math.two.embed", "(cell: Cell2d, payload: [u8]) -> Cell2d", Some(door_math_two_embed)),
    ("math.two.euler", "(cell: Cell2d) -> i64", Some(door_math_two_euler)),
    ("math.two.extract", "(carrier: Cell2d, carried: Cell2d) -> [u8]", Some(door_math_two_extract)),
    ("math.two.fills", "(cell: Cell2d) -> usize", Some(door_math_two_fills)),
    ("math.two.from_corners", "(corners: [[u8]], number: usize, level: usize, rotation: usize, base: usize) -> Cell2d", Some(door_math_two_from_corners)),
    ("math.two.from_json", "(text: str) -> Cell2d", Some(door_math_two_from_json)),
    ("math.two.from_strings", "(rows: [String]) -> Cell2d", Some(door_math_two_from_strings)),
    ("math.two.hline", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_hline)),
    ("math.two.htree", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_htree)),
    ("math.two.level_set", "(number: usize, levels: [usize], level: usize, rotation: usize, base: usize) -> Cell2d", Some(door_math_two_level_set)),
    ("math.two.mask", "(mask: Tensor, shape: [usize]) -> Tensor", Some(door_math_two_mask)),
    ("math.two.merge", "(cells: [Cell2d], width: usize, height: usize) -> Cell2d", Some(door_math_two_merge)),
    ("math.two.named", "(design: gen.recipe.Design, number: usize, level: usize, rotation: usize) -> Cell2d", Some(door_math_two_named)),
    ("math.two.net", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_net)),
    ("math.two.noise", "(number: usize, level: usize, density: f64, rng: Rng(seed)) -> Cell2d", Some(door_math_two_noise)),
    ("math.two.ones", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_ones)),
    ("math.two.payload.frame", "() -> Tensor", Some(door_math_two_payload_frame)),
    ("math.two.perimeter", "(cell: Cell2d) -> u128", Some(door_math_two_perimeter)),
    ("math.two.png", "(cell: Cell2d, scale: usize, outline: Color?, width: usize, shape: math.two.Shape) -> [u8]", Some(door_math_two_png)),
    ("math.two.point", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_point)),
    ("math.two.read", "(sheet: Cell2d, carrier: Cell2d) -> [u8]", Some(door_math_two_read)),
    ("math.two.sheet", "(cells: [Cell2d; 4], payload: [u8]) -> Cell2d", Some(door_math_two_sheet)),
    ("math.two.special", "(mask: Tensor, cell: Cell2d) -> Cell2d", Some(door_math_two_special)),
    ("math.two.star", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_star)),
    ("math.two.svg", "(cell: Cell2d, scale: usize, outline: Color?, width: usize, shape: math.two.Shape) -> String", Some(door_math_two_svg)),
    ("math.two.text", "(cell: Cell2d, glyphs: {u8: String}?) -> [String]", Some(door_math_two_text)),
    ("math.two.to_3d", "(cell: Cell2d) -> Cell3d", Some(door_math_two_to_3d)),
    ("math.two.to_json", "(cell: Cell2d) -> String", Some(door_math_two_to_json)),
    ("math.two.vline", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_vline)),
    ("math.two.void", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_void)),
    ("math.two.voids", "(cell: Cell2d) -> usize", Some(door_math_two_voids)),
    ("math.two.vtree", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_vtree)),
    ("math.two.zeros", "(number: usize, level: usize) -> Cell2d", Some(door_math_two_zeros)),
    ("num.apollonian.Circle.centre", "(self: num.apollonian.Circle) -> (f64, f64)?", Some(door_num_apollonian_circle_centre)),
    ("num.apollonian.Circle.is_line", "(self: num.apollonian.Circle) -> bool", Some(door_num_apollonian_circle_is_line)),
    ("num.apollonian.Circle.radius", "(self: num.apollonian.Circle) -> f64?", Some(door_num_apollonian_circle_radius)),
    ("num.apollonian.form", "(u: [i64; 4], v: [i64; 4]) -> i128", Some(door_num_apollonian_form)),
    ("num.apollonian.frame", "(p: num.apollonian.Packing) -> [f64; 4]", Some(door_num_apollonian_frame)),
    ("num.apollonian.grow", "(name: str, cap: i64) -> num.apollonian.Packing", Some(door_num_apollonian_grow)),
    ("num.apollonian.is_ford", "(c: num.apollonian.Circle) -> bool", Some(door_num_apollonian_is_ford)),
    ("num.apollonian.on_line", "(c: num.apollonian.Circle) -> bool", Some(door_num_apollonian_on_line)),
    ("num.apollonian.reflect", "(q: [num.apollonian.Circle; 4], at: usize) -> num.apollonian.Circle", Some(door_num_apollonian_reflect)),
    ("num.apollonian.root", "(name: str) -> [num.apollonian.Circle; 4]", Some(door_num_apollonian_root)),
    ("num.apollonian.shadow", "(p: num.apollonian.Packing, order: usize) -> num.apollonian.Shadow", Some(door_num_apollonian_shadow)),
    ("num.apollonian.sound", "(q: [num.apollonian.Circle; 4]) -> bool", Some(door_num_apollonian_sound)),
    ("num.apollonian.swap", "(q: [num.apollonian.Circle; 4], at: usize) -> [num.apollonian.Circle; 4]", Some(door_num_apollonian_swap)),
    ("num.apollonian.touches", "(p: num.apollonian.Packing) -> [num.apollonian.Touch]", Some(door_num_apollonian_touches)),
    ("num.automaton.Automaton.abscissa", "(self: num.automaton.Automaton) -> f64", Some(door_num_automaton_automaton_abscissa)),
    ("num.automaton.Automaton.base", "(self: num.automaton.Automaton) -> u64", Some(door_num_automaton_automaton_base)),
    ("num.automaton.Automaton.cofactor", "(self: num.automaton.Automaton, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)", Some(door_num_automaton_automaton_cofactor)),
    ("num.automaton.Automaton.denominator", "(self: num.automaton.Automaton) -> [f64]", Some(door_num_automaton_automaton_denominator)),
    ("num.automaton.Automaton.matrix", "(self: num.automaton.Automaton) -> [[f64]]", Some(door_num_automaton_automaton_matrix)),
    ("num.automaton.Automaton.new", "(rule: num.memory.Rule) -> num.automaton.Automaton", Some(door_num_automaton_automaton_new)),
    ("num.automaton.Automaton.peel", "(self: num.automaton.Automaton) -> usize", Some(door_num_automaton_automaton_peel)),
    ("num.automaton.Automaton.period", "(self: num.automaton.Automaton) -> f64", Some(door_num_automaton_automaton_period)),
    ("num.automaton.Automaton.perron", "(self: num.automaton.Automaton) -> (f64, f64)", Some(door_num_automaton_automaton_perron)),
    ("num.automaton.Automaton.residue", "(self: num.automaton.Automaton, w0: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)", Some(door_num_automaton_automaton_residue)),
    ("num.automaton.Automaton.rule", "(self: num.automaton.Automaton) -> num.memory.Rule", Some(door_num_automaton_automaton_rule)),
    ("num.automaton.Automaton.states", "(self: num.automaton.Automaton) -> usize", Some(door_num_automaton_automaton_states)),
    ("num.automaton.Automaton.with_peel", "(rule: num.memory.Rule, peel: usize) -> num.automaton.Automaton", Some(door_num_automaton_automaton_with_peel)),
    ("num.automaton.Automaton.zeta", "(self: num.automaton.Automaton, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)", Some(door_num_automaton_automaton_zeta)),
    ("num.blend.add", "(a: [i128], b: [i128]) -> [i128]", Some(door_num_blend_add)),
    ("num.blend.cauchy", "(a: [i128], b: [i128]) -> [i128]", Some(door_num_blend_cauchy)),
    ("num.blend.characteristic", "(coefficients: [(i128, i128)]) -> [(i128, i128)]", Some(door_num_blend_characteristic)),
    ("num.blend.decimate", "(a: [i128], step: usize, offset: usize) -> [i128]", Some(door_num_blend_decimate)),
    ("num.blend.delta", "(a: [i128]) -> [i128]", Some(door_num_blend_delta)),
    ("num.blend.growth", "(coefficients: [(i128, i128)]) -> f64", Some(door_num_blend_growth)),
    ("num.blend.hadamard", "(a: [i128], b: [i128]) -> [i128]", Some(door_num_blend_hadamard)),
    ("num.blend.recurrence", "(terms: [i128]) -> [(i128, i128)]?", Some(door_num_blend_recurrence)),
    ("num.blend.scale", "(a: [i128], factor: i128) -> [i128]", Some(door_num_blend_scale)),
    ("num.blend.shift", "(a: [i128], count: usize) -> [i128]", Some(door_num_blend_shift)),
    ("num.blend.sigma", "(a: [i128]) -> [i128]", Some(door_num_blend_sigma)),
    ("num.blend.sub", "(a: [i128], b: [i128]) -> [i128]", Some(door_num_blend_sub)),
    ("num.boolean.is_balanced", "(code: u128, n: usize) -> bool", Some(door_num_boolean_is_balanced)),
    ("num.boolean.nonlinearity", "(code: u128, n: usize) -> i64", Some(door_num_boolean_nonlinearity)),
    ("num.boolean.sac", "(code: u128, n: usize) -> f64", Some(door_num_boolean_sac)),
    ("num.boolean.walsh_spectrum", "(code: u128, n: usize) -> [i64]", Some(door_num_boolean_walsh_spectrum)),
    ("num.design.digits_of", "(mask: u32, base: u64) -> [u64]", Some(door_num_design_digits_of)),
    ("num.design.echo_series", "(values: [u64], log_x: [f64], exponent: f64) -> [f64]", Some(door_num_design_echo_series)),
    ("num.design.elements", "(base: u64, digits: [u64], depth: usize) -> [u64]", Some(door_num_design_elements)),
    ("num.design.log_grid", "(values: [u64], samples: usize) -> [f64]", Some(door_num_design_log_grid)),
    ("num.design.median_floor", "(power: [f64], width: usize) -> [f64]", Some(door_num_design_median_floor)),
    ("num.design.meter", "(mu: [i8]) -> [i64]", Some(door_num_design_meter)),
    ("num.design.nearest", "(value: f64, list: [f64]) -> f64", Some(door_num_design_nearest)),
    ("num.design.peaks", "(gamma: [f64], score: [f64], band: (f64, f64), threshold: f64) -> [usize]", Some(door_num_design_peaks)),
    ("num.design.pole_lattice", "(base: u64, top: f64) -> [f64]", Some(door_num_design_pole_lattice)),
    ("num.design.resample", "(values: [u64], running: [i64], exponent: f64, log_x: [f64]) -> [f64]", Some(door_num_design_resample)),
    ("num.design.score", "(power: [f64], width: usize) -> [f64]", Some(door_num_design_score)),
    ("num.design.size", "(digits: [u64], depth: usize) -> u128", Some(door_num_design_size)),
    ("num.design.spectrum", "(log_x: [f64], series: [f64]) -> ([f64], [f64])", Some(door_num_design_spectrum)),
    ("num.design.upper_rms", "(series: [f64]) -> f64", Some(door_num_design_upper_rms)),
    ("num.factor.aliquot", "(number: usize) -> usize", Some(door_num_factor_aliquot)),
    ("num.factor.coprime", "(a: usize, b: usize) -> bool", Some(door_num_factor_coprime)),
    ("num.factor.divisors", "(number: u64) -> [u64]", Some(door_num_factor_divisors)),
    ("num.factor.factorial", "(number: usize) -> u128", Some(door_num_factor_factorial)),
    ("num.factor.factorize", "(number: usize) -> [(usize, u32)]", Some(door_num_factor_factorize)),
    ("num.factor.factorize_wide", "(number: u64) -> [(u64, u32)]", Some(door_num_factor_factorize_wide)),
    ("num.factor.gcd", "(a: u128, b: u128) -> u128", Some(door_num_factor_gcd)),
    ("num.factor.lcm", "(a: usize, b: usize) -> usize", Some(door_num_factor_lcm)),
    ("num.factor.mobius", "(number: usize) -> i8", Some(door_num_factor_mobius)),
    ("num.factor.mobius_sieve", "(limit: usize) -> [i8]", Some(door_num_factor_mobius_sieve)),
    ("num.factor.radical", "(number: usize) -> usize", Some(door_num_factor_radical)),
    ("num.factor.reduce", "(numerator: u128, denominator: u128) -> (u128, u128)", Some(door_num_factor_reduce)),
    ("num.factor.sigma", "(number: usize, power: u32) -> u128", Some(door_num_factor_sigma)),
    ("num.factor.squarefree", "(number: usize) -> bool", Some(door_num_factor_squarefree)),
    ("num.factor.totient", "(number: usize) -> usize", Some(door_num_factor_totient)),
    ("num.factor.totients", "(n: usize) -> [u64]", Some(door_num_factor_totients)),
    ("num.factor.twisted", "(number: usize, rhythm: [i8]) -> i64", Some(door_num_factor_twisted)),
    ("num.fft.convolve", "(field: [f64], kernel: [f64], size: usize) -> [f64]", Some(door_num_fft_convolve)),
    ("num.fft.convolve_with", "(field: [f64], kernel_re: [f64], kernel_im: [f64], size: usize) -> [f64]", Some(door_num_fft_convolve_with)),
    ("num.fft.embed_kernel", "(mask: [u8], side: usize, size: usize) -> [f64]", Some(door_num_fft_embed_kernel)),
    ("num.fft.log_spectrum", "(field: [f64], size: usize) -> [f64]", Some(door_num_fft_log_spectrum)),
    ("num.fft.magnitude_spectrum", "(field: [f64], size: usize) -> [f64]", Some(door_num_fft_magnitude_spectrum)),
    ("num.fft.peak_ring", "(profile: [f64]) -> usize", Some(door_num_fft_peak_ring)),
    ("num.fft.peak_wavelength", "(profile: [f64], size: usize) -> f64", Some(door_num_fft_peak_wavelength)),
    ("num.fft.radial_profile", "(spectrum: [f64], size: usize) -> [f64]", Some(door_num_fft_radial_profile)),
    ("num.fft.transform", "(field: [f64], size: usize) -> ([f64], [f64])", Some(door_num_fft_transform)),
    ("num.gauss.Class.prime", "(self: num.gauss.Class) -> bool", Some(door_num_gauss_class_prime)),
    ("num.gauss.Class.word", "(self: num.gauss.Class) -> String", Some(door_num_gauss_class_word)),
    ("num.gauss.Ring.associates", "(self: num.gauss.Ring, a: i64, b: i64) -> [(i64, i64)]", Some(door_num_gauss_ring_associates)),
    ("num.gauss.Ring.canon", "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)", Some(door_num_gauss_ring_canon)),
    ("num.gauss.Ring.conjugate", "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)", Some(door_num_gauss_ring_conjugate)),
    ("num.gauss.Ring.count", "(self: num.gauss.Ring, radius: u64) -> usize", Some(door_num_gauss_ring_count)),
    ("num.gauss.Ring.div_rem", "(self: num.gauss.Ring, z: (i64, i64), w: (i64, i64)) -> ((i64, i64), (i64, i64))", Some(door_num_gauss_ring_div_rem)),
    ("num.gauss.Ring.fate", "(self: num.gauss.Ring, n: u64) -> num.gauss.Class", Some(door_num_gauss_ring_fate)),
    ("num.gauss.Ring.gaussian_gcd", "(self: num.gauss.Ring, z: (i64, i64), w: (i64, i64)) -> (i64, i64)", Some(door_num_gauss_ring_gaussian_gcd)),
    ("num.gauss.Ring.inert", "(self: num.gauss.Ring, p: u64) -> bool", Some(door_num_gauss_ring_inert)),
    ("num.gauss.Ring.mul", "(self: num.gauss.Ring, arg1: (i64, i64), arg2: (i64, i64)) -> (i64, i64)", Some(door_num_gauss_ring_mul)),
    ("num.gauss.Ring.named", "(name: str) -> num.gauss.Ring?", Some(door_num_gauss_ring_named)),
    ("num.gauss.Ring.nearest", "(self: num.gauss.Ring, x: f64, y: f64) -> (i64, i64)", Some(door_num_gauss_ring_nearest)),
    ("num.gauss.Ring.norm", "(self: num.gauss.Ring, a: i64, b: i64) -> u64", Some(door_num_gauss_ring_norm)),
    ("num.gauss.Ring.place", "(self: num.gauss.Ring, a: i64, b: i64) -> (f64, f64)", Some(door_num_gauss_ring_place)),
    ("num.gauss.Ring.ramified", "(self: num.gauss.Ring) -> u64", Some(door_num_gauss_ring_ramified)),
    ("num.gauss.Ring.reach", "(self: num.gauss.Ring, a: i64, b: i64) -> u64", Some(door_num_gauss_ring_reach)),
    ("num.gauss.Ring.symmetry", "(self: num.gauss.Ring) -> usize", Some(door_num_gauss_ring_symmetry)),
    ("num.gauss.Ring.top", "(self: num.gauss.Ring, radius: u64) -> u64", Some(door_num_gauss_ring_top)),
    ("num.gauss.Ring.turn", "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)", Some(door_num_gauss_ring_turn)),
    ("num.gauss.Ring.units", "(self: num.gauss.Ring) -> usize", Some(door_num_gauss_ring_units)),
    ("num.gauss.Ring.whole", "(self: num.gauss.Ring, a: i64, b: i64) -> u64?", Some(door_num_gauss_ring_whole)),
    ("num.gauss.Window.census", "(self: num.gauss.Window) -> num.gauss.Census", Some(door_num_gauss_window_census)),
    ("num.gauss.Window.class", "(self: num.gauss.Window, a: i64, b: i64) -> num.gauss.Class", Some(door_num_gauss_window_class)),
    ("num.gauss.Window.holds", "(self: num.gauss.Window, a: i64, b: i64) -> bool", Some(door_num_gauss_window_holds)),
    ("num.gauss.Window.new", "(ring: num.gauss.Ring, radius: u64) -> num.gauss.Window", Some(door_num_gauss_window_new)),
    ("num.gauss.Window.points", "(self: num.gauss.Window) -> [(i64, i64)]", Some(door_num_gauss_window_points)),
    ("num.gauss.Window.radius", "(self: num.gauss.Window) -> u64", Some(door_num_gauss_window_radius)),
    ("num.gauss.Window.ring", "(self: num.gauss.Window) -> num.gauss.Ring", Some(door_num_gauss_window_ring)),
    ("num.gauss.classes", "(ring: num.gauss.Ring, bound: u64) -> [(i64, i64)]", Some(door_num_gauss_classes)),
    ("num.gauss.peak", "(ring: num.gauss.Ring, limit: usize) -> (usize, u32)", Some(door_num_gauss_peak)),
    ("num.gauss.shells", "(ring: num.gauss.Ring, limit: usize) -> [u32]", Some(door_num_gauss_shells)),
    ("num.ladder.Design.abscissa", "(self: num.ladder.Design) -> f64", Some(door_num_ladder_design_abscissa)),
    ("num.ladder.Design.base", "(self: num.ladder.Design) -> u64", Some(door_num_ladder_design_base)),
    ("num.ladder.Design.digits", "(self: num.ladder.Design) -> [u64]", Some(door_num_ladder_design_digits)),
    ("num.ladder.Design.new", "(base: u64, digits: [u64]) -> num.ladder.Design", Some(door_num_ladder_design_new)),
    ("num.ladder.Design.peel", "(self: num.ladder.Design) -> usize", Some(door_num_ladder_design_peel)),
    ("num.ladder.Design.period", "(self: num.ladder.Design) -> f64", Some(door_num_ladder_design_period)),
    ("num.ladder.Design.pole", "(self: num.ladder.Design, m: usize, j: i64) -> num.zeta.Complex", Some(door_num_ladder_design_pole)),
    ("num.ladder.Design.with_peel", "(base: u64, digits: [u64], peel: usize) -> num.ladder.Design", Some(door_num_ladder_design_with_peel)),
    ("num.ladder.cofactor", "(design: num.ladder.Design, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)", Some(door_num_ladder_cofactor)),
    ("num.ladder.residue", "(design: num.ladder.Design, m: usize, j: i64, tolerance: f64) -> (num.zeta.Complex, f64)", Some(door_num_ladder_residue)),
    ("num.ladder.zeta", "(design: num.ladder.Design, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)", Some(door_num_ladder_zeta)),
    ("num.lattice.coprime_pairs", "(n: usize) -> u64", Some(door_num_lattice_coprime_pairs)),
    ("num.lattice.farey", "(order: usize) -> [num.lattice.Node]", Some(door_num_lattice_farey)),
    ("num.lattice.grid", "(n: usize) -> [num.lattice.Node2d]", Some(door_num_lattice_grid)),
    ("num.lattice.new_nodes", "(n: usize) -> u64", Some(door_num_lattice_new_nodes)),
    ("num.lattice.pi_estimate", "(n: usize) -> f64", Some(door_num_lattice_pi_estimate)),
    ("num.lattice.recovered", "(n: usize, dimension: u32) -> f64", Some(door_num_lattice_recovered)),
    ("num.lattice.visible_density", "(dimension: u32) -> f64", Some(door_num_lattice_visible_density)),
    ("num.lattice.zeta_factor", "(dimension: u32) -> f64?", Some(door_num_lattice_zeta_factor)),
    ("num.lattice.zeta_whole", "(s: u32) -> f64", Some(door_num_lattice_zeta_whole)),
    ("num.memory.Rule.accepts", "(self: num.memory.Rule, word: [usize]) -> bool", Some(door_num_memory_rule_accepts)),
    ("num.memory.Rule.allowed", "(self: num.memory.Rule, window: usize) -> bool", Some(door_num_memory_rule_allowed)),
    ("num.memory.Rule.alphabet", "(self: num.memory.Rule) -> [usize]", Some(door_num_memory_rule_alphabet)),
    ("num.memory.Rule.codes", "(self: num.memory.Rule) -> u128", Some(door_num_memory_rule_codes)),
    ("num.memory.Rule.full", "(dimension: usize, width: usize) -> num.memory.Rule", Some(door_num_memory_rule_full)),
    ("num.memory.Rule.letters", "(self: num.memory.Rule) -> usize", Some(door_num_memory_rule_letters)),
    ("num.memory.Rule.new", "(dimension: usize, width: usize, code: u64) -> num.memory.Rule", Some(door_num_memory_rule_new)),
    ("num.memory.Rule.states", "(self: num.memory.Rule) -> usize", Some(door_num_memory_rule_states)),
    ("num.memory.Rule.windows", "(self: num.memory.Rule) -> usize", Some(door_num_memory_rule_windows)),
    ("num.memory.allowed_windows", "(rule: num.memory.Rule) -> usize", Some(door_num_memory_allowed_windows)),
    ("num.memory.cells", "(rule: num.memory.Rule, level: usize) -> [u64]", Some(door_num_memory_cells)),
    ("num.memory.counts", "(rule: num.memory.Rule, levels: usize) -> [u64]", Some(door_num_memory_counts)),
    ("num.memory.exponent", "(rule: num.memory.Rule) -> f64", Some(door_num_memory_exponent)),
    ("num.memory.kappa", "(rule: num.memory.Rule) -> f64", Some(door_num_memory_kappa)),
    ("num.memory.perron", "(rule: num.memory.Rule) -> f64", Some(door_num_memory_perron)),
    ("num.memory.transfer", "(rule: num.memory.Rule) -> [[u64]]", Some(door_num_memory_transfer)),
    ("num.morse.Lift.all", "() -> [num.morse.Lift; 4]", Some(door_num_morse_lift_all)),
    ("num.morse.Lift.at", "(self: num.morse.Lift, i: u64, j: u64) -> u8", Some(door_num_morse_lift_at)),
    ("num.morse.Lift.formula", "(self: num.morse.Lift) -> String", Some(door_num_morse_lift_formula)),
    ("num.morse.boundary", "(word: [u8]) -> [u8]", Some(door_num_morse_boundary)),
    ("num.morse.difference", "(a: [u8], b: [u8]) -> [u8]", Some(door_num_morse_difference)),
    ("num.morse.digits", "(length: usize) -> [u8]", Some(door_num_morse_digits)),
    ("num.morse.doubling", "(length: usize) -> [u8]", Some(door_num_morse_doubling)),
    ("num.morse.faults", "(a: [u8], b: [u8]) -> usize", Some(door_num_morse_faults)),
    ("num.morse.fold", "(grid: [u8], side: usize, number: usize) -> num.morse.Fold", Some(door_num_morse_fold)),
    ("num.morse.letter", "(place: u64) -> u8", Some(door_num_morse_letter)),
    ("num.morse.lift", "(kind: num.morse.Lift, side: usize) -> [u8]", Some(door_num_morse_lift)),
    ("num.morse.power", "(tile: [u8], number: usize, level: usize) -> [u8]", Some(door_num_morse_power)),
    ("num.morse.repeat", "(tile: [u8], number: usize, side: usize) -> [u8]", Some(door_num_morse_repeat)),
    ("num.morse.runs", "(word: [u8]) -> [usize]", Some(door_num_morse_runs)),
    ("num.morse.stage", "(rounds: usize) -> [u8]", Some(door_num_morse_stage)),
    ("num.morse.substitution", "(length: usize) -> [u8]", Some(door_num_morse_substitution)),
    ("num.morse.upsample", "(grid: [u8], side: usize, scale: usize) -> [u8]", Some(door_num_morse_upsample)),
    ("num.prime.Sieve.count", "(self: num.prime.Sieve) -> usize", Some(door_num_prime_sieve_count)),
    ("num.prime.Sieve.done", "(self: num.prime.Sieve) -> bool", Some(door_num_prime_sieve_done)),
    ("num.prime.Sieve.finish", "(self: num.prime.Sieve) -> null # uncallable: mutates its argument in place", None),
    ("num.prime.Sieve.new", "(limit: usize) -> num.prime.Sieve", Some(door_num_prime_sieve_new)),
    ("num.prime.Sieve.rank", "(self: num.prime.Sieve) -> usize", Some(door_num_prime_sieve_rank)),
    ("num.prime.Sieve.step", "(self: num.prime.Sieve) -> usize # uncallable: mutates its argument in place", None),
    ("num.prime.Sieve.struck", "(self: num.prime.Sieve) -> usize", Some(door_num_prime_sieve_struck)),
    ("num.prime.Sieve.types", "(self: num.prime.Sieve) -> [u8]", Some(door_num_prime_sieve_types)),
    ("num.prime.chart", "(top: usize, bins: usize) -> [num.prime.Reading]", Some(door_num_prime_chart)),
    ("num.prime.flags", "(limit: usize) -> [bool]", Some(door_num_prime_flags)),
    ("num.prime.goldbach", "(number: usize) -> usize", Some(door_num_prime_goldbach)),
    ("num.prime.goldbach_record", "(top: usize) -> [usize]", Some(door_num_prime_goldbach_record)),
    ("num.prime.is_prime", "(number: usize) -> bool", Some(door_num_prime_is_prime)),
    ("num.prime.pile", "(number: u64) -> num.prime.Pile", Some(door_num_prime_pile)),
    ("num.prime.prime_count", "(n: usize) -> usize", Some(door_num_prime_prime_count)),
    ("num.prime.prime_from", "(number: usize) -> usize", Some(door_num_prime_prime_from)),
    ("num.prime.primes", "(limit: usize) -> [usize]", Some(door_num_prime_primes)),
    ("num.prime.rectangles", "(number: usize) -> [(usize, usize)]", Some(door_num_prime_rectangles)),
    ("num.prime.splits", "(number: usize) -> [(usize, usize)]", Some(door_num_prime_splits)),
    ("num.prime.squares", "(number: usize) -> (usize, usize)?", Some(door_num_prime_squares)),
    ("num.prime.study", "(limit: usize) -> [num.prime.Prime]", Some(door_num_prime_study)),
    ("num.radix.Base.class", "(self: num.radix.Base, z: (i64, i64)) -> usize", Some(door_num_radix_base_class)),
    ("num.radix.Base.congruent", "(self: num.radix.Base, z: (i64, i64), w: (i64, i64)) -> bool", Some(door_num_radix_base_congruent)),
    ("num.radix.Base.group", "(self: num.radix.Base) -> [[usize]]", Some(door_num_radix_base_group)),
    ("num.radix.Base.mirrored", "(self: num.radix.Base) -> bool", Some(door_num_radix_base_mirrored)),
    ("num.radix.Base.new", "(ring: num.gauss.Ring, value: (i64, i64)) -> num.radix.Base", Some(door_num_radix_base_new)),
    ("num.radix.Base.norm", "(self: num.radix.Base) -> u64", Some(door_num_radix_base_norm)),
    ("num.radix.Base.power", "(self: num.radix.Base, level: usize) -> (i64, i64)", Some(door_num_radix_base_power)),
    ("num.radix.Base.residues", "(self: num.radix.Base) -> [(i64, i64)]", Some(door_num_radix_base_residues)),
    ("num.radix.Base.ring", "(self: num.radix.Base) -> num.gauss.Ring", Some(door_num_radix_base_ring)),
    ("num.radix.Base.value", "(self: num.radix.Base) -> (i64, i64)", Some(door_num_radix_base_value)),
    ("num.radix.Radix.base", "(self: num.radix.Radix) -> num.radix.Base", Some(door_num_radix_radix_base)),
    ("num.radix.Radix.canonical", "(self: num.radix.Radix) -> bool", Some(door_num_radix_radix_canonical)),
    ("num.radix.Radix.code", "(self: num.radix.Radix) -> u128", Some(door_num_radix_radix_code)),
    ("num.radix.Radix.digits", "(self: num.radix.Radix) -> [(i64, i64)]", Some(door_num_radix_radix_digits)),
    ("num.radix.Radix.dimension", "(self: num.radix.Radix) -> f64", Some(door_num_radix_radix_dimension)),
    ("num.radix.Radix.distinct", "(self: num.radix.Radix, level: usize) -> usize", Some(door_num_radix_radix_distinct)),
    ("num.radix.Radix.fill", "(self: num.radix.Radix, level: usize) -> u128", Some(door_num_radix_radix_fill)),
    ("num.radix.Radix.from_code", "(base: num.radix.Base, code: u128) -> num.radix.Radix", Some(door_num_radix_radix_from_code)),
    ("num.radix.Radix.new", "(base: num.radix.Base, digits: [(i64, i64)], twists: [(i64, i64)]) -> num.radix.Radix", Some(door_num_radix_radix_new)),
    ("num.radix.Radix.plane", "(self: num.radix.Radix, level: usize) -> [(f64, f64)]", Some(door_num_radix_radix_plane)),
    ("num.radix.Radix.ring", "(self: num.radix.Radix) -> num.gauss.Ring", Some(door_num_radix_radix_ring)),
    ("num.radix.Radix.size", "(self: num.radix.Radix) -> usize", Some(door_num_radix_radix_size)),
    ("num.radix.Radix.twists", "(self: num.radix.Radix) -> [(i64, i64)]", Some(door_num_radix_radix_twists)),
    ("num.radix.Radix.with_twists", "(self: num.radix.Radix, units: [usize]) -> num.radix.Radix", Some(door_num_radix_radix_with_twists)),
    ("num.radix.Radix.words", "(self: num.radix.Radix, level: usize) -> [(i64, i64)]", Some(door_num_radix_radix_words)),
    ("num.radix.flowsnake", "() -> num.radix.Radix", Some(door_num_radix_flowsnake)),
    ("num.radix.gasket", "() -> num.radix.Radix", Some(door_num_radix_gasket)),
    ("num.radix.koch", "() -> num.radix.Radix", Some(door_num_radix_koch)),
    ("num.radix.terdragon", "() -> num.radix.Radix", Some(door_num_radix_terdragon)),
    ("num.radix.tile", "(m: u64, code: u128) -> num.radix.Radix", Some(door_num_radix_tile)),
    ("num.radix.twindragon", "() -> num.radix.Radix", Some(door_num_radix_twindragon)),
    ("num.series.basel", "(n: usize) -> f64", Some(door_num_series_basel)),
    ("num.series.bernoulli", "(count: usize) -> [(i128, i128)]", Some(door_num_series_bernoulli)),
    ("num.series.beta", "(s: f64, terms: usize) -> f64", Some(door_num_series_beta)),
    ("num.series.binary", "(limit: usize) -> [usize]", Some(door_num_series_binary)),
    ("num.series.catalan", "(limit: usize) -> [usize]", Some(door_num_series_catalan)),
    ("num.series.chi3", "(number: usize) -> i8", Some(door_num_series_chi3)),
    ("num.series.chi4", "(number: usize) -> i8", Some(door_num_series_chi4)),
    ("num.series.chi8", "(number: usize) -> i8", Some(door_num_series_chi8)),
    ("num.series.dirichlet", "(s: f64, rhythm: [i8], terms: usize) -> f64", Some(door_num_series_dirichlet)),
    ("num.series.e_partial", "(n: usize) -> f64", Some(door_num_series_e_partial)),
    ("num.series.euler_gamma_partial", "(n: usize) -> f64", Some(door_num_series_euler_gamma_partial)),
    ("num.series.euler_product", "(s: f64, limit: usize) -> f64", Some(door_num_series_euler_product)),
    ("num.series.evens", "(limit: usize) -> [usize]", Some(door_num_series_evens)),
    ("num.series.fibonacci", "(limit: usize) -> [usize]", Some(door_num_series_fibonacci)),
    ("num.series.harmonic", "(terms: usize) -> f64", Some(door_num_series_harmonic)),
    ("num.series.lambda", "(s: f64, terms: usize) -> f64", Some(door_num_series_lambda)),
    ("num.series.leibniz", "(n: usize) -> f64", Some(door_num_series_leibniz)),
    ("num.series.li", "(x: f64) -> f64", Some(door_num_series_li)),
    ("num.series.mertens", "(n: usize) -> i64", Some(door_num_series_mertens)),
    ("num.series.odds", "(limit: usize) -> [usize]", Some(door_num_series_odds)),
    ("num.series.visible", "(limit: usize, dimension: u32) -> u128", Some(door_num_series_visible)),
    ("num.series.wallis_half_pi", "(n: usize) -> f64", Some(door_num_series_wallis_half_pi)),
    ("num.series.wallis_quarter_pi", "(factors: usize) -> f64", Some(door_num_series_wallis_quarter_pi)),
    ("num.series.zeta", "(s: f64, terms: usize) -> f64", Some(door_num_series_zeta)),
    ("num.sieve.cells", "(word: [u64], dimension: u32) -> u128", Some(door_num_sieve_cells)),
    ("num.sieve.exponent", "(word: [u64], dimension: u32) -> f64", Some(door_num_sieve_exponent)),
    ("num.sieve.flat_word", "(side: u64, levels: usize) -> [u64]", Some(door_num_sieve_flat_word)),
    ("num.sieve.holes", "(word: [u64], dimension: u32) -> u128", Some(door_num_sieve_holes)),
    ("num.sieve.limit", "(word: [u64], dimension: u32) -> f64?", Some(door_num_sieve_limit)),
    ("num.sieve.odd_word", "(levels: usize) -> [u64]", Some(door_num_sieve_odd_word)),
    ("num.sieve.punctures", "(word: [u64], dimension: u32) -> [u64]", Some(door_num_sieve_punctures)),
    ("num.sieve.raster", "(word: [u64]) -> (usize, [u8])", Some(door_num_sieve_raster)),
    ("num.sieve.ratio", "(word: [u64], dimension: u32) -> f64", Some(door_num_sieve_ratio)),
    ("num.sieve.side", "(word: [u64]) -> u128", Some(door_num_sieve_side)),
    ("num.sieve.solid_limit", "() -> f64", Some(door_num_sieve_solid_limit)),
    ("num.spiral.Growth.all", "() -> [num.spiral.Growth; 2]", Some(door_num_spiral_growth_all)),
    ("num.spiral.Lattice.all", "() -> [num.spiral.Lattice; 2]", Some(door_num_spiral_lattice_all)),
    ("num.spiral.Lattice.count", "(self: num.spiral.Lattice, side: usize) -> usize", Some(door_num_spiral_lattice_count)),
    ("num.spiral.Lattice.n", "(self: num.spiral.Lattice, x: i64, y: i64) -> u64", Some(door_num_spiral_lattice_n)),
    ("num.spiral.Lattice.radius", "(self: num.spiral.Lattice, side: usize) -> usize", Some(door_num_spiral_lattice_radius)),
    ("num.spiral.Lattice.ring", "(self: num.spiral.Lattice, n: u64) -> u64", Some(door_num_spiral_lattice_ring)),
    ("num.spiral.Lattice.ring_of", "(self: num.spiral.Lattice, x: i64, y: i64) -> u64", Some(door_num_spiral_lattice_ring_of)),
    ("num.spiral.Lattice.xy", "(self: num.spiral.Lattice, n: u64) -> (i64, i64)", Some(door_num_spiral_lattice_xy)),
    ("num.spiral.Mark.all", "() -> [num.spiral.Mark; 4]", Some(door_num_spiral_mark_all)),
    ("num.spiral.diagonal", "(lattice: num.spiral.Lattice, side: usize, a: i64, b: i64, c: i64) -> num.spiral.Diagonal", Some(door_num_spiral_diagonal)),
    ("num.spiral.level_of", "(n: u64, base: u64) -> u32", Some(door_num_spiral_level_of)),
    ("num.spiral.marks", "(mark: num.spiral.Mark, limit: usize) -> [i8]", Some(door_num_spiral_marks)),
    ("num.spiral.snail", "(base: u64, top: u64, growth: num.spiral.Growth) -> num.spiral.Snail", Some(door_num_spiral_snail)),
    ("num.zeta.Complex.abs", "(self: num.zeta.Complex) -> f64", Some(door_num_zeta_complex_abs)),
    ("num.zeta.Complex.arg", "(self: num.zeta.Complex) -> f64", Some(door_num_zeta_complex_arg)),
    ("num.zeta.Complex.exp", "(self: num.zeta.Complex) -> num.zeta.Complex", Some(door_num_zeta_complex_exp)),
    ("num.zeta.Complex.ln", "(self: num.zeta.Complex) -> num.zeta.Complex", Some(door_num_zeta_complex_ln)),
    ("num.zeta.Complex.new", "(re: f64, im: f64) -> num.zeta.Complex", Some(door_num_zeta_complex_new)),
    ("num.zeta.Complex.turn", "(angle: f64) -> num.zeta.Complex", Some(door_num_zeta_complex_turn)),
    ("num.zeta.Line.count", "(self: num.zeta.Line, t: f64) -> usize", Some(door_num_zeta_line_count)),
    ("num.zeta.Line.exact", "(self: num.zeta.Line, t: f64) -> f64", Some(door_num_zeta_line_exact)),
    ("num.zeta.Line.gram", "(self: num.zeta.Line, n: i64) -> f64", Some(door_num_zeta_line_gram)),
    ("num.zeta.Line.maclaurin", "(self: num.zeta.Line, t: f64) -> num.zeta.Complex", Some(door_num_zeta_line_maclaurin)),
    ("num.zeta.Line.new", "() -> num.zeta.Line", Some(door_num_zeta_line_new)),
    ("num.zeta.Line.novelty_coefficients", "(self: num.zeta.Line, gammas: [f64]) -> [num.zeta.Complex]", Some(door_num_zeta_line_novelty_coefficients)),
    ("num.zeta.Line.pair", "(self: num.zeta.Line, s: num.zeta.Complex) -> (num.zeta.Complex, num.zeta.Complex)", Some(door_num_zeta_line_pair)),
    ("num.zeta.Line.point", "(self: num.zeta.Line, t: f64) -> (num.zeta.Complex, f64)", Some(door_num_zeta_line_point)),
    ("num.zeta.Line.seam", "(self: num.zeta.Line, t0: f64, t1: f64, steps: usize) -> f64", Some(door_num_zeta_line_seam)),
    ("num.zeta.Line.siegel", "(self: num.zeta.Line, t: f64) -> f64", Some(door_num_zeta_line_siegel)),
    ("num.zeta.Line.theta", "(self: num.zeta.Line, t: f64) -> f64", Some(door_num_zeta_line_theta)),
    ("num.zeta.Line.z", "(self: num.zeta.Line, t: f64) -> f64", Some(door_num_zeta_line_z)),
    ("num.zeta.Line.zeros", "(self: num.zeta.Line, count: usize) -> [f64]", Some(door_num_zeta_line_zeros)),
    ("num.zeta.bump", "(u: f64) -> f64", Some(door_num_zeta_bump)),
    ("num.zeta.corrections", "(p: f64) -> [f64; 4]", Some(door_num_zeta_corrections)),
    ("num.zeta.kernel", "(p: f64) -> f64", Some(door_num_zeta_kernel)),
    ("num.zeta.mellin", "(s: num.zeta.Complex) -> num.zeta.Complex", Some(door_num_zeta_mellin)),
    ("num.zeta.novelty_main", "() -> f64", Some(door_num_zeta_novelty_main)),
    ("num.zeta.novelty_wave", "(gammas: [f64], coef: [num.zeta.Complex], log_y: f64) -> f64", Some(door_num_zeta_novelty_wave)),
    ("num.zeta.psi_formula", "(x: f64, gammas: [f64]) -> f64", Some(door_num_zeta_psi_formula)),
    ("num.zeta.psi_stair", "(x: usize) -> [f64]", Some(door_num_zeta_psi_stair)),
    ("num.zeta.raise", "(base: f64, exponent: num.zeta.Complex) -> num.zeta.Complex", Some(door_num_zeta_raise)),
    ("num.zeta.sharp_novelty", "(prefix: [u64], y: f64) -> f64", Some(door_num_zeta_sharp_novelty)),
    ("num.zeta.smoothed_novelty", "(phi: [u64], y: f64, main: f64) -> f64", Some(door_num_zeta_smoothed_novelty)),
];

// CALLS

fn door_core_colorizer_diverge(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::core::Colorizer::diverge()))
}

fn door_core_colorizer_fire(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::core::Colorizer::fire()))
}

fn door_core_colorizer_gradient_bins(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Color = color(name, 0, &args[0])?;
    let a1: Vec<mrlyrs::core::Color> = { let mut list0 = Vec::new(); for item0 in items(name, 1, &args[1])? { list0.push(color(name, 1, item0)?); } list0 };
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::core::Colorizer::gradient_bins(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_colorizer_heat(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::core::Colorizer::heat()))
}

fn door_core_dtype_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Dtype = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Dtype::max(a0)))
}

fn door_core_image_colors(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Image = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Image::colors(&a0)))
}

fn door_core_image_from_pixels(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<[u8; 4]> = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::core::Image::from_pixels(a0, a1, &a2)))
}

fn door_core_image_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<Vec<usize>> = take!(name, 2, &args[2]);
    let a3: Vec<mrlyrs::core::Color> = { let mut list0 = Vec::new(); for item0 in items(name, 3, &args[3])? { list0.push(color(name, 3, item0)?); } list0 };
    Ok(give!(mrlyrs::core::Image::new(a0, a1, a2, a3)))
}

fn door_core_image_png(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Image = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::core::Image::png(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_image_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::core::Image = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrlyrs::core::Filter = take!(name, 3, &args[3]);
    match mrlyrs::core::Image::resample(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_anti(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Cell::anti(a0)))
}

fn door_core_cell_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Cell::binarize(a0, a1)))
}

fn door_core_cell_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Cell::binarize_otsu(a0)))
}

fn door_core_cell_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrlyrs::core::Cell::blur(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_color_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Cell::color_at(&a0, a1)))
}

fn door_core_cell_combine(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Cell = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Cell::combine(&a0, &a1)))
}

fn door_core_cell_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::core::Cell::fractal(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Cell::invert(a0)))
}

fn door_core_cell_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Cell::layers(a0, a1)))
}

fn door_core_cell_magic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::core::Cell> = take!(name, 0, &args[0]);
    match mrlyrs::core::cell::magic(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_mapping(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok({ let mut pairs0 = serde_json::Map::new(); for (key0, item0) in mrlyrs::core::cell::mapping() { pairs0.insert(key0.to_string(), { let mut list1 = Vec::new(); for item1 in item0 { list1.push(tint(item1)); } Value::Array(list1) }); } Value::Object(pairs0) })
}

fn door_core_cell_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrlyrs::core::Cell> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::cell::merge(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_moore(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::cell::moore(a0)))
}

fn door_core_cell_mosaic(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::core::Cell> = take!(name, 1, &args[1]);
    match mrlyrs::core::cell::mosaic(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_neighbors(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: mrlyrs::core::Dtype = take!(name, 4, &args[4]);
    match mrlyrs::core::Cell::neighbors(a0, &a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Cell::new(a0)))
}

fn door_core_cell_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::core::Cell::pad(a0, a1, a2)))
}

fn door_core_cell_paint(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1 = { let mut pairs0: Vec<(u8, Vec<mrlyrs::core::Color>)> = Vec::new(); for (text0, item0) in fields(name, 1, &args[1])? { let key0 = match text0.parse() { Ok(key) => key, Err(_) => return Err(bad(name, 1, "an object key")) }; pairs0.push((key0, { let mut list1 = Vec::new(); for item1 in items(name, 1, item0)? { list1.push(color(name, 1, item1)?); } list1 })); } pairs0 }.into_iter().collect();
    let a2: mrlyrs::core::Mode = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() { None } else { Some(mrlyrs::core::Rng::new(seed(name, 3, &args[3])?)) };
    match mrlyrs::core::Cell::paint(a0, &a1, a2, a3.as_mut()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrlyrs::core::Cell::perforate(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_remap(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    match mrlyrs::core::cell::remap(&a0, &a1, &a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_rgba(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Cell::rgba(&a0)))
}

fn door_core_cell_rot90_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrlyrs::core::cell::rot90_map(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_rotate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrlyrs::core::Cell::rotate(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_shape(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Cell::shape(&a0)))
}

fn door_core_cell_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Cell::size(&a0)))
}

fn door_core_cell_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Cell = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::Cell::tile(a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_cell_tile_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::cell::tile_map(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_codec_gif(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]); let b0: Vec<&[u8]> = a0.iter().map(|item| item.as_slice()).collect();
    let a1: Vec<[u8; 4]> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrlyrs::core::codec::gif(&b0, &a1, a2, a3, a4, a5) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_codec_png(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::core::codec::png(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_colors_theme_hues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::colors::Theme = take!(name, 0, &args[0]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::core::colors::Theme::hues(&a0) { list0.push(tint(item0)); } Value::Array(list0) })
}

fn door_core_colors_theme_inks(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::colors::Theme = take!(name, 0, &args[0]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::core::colors::Theme::inks(&a0) { list0.push(tint(item0)); } Value::Array(list0) })
}

fn door_core_colors_alpha(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Color = color(name, 0, &args[0])?;
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(tint(mrlyrs::core::Color::alpha(&a0, a1)))
}

fn door_core_colors_board(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: bool = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::colors::board(a0)))
}

fn door_core_colors_css(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Color = color(name, 0, &args[0])?;
    Ok(give!(mrlyrs::core::Color::css(&a0)))
}

fn door_core_colors_from_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::core::Color::from_hex(a0.as_str()) { Ok(value) => Ok(tint(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_colors_gradient(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrlyrs::core::Color> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(color(name, 0, item0)?); } list0 };
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::core::colors::gradient(&a0, a1) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(tint(item0)); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_colors_ink(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: bool = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::colors::ink(a0)))
}

fn door_core_colors_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Color = color(name, 0, &args[0])?;
    Ok(tint(mrlyrs::core::Color::invert(&a0)))
}

fn door_core_colors_lightness(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Color = color(name, 0, &args[0])?;
    let a1: u8 = take!(name, 1, &args[1]);
    match mrlyrs::core::Color::lightness(&a0, a1) { Ok(value) => Ok(tint(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_colors_luma_types(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::core::colors::luma_types(&a0, a1, a2, a3)))
}

fn door_core_colors_mix(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Color = color(name, 0, &args[0])?;
    let a1: mrlyrs::core::Color = color(name, 1, &args[1])?;
    let a2: f64 = take!(name, 2, &args[2]);
    match mrlyrs::core::colors::mix(a0, a1, a2) { Ok(value) => Ok(tint(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_colors_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::core::colors::named(a0.as_str()) { Ok(value) => Ok(tint(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_colors_random(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: bool = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(tint(mrlyrs::core::Color::random(a0, &mut a1)))
}

fn door_core_colors_rgb(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(tint(mrlyrs::core::Color::rgb(a0, a1, a2)))
}

fn door_core_colors_rgba(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(tint(mrlyrs::core::Color::rgba(a0, a1, a2, a3)))
}

fn door_core_colors_shades(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Color = color(name, 0, &args[0])?;
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::core::colors::shades(a0) { list0.push(tint(item0)); } Value::Array(list0) })
}

fn door_core_colors_snap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::core::Color> = { let mut list0 = Vec::new(); for item0 in items(name, 1, &args[1])? { list0.push(color(name, 1, item0)?); } list0 };
    Ok(give!(mrlyrs::core::colors::snap(&a0, &a1)))
}

fn door_core_colors_to_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Color = color(name, 0, &args[0])?;
    Ok(give!(mrlyrs::core::Color::to_hex(&a0)))
}

fn door_core_error_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::core::error::parse(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_hex_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: mrlyrs::core::Filter = take!(name, 4, &args[4]);
    match mrlyrs::core::hex_fit(&a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_hex_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::core::hex_size(a0, a1, a2)))
}

fn door_core_image_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::core::image::blur(&a0, a1, a2, a3)))
}

fn door_core_paint_edition_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::core::paint::Edition::all()))
}

fn door_core_paint_edition_mode(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::paint::Edition = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::paint::Edition::mode(a0)))
}

fn door_core_paint_ink_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::core::paint::Ink::all()))
}

fn door_core_paint_ink_color(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::paint::Ink = take!(name, 0, &args[0]);
    Ok(tint(mrlyrs::core::paint::Ink::color(a0)))
}

fn door_core_paint_paint_is_simple(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::paint::Paint = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::paint::Paint::is_simple(&a0)))
}

fn door_core_paint_paint_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::paint::Edition = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::paint::Paint::new(a0)))
}

fn door_core_paint_scheme_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::core::paint::Scheme::all()))
}

fn door_core_paint_target_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::core::paint::Target::all()))
}

fn door_core_paint_random_edition(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Option<Vec<mrlyrs::core::paint::Edition>> = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrlyrs::core::paint::random_edition(a0.as_deref(), &mut a1)))
}

fn door_core_paint_reroll(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::paint::Paint = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrlyrs::core::paint::reroll(a0, &mut a1)))
}

fn door_core_paint_setup(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::paint::Paint = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::paint::Config = take!(name, 1, &args[1]);
    let mut a2 = mrlyrs::core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrlyrs::core::paint::setup(a0, &a1, &mut a2)))
}

fn door_core_ramp_color(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Colorizer = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(tint(mrlyrs::core::ramp::color(&a0, a1, a2)))
}

fn door_core_ramp_colors(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Colorizer = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::core::ramp::colors(&a0, &a1, a2)))
}

fn door_core_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: mrlyrs::core::Filter = take!(name, 5, &args[5]);
    match mrlyrs::core::resample(&a0, a1, a2, a3, a4, a5) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_rng_below(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let mut a0 = mrlyrs::core::Rng::new(seed(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Rng::below(&mut a0, a1)))
}

fn door_core_rng_boolean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrlyrs::core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrlyrs::core::Rng::boolean(&mut a0)))
}

fn door_core_rng_chance(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let mut a0 = mrlyrs::core::Rng::new(seed(name, 0, &args[0])?);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Rng::chance(&mut a0, a1)))
}

fn door_core_rng_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Rng::new(a0)))
}

fn door_core_rng_range(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let mut a0 = mrlyrs::core::Rng::new(seed(name, 0, &args[0])?);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::core::Rng::range(&mut a0, a1, a2)))
}

fn door_core_rng_sample_indices(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let mut a0 = mrlyrs::core::Rng::new(seed(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::core::Rng::sample_indices(&mut a0, a1, a2)))
}

fn door_core_rng_unit(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrlyrs::core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrlyrs::core::Rng::unit(&mut a0)))
}

fn door_core_tensor_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::at(&a0, a1)))
}

fn door_core_tensor_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::binarize(&a0, a1)))
}

fn door_core_tensor_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Tensor::binarize_otsu(&a0)))
}

fn door_core_tensor_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrlyrs::core::Tensor::blur(&a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_bytes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::core::Tensor::bytes(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::count(&a0, a1)))
}

fn door_core_tensor_dtype(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Tensor::dtype(&a0)))
}

fn door_core_tensor_exposed(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::core::Tensor::exposed(&a0).to_string()))
}

fn door_core_tensor_filled(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: mrlyrs::core::Dtype = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::core::Tensor::filled(a0, a1, a2)))
}

fn door_core_tensor_flip(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::core::Tensor::flip(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::fractal(&a0, a1)))
}

fn door_core_tensor_full(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::full(a0, a1)))
}

fn door_core_tensor_get(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::Tensor::get(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_i32(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i32> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::Tensor::i32(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_i32s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::core::Tensor::i32s(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::index(&a0, &a1)))
}

fn door_core_tensor_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Tensor::invert(&a0)))
}

fn door_core_tensor_kron(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::kron(&a0, &a1)))
}

fn door_core_tensor_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::layers(&a0, a1)))
}

fn door_core_tensor_neighbors(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: mrlyrs::core::Dtype = take!(name, 4, &args[4]);
    match mrlyrs::core::Tensor::neighbors(&a0, &a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Tensor::new(a0)))
}

fn door_core_tensor_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::Tensor::of(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_otsu_threshold(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Tensor::otsu_threshold(&a0)))
}

fn door_core_tensor_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::core::Tensor::pad(&a0, a1, a2)))
}

fn door_core_tensor_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrlyrs::core::Tensor::perforate(&a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_rot90(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrlyrs::core::Tensor::rot90(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Tensor::size(&a0)))
}

fn door_core_tensor_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::core::Tensor::slice(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_sum(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::core::Tensor::sum(&a0)))
}

fn door_core_tensor_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::Tensor::tile(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_transpose(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::core::Tensor::transpose(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_typed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::core::Tensor::typed(a0, a1)))
}

fn door_core_tensor_u16(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u16> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::Tensor::u16(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_u16s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::core::Tensor::u16s(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_u32(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u32> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::Tensor::u32(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_u32s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::core::Tensor::u32s(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_tensor_u8(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::core::Tensor::u8(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_core_unpng(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    match mrlyrs::core::unpng(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_font_glyph_height(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::font::Glyph = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::Glyph::height(&a0)))
}

fn door_font_glyph_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: char = take!(name, 0, &args[0]);
    let a1: Vec<String> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::font::Glyph::new(a0, a1)))
}

fn door_font_glyph_width(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::font::Glyph = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::Glyph::width(&a0)))
}

fn door_font_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::all()))
}

fn door_font_animate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::font::animate(a0.as_str(), a1)))
}

fn door_font_cycle(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::font::Anim = take!(name, 0, &args[0]);
    let a1: Vec<Vec<usize>> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::font::cycle(&a0, &a1, a2)))
}

fn door_font_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::digits()))
}

fn door_font_draft(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::draft(&a0)))
}

fn door_font_extras(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::extras()))
}

fn door_font_floor(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::floor(&a0)))
}

fn door_font_glyph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::glyph(a0)))
}

fn door_font_lower(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]); let b0: Vec<&str> = a0.iter().map(|item| item.as_str()).collect();
    Ok(give!(mrlyrs::font::lower(&b0)))
}

fn door_font_lowers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::lowers()))
}

fn door_font_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::map()))
}

fn door_font_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::font::merge(a0.as_str(), a1)))
}

fn door_font_name_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::name_of(a0)))
}

fn door_font_path(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::path(a0)))
}

fn door_font_paths_penned(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::paths::penned(a0)))
}

fn door_font_pens_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::pens::all()))
}

fn door_font_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::raster(a0.as_str())))
}

fn door_font_specials(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::specials()))
}

fn door_font_strokes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::strokes(a0)))
}

fn door_font_supported(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::supported()))
}

fn door_font_trim(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::font::trim(&a0)))
}

fn door_font_uppers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::font::uppers()))
}

fn door_gen_group_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::gen::Group::all()))
}

fn door_gen_parity_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::gen::Parity::all()))
}

fn door_gen_parity_keep(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::gen::Parity = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::gen::Parity::keep(a0, a1)))
}

fn door_gen_tile_check(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::Tile = take!(name, 0, &args[0]);
    match mrlyrs::gen::Tile::check(&a0) { Ok(()) => Ok(Value::Null), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_tile_degenerate(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::Tile = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::gen::Tile::degenerate(&a0)))
}

fn door_gen_tile_max_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::Tile = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::gen::Tile::max_size(&a0)))
}

fn door_gen_tile_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::Group = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::gen::Tile::new(a0)))
}

fn door_gen_tile_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::gen::Tile = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::gen::Tile::size(a0, a1, a2)))
}

fn door_gen_background(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::gen::background(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_build_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::Tile = take!(name, 0, &args[0]);
    match mrlyrs::gen::build::build_2d(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_build_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::Tile = take!(name, 0, &args[0]);
    match mrlyrs::gen::build::build_3d(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_build_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::build::HexTile = take!(name, 0, &args[0]);
    match mrlyrs::gen::build::build_6d(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_create_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    match mrlyrs::gen::build::create_2d(&a0, &mut a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_create_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    match mrlyrs::gen::build::create_3d(&a0, &mut a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_create_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    match mrlyrs::gen::build::create_6d(&a0, &mut a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_random_tile_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    match mrlyrs::gen::build::random_tile_2d(a0, &mut a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_random_tile_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    match mrlyrs::gen::build::random_tile_3d(a0, &mut a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_build_random_tile_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    match mrlyrs::gen::build::random_tile_6d(a0, &mut a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_classic_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::recipe::Design = take!(name, 0, &args[0]);
    Ok(match mrlyrs::gen::classic_code(a0) { Some(item0) => Value::String(item0.to_string()), None => Value::Null })
}

fn door_gen_classic_code_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::gen::recipe::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(match mrlyrs::gen::classic_code_nd(a0, a1) { Some(item0) => Value::String(item0.to_string()), None => Value::Null })
}

fn door_gen_hex_key(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrlyrs::gen::hex_key(a0, &mut a1)))
}

fn door_gen_name_tile_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::name::Tile = take!(name, 0, &args[0]);
    match <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::checked(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_name_tile_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_file(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_name_tile_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_json(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_name_tile_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::from_url(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_name_tile_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::Tile = take!(name, 0, &args[0]);
    match mrlyrs::gen::name::Tile::of(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_name_tile_recipe(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::name::Tile = take!(name, 0, &args[0]);
    match mrlyrs::gen::name::Tile::recipe(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_name_tile_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::name::Tile = take!(name, 0, &args[0]);
    match <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_file(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_name_tile_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::name::Tile = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_id(&a0)))
}

fn door_gen_name_tile_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::name::Tile = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_json(&a0)))
}

fn door_gen_name_tile_to_mrly(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::name::Tile = take!(name, 0, &args[0]);
    match <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_mrly(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_name_tile_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::name::Tile = take!(name, 0, &args[0]);
    match <mrlyrs::gen::name::Tile as mrlyrs::math::name::Named>::to_url(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_random_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrlyrs::core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrlyrs::gen::random_design(&mut a0)))
}

fn door_gen_random_rotation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::gen::recipe::Design = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrlyrs::gen::random_rotation(a0, &mut a1)))
}

fn door_gen_recipe_design_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::gen::recipe::Design::all()))
}

fn door_gen_recipe_classics(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::gen::recipe::classics(a0)))
}

fn door_gen_recipe_generals(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrlyrs::gen::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::gen::recipe::generals(a0, a1, a2)))
}

fn door_gen_recipe_nestings(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrlyrs::gen::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::gen::recipe::nestings(a0, a1, a2)))
}

fn door_gen_recipe_powers(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrlyrs::gen::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::gen::recipe::powers(a0, a1, a2)))
}

fn door_gen_recipe_products(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrlyrs::gen::Parity = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::gen::recipe::products(a0, a1, a2, a3)))
}

fn door_gen_recipe_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::gen::recipe::size(a0, a1)))
}

fn door_gen_tree_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::gen::tree_mask(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_variation_file_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::gen::variation::File::new(a0, a1)))
}

fn door_gen_variation_variation_is_cover(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::variation::Variation = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::gen::variation::Variation::is_cover(&a0)))
}

fn door_gen_variation_variation_is_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::gen::variation::Variation = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::gen::variation::Variation::is_prime(&a0)))
}

fn door_gen_variation_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::gen::variation::Config = take!(name, 0, &args[0]);
    let mut a1 = mrlyrs::core::Rng::new(seed(name, 1, &args[1])?);
    match mrlyrs::gen::variation::create(&a0, &mut a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_variation_generate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::gen::variation::Variation = take!(name, 0, &args[0]);
    let a1: mrlyrs::gen::variation::Config = take!(name, 1, &args[1]);
    let mut a2 = mrlyrs::core::Rng::new(seed(name, 2, &args[2])?);
    match mrlyrs::gen::variation::generate(a0, &a1, &mut a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_gen_variation_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::gen::variation::Variation = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let mut a2 = mrlyrs::core::Rng::new(seed(name, 2, &args[2])?);
    match mrlyrs::gen::variation::render(a0, a1, &mut a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_boundary_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::life::Boundary::all()))
}

fn door_life_boundary_wrap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Boundary = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Boundary::wrap(a0)))
}

fn door_life_config_budget(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Config = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Config::budget(&a0)))
}

fn door_life_config_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Config = take!(name, 0, &args[0]);
    match mrlyrs::life::Config::counts(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_config_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::life::Counts = take!(name, 1, &args[1]);
    let a2: mrlyrs::life::Counts = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::life::Config::new(a0, a1, a2)))
}

fn door_life_counts_drawn(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::life::Source = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::life::Counts::drawn(a0, a1, a2)))
}

fn door_life_counts_list(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Counts::list(a0)))
}

fn door_life_counts_values(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::life::Counts = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::life::Counts::values(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_fate_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::life::Fate::all()))
}

fn door_life_life_last(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Life = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Life::last(&a0)))
}

fn door_life_rule_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Rule::boundary(&a0)))
}

fn door_life_rule_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Rule = take!(name, 0, &args[0]);
    match <mrlyrs::life::Rule as mrlyrs::math::name::Named>::checked(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_rule_config(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::life::Rule = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::life::Rule::config(&a0, a1)))
}

fn door_life_rule_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_file(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_rule_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_json(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_rule_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::life::Rule as mrlyrs::math::name::Named>::from_url(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_rule_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::life::Counts = take!(name, 0, &args[0]);
    let a1: mrlyrs::life::Counts = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::life::Rule::new(a0, a1, a2)))
}

fn door_life_rule_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Config = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Rule::of(&a0)))
}

fn door_life_rule_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Rule = take!(name, 0, &args[0]);
    match <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_file(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_rule_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Rule = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_id(&a0)))
}

fn door_life_rule_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Rule = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_json(&a0)))
}

fn door_life_rule_to_mrly(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Rule = take!(name, 0, &args[0]);
    match <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_mrly(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_rule_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Rule = take!(name, 0, &args[0]);
    match <mrlyrs::life::Rule as mrlyrs::math::name::Named>::to_url(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_source_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::life::Source::all()))
}

fn door_life_source_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::life::Source::designs()))
}

fn door_life_source_is_random(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Source::is_random(a0)))
}

fn door_life_source_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Source::name(a0)))
}

fn door_life_source_numbers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::life::Source::numbers()))
}

fn door_life_source_oeis(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Source::oeis(a0)))
}

fn door_life_source_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::life::Source::parse(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_source_read(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::Source::read(a0.as_str())))
}

fn door_life_affine(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::affine(a0)))
}

fn door_life_animate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::life::Config = take!(name, 1, &args[1]);
    match mrlyrs::life::animate(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_churn(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::churn(&a0)))
}

fn door_life_corner_bits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::corner_bits(a0)))
}

fn door_life_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::life::Source = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrlyrs::life::counts(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    match mrlyrs::life::crop(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_cube_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::cube_orbit(a0)))
}

fn door_life_design_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 1, &args[1])?);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::life::design_mask(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_elementary_output(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::life::elementary::output(a0, a1, a2, a3)))
}

fn door_life_entropy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::entropy(&a0)))
}

fn door_life_frames(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::life::frames(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::gasket(a0)))
}

fn door_life_genus(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::genus(a0)))
}

fn door_life_heatmap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::life::heatmap(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_history(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrlyrs::life::history(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_lambda(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::lambda(a0)))
}

fn door_life_lattice_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::lattice_index(&a0)))
}

fn door_life_mask_offsets(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::mask_offsets(&a0)))
}

fn door_life_moore(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrlyrs::life::moore() { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_movie(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::life::movie(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_next_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    let a3: mrlyrs::core::Tensor = take!(name, 3, &args[3]);
    let a4: mrlyrs::life::Boundary = take!(name, 4, &args[4]);
    match mrlyrs::life::next_grid(&a0, &a1, &a2, &a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_npn_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::npn_class(a0)))
}

fn door_life_outer_totalistic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::outer_totalistic(a0)))
}

fn door_life_popcount(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::popcount(a0)))
}

fn door_life_render_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::life::render::frame(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_reversible(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::reversible(a0)))
}

fn door_life_rule_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::rule_degree(a0)))
}

fn door_life_rule_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    match mrlyrs::life::rule_name(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_single_seed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::life::single_seed(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_source_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::life::Source = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::life::source::sequence(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_step(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrlyrs::life::step(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_surjective(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::surjective(a0)))
}

fn door_life_tessellate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::life::tessellate(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_life_wolfram_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::life::wolfram_class(a0)))
}

fn door_math_atoms_carpet_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::carpet_2d(a0)))
}

fn door_math_atoms_carpet_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::carpet_3d(a0)))
}

fn door_math_atoms_carpet_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::atoms::carpet_nd(a0, a1)))
}

fn door_math_atoms_dust_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::dust_2d(a0)))
}

fn door_math_atoms_dust_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::dust_3d(a0)))
}

fn door_math_atoms_dust_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::atoms::dust_nd(a0, a1)))
}

fn door_math_atoms_hline_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::hline_2d(a0)))
}

fn door_math_atoms_htree_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::htree_2d(a0)))
}

fn door_math_atoms_line_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::atoms::line_nd(a0, a1, a2)))
}

fn door_math_atoms_net_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::net_2d(a0)))
}

fn door_math_atoms_net_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::net_3d(a0)))
}

fn door_math_atoms_net_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::atoms::net_nd(a0, a1)))
}

fn door_math_atoms_noise_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let mut a2 = mrlyrs::core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrlyrs::math::atoms::noise_2d(a0, a1, &mut a2)))
}

fn door_math_atoms_noise_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let mut a2 = mrlyrs::core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrlyrs::math::atoms::noise_3d(a0, a1, &mut a2)))
}

fn door_math_atoms_ones_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::ones_2d(a0)))
}

fn door_math_atoms_ones_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::ones_3d(a0)))
}

fn door_math_atoms_point_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::point_2d(a0)))
}

fn door_math_atoms_point_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::point_3d(a0)))
}

fn door_math_atoms_point_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::atoms::point_nd(a0, a1)))
}

fn door_math_atoms_star_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::star_2d(a0)))
}

fn door_math_atoms_star_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::star_3d(a0)))
}

fn door_math_atoms_star_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::atoms::star_nd(a0, a1)))
}

fn door_math_atoms_tree_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::atoms::tree_nd(a0, a1, a2)))
}

fn door_math_atoms_vline_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::vline_2d(a0)))
}

fn door_math_atoms_void_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::void_2d(a0)))
}

fn door_math_atoms_void_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::void_3d(a0)))
}

fn door_math_atoms_void_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::atoms::void_nd(a0, a1)))
}

fn door_math_atoms_vtree_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::vtree_2d(a0)))
}

fn door_math_atoms_xline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::xline_3d(a0)))
}

fn door_math_atoms_xtree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::xtree_3d(a0)))
}

fn door_math_atoms_yline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::yline_3d(a0)))
}

fn door_math_atoms_ytree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::ytree_3d(a0)))
}

fn door_math_atoms_zeros_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::zeros_2d(a0)))
}

fn door_math_atoms_zeros_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::zeros_3d(a0)))
}

fn door_math_atoms_zline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::zline_3d(a0)))
}

fn door_math_atoms_ztree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::atoms::ztree_3d(a0)))
}

fn door_math_bang_design_anf(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::Design::anf(&a0)))
}

fn door_math_bang_design_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::Design::degree(&a0)))
}

fn door_math_bang_design_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::Design = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::Design::name(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_design_rule(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::Design::rule(&a0)))
}

fn door_math_bang_magiclayer_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::name::Bang = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::MagicLayer::new(a0, a1)))
}

fn door_math_bang_universe_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::Universe::all(&a0)))
}

fn door_math_bang_universe_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::Universe::canonical(&a0)))
}

fn door_math_bang_universe_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::bang::Universe = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(give!(mrlyrs::math::bang::Universe::design(&a0, a1)))
}

fn door_math_bang_universe_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::Universe::distinct(&a0)))
}

fn door_math_bang_universe_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::Universe::new(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_bang(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::bang(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_axis_maps(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::baseq::axis_maps(a0)))
}

fn door_math_bang_baseq_bracelets(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::baseq::bracelets(a0) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<usize>> = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 1, &args[1])?);
    match mrlyrs::math::bang::baseq::canonical(&a0, a1) { Ok(value) => Ok(Value::String(value.get().to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_carry(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(Value::String(mrlyrs::math::bang::baseq::carry(&a0, a1).get().to_string()))
}

fn door_math_bang_baseq_class_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::math::bang::baseq::class_sequence(a0) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_math_bang_baseq_classes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::math::bang::baseq::classes(a0).to_string()))
}

fn door_math_bang_baseq_distinct_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::bang::baseq::distinct_designs(a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_even_fill_is_balanced(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u128 = big(name, 2, &args[2])?;
    match mrlyrs::math::bang::baseq::even_fill_is_balanced(a0, a1, a2) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_fill_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(mrlyrs::math::bang::baseq::fill_from_corners(&a0, a1, a2).to_string()))
}

fn door_math_bang_baseq_group(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::baseq::group(a0, a1)))
}

fn door_math_bang_baseq_group_order(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::bang::baseq::group_order(a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<usize>> = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 1, &args[1])?);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::math::bang::baseq::orbit(&a0, a1) { list0.push(Value::String(item0.get().to_string())); } Value::Array(list0) })
}

fn door_math_bang_baseq_predicted_group_order(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::bang::baseq::predicted_group_order(a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_representatives(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::bang::baseq::representatives(a0, a1) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push({ let parts1 = item0; Value::Array(vec![Value::String(parts1.0.get().to_string()), give!(parts1.1)]) }); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::bang::baseq::sequence(a0, a1) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_baseq_total_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::bang::baseq::total_designs(a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_catalog_antis(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::catalog::antis(a0)))
}

fn door_math_bang_code_get(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::Code = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::math::bang::Code::get(a0).to_string()))
}

fn door_math_bang_code_to_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::bang::code_to_corners(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::corners(a0)))
}

fn door_math_bang_corners_to_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(mrlyrs::math::bang::corners_to_code(&a0, a1, a2).get().to_string()))
}

fn door_math_bang_factory_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::bang::factory::create(a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_factory_create_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::bang::factory::create_from_corners(&a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_factory_create_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::bang::factory::create_named(a0.as_str(), a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_factory_residue_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::factory::residue_corners(a0, a1)))
}

fn door_math_bang_factory_total_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::bang::factory::total_codes(a0, a1) { Ok(value) => Ok(Value::String(value.get().to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_levels_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    Ok(Value::String(mrlyrs::math::bang::levels_code(a0, a1, &a2).get().to_string()))
}

fn door_math_bang_magic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::magic(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_magic_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(String, usize)> = take!(name, 0, &args[0]); let b0: Vec<(&str, usize)> = a0.iter().map(|item| (item.0.as_str(), item.1)).collect();
    match mrlyrs::math::bang::magic_named(&b0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_sources(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::gen::recipe::Catalog = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::bang::sources(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_symmetries(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::symmetries(a0)))
}

fn door_math_bang_total_exposure(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::total_exposure(a0, a1)))
}

fn door_math_bang_touches_every_corner(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::touches_every_corner(a0, a1)))
}

fn door_math_bang_universe_anf(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::universe::anf(a0, a1)))
}

fn door_math_bang_universe_anf_string(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::universe::anf_string(a0, a1)))
}

fn door_math_bang_universe_apply(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: (Vec<usize>, Vec<u8>) = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::universe::apply(&a0, &a1)))
}

fn door_math_bang_universe_corner_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::universe::corner_index(&a0)))
}

fn door_math_bang_universe_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::universe::degree(a0, a1)))
}

fn door_math_bang_universe_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::math::bang::universe::orbit(a0, a1) { list0.push(Value::String(item0.get().to_string())); } Value::Array(list0) })
}

fn door_math_bang_universe_permutations(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::universe::permutations(a0)))
}

fn door_math_bang_universe_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::universe_codes(a0) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_schedule_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::math::bang::word::Schedule::all()))
}

fn door_math_bang_word_schedule_frequencies(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::word::Schedule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::word::Schedule::frequencies(a0)))
}

fn door_math_bang_word_schedule_place(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::bang::word::Schedule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::bang::word::Schedule::place(a0, a1)))
}

fn door_math_bang_word_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::components(&a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_constant_functional(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::constant_functional(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::dimension(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::fill(&a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::fills(&a0) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_letter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::MagicLayer = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::letter(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_native(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::word::native(&a0)))
}

fn door_math_bang_word_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::word::period(&a0)))
}

fn door_math_bang_word_prefixes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::prefixes(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_rates(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::rates(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::side(&a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_spell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::bang::word::Schedule = take!(name, 0, &args[0]);
    let a1: (mrlyrs::math::bang::MagicLayer, mrlyrs::math::bang::MagicLayer) = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::bang::word::spell(a0, a1, a2)))
}

fn door_math_bang_word_staircase(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::bang::word::staircase(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_bang_word_thue_morse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::bang::word::thue_morse(a0)))
}

fn door_math_cell_census_edges(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::census::edges::<2>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::census::edges::<3>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_census_exposure(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::math::cell::census::exposure::<2>(&a0).to_string()))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::math::cell::census::exposure::<3>(&a0).to_string()))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_census_fills(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::census::fills::<2>(&a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::census::fills::<3>(&a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_census_vertices(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::census::vertices::<2>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::census::vertices::<3>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_census_voids(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::census::voids::<2>(&a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::census::voids::<3>(&a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_geometry_merge_reps(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 0)? {
    2 => {
    count(name, args, 2)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::geometry::merge_reps::<2>(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 2)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::geometry::merge_reps::<3>(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_geometry_perforate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 1)? {
    2 => {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrlyrs::math::cell::geometry::perforate::<2>(&a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrlyrs::math::cell::geometry::perforate::<3>(&a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_grow(name: &str, args: &[Value]) -> Done {
    match tensor_rank(name, args, 0)? {
    2 => {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::grow::<2>(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::grow::<3>(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_anti(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::anti(a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::anti(a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_binarize(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::binarize(a0, a1)))
    }
    3 => {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::binarize(a0, a1)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_binarize_otsu(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::binarize_otsu(a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::binarize_otsu(a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_blur(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrlyrs::math::cell::models::CellNd::<2>::blur(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrlyrs::math::cell::models::CellNd::<3>::blur(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_combine(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::combine(&a0, &a1)))
    }
    3 => {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::combine(&a0, &a1)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_counting_dtype(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::counting_dtype(&a0)))
}

fn door_math_cell_models_depth(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::depth(&a0)))
}

fn door_math_cell_models_dtype_for(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: i64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::dtype_for(a0)))
}

fn door_math_cell_models_fractal(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::models::CellNd::<2>::fractal(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::models::CellNd::<3>::fractal(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_height(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::height(&a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::height(&a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_invert(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::invert(a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::invert(a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_layers(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::layers(a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::layers(a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_neighbors(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 4)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrlyrs::math::cell::models::CellNd::<2>::neighbors(a0, &a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 4)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrlyrs::math::cell::models::CellNd::<3>::neighbors(a0, &a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_new(name: &str, args: &[Value]) -> Done {
    match tensor_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::models::CellNd::<2>::new(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::models::CellNd::<3>::new(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_orient(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::models::CellNd::<3>::orient(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_cell_models_pad(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::pad(a0, a1, a2)))
    }
    3 => {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::pad(a0, a1, a2)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_paint(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 4)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1 = { let mut pairs0: Vec<(u8, Vec<mrlyrs::core::Color>)> = Vec::new(); for (text0, item0) in fields(name, 1, &args[1])? { let key0 = match text0.parse() { Ok(key) => key, Err(_) => return Err(bad(name, 1, "an object key")) }; pairs0.push((key0, { let mut list1 = Vec::new(); for item1 in items(name, 1, item0)? { list1.push(color(name, 1, item1)?); } list1 })); } pairs0 }.into_iter().collect();
    let a2: mrlyrs::core::Mode = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() { None } else { Some(mrlyrs::core::Rng::new(seed(name, 3, &args[3])?)) };
    match mrlyrs::math::cell::models::CellNd::<2>::paint(a0, &a1, a2, a3.as_mut()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 4)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1 = { let mut pairs0: Vec<(u8, Vec<mrlyrs::core::Color>)> = Vec::new(); for (text0, item0) in fields(name, 1, &args[1])? { let key0 = match text0.parse() { Ok(key) => key, Err(_) => return Err(bad(name, 1, "an object key")) }; pairs0.push((key0, { let mut list1 = Vec::new(); for item1 in items(name, 1, item0)? { list1.push(color(name, 1, item1)?); } list1 })); } pairs0 }.into_iter().collect();
    let a2: mrlyrs::core::Mode = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() { None } else { Some(mrlyrs::core::Rng::new(seed(name, 3, &args[3])?)) };
    match mrlyrs::math::cell::models::CellNd::<3>::paint(a0, &a1, a2, a3.as_mut()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_perforate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrlyrs::math::cell::models::CellNd::<2>::perforate(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrlyrs::math::cell::models::CellNd::<3>::perforate(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_rotate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::models::CellNd::<2>::rotate(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrlyrs::math::cell::models::CellNd::<3>::rotate(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_tile(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::cell::models::CellNd::<2>::tile(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 4)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::cell::models::CellNd::<3>::tile(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_types(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::types(&a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::types(&a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_models_width(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<2>::width(&a0)))
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::cell::models::CellNd::<3>::width(&a0)))
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_paint(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 4)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Option<_> = if Value::is_null(&args[1]) { None } else { Some({ let mut pairs0: Vec<(u8, Vec<mrlyrs::core::Color>)> = Vec::new(); for (text0, item0) in fields(name, 1, &args[1])? { let key0 = match text0.parse() { Ok(key) => key, Err(_) => return Err(bad(name, 1, "an object key")) }; pairs0.push((key0, { let mut list1 = Vec::new(); for item1 in items(name, 1, item0)? { list1.push(color(name, 1, item1)?); } list1 })); } pairs0 }.into_iter().collect()) };
    let a2: Option<mrlyrs::core::Mode> = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() { None } else { Some(mrlyrs::core::Rng::new(seed(name, 3, &args[3])?)) };
    match mrlyrs::math::cell::paint::<2>(a0, a1.as_ref(), a2, a3.as_mut()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 4)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: Option<_> = if Value::is_null(&args[1]) { None } else { Some({ let mut pairs0: Vec<(u8, Vec<mrlyrs::core::Color>)> = Vec::new(); for (text0, item0) in fields(name, 1, &args[1])? { let key0 = match text0.parse() { Ok(key) => key, Err(_) => return Err(bad(name, 1, "an object key")) }; pairs0.push((key0, { let mut list1 = Vec::new(); for item1 in items(name, 1, item0)? { list1.push(color(name, 1, item1)?); } list1 })); } pairs0 }.into_iter().collect()) };
    let a2: Option<mrlyrs::core::Mode> = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() { None } else { Some(mrlyrs::core::Rng::new(seed(name, 3, &args[3])?)) };
    match mrlyrs::math::cell::paint::<3>(a0, a1.as_ref(), a2, a3.as_mut()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_cell_serializer_byte_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::serializer::byte_cube(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_cell_serializer_byte_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::serializer::byte_grid(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_cell_serializer_color_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::serializer::color_grid(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_cell_serializer_count_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::serializer::count_cube(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_cell_serializer_count_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::serializer::count_grid(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_cell_serializer_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::serializer::parse(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_cell_serializer_tag_layer(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i64> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::math::cell::serializer::tag_layer(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_cell_serializer_types_field(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrlyrs::math::cell::serializer::types_field(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_exposure_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::counts::Exposure = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrlyrs::math::counts::Exposure::at(&a0, a1) { Some(item0) => Value::String(item0.to_string()), None => Value::Null })
}

fn door_math_counts_exposure_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::math::counts::Exposure::from_corners(&a0, a1, a2, a3)))
}

fn door_math_counts_exposure_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::counts::Exposure::of_tile(&a0)))
}

fn door_math_counts_exposure_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::counts::Exposure = take!(name, 0, &args[0]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::math::counts::Exposure::recurrence(&a0) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_math_counts_centered_hexagonal(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::math::counts::centered_hexagonal(a0).to_string()))
}

fn door_math_counts_cut_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrlyrs::math::counts::cut_fills(a0, a1, a2) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_cut_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrlyrs::math::counts::cut_voids(a0, a1, a2) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::counts::dimension(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_edges_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrlyrs::math::counts::edges_of_tile(&a0, a1) { Some(item0) => Value::String(item0.to_string()), None => Value::Null })
}

fn door_math_counts_exposure(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::counts::exposure(a0, a1, a2, a3, a4) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_exposure_of_tile_2(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrlyrs::math::counts::exposure_of_tile(&a0, a1) { Some(item0) => Value::String(item0.to_string()), None => Value::Null })
}

fn door_math_counts_exposure_recurrence_2(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::math::counts::exposure_recurrence(&a0) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_math_counts_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::counts::fill(a0, a1, a2, a3, a4) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_fill_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    Ok(Value::String(mrlyrs::math::counts::fill_from_corners(&a0, a1, a2, a3, a4).to_string()))
}

fn door_math_counts_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(Value::String(mrlyrs::math::counts::grid(a0, a1, a2).to_string()))
}

fn door_math_counts_ladder_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::counts::ladder::cap(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_carry_matrix(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::counts::ladder::carry_matrix(a0, a1) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push({ let mut list1 = Vec::new(); for item1 in item0 { list1.push(Value::String(item1.to_string())); } Value::Array(list1) }); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_characteristic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push({ let mut list1 = Vec::new(); for item1 in items(name, 0, item0)? { list1.push(small(name, 0, item1)?); } list1 }); } list0 };
    match mrlyrs::math::counts::ladder::characteristic(&a0) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_determinant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push({ let mut list1 = Vec::new(); for item1 in items(name, 0, item0)? { list1.push(small(name, 0, item1)?); } list1 }); } list0 };
    match mrlyrs::math::counts::ladder::determinant(&a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_digit_polynomial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::counts::ladder::digit_polynomial(a0, a1) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_even_block(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::counts::ladder::even_block(a0, a1) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push({ let mut list1 = Vec::new(); for item1 in item0 { list1.push(Value::String(item1.to_string())); } Value::Array(list1) }); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::counts::ladder::fill(a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_ladder(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::counts::ladder::ladder(a0, a1, a2) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push({ let mut list1 = Vec::new(); for item1 in items(name, 0, item0)? { list1.push(small(name, 0, item1)?); } list1 }); } list0 };
    match mrlyrs::math::counts::ladder::perron(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_sign(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::counts::ladder::sign(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_spectral_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::counts::ladder::spectral_ratio(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ladder_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push({ let mut list1 = Vec::new(); for item1 in items(name, 0, item0)? { list1.push(small(name, 0, item1)?); } list1 }); } list0 };
    Ok(Value::String(mrlyrs::math::counts::ladder::trace(&a0).to_string()))
}

fn door_math_counts_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::counts::limit(a0, a1, a2, a3) { Ok(value) => Ok({ let parts0 = value; Value::Array(vec![Value::String(parts0.0.to_string()), Value::String(parts0.1.to_string())]) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::math::counts::pairs(&a0) { list0.push({ let parts1 = item0; Value::Array(vec![Value::String(parts1.0.to_string()), Value::String(parts1.1.to_string())]) }); } Value::Array(list0) })
}

fn door_math_counts_positions(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(mrlyrs::math::counts::positions(a0, a1, a2).to_string()))
}

fn door_math_counts_pro_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrlyrs::math::counts::pro_fills(a0, a1, a2) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_pro_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrlyrs::math::counts::pro_voids(a0, a1, a2) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_profile_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::math::counts::profile_of_tile(&a0, a1) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::counts::ratio(a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_rational(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::counts::rational(a0, a1, a2, a3, a4) { Ok(value) => Ok({ let parts0 = value; Value::Array(vec![Value::String(parts0.0.to_string()), Value::String(parts0.1.to_string())]) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_six_grid_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(Value::String(mrlyrs::math::counts::six::grid_triangles(a0, a1).to_string()))
}

fn door_math_counts_six_solid_slice_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::counts::six::solid_slice_boundary(a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_six_solid_slice_core_edges(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::counts::six::solid_slice_core_edges(a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_six_solid_slice_core_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::counts::six::solid_slice_core_nodes(a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_six_solid_slice_edges(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::counts::six::solid_slice_edges(a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_six_solid_slice_interior(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::counts::six::solid_slice_interior(a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_six_solid_slice_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::counts::six::solid_slice_triangles(a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_six_solid_slice_vertices(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::counts::six::solid_slice_vertices(a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_surface(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::counts::surface(a0, a1, a2, a3) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_counts_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::counts::void(a0, a1, a2, a3, a4) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_network_adjacency(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::Network::adjacency(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_network_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::Network::degree(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_network_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::graph::Network::new(a0)))
}

fn door_math_graph_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::census(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::components(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_core_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::core_graph(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_edge_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::edge_graph(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_fractal_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::graph::fractal_dimension(&a0, a1)))
}

fn door_math_graph_junctions(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::junctions(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_largest_component(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::largest_component(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_roles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::roles(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_tips(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::tips(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_graph_total_length(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::graph::total_length(&a0)))
}

fn door_math_graph_tunnel_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    match mrlyrs::math::graph::tunnel_graph(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_field_as_f64(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Field::as_f64(&a0)))
}

fn door_math_moire_field_from_data(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::moire::Field::from_data(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_field_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Field::max(&a0)))
}

fn door_math_moire_field_mean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Field::mean(&a0)))
}

fn door_math_moire_field_min(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Field::min(&a0)))
}

fn door_math_moire_field_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Field::new(a0)))
}

fn door_math_moire_field_normalized(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::moire::Field = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::moire::Field::normalized(&a0, a1)))
}

fn door_math_moire_layer_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::moire::Spec = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::moire::Layer::new(a0, a1)))
}

fn door_math_moire_preset_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Preset::carpet(a0)))
}

fn door_math_moire_preset_heatmap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Preset::heatmap(a0)))
}

fn door_math_moire_preset_hive(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Preset::hive(a0)))
}

fn door_math_moire_preset_weave(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Preset::weave(a0)))
}

fn door_math_moire_spec_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::moire::Spec::new(a0, a1, a2)))
}

fn door_math_moire_volume_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::math::moire::Volume::at(&a0, a1, a2, a3)))
}

fn door_math_moire_volume_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::moire::Volume::count(&a0, a1)))
}

fn door_math_moire_volume_from_data(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::moire::Volume::from_data(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_volume_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::moire::Volume = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Volume::max(&a0)))
}

fn door_math_moire_volume_min(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::moire::Volume = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Volume::min(&a0)))
}

fn door_math_moire_volume_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::Volume::new(a0)))
}

fn door_math_moire_volume_plane(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::moire::Frame = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::moire::Volume::plane(&a0, &a1, a2)))
}

fn door_math_moire_volume_sample(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: [f64; 3] = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::moire::Volume::sample(&a0, a1)))
}

fn door_math_moire_volume_solid(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::moire::Volume::solid(&a0, a1)))
}

fn door_math_moire_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::moire::all(a0)))
}

fn door_math_moire_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [f64; 3] = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrlyrs::math::moire::frame(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_layer(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::moire::Layer = take!(name, 0, &args[0]);
    match mrlyrs::math::moire::layer(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::moire::named(a0.as_str(), a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_pairs_correlation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::moire::pairs::correlation(a0, a1)))
}

fn door_math_moire_pairs_sampled(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::moire::pairs::sampled(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_pairs_witness(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::moire::pairs::witness(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: mrlyrs::math::moire::Field = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Colorizer = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: bool = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrlyrs::math::moire::render(&a0, &a1, a2, a3, a4, a5) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_sample_axes(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::moire::Lattice = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::moire::sample::axes(a0, a1, a2)))
}

fn door_math_moire_sample_membership(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::moire::sample::membership(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_sample_pack(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::moire::sample::pack(&a0, a1)))
}

fn door_math_moire_stack(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: mrlyrs::math::moire::Spec = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: mrlyrs::math::moire::Combine = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrlyrs::math::moire::Lattice = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    let a6: Vec<f64> = take!(name, 6, &args[6]);
    match mrlyrs::math::moire::stack(a0, &a1, a2, a3, a4, a5, &a6) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_stack_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<mrlyrs::math::moire::Spec> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrlyrs::math::moire::Lattice = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: Vec<f64> = take!(name, 5, &args[5]);
    match mrlyrs::math::moire::stack_codes(&a0, a1, a2, a3, a4, &a5) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_moire_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::moire::Spec = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: mrlyrs::math::moire::Combine = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::moire::volume(a0, &a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_bang_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Bang = take!(name, 0, &args[0]);
    match mrlyrs::math::name::Bang::cells(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_bang_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Bang = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::checked(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_bang_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_file(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_bang_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_json(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_bang_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::from_url(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_bang_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::name::Bang::new(a0, a1, a2)))
}

fn door_math_name_bang_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Bang = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_file(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_bang_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Bang = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_id(&a0)))
}

fn door_math_name_bang_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Bang = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_json(&a0)))
}

fn door_math_name_bang_to_mrly(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Bang = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_mrly(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_bang_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Bang = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Bang as mrlyrs::math::name::Named>::to_url(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_lattice_is_square(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Lattice = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::name::Lattice::is_square(&a0)))
}

fn door_math_name_lattice_units(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Lattice = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::name::Lattice::units(a0)))
}

fn door_math_name_sequence_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Sequence = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::checked(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_sequence_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::name::Sequence::design(&a0)))
}

fn door_math_name_sequence_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_file(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_sequence_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_json(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_sequence_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::from_url(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_sequence_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    Ok(give!(mrlyrs::math::name::Sequence::new(a0, a1, a2, a3.as_str(), a4.as_str())))
}

fn door_math_name_sequence_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Sequence = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_file(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_sequence_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_id(&a0)))
}

fn door_math_name_sequence_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_json(&a0)))
}

fn door_math_name_sequence_to_mrly(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Sequence = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_mrly(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_sequence_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Sequence = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Sequence as mrlyrs::math::name::Named>::to_url(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_word_bases(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::name::Word::bases(&a0)))
}

fn door_math_name_word_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Word = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::checked(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_word_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_file(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_word_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_json(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_word_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::from_url(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_word_letters(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::name::Word::letters(&a0)))
}

fn door_math_name_word_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<(u128, usize)> = { let mut list0 = Vec::new(); for item0 in items(name, 1, &args[1])? { list0.push({ let parts1 = parts(name, 1, item0, 2)?; (big(name, 1, &parts1[0])?, take!(name, 1, &parts1[1])) }); } list0 };
    match mrlyrs::math::name::Word::new(a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_word_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Word = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_file(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_word_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_id(&a0)))
}

fn door_math_name_word_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(<mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_json(&a0)))
}

fn door_math_name_word_to_mrly(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Word = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_mrly(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_name_word_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::name::Word = take!(name, 0, &args[0]);
    match <mrlyrs::math::name::Word as mrlyrs::math::name::Named>::to_url(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_press_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::press::Press::new(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_press_total(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::press::Press = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(Value::String(mrlyrs::math::press::Press::total(&a0, a1).to_string()))
}

fn door_math_press_press_totals(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::press::Press = take!(name, 0, &args[0]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::math::press::Press::totals(&a0) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_math_press_containing(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::press::containing(a0, a1, a2) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_coordinates(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::press::coordinates(a0, a1, a2) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_count_below(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u128 = big(name, 3, &args[3])?;
    match mrlyrs::math::press::count_below(a0, a1, a2, a3) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::press::distinct(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_interleave(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(big(name, 0, item0)?); } list0 };
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::press::interleave(&a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_layer_table(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::bang::MagicLayer = take!(name, 0, &args[0]);
    match mrlyrs::math::press::layer_table(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: u128 = big(name, 1, &args[1])?;
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::press::member(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_members(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::press::members(a0, a1, a2, a3) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::press::profile(a0, a1, a2, a3) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_usage(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::press::usage(a0, a1, a2) { Ok(value) => Ok(Value::String(value.get().to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_word_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::press::word_count(&a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_word_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrlyrs::math::press::word_member(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_word_members(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::press::word_members(&a0) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_press_word_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrlyrs::math::press::word_profile(&a0) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_roulette_nodes_pair(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::roulette::Nodes = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::roulette::Nodes::pair(&a0, a1, a2)))
}

fn door_math_roulette_nodes_paired(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::roulette::Nodes::paired(&a0)))
}

fn door_math_roulette_nodes_selved(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::roulette::Nodes::selved(&a0)))
}

fn door_math_roulette_nodes_total(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::roulette::Nodes::total(&a0)))
}

fn door_math_roulette_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrlyrs::math::roulette::nodes(&a0, &a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_roulette_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: [f64; 2] = take!(name, 0, &args[0]);
    let a1: [f64; 2] = take!(name, 1, &args[1]);
    let a2: [f64; 2] = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::roulette::side(a0, a1, a2)))
}

fn door_math_roulette_spread(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::roulette::spread(&a0, &a1, a2)))
}

fn door_math_rules_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::rules::render(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_rules_tree_axes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::rules::tree_axes(a0, a1)))
}

fn door_math_shape_frac_minus(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::shape::Frac = take!(name, 1, &args[1]);
    match mrlyrs::math::shape::Frac::minus(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_frac_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrlyrs::math::shape::Frac::new(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_frac_plus(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::shape::Frac = take!(name, 1, &args[1]);
    match mrlyrs::math::shape::Frac::plus(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_frac_times(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::shape::Frac = take!(name, 1, &args[1]);
    match mrlyrs::math::shape::Frac::times(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_frac_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: i64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::shape::Frac::whole(a0)))
}

fn door_math_shape_region_flip(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::shape::Region = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::shape::Region::flip(a0)))
}

fn door_math_shape_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::shape::Shape = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    match mrlyrs::math::shape::census(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_classify(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::shape::Shape = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    match mrlyrs::math::shape::classify(&a0, a1, &a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::shape::Shape = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrlyrs::math::shape::crop(&a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_crossing_shell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::shape::crossing_shell(a0, a1, a2)))
}

fn door_math_shape_crossing_tree(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: Vec<bool> = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::shape::crossing_tree(a0, a1, &a2)))
}

fn door_math_shape_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrlyrs::math::shape::Frac = take!(name, 2, &args[2]);
    match mrlyrs::math::shape::named(a0.as_str(), a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_radial_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<i64> = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::shape::radial_census(&a0, &a1, a2)))
}

fn door_math_shape_refine(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::shape::Shape = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: bool = take!(name, 4, &args[4]);
    match mrlyrs::math::shape::refine(&a0, &a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_regions(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::shape::Shape = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::math::shape::regions(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_shape_shapes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::shape::shapes(a0)))
}

fn door_math_six_anti(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::Cell6d::anti(a0)))
}

fn door_math_six_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::six::Cell6d::binarize(a0, a1)))
}

fn door_math_six_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::Cell6d::binarize_otsu(a0)))
}

fn door_math_six_blank(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::six::Orientation = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    match mrlyrs::math::six::blank(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrlyrs::math::six::Cell6d::blur(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::six::census(&a0, a1)))
}

fn door_math_six_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::components(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_cut(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::six::cut(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_cut_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::six::cut_design(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_east(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::six::east(a0, a1)))
}

fn door_math_six_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::six::euler(&a0, a1)))
}

fn door_math_six_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::fills(&a0)))
}

fn door_math_six_fills_only(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::fills_only(&a0)))
}

fn door_math_six_framed(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::framed(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::math::six::from_json(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_giant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::giant(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_giant_network(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::giant_network(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_height(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::Cell6d::height(&a0)))
}

fn door_math_six_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::holes(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_is_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::is_cube(&a0)))
}

fn door_math_six_is_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::is_hex(&a0)))
}

fn door_math_six_iso(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::six::iso(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_iso_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::six::iso_design(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::six::Projection = take!(name, 1, &args[1]);
    let a2: mrlyrs::math::six::Orientation = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::math::six::Cell6d::new(a0, a1, a2, a3)))
}

fn door_math_six_north(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::six::north(a0, a1)))
}

fn door_math_six_orientation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::six::orientation(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrlyrs::math::six::pad(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_paint(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<_> = if Value::is_null(&args[1]) { None } else { Some({ let mut pairs0: Vec<(u8, Vec<mrlyrs::core::Color>)> = Vec::new(); for (text0, item0) in fields(name, 1, &args[1])? { let key0 = match text0.parse() { Ok(key) => key, Err(_) => return Err(bad(name, 1, "an object key")) }; pairs0.push((key0, { let mut list1 = Vec::new(); for item1 in items(name, 1, item0)? { list1.push(color(name, 1, item1)?); } list1 })); } pairs0 }.into_iter().collect()) };
    let a2: Option<mrlyrs::core::Mode> = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() { None } else { Some(mrlyrs::core::Rng::new(seed(name, 3, &args[3])?)) };
    match mrlyrs::math::six::paint(a0, a1.as_ref(), a2, a3.as_mut()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrlyrs::math::six::Cell6d::perforate(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_png(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrlyrs::core::Color> = if Value::is_null(&args[2]) { None } else { Some(color(name, 2, &args[2])?) };
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::six::png(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_pro(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::six::pro(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_pro_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::six::pro_design(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_radial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::six::radial(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_radial_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrlyrs::math::six::radial_crop(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_radial_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::six::Orientation = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::six::radial_mask(a0, a1)))
}

fn door_math_six_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::six::raster(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_rect_png(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<usize> = take!(name, 2, &args[2]);
    match mrlyrs::math::six::rect_png(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_rect_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<usize> = take!(name, 2, &args[2]);
    match mrlyrs::math::six::rect_svg(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_rim_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::rim_holes(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_skin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::skin(&a0)))
}

fn door_math_six_slice_core_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::slice_core_graph(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_slice_dual_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::slice_dual_graph(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_slice_edge_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<u8> = take!(name, 1, &args[1]);
    match mrlyrs::math::six::slice_edge_graph(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_slice_tunnel_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrlyrs::math::six::slice_tunnel_graph(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_south(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::six::south(a0, a1)))
}

fn door_math_six_spectral_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrlyrs::math::six::spectral_exponent(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_star_branch_constant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::star::Branch::constant(a0)))
}

fn door_math_six_star_branch_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::star::Branch::name(a0)))
}

fn door_math_six_star_branch_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::star::Branch::of(a0)))
}

fn door_math_six_star_branch_residual(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::star::Branch::residual(a0)))
}

fn door_math_six_star_share_reduced(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::star::Share = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::star::Share::reduced(a0)))
}

fn door_math_six_star_share_value(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::star::Share = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::star::Share::value(a0)))
}

fn door_math_six_star_star_arm(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::six::star::Star::arm(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_star_star_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::math::six::star::Star::cell(&a0, a1, a2, a3)))
}

fn door_math_six_star_star_excesses(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::six::star::Star::excesses(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_star_star_hexagon(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::six::star::Star::hexagon(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_star_star_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u128 = big(name, 0, &args[0])?;
    match mrlyrs::math::six::star::Star::new(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_star_arm_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::math::six::star::arm_law(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_star_chi8(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::star::chi8(a0)))
}

fn door_math_six_star_constant(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::math::six::star::constant()))
}

fn door_math_six_star_decay(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::six::star::decay(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_star_width_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::star::width_law(a0)))
}

fn door_math_six_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrlyrs::core::Color> = if Value::is_null(&args[2]) { None } else { Some(color(name, 2, &args[2])?) };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Option<usize> = take!(name, 4, &args[4]);
    match mrlyrs::math::six::svg(&a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_tessellate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrlyrs::core::Tensor = take!(name, 1, &args[1]);
    match mrlyrs::math::six::tessellate(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::six::tile(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_tile_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrlyrs::math::six::tile_cell(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_tile_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: (usize, usize) = take!(name, 1, &args[1]);
    match mrlyrs::math::six::tile_crop(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_tile_step(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: (usize, usize) = take!(name, 0, &args[0]);
    match mrlyrs::math::six::tile_step(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::to_json(&a0)))
}

fn door_math_six_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<usize> = take!(name, 1, &args[1]);
    match mrlyrs::math::six::triangles(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_six_west(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::six::west(a0, a1)))
}

fn door_math_six_width(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::six::Cell6d::width(&a0)))
}

fn door_math_spectrum_clusters(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrlyrs::math::spectrum::clusters(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spectrum_laplacian(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrlyrs::math::spectrum::laplacian(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spectrum_laplacian_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::graph::Network = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrlyrs::math::spectrum::laplacian_spectrum(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spectrum_multiplicity(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::spectrum::multiplicity(&a0, a1, a2)))
}

fn door_math_spectrum_spectral_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spectrum::spectral_exponent(&a0, a1)))
}

fn door_math_spectrum_spectral_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spectrum::spectral_fit(&a0, a1)))
}

fn door_math_spectrum_spectral_points(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::spectrum::spectral_points(&a0)))
}

fn door_math_spectrum_symmetric_eigenvalues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<f64>> = take!(name, 0, &args[0]);
    match mrlyrs::math::spectrum::symmetric_eigenvalues(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spin_blend_fold(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::spin::Blend = take!(name, 0, &args[0]);
    let a1: Vec<f32> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spin::Blend::fold(a0, &a1)))
}

fn door_math_spin_blend_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::spin::Blend::named(a0.as_str())))
}

fn door_math_spin_arcs(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrlyrs::math::spin::arcs(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spin_harmonics(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::spin::harmonics(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spin_mass(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spin::mass(&a0, a1)))
}

fn door_math_spin_mass_within(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::spin::mass_within(&a0, a1, a2)))
}

fn door_math_spin_petals(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spin::petals(a0, a1)))
}

fn door_math_spin_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::spin::profile(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spin_radial(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: mrlyrs::math::spin::Blend = take!(name, 5, &args[5]);
    let a6: usize = take!(name, 6, &args[6]);
    match mrlyrs::math::spin::radial(&a0, a1, a2, a3, a4, a5, a6) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spin_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::spin::reach(a0)))
}

fn door_math_spin_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrlyrs::math::spin::ring(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spin_turns(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::spin::turns(&a0)))
}

fn door_math_spin_wheel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spin::wheel(&a0, a1)))
}

fn door_math_spirograph_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::spirograph::cell(a0, a1, a2)))
}

fn door_math_spirograph_cover(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::spirograph::cover(&a0, &a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spirograph_disc(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    match mrlyrs::math::spirograph::disc(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spirograph_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::spirograph::distinct(&a0, &a1, a2)))
}

fn door_math_spirograph_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spirograph::frame(&a0, &a1)))
}

fn door_math_spirograph_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::spirograph::nodes(&a0, &a1, a2)))
}

fn door_math_spirograph_pencils(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: f64 = take!(name, 5, &args[5]);
    let a6: u32 = take!(name, 6, &args[6]);
    match mrlyrs::math::spirograph::pencils(&a0, a1, a2, a3.as_str(), a4, a5, a6) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spirograph_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::spirograph::Pencil = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::spirograph::point(&a0, &a1, a2)))
}

fn door_math_spirograph_pose(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spirograph::pose(&a0, a1)))
}

fn door_math_spirograph_representatives(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::spirograph::representatives(&a0, &a1, a2)))
}

fn door_math_spirograph_seats(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::spirograph::seats(&a0)))
}

fn door_math_spirograph_signed_area(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::spirograph::Pencil = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spirograph::signed_area(&a0, &a1)))
}

fn door_math_spirograph_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::spirograph::trace(&a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spirograph_track(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::spirograph::track(a0.as_str(), a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_spirograph_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::spirograph::turn(&a0, a1)))
}

fn door_math_three_vec3_cross(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::three::Vec3 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::three::Vec3::cross(a0, a1)))
}

fn door_math_three_vec3_dot(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::three::Vec3 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::three::Vec3::dot(a0, a1)))
}

fn door_math_three_vec3_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: f32 = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    let a2: f32 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::math::three::Vec3::new(a0, a1, a2)))
}

fn door_math_three_vec3_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::three::Vec3::scale(a0, a1)))
}

fn door_math_three_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::carpet(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::census(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_core_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::core_graph::<2>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::core_graph::<3>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_three_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::three::create(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_diagonal_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::three::diagonal_slice(a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_diagonal_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Vec<usize> = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrlyrs::math::three::diagonal_svg(a0, a1, a2, a3, &a4, a5) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_dust(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::dust(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_edge_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::edge_graph::<2>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::edge_graph::<3>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_three_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::euler(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_extrude(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::three::extrude(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_extrude_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::extrude_cube(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_faces(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::faces(&a0)))
}

fn door_math_three_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::fills(&a0)))
}

fn door_math_three_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::three::from_corners(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::math::three::from_json(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_from_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<String>> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::from_strings(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_hidden(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::math::three::hidden(&a0).to_string()))
}

fn door_math_three_level_set(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::three::level_set(a0, &a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_magic(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::magic::<2>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::magic::<3>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_three_manhattan_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::manhattan_layers(a0)))
}

fn door_math_three_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::three::merge(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_mosaic(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 1)? {
    2 => {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 1, &args[1]);
    match mrlyrs::math::three::mosaic::<2>(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::math::cell::models::CellNd<3>> = take!(name, 1, &args[1]);
    match mrlyrs::math::three::mosaic::<3>(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_three_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::gen::recipe::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::three::named(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_net(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::net(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_noise(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let mut a3 = mrlyrs::core::Rng::new(seed(name, 3, &args[3])?);
    match mrlyrs::math::three::noise(a0, a1, a2, &mut a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_ones(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::ones(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_orientations(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::math::three::orientations()))
}

fn door_math_three_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::point(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::three::profile(a0, a1, a2, a3) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_project(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [u32; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::project(a0)))
}

fn door_math_three_quads(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::quads(&a0)))
}

fn door_math_three_shadow(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [u32; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::shadow(a0)))
}

fn door_math_three_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::three::slice(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_special(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
    match mrlyrs::math::three::special(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_star(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::star(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_support(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(big(name, 0, item0)?); } list0 };
    Ok(give!(mrlyrs::math::three::support(&a0)))
}

fn door_math_three_surface(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::math::three::surface(&a0).to_string()))
}

fn door_math_three_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: Option<_> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::three::text(&a0, a1.as_ref())))
}

fn door_math_three_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::to_json(&a0)))
}

fn door_math_three_to_obj(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::to_obj(&a0)))
}

fn door_math_three_to_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::to_strings(&a0)))
}

fn door_math_three_tunnel_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
    2 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::tunnel_graph::<2>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    3 => {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrlyrs::math::three::tunnel_graph::<3>(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
    }
    other => Err(usage(format!("{name} wants a 2d or 3d argument, got {other}d"))),
    }
}

fn door_math_three_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::void(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::voids(&a0)))
}

fn door_math_three_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::volume(&a0)))
}

fn door_math_three_wires(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::three::wires(&a0)))
}

fn door_math_three_xline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::xline(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_xtree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::xtree(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_yline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::yline(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_ytree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::ytree(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::zeros(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_zline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::zline(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_three_ztree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::three::ztree(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_tourbillon_eyes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::tourbillon::eyes(a0)))
}

fn door_math_tourbillon_field(name: &str, args: &[Value]) -> Done {
    count(name, args, 9)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: String = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    let a5: String = take!(name, 5, &args[5]);
    let a6: String = take!(name, 6, &args[6]);
    let a7: String = take!(name, 7, &args[7]);
    let a8: u32 = take!(name, 8, &args[8]);
    match mrlyrs::math::tourbillon::field(a0, a1, a2.as_str(), a3, a4.as_str(), a5.as_str(), a6.as_str(), a7.as_str(), a8) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_tourbillon_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: String = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    let a5: u32 = take!(name, 5, &args[5]);
    match mrlyrs::math::tourbillon::layers(a0, a1.as_str(), a2, a3.as_str(), a4.as_str(), a5) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_tourbillon_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::tourbillon::period(a0)))
}

fn door_math_tourbillon_sharing(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrlyrs::math::tourbillon::Layer> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::tourbillon::sharing(&a0)))
}

fn door_math_tourbillon_stack(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<mrlyrs::math::tourbillon::Layer> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: String = take!(name, 2, &args[2]);
    let a3: mrlyrs::math::spin::Blend = take!(name, 3, &args[3]);
    match mrlyrs::math::tourbillon::stack(&a0, a1, a2.as_str(), a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_tourbillon_stats(name: &str, args: &[Value]) -> Done {
    count(name, args, 9)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: String = take!(name, 5, &args[5]);
    let a6: String = take!(name, 6, &args[6]);
    let a7: String = take!(name, 7, &args[7]);
    let a8: u32 = take!(name, 8, &args[8]);
    match mrlyrs::math::tourbillon::stats(&a0, a1, a2, a3.as_str(), a4, a5.as_str(), a6.as_str(), a7.as_str(), a8) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_capacity(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::two::capacity(&a0)))
}

fn door_math_two_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::carpet(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrlyrs::math::two::census(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::bang::Code = mrlyrs::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::two::create(a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_dust(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::dust(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_embed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    match mrlyrs::math::two::embed(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrlyrs::math::two::euler(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_extract(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrlyrs::math::two::extract(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::two::fills(&a0)))
}

fn door_math_two_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::two::from_corners(&a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::math::two::from_json(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_from_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    match mrlyrs::math::two::from_strings(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_hline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::hline(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_htree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::htree(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_level_set(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrlyrs::math::two::level_set(a0, &a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::math::two::mask(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<mrlyrs::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::math::two::merge(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::gen::recipe::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::math::two::named(a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_net(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::net(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_noise(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let mut a3 = mrlyrs::core::Rng::new(seed(name, 3, &args[3])?);
    match mrlyrs::math::two::noise(a0, a1, a2, &mut a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_ones(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::ones(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_payload_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::math::two::payload::frame()))
}

fn door_math_two_perimeter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::math::two::perimeter(&a0).to_string()))
}

fn door_math_two_png(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrlyrs::core::Color> = if Value::is_null(&args[2]) { None } else { Some(color(name, 2, &args[2])?) };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrlyrs::math::two::Shape = take!(name, 4, &args[4]);
    match mrlyrs::math::two::png(&a0, a1, a2, a3, a4) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::point(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_read(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrlyrs::math::two::read(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_sheet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrlyrs::math::cell::models::CellNd<2>; 4] = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    match mrlyrs::math::two::sheet(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_special(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrlyrs::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrlyrs::math::two::special(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_star(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::star(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrlyrs::core::Color> = if Value::is_null(&args[2]) { None } else { Some(color(name, 2, &args[2])?) };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrlyrs::math::two::Shape = take!(name, 4, &args[4]);
    Ok(give!(mrlyrs::math::two::svg(&a0, a1, a2, a3, a4)))
}

fn door_math_two_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Option<_> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::math::two::text(&a0, a1.as_ref())))
}

fn door_math_two_to_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::two::to_3d(&a0)))
}

fn door_math_two_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::two::to_json(&a0)))
}

fn door_math_two_vline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::vline(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::void(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::math::two::voids(&a0)))
}

fn door_math_two_vtree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::vtree(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_math_two_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::math::two::zeros(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_apollonian_circle_centre(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::apollonian::Circle::centre(a0)))
}

fn door_num_apollonian_circle_is_line(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::apollonian::Circle::is_line(a0)))
}

fn door_num_apollonian_circle_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::apollonian::Circle::radius(a0)))
}

fn door_num_apollonian_form(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [i64; 4] = take!(name, 0, &args[0]);
    let a1: [i64; 4] = take!(name, 1, &args[1]);
    Ok(Value::String(mrlyrs::num::apollonian::form(a0, a1).to_string()))
}

fn door_num_apollonian_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::apollonian::Packing = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::apollonian::frame(&a0)))
}

fn door_num_apollonian_grow(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrlyrs::num::apollonian::grow(a0.as_str(), a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_apollonian_is_ford(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::apollonian::is_ford(a0)))
}

fn door_num_apollonian_on_line(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::apollonian::on_line(a0)))
}

fn door_num_apollonian_reflect(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrlyrs::num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::apollonian::reflect(&a0, a1)))
}

fn door_num_apollonian_root(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrlyrs::num::apollonian::root(a0.as_str()) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_apollonian_shadow(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::apollonian::Packing = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::apollonian::shadow(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_apollonian_sound(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [mrlyrs::num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::apollonian::sound(&a0)))
}

fn door_num_apollonian_swap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrlyrs::num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::apollonian::swap(&a0, a1)))
}

fn door_num_apollonian_touches(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::apollonian::Packing = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::apollonian::touches(&a0)))
}

fn door_num_automaton_automaton_abscissa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::abscissa(&a0)))
}

fn door_num_automaton_automaton_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::base(&a0)))
}

fn door_num_automaton_automaton_cofactor(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrlyrs::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrlyrs::num::automaton::Automaton::cofactor(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_automaton_automaton_denominator(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::denominator(&a0)))
}

fn door_num_automaton_automaton_matrix(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::matrix(&a0)))
}

fn door_num_automaton_automaton_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    match mrlyrs::num::automaton::Automaton::new(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_automaton_automaton_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::peel(&a0)))
}

fn door_num_automaton_automaton_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::period(&a0)))
}

fn door_num_automaton_automaton_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::perron(&a0)))
}

fn door_num_automaton_automaton_residue(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrlyrs::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrlyrs::num::automaton::Automaton::residue(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_automaton_automaton_rule(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::rule(&a0)))
}

fn door_num_automaton_automaton_states(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::automaton::Automaton::states(&a0)))
}

fn door_num_automaton_automaton_with_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::automaton::Automaton::with_peel(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_automaton_automaton_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrlyrs::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrlyrs::num::automaton::Automaton::zeta(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_blend_add(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    let a1: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 1, &args[1])? { list0.push(small(name, 1, item0)?); } list0 };
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::num::blend::add(&a0, &a1) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_num_blend_cauchy(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    let a1: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 1, &args[1])? { list0.push(small(name, 1, item0)?); } list0 };
    match mrlyrs::num::blend::cauchy(&a0, &a1) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_blend_characteristic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(i128, i128)> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push({ let parts1 = parts(name, 0, item0, 2)?; (small(name, 0, &parts1[0])?, small(name, 0, &parts1[1])?) }); } list0 };
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::num::blend::characteristic(&a0) { list0.push({ let parts1 = item0; Value::Array(vec![Value::String(parts1.0.to_string()), Value::String(parts1.1.to_string())]) }); } Value::Array(list0) })
}

fn door_num_blend_decimate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::num::blend::decimate(&a0, a1, a2) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_blend_delta(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::num::blend::delta(&a0) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_num_blend_growth(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(i128, i128)> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push({ let parts1 = parts(name, 0, item0, 2)?; (small(name, 0, &parts1[0])?, small(name, 0, &parts1[1])?) }); } list0 };
    Ok(give!(mrlyrs::num::blend::growth(&a0)))
}

fn door_num_blend_hadamard(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    let a1: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 1, &args[1])? { list0.push(small(name, 1, item0)?); } list0 };
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::num::blend::hadamard(&a0, &a1) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_num_blend_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    Ok(match mrlyrs::num::blend::recurrence(&a0) { Some(item0) => { let mut list1 = Vec::new(); for item1 in item0 { list1.push({ let parts2 = item1; Value::Array(vec![Value::String(parts2.0.to_string()), Value::String(parts2.1.to_string())]) }); } Value::Array(list1) }, None => Value::Null })
}

fn door_num_blend_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    let a1: i128 = small(name, 1, &args[1])?;
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::num::blend::scale(&a0, a1) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_num_blend_shift(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    let a1: usize = take!(name, 1, &args[1]);
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::num::blend::shift(&a0, a1) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_num_blend_sigma(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    match mrlyrs::num::blend::sigma(&a0) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push(Value::String(item0.to_string())); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_blend_sub(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 0, &args[0])? { list0.push(small(name, 0, item0)?); } list0 };
    let a1: Vec<i128> = { let mut list0 = Vec::new(); for item0 in items(name, 1, &args[1])? { list0.push(small(name, 1, item0)?); } list0 };
    Ok({ let mut list0 = Vec::new(); for item0 in mrlyrs::num::blend::sub(&a0, &a1) { list0.push(Value::String(item0.to_string())); } Value::Array(list0) })
}

fn door_num_boolean_is_balanced(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::boolean::is_balanced(a0, a1)))
}

fn door_num_boolean_nonlinearity(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::boolean::nonlinearity(a0, a1)))
}

fn door_num_boolean_sac(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::boolean::sac(a0, a1)))
}

fn door_num_boolean_walsh_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::boolean::walsh_spectrum(a0, a1)))
}

fn door_num_design_digits_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::design::digits_of(a0, a1)))
}

fn door_num_design_echo_series(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::design::echo_series(&a0, &a1, a2)))
}

fn door_num_design_elements(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::design::elements(a0, &a1, a2)))
}

fn door_num_design_log_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::design::log_grid(&a0, a1)))
}

fn door_num_design_median_floor(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::design::median_floor(&a0, a1)))
}

fn door_num_design_meter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i8> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::design::meter(&a0)))
}

fn door_num_design_nearest(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::design::nearest(a0, &a1)))
}

fn door_num_design_peaks(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: (f64, f64) = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrlyrs::num::design::peaks(&a0, &a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_design_pole_lattice(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::design::pole_lattice(a0, a1)))
}

fn door_num_design_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<i64> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: Vec<f64> = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::num::design::resample(&a0, &a1, a2, &a3)))
}

fn door_num_design_score(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::design::score(&a0, a1)))
}

fn door_num_design_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(Value::String(mrlyrs::num::design::size(&a0, a1).to_string()))
}

fn door_num_design_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    match mrlyrs::num::design::spectrum(&a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_design_upper_rms(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::design::upper_rms(&a0)))
}

fn door_num_factor_aliquot(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::aliquot(a0)))
}

fn door_num_factor_coprime(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::factor::coprime(a0, a1)))
}

fn door_num_factor_divisors(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::divisors(a0)))
}

fn door_num_factor_factorial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::num::factor::factorial(a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_factor_factorize(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::factorize(a0)))
}

fn door_num_factor_factorize_wide(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::factorize_wide(a0)))
}

fn door_num_factor_gcd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: u128 = big(name, 1, &args[1])?;
    Ok(Value::String(mrlyrs::num::factor::gcd(a0, a1).to_string()))
}

fn door_num_factor_lcm(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::factor::lcm(a0, a1)))
}

fn door_num_factor_mobius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::mobius(a0)))
}

fn door_num_factor_mobius_sieve(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::mobius_sieve(a0)))
}

fn door_num_factor_radical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::radical(a0)))
}

fn door_num_factor_reduce(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: u128 = big(name, 1, &args[1])?;
    Ok({ let parts0 = mrlyrs::num::factor::reduce(a0, a1); Value::Array(vec![Value::String(parts0.0.to_string()), Value::String(parts0.1.to_string())]) })
}

fn door_num_factor_sigma(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(Value::String(mrlyrs::num::factor::sigma(a0, a1).to_string()))
}

fn door_num_factor_squarefree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::squarefree(a0)))
}

fn door_num_factor_totient(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::totient(a0)))
}

fn door_num_factor_totients(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::factor::totients(a0)))
}

fn door_num_factor_twisted(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<i8> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::factor::twisted(a0, &a1)))
}

fn door_num_fft_convolve(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::num::fft::convolve(&a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_fft_convolve_with(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: Vec<f64> = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrlyrs::num::fft::convolve_with(&a0, &a1, &a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_fft_embed_kernel(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::num::fft::embed_kernel(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_fft_log_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::fft::log_spectrum(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_fft_magnitude_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::fft::magnitude_spectrum(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_fft_peak_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::fft::peak_ring(&a0)))
}

fn door_num_fft_peak_wavelength(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::fft::peak_wavelength(&a0, a1)))
}

fn door_num_fft_radial_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::fft::radial_profile(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_fft_transform(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::fft::transform(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_gauss_class_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Class = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Class::prime(a0)))
}

fn door_num_gauss_class_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Class = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Class::word(a0)))
}

fn door_num_gauss_ring_associates(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::associates(a0, a1, a2)))
}

fn door_num_gauss_ring_canon(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::canon(a0, a1, a2)))
}

fn door_num_gauss_ring_conjugate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::conjugate(a0, a1, a2)))
}

fn door_num_gauss_ring_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::gauss::Ring::count(a0, a1)))
}

fn door_num_gauss_ring_div_rem(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::div_rem(a0, a1, a2)))
}

fn door_num_gauss_ring_fate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::gauss::Ring::fate(a0, a1)))
}

fn door_num_gauss_ring_gaussian_gcd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::gaussian_gcd(a0, a1, a2)))
}

fn door_num_gauss_ring_inert(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::gauss::Ring::inert(a0, a1)))
}

fn door_num_gauss_ring_mul(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::mul(a0, a1, a2)))
}

fn door_num_gauss_ring_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Ring::named(a0.as_str())))
}

fn door_num_gauss_ring_nearest(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::nearest(a0, a1, a2)))
}

fn door_num_gauss_ring_norm(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::norm(a0, a1, a2)))
}

fn door_num_gauss_ring_place(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::place(a0, a1, a2)))
}

fn door_num_gauss_ring_ramified(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Ring::ramified(a0)))
}

fn door_num_gauss_ring_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::reach(a0, a1, a2)))
}

fn door_num_gauss_ring_symmetry(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Ring::symmetry(a0)))
}

fn door_num_gauss_ring_top(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::gauss::Ring::top(a0, a1)))
}

fn door_num_gauss_ring_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::turn(a0, a1, a2)))
}

fn door_num_gauss_ring_units(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Ring::units(a0)))
}

fn door_num_gauss_ring_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Ring::whole(a0, a1, a2)))
}

fn door_num_gauss_window_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Window::census(&a0)))
}

fn door_num_gauss_window_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Window = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Window::class(&a0, a1, a2)))
}

fn door_num_gauss_window_holds(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::gauss::Window = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::gauss::Window::holds(&a0, a1, a2)))
}

fn door_num_gauss_window_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::gauss::Window::new(a0, a1)))
}

fn door_num_gauss_window_points(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Window::points(&a0)))
}

fn door_num_gauss_window_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Window::radius(&a0)))
}

fn door_num_gauss_window_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::gauss::Window::ring(&a0)))
}

fn door_num_gauss_classes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::gauss::classes(a0, a1)))
}

fn door_num_gauss_peak(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::gauss::peak(a0, a1)))
}

fn door_num_gauss_shells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::gauss::shells(a0, a1)))
}

fn door_num_ladder_design_abscissa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::ladder::Design::abscissa(&a0)))
}

fn door_num_ladder_design_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::ladder::Design::base(&a0)))
}

fn door_num_ladder_design_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::ladder::Design::digits(&a0)))
}

fn door_num_ladder_design_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    match mrlyrs::num::ladder::Design::new(a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_ladder_design_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::ladder::Design::peel(&a0)))
}

fn door_num_ladder_design_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::ladder::Design::period(&a0)))
}

fn door_num_ladder_design_pole(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::ladder::Design::pole(&a0, a1, a2)))
}

fn door_num_ladder_design_with_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::num::ladder::Design::with_peel(a0, &a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_ladder_cofactor(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    let a1: mrlyrs::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrlyrs::num::ladder::cofactor(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_ladder_residue(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrlyrs::num::ladder::residue(&a0, a1, a2, a3) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_ladder_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::ladder::Design = take!(name, 0, &args[0]);
    let a1: mrlyrs::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrlyrs::num::ladder::zeta(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_lattice_coprime_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::lattice::coprime_pairs(a0)))
}

fn door_num_lattice_farey(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::lattice::farey(a0)))
}

fn door_num_lattice_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::lattice::grid(a0)))
}

fn door_num_lattice_new_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::lattice::new_nodes(a0)))
}

fn door_num_lattice_pi_estimate(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::lattice::pi_estimate(a0)))
}

fn door_num_lattice_recovered(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::num::lattice::recovered(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_lattice_visible_density(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrlyrs::num::lattice::visible_density(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_lattice_zeta_factor(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::lattice::zeta_factor(a0)))
}

fn door_num_lattice_zeta_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrlyrs::num::lattice::zeta_whole(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_memory_rule_accepts(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::memory::Rule::accepts(&a0, &a1)))
}

fn door_num_memory_rule_allowed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::memory::Rule::allowed(&a0, a1)))
}

fn door_num_memory_rule_alphabet(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::Rule::alphabet(&a0)))
}

fn door_num_memory_rule_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(Value::String(mrlyrs::num::memory::Rule::codes(&a0).to_string()))
}

fn door_num_memory_rule_full(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::memory::Rule::full(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_memory_rule_letters(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::Rule::letters(&a0)))
}

fn door_num_memory_rule_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrlyrs::num::memory::Rule::new(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_memory_rule_states(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::Rule::states(&a0)))
}

fn door_num_memory_rule_windows(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::Rule::windows(&a0)))
}

fn door_num_memory_allowed_windows(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::allowed_windows(&a0)))
}

fn door_num_memory_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::memory::cells(&a0, a1)))
}

fn door_num_memory_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::memory::counts(&a0, a1)))
}

fn door_num_memory_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::exponent(&a0)))
}

fn door_num_memory_kappa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::kappa(&a0)))
}

fn door_num_memory_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::perron(&a0)))
}

fn door_num_memory_transfer(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::memory::transfer(&a0)))
}

fn door_num_morse_lift_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::num::morse::Lift::all()))
}

fn door_num_morse_lift_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::morse::Lift = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::morse::Lift::at(a0, a1, a2)))
}

fn door_num_morse_lift_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::morse::Lift = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::morse::Lift::formula(a0)))
}

fn door_num_morse_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::morse::boundary(&a0)))
}

fn door_num_morse_difference(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::morse::difference(&a0, &a1)))
}

fn door_num_morse_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::morse::digits(a0)))
}

fn door_num_morse_doubling(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::morse::doubling(a0)))
}

fn door_num_morse_faults(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::morse::faults(&a0, &a1)))
}

fn door_num_morse_fold(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::num::morse::fold(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_morse_letter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::morse::letter(a0)))
}

fn door_num_morse_lift(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::morse::Lift = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::morse::lift(a0, a1)))
}

fn door_num_morse_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrlyrs::num::morse::power(&a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_morse_repeat(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::morse::repeat(&a0, a1, a2)))
}

fn door_num_morse_runs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::morse::runs(&a0)))
}

fn door_num_morse_stage(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::morse::stage(a0)))
}

fn door_num_morse_substitution(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::morse::substitution(a0)))
}

fn door_num_morse_upsample(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::morse::upsample(&a0, a1, a2)))
}

fn door_num_prime_sieve_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::Sieve::count(&a0)))
}

fn door_num_prime_sieve_done(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::Sieve::done(&a0)))
}

fn door_num_prime_sieve_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::Sieve::new(a0)))
}

fn door_num_prime_sieve_rank(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::Sieve::rank(&a0)))
}

fn door_num_prime_sieve_struck(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::Sieve::struck(&a0)))
}

fn door_num_prime_sieve_types(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::Sieve::types(&a0)))
}

fn door_num_prime_chart(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::prime::chart(a0, a1)))
}

fn door_num_prime_flags(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::flags(a0)))
}

fn door_num_prime_goldbach(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::goldbach(a0)))
}

fn door_num_prime_goldbach_record(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::goldbach_record(a0)))
}

fn door_num_prime_is_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::is_prime(a0)))
}

fn door_num_prime_pile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::pile(a0)))
}

fn door_num_prime_prime_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::prime_count(a0)))
}

fn door_num_prime_prime_from(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::prime_from(a0)))
}

fn door_num_prime_primes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::primes(a0)))
}

fn door_num_prime_rectangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::rectangles(a0)))
}

fn door_num_prime_splits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::splits(a0)))
}

fn door_num_prime_squares(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::squares(a0)))
}

fn door_num_prime_study(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::prime::study(a0)))
}

fn door_num_radix_base_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    match mrlyrs::num::radix::Base::class(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_base_congruent(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::radix::Base::congruent(a0, a1, a2)))
}

fn door_num_radix_base_group(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    match mrlyrs::num::radix::Base::group(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_base_mirrored(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Base::mirrored(a0)))
}

fn door_num_radix_base_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    match mrlyrs::num::radix::Base::new(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_base_norm(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Base::norm(a0)))
}

fn door_num_radix_base_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::radix::Base::power(a0, a1)))
}

fn door_num_radix_base_residues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    match mrlyrs::num::radix::Base::residues(a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_base_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Base::ring(a0)))
}

fn door_num_radix_base_value(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Base::value(a0)))
}

fn door_num_radix_radix_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Radix::base(&a0)))
}

fn door_num_radix_radix_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    match mrlyrs::num::radix::Radix::canonical(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_radix_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    match mrlyrs::num::radix::Radix::code(&a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_radix_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Radix::digits(&a0)))
}

fn door_num_radix_radix_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Radix::dimension(&a0)))
}

fn door_num_radix_radix_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::radix::Radix::distinct(&a0, a1)))
}

fn door_num_radix_radix_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(Value::String(mrlyrs::num::radix::Radix::fill(&a0, a1).to_string()))
}

fn door_num_radix_radix_from_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrlyrs::num::radix::Radix::from_code(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_radix_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::radix::Base = take!(name, 0, &args[0]);
    let a1: Vec<(i64, i64)> = take!(name, 1, &args[1]);
    let a2: Vec<(i64, i64)> = take!(name, 2, &args[2]);
    match mrlyrs::num::radix::Radix::new(a0, a1, a2) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_radix_plane(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::radix::Radix::plane(&a0, a1)))
}

fn door_num_radix_radix_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Radix::ring(&a0)))
}

fn door_num_radix_radix_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Radix::size(&a0)))
}

fn door_num_radix_radix_twists(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::radix::Radix::twists(&a0)))
}

fn door_num_radix_radix_with_twists(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrlyrs::num::radix::Radix::with_twists(a0, &a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_radix_words(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::radix::Radix::words(&a0, a1)))
}

fn door_num_radix_flowsnake(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrlyrs::num::radix::flowsnake() { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrlyrs::num::radix::gasket() { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_koch(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrlyrs::num::radix::koch() { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_terdragon(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrlyrs::num::radix::terdragon() { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrlyrs::num::radix::tile(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_radix_twindragon(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrlyrs::num::radix::twindragon() { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_series_basel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::basel(a0)))
}

fn door_num_series_bernoulli(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrlyrs::num::series::bernoulli(a0) { Ok(value) => Ok({ let mut list0 = Vec::new(); for item0 in value { list0.push({ let parts1 = item0; Value::Array(vec![Value::String(parts1.0.to_string()), Value::String(parts1.1.to_string())]) }); } Value::Array(list0) }), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_series_beta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::series::beta(a0, a1)))
}

fn door_num_series_binary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::binary(a0)))
}

fn door_num_series_catalan(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::catalan(a0)))
}

fn door_num_series_chi3(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::chi3(a0)))
}

fn door_num_series_chi4(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::chi4(a0)))
}

fn door_num_series_chi8(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::chi8(a0)))
}

fn door_num_series_dirichlet(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<i8> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::series::dirichlet(a0, &a1, a2)))
}

fn door_num_series_e_partial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::e_partial(a0)))
}

fn door_num_series_euler_gamma_partial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::euler_gamma_partial(a0)))
}

fn door_num_series_euler_product(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::series::euler_product(a0, a1)))
}

fn door_num_series_evens(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::evens(a0)))
}

fn door_num_series_fibonacci(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::fibonacci(a0)))
}

fn door_num_series_harmonic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::harmonic(a0)))
}

fn door_num_series_lambda(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::series::lambda(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_series_leibniz(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::leibniz(a0)))
}

fn door_num_series_li(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::li(a0)))
}

fn door_num_series_mertens(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::mertens(a0)))
}

fn door_num_series_odds(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::odds(a0)))
}

fn door_num_series_visible(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::num::series::visible(a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_series_wallis_half_pi(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::wallis_half_pi(a0)))
}

fn door_num_series_wallis_quarter_pi(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::series::wallis_quarter_pi(a0)))
}

fn door_num_series_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrlyrs::num::series::zeta(a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::num::sieve::cells(&a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::num::sieve::exponent(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_flat_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::sieve::flat_word(a0, a1)))
}

fn door_num_sieve_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::num::sieve::holes(&a0, a1) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::num::sieve::limit(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_odd_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::sieve::odd_word(a0)))
}

fn door_num_sieve_punctures(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::num::sieve::punctures(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrlyrs::num::sieve::raster(&a0) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrlyrs::num::sieve::ratio(&a0, a1) { Ok(value) => Ok(give!(value)), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrlyrs::num::sieve::side(&a0) { Ok(value) => Ok(Value::String(value.to_string())), Err(error) => Err(Fail::Error(error.to_string())) }
}

fn door_num_sieve_solid_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::num::sieve::solid_limit()))
}

fn door_num_spiral_growth_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::num::spiral::Growth::all()))
}

fn door_num_spiral_lattice_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::num::spiral::Lattice::all()))
}

fn door_num_spiral_lattice_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::spiral::Lattice::count(a0, a1)))
}

fn door_num_spiral_lattice_n(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::spiral::Lattice::n(a0, a1, a2)))
}

fn door_num_spiral_lattice_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::spiral::Lattice::radius(a0, a1)))
}

fn door_num_spiral_lattice_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::spiral::Lattice::ring(a0, a1)))
}

fn door_num_spiral_lattice_ring_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrlyrs::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::spiral::Lattice::ring_of(a0, a1, a2)))
}

fn door_num_spiral_lattice_xy(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::spiral::Lattice::xy(a0, a1)))
}

fn door_num_spiral_mark_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::num::spiral::Mark::all()))
}

fn door_num_spiral_diagonal(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrlyrs::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    let a4: i64 = take!(name, 4, &args[4]);
    Ok(give!(mrlyrs::num::spiral::diagonal(a0, a1, a2, a3, a4)))
}

fn door_num_spiral_level_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::spiral::level_of(a0, a1)))
}

fn door_num_spiral_marks(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::spiral::Mark = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::spiral::marks(a0, a1)))
}

fn door_num_spiral_snail(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: mrlyrs::num::spiral::Growth = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::spiral::snail(a0, a1, a2)))
}

fn door_num_zeta_complex_abs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::Complex::abs(a0)))
}

fn door_num_zeta_complex_arg(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::Complex::arg(a0)))
}

fn door_num_zeta_complex_exp(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::Complex::exp(a0)))
}

fn door_num_zeta_complex_ln(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::Complex::ln(a0)))
}

fn door_num_zeta_complex_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Complex::new(a0, a1)))
}

fn door_num_zeta_complex_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::Complex::turn(a0)))
}

fn door_num_zeta_line_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::count(&a0, a1)))
}

fn door_num_zeta_line_exact(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::exact(&a0, a1)))
}

fn door_num_zeta_line_gram(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::gram(&a0, a1)))
}

fn door_num_zeta_line_maclaurin(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::maclaurin(&a0, a1)))
}

fn door_num_zeta_line_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::num::zeta::Line::new()))
}

fn door_num_zeta_line_novelty_coefficients(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::novelty_coefficients(&a0, &a1)))
}

fn door_num_zeta_line_pair(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: mrlyrs::num::zeta::Complex = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::pair(&a0, a1)))
}

fn door_num_zeta_line_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::point(&a0, a1)))
}

fn door_num_zeta_line_seam(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrlyrs::num::zeta::Line::seam(&a0, a1, a2, a3)))
}

fn door_num_zeta_line_siegel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::siegel(&a0, a1)))
}

fn door_num_zeta_line_theta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::theta(&a0, a1)))
}

fn door_num_zeta_line_z(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::z(&a0, a1)))
}

fn door_num_zeta_line_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrlyrs::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::Line::zeros(&a0, a1)))
}

fn door_num_zeta_bump(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::bump(a0)))
}

fn door_num_zeta_corrections(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::corrections(a0)))
}

fn door_num_zeta_kernel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::kernel(a0)))
}

fn door_num_zeta_mellin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrlyrs::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::mellin(a0)))
}

fn door_num_zeta_novelty_main(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrlyrs::num::zeta::novelty_main()))
}

fn door_num_zeta_novelty_wave(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<mrlyrs::num::zeta::Complex> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::zeta::novelty_wave(&a0, &a1, a2)))
}

fn door_num_zeta_psi_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::psi_formula(a0, &a1)))
}

fn door_num_zeta_psi_stair(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrlyrs::num::zeta::psi_stair(a0)))
}

fn door_num_zeta_raise(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: mrlyrs::num::zeta::Complex = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::raise(a0, a1)))
}

fn door_num_zeta_sharp_novelty(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrlyrs::num::zeta::sharp_novelty(&a0, a1)))
}

fn door_num_zeta_smoothed_novelty(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrlyrs::num::zeta::smoothed_novelty(&a0, a1, a2)))
}
