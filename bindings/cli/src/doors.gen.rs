use super::*;
use serde_json::Value;

pub(crate) static DOORS: &[Function] = &[
    Function {
        name: "core.Colorizer.default",
        signature: "() -> core.Colorizer",
        docs: "Returns the default Colorizer.",
        uncallable: None,
        door: Some(door_core_colorizer_default),
    },
    Function {
        name: "core.Colorizer.diverge",
        signature: "() -> core.Colorizer",
        docs: "Builds the blue-to-red diverging ramp around a white middle.",
        uncallable: None,
        door: Some(door_core_colorizer_diverge),
    },
    Function {
        name: "core.Colorizer.fire",
        signature: "() -> core.Colorizer",
        docs: "Builds the black-through-ember fire ramp: black, dark red, orange, light yellow.",
        uncallable: None,
        door: Some(door_core_colorizer_fire),
    },
    Function {
        name: "core.Colorizer.gradient_bins",
        signature: "(background: Color, colors: [Color], shades: usize) -> core.Colorizer",
        docs: "Builds a binned colorizer from a gradient through the given stops.\n\n# Errors\n\nErrs on an empty stop list.",
        uncallable: None,
        door: Some(door_core_colorizer_gradient_bins),
    },
    Function {
        name: "core.Colorizer.heat",
        signature: "() -> core.Colorizer",
        docs: "Builds the white-to-black heat ramp.",
        uncallable: None,
        door: Some(door_core_colorizer_heat),
    },
    Function {
        name: "core.Dtype.max",
        signature: "(self: core.Dtype) -> i64",
        docs: "Returns the largest value the width can hold.",
        uncallable: None,
        door: Some(door_core_dtype_max),
    },
    Function {
        name: "core.cell.anti",
        signature: "(self: Cell) -> Cell",
        docs: "Flips every type to one minus itself, same as invert.",
        uncallable: None,
        door: Some(door_core_cell_anti),
    },
    Function {
        name: "core.cell.binarize",
        signature: "(self: Cell, threshold: u8) -> Cell",
        docs: "Maps every type to one at or above the threshold and zero below, dropping colors.",
        uncallable: None,
        door: Some(door_core_cell_binarize),
    },
    Function {
        name: "core.cell.binarize_otsu",
        signature: "(self: Cell) -> Cell",
        docs: "Binarizes the types at Otsu's threshold, dropping colors.",
        uncallable: None,
        door: Some(door_core_cell_binarize_otsu),
    },
    Function {
        name: "core.cell.blur",
        signature: "(self: Cell, mask: Tensor, wrap: bool) -> Cell",
        docs: "Replaces every type with the rounded mean of its masked neighborhood, dropping colors.\n\n# Errors\n\nErrs on a mask of another rank, an even side, or a mask with nothing on.",
        uncallable: None,
        door: Some(door_core_cell_blur),
    },
    Function {
        name: "core.cell.color_at",
        signature: "(self: Cell, flat: usize) -> [u8; 4]",
        docs: "Returns the painted color at a flat index, or transparent while unpainted or past the end.",
        uncallable: None,
        door: Some(door_core_cell_color_at),
    },
    Function {
        name: "core.cell.combine",
        signature: "(self: Cell, other: Cell) -> Cell",
        docs: "Builds the Kronecker product of the two cells' types.",
        uncallable: None,
        door: Some(door_core_cell_combine),
    },
    Function {
        name: "core.cell.fractal",
        signature: "(self: Cell, level: usize) -> Cell",
        docs: "Grows the types to the level-fold Kronecker power of themselves, dropping colors and tags.\n\n# Errors\n\nErrs when the level is below one.",
        uncallable: None,
        door: Some(door_core_cell_fractal),
    },
    Function {
        name: "core.cell.invert",
        signature: "(self: Cell) -> Cell",
        docs: "Flips every type to one minus itself.",
        uncallable: None,
        door: Some(door_core_cell_invert),
    },
    Function {
        name: "core.cell.layers",
        signature: "(self: Cell, dtype: core.Dtype) -> Cell",
        docs: "Tags every cell with its concentric shell distance from the center.",
        uncallable: None,
        door: Some(door_core_cell_layers),
    },
    Function {
        name: "core.cell.magic",
        signature: "(cells: [Cell]) -> Cell",
        docs: "Folds at least two cells into one by chained Kronecker products.\n\n# Errors\n\nErrs with fewer than two cells.",
        uncallable: None,
        door: Some(door_core_cell_magic),
    },
    Function {
        name: "core.cell.mapping",
        signature: "() -> {u8: [Color]}",
        docs: "Returns the default mapping of the first six types to white, black, alpha, red, green, and blue.",
        uncallable: None,
        door: Some(door_core_cell_mapping),
    },
    Function {
        name: "core.cell.merge",
        signature: "(cells: [Cell], reps: [usize]) -> Cell",
        docs: "Stitches same-shaped cells into one grid of reps blocks per axis.\n\n# Errors\n\nErrs on an empty list, a cell count that is not the product of reps, a rank mismatch, or cells of unequal shape.",
        uncallable: None,
        door: Some(door_core_cell_merge),
    },
    Function {
        name: "core.cell.moore",
        signature: "(dimension: usize) -> Tensor",
        docs: "Builds the 3-wide Moore mask of the dimension, every site on but the center.",
        uncallable: None,
        door: Some(door_core_cell_moore),
    },
    Function {
        name: "core.cell.mosaic",
        signature: "(mask: Tensor, cells: [Cell]) -> Cell",
        docs: "Lays the cell each mask entry indexes into that entry's place and merges the lot.\n\n# Errors\n\nErrs when a mask entry indexes no cell, or the picked cells do not merge.",
        uncallable: None,
        door: Some(door_core_cell_mosaic),
    },
    Function {
        name: "core.cell.neighbors",
        signature: "(self: Cell, mask: Tensor, target: u8, wrap: bool, dtype: core.Dtype) -> Cell",
        docs: "Tags every cell with its count of target-valued neighbors under the mask.\n\n# Errors\n\nErrs on a mask of another rank or an even side, a target above one, or a count past the dtype.",
        uncallable: None,
        door: Some(door_core_cell_neighbors),
    },
    Function {
        name: "core.cell.new",
        signature: "(types: Tensor) -> Cell",
        docs: "Wraps a tensor of types in a bare cell, colorless and tagless.",
        uncallable: None,
        door: Some(door_core_cell_new),
    },
    Function {
        name: "core.cell.pad",
        signature: "(self: Cell, count: usize, value: u8) -> Cell",
        docs: "Wraps the cell in count layers of value on every side, dropping colors.",
        uncallable: None,
        door: Some(door_core_cell_pad),
    },
    Function {
        name: "core.cell.paint",
        signature: "(self: Cell, mapping: {u8: [Color]}, mode: core.Mode, rng: Rng(seed)?) -> Cell",
        docs: "Colors every mapped cell, picking within each type's palette by the mode.\n\n# Errors\n\nErrs when the Random mode arrives without a stream to draw from.",
        uncallable: None,
        door: Some(door_core_cell_paint),
    },
    Function {
        name: "core.cell.perforate",
        signature: "(self: Cell, mask: Tensor, value: u8) -> Cell",
        docs: "Stamps value wherever the tiled mask is on, dropping colors.\n\n# Errors\n\nErrs on a mask of another rank or a side that does not evenly tile the cell.",
        uncallable: None,
        door: Some(door_core_cell_perforate),
    },
    Function {
        name: "core.cell.remap",
        signature: "(cell: Cell, map: [usize], shape: [usize]) -> Cell",
        docs: "Rebuilds a cell's types, colors and tags at the new shape from one destination-to-source index map.\n\n# Errors\n\nErrs when the map is not the shape's size, or points past the cell.",
        uncallable: None,
        door: Some(door_core_cell_remap),
    },
    Function {
        name: "core.cell.rgba",
        signature: "(self: Cell) -> [u8]",
        docs: "Returns the flat rgba bytes of the cells, four to a cell, the stored color where there is one and opaque black everywhere else.",
        uncallable: None,
        door: Some(door_core_cell_rgba),
    },
    Function {
        name: "core.cell.rot90_map",
        signature: "(shape: [usize], k: usize, axes: (usize, usize)) -> [usize]",
        docs: "Builds the flat source index of every destination cell after k quarter turns in the plane of the axes.\n\n# Errors\n\nErrs when the axes are not two distinct axes of the shape.",
        uncallable: None,
        door: Some(door_core_cell_rot90_map),
    },
    Function {
        name: "core.cell.rotate",
        signature: "(self: Cell, k: usize, axes: (usize, usize)) -> Cell",
        docs: "Rotates the cell k quarter turns in the plane of the given axes, carrying colors and tags along.\n\n# Errors\n\nErrs when the axes are not two distinct axes of the cell.",
        uncallable: None,
        door: Some(door_core_cell_rotate),
    },
    Function {
        name: "core.cell.shape",
        signature: "(self: Cell) -> [usize]",
        docs: "Returns the shape of the type tensor.",
        uncallable: None,
        door: Some(door_core_cell_shape),
    },
    Function {
        name: "core.cell.size",
        signature: "(self: Cell) -> usize",
        docs: "Returns the number of cells.",
        uncallable: None,
        door: Some(door_core_cell_size),
    },
    Function {
        name: "core.cell.tile",
        signature: "(self: Cell, reps: [usize]) -> Cell",
        docs: "Repeats the cell reps times along each axis, carrying colors and tags along.\n\n# Errors\n\nErrs without one count per axis.",
        uncallable: None,
        door: Some(door_core_cell_tile),
    },
    Function {
        name: "core.cell.tile_map",
        signature: "(shape: [usize], reps: [usize]) -> [usize]",
        docs: "Builds the flat source index of every destination cell after tiling reps copies per axis.\n\n# Errors\n\nErrs without one count per axis.",
        uncallable: None,
        door: Some(door_core_cell_tile_map),
    },
    Function {
        name: "core.colors.Theme.hues",
        signature: "(self: core.colors.Theme) -> [Color; 13]",
        docs: "The thirteen inks in name order.",
        uncallable: None,
        door: Some(door_core_colors_theme_hues),
    },
    Function {
        name: "core.colors.Theme.inks",
        signature: "(self: core.colors.Theme) -> [Color; 6]",
        docs: "The six inks a figure cycles through: blue, orange, yellow, green, pink, indigo.",
        uncallable: None,
        door: Some(door_core_colors_theme_inks),
    },
    Function {
        name: "core.colors.alpha",
        signature: "(self: Color, level: u8) -> Color",
        docs: "Returns the color with its alpha set to level.",
        uncallable: None,
        door: Some(door_core_colors_alpha),
    },
    Function {
        name: "core.colors.board",
        signature: "(dark: bool) -> [u8; 4]",
        docs: "Returns the ground rgba of the dark or the light theme.",
        uncallable: None,
        door: Some(door_core_colors_board),
    },
    Function {
        name: "core.colors.css",
        signature: "(self: Color) -> String",
        docs: "Formats the color as a css rgb or rgba call.",
        uncallable: None,
        door: Some(door_core_colors_css),
    },
    Function {
        name: "core.colors.from_hex",
        signature: "(hex: str) -> Color",
        docs: "Parses a #RRGGBB or #RRGGBBAA code, hash optional.\n\n# Errors\n\nErrs when the text is not six or eight ascii hex digits after an optional hash.",
        uncallable: None,
        door: Some(door_core_colors_from_hex),
    },
    Function {
        name: "core.colors.gradient",
        signature: "(colors: [Color], steps: usize) -> [Color]",
        docs: "Builds a gradient of steps colors sweeping evenly through the given stops.\n\n# Errors\n\nErrs on an empty stop list or fewer than one step.",
        uncallable: None,
        door: Some(door_core_colors_gradient),
    },
    Function {
        name: "core.colors.ink",
        signature: "(dark: bool) -> [u8; 4]",
        docs: "Returns the foreground rgba of the dark or the light theme.",
        uncallable: None,
        door: Some(door_core_colors_ink),
    },
    Function {
        name: "core.colors.invert",
        signature: "(self: Color) -> Color",
        docs: "Returns the color with every channel flipped and the alpha kept.",
        uncallable: None,
        door: Some(door_core_colors_invert),
    },
    Function {
        name: "core.colors.lightness",
        signature: "(self: Color, level: u8) -> Color",
        docs: "Returns the color scaled toward black below level 50 and toward white above.\n\n# Errors\n\nErrs when the level is past 100.",
        uncallable: None,
        door: Some(door_core_colors_lightness),
    },
    Function {
        name: "core.colors.luma_types",
        signature: "(pixels: [[u8; 4]], width: usize, height: usize, level: u8) -> Tensor",
        docs: "Reads rgba pixels as a type grid, one wherever the rgb mean falls below the level.",
        uncallable: None,
        door: Some(door_core_colors_luma_types),
    },
    Function {
        name: "core.colors.mix",
        signature: "(color_1: Color, color_2: Color, ratio: f64) -> Color",
        docs: "Blends two colors linearly by ratio.\n\n# Errors\n\nErrs when the ratio falls outside zero to one.",
        uncallable: None,
        door: Some(door_core_colors_mix),
    },
    Function {
        name: "core.colors.named",
        signature: "(name: str) -> Color",
        docs: "Returns the palette color a name spells.\n\n# Errors\n\nErrs when no palette color carries the name.",
        uncallable: None,
        door: Some(door_core_colors_named),
    },
    Function {
        name: "core.colors.random",
        signature: "(alpha: bool, rng: Rng(seed)) -> Color",
        docs: "Draws a color from the stream, opaque unless alpha is asked for.",
        uncallable: None,
        door: Some(door_core_colors_random),
    },
    Function {
        name: "core.colors.rgb",
        signature: "(r: u8, g: u8, b: u8) -> Color",
        docs: "Builds an opaque color.",
        uncallable: None,
        door: Some(door_core_colors_rgb),
    },
    Function {
        name: "core.colors.rgba",
        signature: "(r: u8, g: u8, b: u8, a: u8) -> Color",
        docs: "Builds a color with an explicit alpha.",
        uncallable: None,
        door: Some(door_core_colors_rgba),
    },
    Function {
        name: "core.colors.shades",
        signature: "(hue: Color) -> [Color; 3]",
        docs: "Returns a hue one shade lighter, itself, and one shade darker.",
        uncallable: None,
        door: Some(door_core_colors_shades),
    },
    Function {
        name: "core.colors.snap",
        signature: "(pixels: [[u8; 4]], palette: [Color]) -> [[u8; 4]]",
        docs: "Snaps every pixel to the palette color nearest it in squared rgba distance, first on a tie.\n\nReturns the pixels untouched when the palette is empty.",
        uncallable: None,
        door: Some(door_core_colors_snap),
    },
    Function {
        name: "core.colors.to_hex",
        signature: "(self: Color) -> String",
        docs: "Formats the color as lowercase hex, appending the alpha pair only when not opaque.",
        uncallable: None,
        door: Some(door_core_colors_to_hex),
    },
    Function {
        name: "core.error.parse",
        signature: "(text: str) -> Json",
        docs: "Parses JSON text into a value.\n\n# Errors\n\nErrs when the text is not valid json, naming where it broke.",
        uncallable: None,
        door: Some(door_core_error_parse),
    },
    Function {
        name: "core.hex_fit",
        signature: "(pixels: [[u8; 4]], width: usize, height: usize, vertical: bool, filter: core.Filter) -> (usize, usize, [[u8; 4]])",
        docs: "Squashes rgba pixels to the hex aspect, returning the new width, height and pixels.\n\n# Errors\n\nErrs on a side of zero or a pixels length that is not width by height.",
        uncallable: None,
        door: Some(door_core_hex_fit),
    },
    Function {
        name: "core.hex_size",
        signature: "(width: usize, height: usize, vertical: bool) -> (usize, usize)",
        docs: "Returns the size a hex rendering wears, the named axis squashed by the triangle ratio.",
        uncallable: None,
        door: Some(door_core_hex_size),
    },
    Function {
        name: "core.image.blur",
        signature: "(pixels: [[u8; 4]], width: usize, height: usize, radius: usize) -> [[u8; 4]]",
        docs: "Box-blurs rgba pixels by radius, each channel the mean of its edge-padded window.",
        uncallable: None,
        door: Some(door_core_image_blur),
    },
    Function {
        name: "core.image.new",
        signature: "(width: usize, height: usize, colors: [[u8; 4]]) -> Image",
        docs: "Builds an image from its width, its height and its colors.\n\n# Errors\n\nErrs when the colors do not count width by height.",
        uncallable: None,
        door: Some(door_core_image_new),
    },
    Function {
        name: "core.image.resample",
        signature: "(self: Image, width: usize, height: usize, filter: core.Filter) -> Image",
        docs: "Resamples the image to a new size.\n\n# Errors\n\nErrs on a side of zero, colors that do not count width by height, or a size that overflows.",
        uncallable: None,
        door: Some(door_core_image_resample),
    },
    Function {
        name: "core.image.scale",
        signature: "(self: Image, scale: usize) -> Image",
        docs: "Draws every pixel as a scale by scale block, growing both sides by scale.\n\n# Errors\n\nErrs on a scale below one, colors that do not count width by height, or a grown size that overflows.",
        uncallable: None,
        door: Some(door_core_image_scale),
    },
    Function {
        name: "core.name.sha.hex",
        signature: "(bytes: [u8]) -> String",
        docs: "Returns the sha256 digest of `bytes` as 64 lowercase hex digits.",
        uncallable: None,
        door: Some(door_core_name_sha_hex),
    },
    Function {
        name: "core.name.sha.sha256",
        signature: "(bytes: [u8]) -> [u8; 32]",
        docs: "Returns the 32-byte sha256 digest of `bytes`.",
        uncallable: None,
        door: Some(door_core_name_sha_sha256),
    },
    Function {
        name: "core.name.sha.short",
        signature: "(bytes: [u8]) -> String",
        docs: "Returns the first eight hex digits of the sha256 digest of `bytes`, the id a name prints.",
        uncallable: None,
        door: Some(door_core_name_sha_short),
    },
    Function {
        name: "core.paint.Config.default",
        signature: "() -> core.paint.Config",
        docs: "Returns the default Config.",
        uncallable: None,
        door: Some(door_core_paint_config_default),
    },
    Function {
        name: "core.paint.Edition.all",
        signature: "() -> [core.paint.Edition; 7]",
        docs: "Returns every Edition in canonical order.",
        uncallable: None,
        door: Some(door_core_paint_edition_all),
    },
    Function {
        name: "core.paint.Edition.mode",
        signature: "(self: core.paint.Edition) -> core.Mode?",
        docs: "Returns the cell-painting mode this edition renders with, or None for Random, which scatters.",
        uncallable: None,
        door: Some(door_core_paint_edition_mode),
    },
    Function {
        name: "core.paint.Ink.all",
        signature: "() -> [core.paint.Ink; 15]",
        docs: "Returns every Ink in canonical order.",
        uncallable: None,
        door: Some(door_core_paint_ink_all),
    },
    Function {
        name: "core.paint.Ink.color",
        signature: "(self: core.paint.Ink) -> Color",
        docs: "Returns the ink's color.",
        uncallable: None,
        door: Some(door_core_paint_ink_color),
    },
    Function {
        name: "core.paint.Paint.is_simple",
        signature: "(self: core.paint.Paint) -> bool",
        docs: "Returns true for the Simple edition.",
        uncallable: None,
        door: Some(door_core_paint_paint_is_simple),
    },
    Function {
        name: "core.paint.Paint.new",
        signature: "(edition: core.paint.Edition) -> core.paint.Paint",
        docs: "Builds a black-primary, fill-target, multicolor paint for an edition.",
        uncallable: None,
        door: Some(door_core_paint_paint_new),
    },
    Function {
        name: "core.paint.Scheme.all",
        signature: "() -> [core.paint.Scheme; 2]",
        docs: "Returns every Scheme in canonical order.",
        uncallable: None,
        door: Some(door_core_paint_scheme_all),
    },
    Function {
        name: "core.paint.Target.all",
        signature: "() -> [core.paint.Target; 2]",
        docs: "Returns every Target in canonical order.",
        uncallable: None,
        door: Some(door_core_paint_target_all),
    },
    Function {
        name: "core.paint.apply",
        signature: "(paint: core.paint.Paint, cell: Cell, rng: Rng(seed)) -> null",
        docs: "Colors the cell from the paint's inks under its edition mode, scattering the Random edition from the stream.\n\n# Errors\n\nErrs when a multitone paint carries no base color to shade.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "core.paint.coat",
        signature: "(cell: Cell, paint: core.paint.Paint, mask: Tensor?, rng: Rng(seed)) -> null",
        docs: "Replays a stored paint onto a cell, tagging first and applying it from the stream.\n\n# Errors\n\nErrs when the tagging or the coloring does.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "core.paint.paint",
        signature: "(cell: Cell, config: core.paint.Config, mask: Tensor?, rng: Rng(seed)) -> core.paint.Paint",
        docs: "Draws a random paint under the config, applies it to the cell, and returns the recipe.\n\n# Errors\n\nErrs when the tagging or the coloring does.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "core.paint.prime",
        signature: "(paint: core.paint.Paint, cell: Cell, mask: Tensor?, rng: Rng(seed)) -> core.paint.Paint",
        docs: "Tags the cell for Layers and Neighbors paints and sizes the palette to the tag count.\n\n# Errors\n\nErrs when the tagging does.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "core.paint.random_edition",
        signature: "(editions: [core.paint.Edition]?, rng: Rng(seed)) -> core.paint.Edition",
        docs: "Draws a random edition from the allowed list, or from all seven.",
        uncallable: None,
        door: Some(door_core_paint_random_edition),
    },
    Function {
        name: "core.paint.reroll",
        signature: "(paint: core.paint.Paint, rng: Rng(seed)) -> core.paint.Paint",
        docs: "Redraws the paint's secondary inks and shades under its scheme.",
        uncallable: None,
        door: Some(door_core_paint_reroll),
    },
    Function {
        name: "core.paint.setup",
        signature: "(paint: core.paint.Paint, config: core.paint.Config, rng: Rng(seed)) -> core.paint.Paint",
        docs: "Draws the paint's scheme, target and primary under the config, then rerolls the rest.",
        uncallable: None,
        door: Some(door_core_paint_setup),
    },
    Function {
        name: "core.paint.tag",
        signature: "(cell: Cell, edition: core.paint.Edition, target: core.paint.Target, mask: Tensor?) -> usize",
        docs: "Tags the cell for the Layers and Neighbors editions and returns the distinct tag count on the secondary side.\n\n# Errors\n\nErrs when the Neighbors mask does not fit the cell.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "core.ramp.color",
        signature: "(colorizer: core.Colorizer, value: usize, max: usize) -> Color",
        docs: "Returns the color for one value against the range maximum: the background at zero, the top of the ramp from the maximum up.",
        uncallable: None,
        door: Some(door_core_ramp_color),
    },
    Function {
        name: "core.ramp.colors",
        signature: "(colorizer: core.Colorizer, values: [usize], max: usize) -> [[u8; 4]]",
        docs: "Maps a slice of values to rgba pixels against the range maximum.",
        uncallable: None,
        door: Some(door_core_ramp_colors),
    },
    Function {
        name: "core.resample",
        signature: "(pixels: [[u8; 4]], width: usize, height: usize, out_w: usize, out_h: usize, filter: core.Filter) -> [[u8; 4]]",
        docs: "Resamples rgba pixels to a new size.\n\n# Errors\n\nErrs on a side of zero, a pixels length that is not width by height, or an output size that overflows.",
        uncallable: None,
        door: Some(door_core_resample),
    },
    Function {
        name: "core.rng.below",
        signature: "(self: Rng(seed), n: usize) -> usize",
        docs: "Draws an integer below n, or zero when n is zero.",
        uncallable: None,
        door: Some(door_core_rng_below),
    },
    Function {
        name: "core.rng.boolean",
        signature: "(self: Rng(seed)) -> bool",
        docs: "Draws a fair coin flip.",
        uncallable: None,
        door: Some(door_core_rng_boolean),
    },
    Function {
        name: "core.rng.chance",
        signature: "(self: Rng(seed), p: f64) -> bool",
        docs: "Returns true with probability p.",
        uncallable: None,
        door: Some(door_core_rng_chance),
    },
    Function {
        name: "core.rng.new",
        signature: "(seed: u64) -> Rng",
        docs: "Builds the stream from a seed.",
        uncallable: None,
        door: Some(door_core_rng_new),
    },
    Function {
        name: "core.rng.range",
        signature: "(self: Rng(seed), lo: i64, hi: i64) -> i64",
        docs: "Draws an integer between lo and hi inclusive, or lo when hi is not above lo.",
        uncallable: None,
        door: Some(door_core_rng_range),
    },
    Function {
        name: "core.rng.sample_indices",
        signature: "(self: Rng(seed), length: usize, amount: usize) -> [usize]",
        docs: "Draws amount distinct indices below length, or every index when amount is larger.",
        uncallable: None,
        door: Some(door_core_rng_sample_indices),
    },
    Function {
        name: "core.rng.unit",
        signature: "(self: Rng(seed)) -> f64",
        docs: "Draws a float at or above zero and below one.",
        uncallable: None,
        door: Some(door_core_rng_unit),
    },
    Function {
        name: "core.tensor.at",
        signature: "(self: Tensor, flat: usize) -> i64",
        docs: "Returns the element at a flat index, which must be below the size like a slice index.",
        uncallable: None,
        door: Some(door_core_tensor_at),
    },
    Function {
        name: "core.tensor.binarize",
        signature: "(self: Tensor, threshold: u8) -> Tensor",
        docs: "Maps every element to one at or above the threshold, zero below.",
        uncallable: None,
        door: Some(door_core_tensor_binarize),
    },
    Function {
        name: "core.tensor.binarize_otsu",
        signature: "(self: Tensor) -> Tensor",
        docs: "Binarizes at one above the Otsu threshold.",
        uncallable: None,
        door: Some(door_core_tensor_binarize_otsu),
    },
    Function {
        name: "core.tensor.blur",
        signature: "(self: Tensor, mask: Tensor, wrap: bool) -> Tensor",
        docs: "Averages every position over its masked neighborhood, rounded.\n\n# Errors\n\nErrs on a mask of another rank, an even side, or a mask with nothing on.",
        uncallable: None,
        door: Some(door_core_tensor_blur),
    },
    Function {
        name: "core.tensor.bytes",
        signature: "(self: Tensor) -> [u8]",
        docs: "Returns the elements as bytes.\n\n# Errors\n\nErrs when the tensor is wider than one byte.",
        uncallable: None,
        door: Some(door_core_tensor_bytes),
    },
    Function {
        name: "core.tensor.count",
        signature: "(self: Tensor, value: u8) -> usize",
        docs: "Counts the cells holding one value.",
        uncallable: None,
        door: Some(door_core_tensor_count),
    },
    Function {
        name: "core.tensor.dtype",
        signature: "(self: Tensor) -> core.Dtype",
        docs: "Returns the element width.",
        uncallable: None,
        door: Some(door_core_tensor_dtype),
    },
    Function {
        name: "core.tensor.exposed",
        signature: "(self: Tensor) -> u128",
        docs: "Counts the faces where filled cells meet empty cells or the boundary: perimeter in 2d, surface in 3d.",
        uncallable: None,
        door: Some(door_core_tensor_exposed),
    },
    Function {
        name: "core.tensor.filled",
        signature: "(shape: [usize], value: i64, dtype: core.Dtype) -> Tensor",
        docs: "Builds a tensor of the shape and width filled with one value.",
        uncallable: None,
        door: Some(door_core_tensor_filled),
    },
    Function {
        name: "core.tensor.flip",
        signature: "(self: Tensor, axis: usize) -> Tensor",
        docs: "Reverses the tensor along one axis.\n\n# Errors\n\nErrs when the axis is past the rank.",
        uncallable: None,
        door: Some(door_core_tensor_flip),
    },
    Function {
        name: "core.tensor.fractal",
        signature: "(self: Tensor, level: usize) -> Tensor",
        docs: "Folds the tensor into its level-fold Kronecker power.",
        uncallable: None,
        door: Some(door_core_tensor_fractal),
    },
    Function {
        name: "core.tensor.full",
        signature: "(shape: [usize], value: u8) -> Tensor",
        docs: "Builds a u8 tensor filled with one value.",
        uncallable: None,
        door: Some(door_core_tensor_full),
    },
    Function {
        name: "core.tensor.get",
        signature: "(self: Tensor, multi: [usize]) -> u8",
        docs: "Returns the byte at a multi-index.\n\n# Errors\n\nErrs when the index misses the shape or the tensor is wider than one byte.",
        uncallable: None,
        door: Some(door_core_tensor_get),
    },
    Function {
        name: "core.tensor.i32",
        signature: "(data: [i32], shape: [usize]) -> Tensor",
        docs: "Wraps an i32 vector as a tensor of the shape.\n\n# Errors\n\nErrs when the data length is not the shape's product.",
        uncallable: None,
        door: Some(door_core_tensor_i32),
    },
    Function {
        name: "core.tensor.i32s",
        signature: "(self: Tensor) -> [i32]",
        docs: "Returns the elements as i32s.\n\n# Errors\n\nErrs when the tensor is not four signed bytes wide.",
        uncallable: None,
        door: Some(door_core_tensor_i32s),
    },
    Function {
        name: "core.tensor.index",
        signature: "(self: Tensor, multi: [usize]) -> usize",
        docs: "Folds a multi-index into its flat index.",
        uncallable: None,
        door: Some(door_core_tensor_index),
    },
    Function {
        name: "core.tensor.invert",
        signature: "(self: Tensor) -> Tensor",
        docs: "Flips every element between zero and one.",
        uncallable: None,
        door: Some(door_core_tensor_invert),
    },
    Function {
        name: "core.tensor.kron",
        signature: "(self: Tensor, other: Tensor) -> Tensor",
        docs: "Builds the Kronecker product of the two tensors.",
        uncallable: None,
        door: Some(door_core_tensor_kron),
    },
    Function {
        name: "core.tensor.layers",
        signature: "(self: Tensor, dtype: core.Dtype) -> Tensor",
        docs: "Numbers every position by its concentric ring out from the center.",
        uncallable: None,
        door: Some(door_core_tensor_layers),
    },
    Function {
        name: "core.tensor.neighbors",
        signature: "(self: Tensor, mask: Tensor, target: u8, wrap: bool, dtype: core.Dtype) -> Tensor",
        docs: "Counts each position's masked neighbors holding the target bit.\n\n# Errors\n\nErrs on a mask of another rank or an even side, a target above one, or a count past the dtype.",
        uncallable: None,
        door: Some(door_core_tensor_neighbors),
    },
    Function {
        name: "core.tensor.new",
        signature: "(shape: [usize]) -> Tensor",
        docs: "Builds a zeroed u8 tensor of the shape.",
        uncallable: None,
        door: Some(door_core_tensor_new),
    },
    Function {
        name: "core.tensor.of",
        signature: "(data: [u8], shape: [usize]) -> Tensor",
        docs: "Wraps a byte vector as a tensor of the shape.\n\n# Errors\n\nErrs when the data length is not the shape's product.",
        uncallable: None,
        door: Some(door_core_tensor_of),
    },
    Function {
        name: "core.tensor.otsu_threshold",
        signature: "(self: Tensor) -> u8",
        docs: "Returns the Otsu threshold splitting the histogram at greatest variance.",
        uncallable: None,
        door: Some(door_core_tensor_otsu_threshold),
    },
    Function {
        name: "core.tensor.pad",
        signature: "(self: Tensor, count: usize, value: u8) -> Tensor",
        docs: "Wraps the tensor in a count-thick border of one value.",
        uncallable: None,
        door: Some(door_core_tensor_pad),
    },
    Function {
        name: "core.tensor.perforate",
        signature: "(self: Tensor, mask: Tensor, value: u8) -> Tensor",
        docs: "Stamps the value wherever the tiled mask is nonzero.\n\n# Errors\n\nErrs on a mask of another rank or a side that does not evenly tile the tensor.",
        uncallable: None,
        door: Some(door_core_tensor_perforate),
    },
    Function {
        name: "core.tensor.put",
        signature: "(self: Tensor, flat: usize, value: i64) -> null",
        docs: "Writes the element at a flat index, which must be below the size like a slice index.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "core.tensor.rot90",
        signature: "(self: Tensor, k: usize, axes: (usize, usize)) -> Tensor",
        docs: "Rotates the tensor k quarter turns in the plane of two axes.\n\n# Errors\n\nErrs when the axes are not two distinct axes of the tensor.",
        uncallable: None,
        door: Some(door_core_tensor_rot90),
    },
    Function {
        name: "core.tensor.set",
        signature: "(self: Tensor, multi: [usize], value: u8) -> null",
        docs: "Writes the byte at a multi-index.\n\n# Errors\n\nErrs when the index misses the shape or the tensor is wider than one byte.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "core.tensor.size",
        signature: "(self: Tensor) -> usize",
        docs: "Returns the number of elements.",
        uncallable: None,
        door: Some(door_core_tensor_size),
    },
    Function {
        name: "core.tensor.slice",
        signature: "(self: Tensor, axis: usize, index: usize) -> Tensor",
        docs: "Drops one axis by fixing it at an index.\n\n# Errors\n\nErrs when the axis is past the rank or the index is past the axis.",
        uncallable: None,
        door: Some(door_core_tensor_slice),
    },
    Function {
        name: "core.tensor.sum",
        signature: "(self: Tensor) -> u64",
        docs: "Returns the sum of all elements.",
        uncallable: None,
        door: Some(door_core_tensor_sum),
    },
    Function {
        name: "core.tensor.tile",
        signature: "(self: Tensor, reps: [usize]) -> Tensor",
        docs: "Repeats the tensor the given number of times along each axis.\n\n# Errors\n\nErrs without one count per axis.",
        uncallable: None,
        door: Some(door_core_tensor_tile),
    },
    Function {
        name: "core.tensor.transpose",
        signature: "(self: Tensor, a: usize, b: usize) -> Tensor",
        docs: "Swaps two axes.\n\n# Errors\n\nErrs when either axis is past the rank.",
        uncallable: None,
        door: Some(door_core_tensor_transpose),
    },
    Function {
        name: "core.tensor.typed",
        signature: "(shape: [usize], dtype: core.Dtype) -> Tensor",
        docs: "Builds a zeroed tensor of the shape and width.",
        uncallable: None,
        door: Some(door_core_tensor_typed),
    },
    Function {
        name: "core.tensor.u16",
        signature: "(data: [u16], shape: [usize]) -> Tensor",
        docs: "Wraps a u16 vector as a tensor of the shape.\n\n# Errors\n\nErrs when the data length is not the shape's product.",
        uncallable: None,
        door: Some(door_core_tensor_u16),
    },
    Function {
        name: "core.tensor.u16s",
        signature: "(self: Tensor) -> [u16]",
        docs: "Returns the elements as u16s.\n\n# Errors\n\nErrs when the tensor is not two bytes wide.",
        uncallable: None,
        door: Some(door_core_tensor_u16s),
    },
    Function {
        name: "core.tensor.u32",
        signature: "(data: [u32], shape: [usize]) -> Tensor",
        docs: "Wraps a u32 vector as a tensor of the shape.\n\n# Errors\n\nErrs when the data length is not the shape's product.",
        uncallable: None,
        door: Some(door_core_tensor_u32),
    },
    Function {
        name: "core.tensor.u32s",
        signature: "(self: Tensor) -> [u32]",
        docs: "Returns the elements as u32s.\n\n# Errors\n\nErrs when the tensor is not four unsigned bytes wide.",
        uncallable: None,
        door: Some(door_core_tensor_u32s),
    },
    Function {
        name: "core.tensor.u8",
        signature: "(data: [u8], shape: [usize]) -> Tensor",
        docs: "Wraps a u8 vector as a tensor of the shape, the same door as `Tensor::of`.\n\n# Errors\n\nErrs when the data length is not the shape's product.",
        uncallable: None,
        door: Some(door_core_tensor_u8),
    },
    Function {
        name: "font.Glyph.height",
        signature: "(self: font.Glyph) -> usize",
        docs: "Returns the number of rows.",
        uncallable: None,
        door: Some(door_font_glyph_height),
    },
    Function {
        name: "font.Glyph.new",
        signature: "(char: char, rows: [String]) -> font.Glyph",
        docs: "Builds a glyph from its character and rows.",
        uncallable: None,
        door: Some(door_font_glyph_new),
    },
    Function {
        name: "font.Glyph.width",
        signature: "(self: font.Glyph) -> usize",
        docs: "Returns the cell width of the first row, or 0 for an empty glyph.",
        uncallable: None,
        door: Some(door_font_glyph_width),
    },
    Function {
        name: "font.all",
        signature: "() -> [font.Glyph]",
        docs: "Builds every glyph in font order: uppers, lowers, digits, extras, specials.",
        uncallable: None,
        door: Some(door_font_all),
    },
    Function {
        name: "font.animate",
        signature: "(text: str, pad: usize) -> font.Anim",
        docs: "Writes the text in stroke order, one cell per frame, from an empty padded board to the full raster.",
        uncallable: None,
        door: Some(door_font_animate),
    },
    Function {
        name: "font.cycle",
        signature: "(write: font.Anim, merge: [[usize]], hold: usize) -> font.Anim",
        docs: "Chains the write, the merge and their reversals into one loop, resting hold frames after each movement that has any.",
        uncallable: None,
        door: Some(door_font_cycle),
    },
    Function {
        name: "font.digits",
        signature: "() -> [font.Glyph]",
        docs: "Builds the ten digit glyphs.",
        uncallable: None,
        door: Some(door_font_digits),
    },
    Function {
        name: "font.draft",
        signature: "(rows: [String]) -> [[(usize, usize)]]",
        docs: "Drafts a stroke order for a trimmed bitmap by walking its lit cells: start at a lowest-left free end, keep heading, lift when stuck.",
        uncallable: None,
        door: Some(door_font_draft),
    },
    Function {
        name: "font.extras",
        signature: "() -> [font.Glyph]",
        docs: "Builds the punctuation, symbol and arrow glyphs.",
        uncallable: None,
        door: Some(door_font_extras),
    },
    Function {
        name: "font.floor",
        signature: "(rows: [String]) -> usize",
        docs: "Returns the least strokes that can write a trimmed bitmap: the minimum cover of its lit cells by 4-adjacent paths, zero for a blank.",
        uncallable: None,
        door: Some(door_font_floor),
    },
    Function {
        name: "font.glyph",
        signature: "(c: char) -> font.Glyph?",
        docs: "Returns an owned copy of the character's glyph, or None outside the font.",
        uncallable: None,
        door: Some(door_font_glyph),
    },
    Function {
        name: "font.lower",
        signature: "(rows: [str]) -> [String]",
        docs: "Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form.",
        uncallable: None,
        door: Some(door_font_lower),
    },
    Function {
        name: "font.lowers",
        signature: "() -> [font.Glyph]",
        docs: "Builds the twenty-six lowercase glyphs by rounding the uppers' corners.",
        uncallable: None,
        door: Some(door_font_lowers),
    },
    Function {
        name: "font.map",
        signature: "() -> {char: [String]}",
        docs: "Returns the whole font as a map from character to bitmap rows.",
        uncallable: None,
        door: Some(door_font_map),
    },
    Function {
        name: "font.merge",
        signature: "(text: str, pad: usize) -> [[usize]]",
        docs: "Folds the written text's glyphs, frame by frame, into one centered stack.",
        uncallable: None,
        door: Some(door_font_merge),
    },
    Function {
        name: "font.name_of",
        signature: "(c: char) -> String",
        docs: "Returns the character's Unicode name, or a U+ code point label for a character outside the font.",
        uncallable: None,
        door: Some(door_font_name_of),
    },
    Function {
        name: "font.path",
        signature: "(c: char) -> [(usize, usize)]",
        docs: "Flattens the character's strokes into one cell-by-cell drawing order.",
        uncallable: None,
        door: Some(door_font_path),
    },
    Function {
        name: "font.paths.penned",
        signature: "(c: char) -> [[(usize, usize)]]?",
        docs: "Returns the character's hand-penned strokes from the pen tables, or None outside the font.",
        uncallable: None,
        door: Some(door_font_paths_penned),
    },
    Function {
        name: "font.pens.all",
        signature: "() -> [(char, [String])]",
        docs: "Returns every pen in font order: uppers, lowers, digits, extras, specials.",
        uncallable: None,
        door: Some(door_font_pens_all),
    },
    Function {
        name: "font.raster",
        signature: "(text: str) -> [[u8]]",
        docs: "Returns the text as a 0/1 grid, its trimmed glyphs one blank column apart.",
        uncallable: None,
        door: Some(door_font_raster),
    },
    Function {
        name: "font.specials",
        signature: "() -> [font.Glyph]",
        docs: "Builds the four seven-row glyphs: dollar, at, copyright and registered.",
        uncallable: None,
        door: Some(door_font_specials),
    },
    Function {
        name: "font.strokes",
        signature: "(c: char) -> [[(usize, usize)]]",
        docs: "Returns the character's ordered strokes over its trimmed bitmap, or none for a character outside the font.",
        uncallable: None,
        door: Some(door_font_strokes),
    },
    Function {
        name: "font.supported",
        signature: "() -> [char]",
        docs: "Returns every character in the font, in font order.",
        uncallable: None,
        door: Some(door_font_supported),
    },
    Function {
        name: "font.trim",
        signature: "(rows: [String]) -> [String]",
        docs: "Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column; a row shorter than the cut keeps what it has.",
        uncallable: None,
        door: Some(door_font_trim),
    },
    Function {
        name: "font.uppers",
        signature: "() -> [font.Glyph]",
        docs: "Builds the twenty-six uppercase glyphs.",
        uncallable: None,
        door: Some(door_font_uppers),
    },
    Function {
        name: "life.Boundary.all",
        signature: "() -> [life.Boundary; 2]",
        docs: "Returns every Boundary in canonical order.",
        uncallable: None,
        door: Some(door_life_boundary_all),
    },
    Function {
        name: "life.Boundary.wrap",
        signature: "(self: life.Boundary) -> bool",
        docs: "Returns whether the edges wrap.",
        uncallable: None,
        door: Some(door_life_boundary_wrap),
    },
    Function {
        name: "life.Config.budget",
        signature: "(self: life.Config) -> usize",
        docs: "Returns the largest neighbor count the mask can reach.",
        uncallable: None,
        door: Some(door_life_config_budget),
    },
    Function {
        name: "life.Config.counts",
        signature: "(self: life.Config) -> ([usize], [usize])",
        docs: "Resolves the birth and survive counts against the mask's budget.\n\n# Errors\n\nErrs when a drawn side's sequence will not build inside the budget.",
        uncallable: None,
        door: Some(door_life_config_counts),
    },
    Function {
        name: "life.Config.new",
        signature: "(mask: Cell2d, birth: life.Counts, survive: life.Counts) -> life.Config",
        docs: "Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding.",
        uncallable: None,
        door: Some(door_life_config_new),
    },
    Function {
        name: "life.Counts.drawn",
        signature: "(seq: life.Source, zeros: bool, ones: bool) -> life.Counts",
        docs: "Builds the counts a sequence lays down, keeping zeros and ones on request.",
        uncallable: None,
        door: Some(door_life_counts_drawn),
    },
    Function {
        name: "life.Counts.list",
        signature: "(counts: [usize]) -> life.Counts",
        docs: "Spells the counts outright.",
        uncallable: None,
        door: Some(door_life_counts_list),
    },
    Function {
        name: "life.Counts.values",
        signature: "(self: life.Counts, budget: usize) -> [usize]",
        docs: "Returns the counts, a drawn side resolved against the mask's neighbor budget.\n\n# Errors\n\nErrs when a drawn side's sequence will not build inside the budget.",
        uncallable: None,
        door: Some(door_life_counts_values),
    },
    Function {
        name: "life.Fate.all",
        signature: "() -> [life.Fate; 4]",
        docs: "Returns every Fate in canonical order.",
        uncallable: None,
        door: Some(door_life_fate_all),
    },
    Function {
        name: "life.Life.last",
        signature: "(self: life.Life) -> Cell2d?",
        docs: "Returns the final grid, or None when the run is empty.",
        uncallable: None,
        door: Some(door_life_life_last),
    },
    Function {
        name: "life.Rule.boundary",
        signature: "(self: life.Rule) -> life.Boundary",
        docs: "Returns the edge policy the rule runs under.",
        uncallable: None,
        door: Some(door_life_rule_boundary),
    },
    Function {
        name: "life.Rule.checked",
        signature: "(self: life.Rule) -> life.Rule",
        docs: "Folds a decoded value to its canonical form, or an error for one outside the kind.",
        uncallable: None,
        door: Some(door_life_rule_checked),
    },
    Function {
        name: "life.Rule.config",
        signature: "(self: life.Rule, mask: Cell2d) -> life.Config",
        docs: "Builds a life config running this rule over a neighborhood mask.",
        uncallable: None,
        door: Some(door_life_rule_config),
    },
    Function {
        name: "life.Rule.from_file",
        signature: "(text: str) -> life.Rule",
        docs: "Reads a filename back into the value, or an error.",
        uncallable: None,
        door: Some(door_life_rule_from_file),
    },
    Function {
        name: "life.Rule.from_json",
        signature: "(text: str) -> life.Rule",
        docs: "Reads a JSON object into its canonical value, or an error naming the broken key.",
        uncallable: None,
        door: Some(door_life_rule_from_json),
    },
    Function {
        name: "life.Rule.from_url",
        signature: "(text: str) -> life.Rule",
        docs: "Reads a path and query string back into the value, or an error.",
        uncallable: None,
        door: Some(door_life_rule_from_url),
    },
    Function {
        name: "life.Rule.new",
        signature: "(birth: life.Counts, survive: life.Counts, wrap: bool) -> life.Rule",
        docs: "Builds a rule from its counts and edge policy, listed counts folded to a sorted set.",
        uncallable: None,
        door: Some(door_life_rule_new),
    },
    Function {
        name: "life.Rule.of",
        signature: "(config: life.Config) -> life.Rule",
        docs: "Reads the rule out of a life config.",
        uncallable: None,
        door: Some(door_life_rule_of),
    },
    Function {
        name: "life.Rule.to_file",
        signature: "(self: life.Rule) -> String",
        docs: "Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_life_rule_to_file),
    },
    Function {
        name: "life.Rule.to_id",
        signature: "(self: life.Rule) -> String",
        docs: "Prints the first eight hex digits of the sha256 of the canonical JSON.",
        uncallable: None,
        door: Some(door_life_rule_to_id),
    },
    Function {
        name: "life.Rule.to_json",
        signature: "(self: life.Rule) -> String",
        docs: "Prints the canonical JSON object.",
        uncallable: None,
        door: Some(door_life_rule_to_json),
    },
    Function {
        name: "life.Rule.to_prose",
        signature: "(self: life.Rule) -> String",
        docs: "Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_life_rule_to_prose),
    },
    Function {
        name: "life.Rule.to_url",
        signature: "(self: life.Rule) -> String",
        docs: "Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_life_rule_to_url),
    },
    Function {
        name: "life.Source.all",
        signature: "() -> [life.Source; 23]",
        docs: "Returns every fixed sequence, the seeded and coded families excluded.",
        uncallable: None,
        door: Some(door_life_source_all),
    },
    Function {
        name: "life.Source.designs",
        signature: "() -> [life.Source; 17]",
        docs: "Returns the seventeen mrly design families: the grid, the four classics and their antis.",
        uncallable: None,
        door: Some(door_life_source_designs),
    },
    Function {
        name: "life.Source.is_random",
        signature: "(self: life.Source) -> bool",
        docs: "Returns whether the sequence is a seeded random draw.",
        uncallable: None,
        door: Some(door_life_source_is_random),
    },
    Function {
        name: "life.Source.name",
        signature: "(self: life.Source) -> String",
        docs: "Returns the sequence's parseable name, the one string that regenerates it.",
        uncallable: None,
        door: Some(door_life_source_name),
    },
    Function {
        name: "life.Source.numbers",
        signature: "() -> [life.Source; 6]",
        docs: "Returns the six number sequences, the random one listed under seed zero.",
        uncallable: None,
        door: Some(door_life_source_numbers),
    },
    Function {
        name: "life.Source.oeis",
        signature: "(self: life.Source) -> String?",
        docs: "Returns the sequence's OEIS id, or None off the encyclopedia.",
        uncallable: None,
        door: Some(door_life_source_oeis),
    },
    Function {
        name: "life.Source.parse",
        signature: "(name: str) -> life.Source",
        docs: "Parses a sequence name back to its source.\n\n# Errors\n\nErrs when the name is not a known sequence, or a code family carries no plain number.",
        uncallable: None,
        door: Some(door_life_source_parse),
    },
    Function {
        name: "life.Source.read",
        signature: "(text: str) -> (life.Source, str)?",
        docs: "Reads a canonical name off the front of the text, returning the tail left over.",
        uncallable: None,
        door: Some(door_life_source_read),
    },
    Function {
        name: "life.affine",
        signature: "(rule: u8) -> bool",
        docs: "Returns whether a rule is affine, its algebraic degree at most one.",
        uncallable: None,
        door: Some(door_life_affine),
    },
    Function {
        name: "life.animate",
        signature: "(seed: Cell2d, config: life.Config) -> life.Life",
        docs: "Runs a seed under a config until it fixes, loops or times out, recording every generation.\n\n# Errors\n\nErrs when the mask will not step the grid, or the drawn counts will not resolve.",
        uncallable: None,
        door: Some(door_life_animate),
    },
    Function {
        name: "life.churn",
        signature: "(grids: [Cell2d]) -> f64",
        docs: "Returns the mean fraction of sites changed between consecutive grids.",
        uncallable: None,
        door: Some(door_life_churn),
    },
    Function {
        name: "life.corner_bits",
        signature: "(rule: u8) -> [u8]",
        docs: "Returns the eight output bits of a rule, corner `i` at index `i = 4 x0 + 2 x1 + x2`.",
        uncallable: None,
        door: Some(door_life_corner_bits),
    },
    Function {
        name: "life.counts",
        signature: "(seq: life.Source, max_neighbors: usize, include_zeros: bool, include_ones: bool) -> [usize]",
        docs: "Returns the sequence up to max_neighbors, keeping zeros and ones only on request.\n\n# Errors\n\nErrs when a design behind the sequence will not build or count.",
        uncallable: None,
        door: Some(door_life_counts),
    },
    Function {
        name: "life.crop",
        signature: "(grids: [Cell2d]) -> [Cell2d]",
        docs: "Crops a frame sequence to the centred square bounding every cell ever alive.\n\n# Errors\n\nErrs when the frames do not share one shape.",
        uncallable: None,
        door: Some(door_life_crop),
    },
    Function {
        name: "life.cube_orbit",
        signature: "(rule: u8) -> [u8]",
        docs: "Returns the rules a rule reaches under the signed axis permutations of the cube, in ascending order.",
        uncallable: None,
        door: Some(door_life_cube_orbit),
    },
    Function {
        name: "life.design_mask",
        signature: "(dimension: usize, code: Code, number: usize, level: usize) -> Tensor",
        docs: "Builds the base-2 design mask a code names at an odd side grown to the given Kronecker\nlevel, its centre popped.\n\n# Errors\n\nErrs when the dimension is not 1 or 2, the side is even, the level is zero, or the side\noverflows a usize.",
        uncallable: None,
        door: Some(door_life_design_mask),
    },
    Function {
        name: "life.elementary.output",
        signature: "(rule: u8, l: u8, c: u8, r: u8) -> u8",
        docs: "Returns the bit a rule sends the neighbourhood to, reading bit `4l + 2c + r` in Wolfram's numbering off the low bit of each cell.",
        uncallable: None,
        door: Some(door_life_elementary_output),
    },
    Function {
        name: "life.entropy",
        signature: "(grid: Cell2d) -> i64",
        docs: "Returns the grid's binary Shannon entropy in millibits.",
        uncallable: None,
        door: Some(door_life_entropy),
    },
    Function {
        name: "life.frames",
        signature: "(grids: [Cell2d], scale: usize) -> [Image]",
        docs: "Renders grids to white-on-black images at a pixel scale.\n\n# Errors\n\nErrs on a scale below one.",
        uncallable: None,
        door: Some(door_life_frames),
    },
    Function {
        name: "life.gasket",
        signature: "(rule: u8) -> String?",
        docs: "Returns the base-2 plane design a rule's single seed draws, or None when it draws none.",
        uncallable: None,
        door: Some(door_life_gasket),
    },
    Function {
        name: "life.genus",
        signature: "(rule: u8) -> String",
        docs: "Returns the genus of a rule's cube class: `iso` when it meets a level set, `axis` when it meets an axis-pinned block, else `comp`.",
        uncallable: None,
        door: Some(door_life_genus),
    },
    Function {
        name: "life.heatmap",
        signature: "(grids: [Cell2d], scale: usize) -> [Image]",
        docs: "Renders a whole run's cumulative-visit heatmap frames with the heat ramp.\n\n# Errors\n\nErrs when the grids differ in shape, or on a scale below one.",
        uncallable: None,
        door: Some(door_life_heatmap),
    },
    Function {
        name: "life.history",
        signature: "(row: [u8], rule: u8, steps: usize, wrap: bool) -> Tensor",
        docs: "Returns the space-time diagram of a seed row, row 0 the seed and then one row per generation.\n\n# Errors\n\nErrs when the row holds a value above one.",
        uncallable: None,
        door: Some(door_life_history),
    },
    Function {
        name: "life.lambda",
        signature: "(rule: u8) -> f64",
        docs: "Returns Langton's lambda, the popcount over eight.",
        uncallable: None,
        door: Some(door_life_lambda),
    },
    Function {
        name: "life.lattice_index",
        signature: "(mask: Tensor) -> usize",
        docs: "Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension.",
        uncallable: None,
        door: Some(door_life_lattice_index),
    },
    Function {
        name: "life.mask_offsets",
        signature: "(mask: Tensor) -> [[i64]]",
        docs: "Returns the offsets a mask's filled sites take from its centre, the centre itself dropped.",
        uncallable: None,
        door: Some(door_life_mask_offsets),
    },
    Function {
        name: "life.moore",
        signature: "() -> Cell2d",
        docs: "Builds the 3 by 3 Moore mask, every site on but the center.\n\n# Errors\n\nErrs when the mask tensor will not make a flat cell.",
        uncallable: None,
        door: Some(door_life_moore),
    },
    Function {
        name: "life.next_grid",
        signature: "(cell: Cell2d, birth: [usize], survive: [usize], mask: Tensor, boundary: life.Boundary) -> Cell2d",
        docs: "Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary.\n\n# Errors\n\nErrs when the mask will not count the grid's neighbors, an even side for one.",
        uncallable: None,
        door: Some(door_life_next_grid),
    },
    Function {
        name: "life.npn_class",
        signature: "(rule: u8) -> [u8]",
        docs: "Returns the rules a rule reaches under the cube group together with the output complement, its NPN class, in ascending order.",
        uncallable: None,
        door: Some(door_life_npn_class),
    },
    Function {
        name: "life.outer_totalistic",
        signature: "(rule: u8) -> ([usize], [usize])?",
        docs: "Returns the birth and survive counts of a rule read outer-totalistically on its two outer cells, or None when it does not read them by count alone.",
        uncallable: None,
        door: Some(door_life_outer_totalistic),
    },
    Function {
        name: "life.popcount",
        signature: "(rule: u8) -> u32",
        docs: "Returns the count of neighbourhoods a rule sends to one.",
        uncallable: None,
        door: Some(door_life_popcount),
    },
    Function {
        name: "life.render.frame",
        signature: "(grid: Cell2d, scale: usize) -> Image",
        docs: "Renders one grid to a white-on-black image at a pixel scale.\n\n# Errors\n\nErrs on a scale below one.",
        uncallable: None,
        door: Some(door_life_render_frame),
    },
    Function {
        name: "life.reversible",
        signature: "(rule: u8) -> bool",
        docs: "Returns whether a rule is reversible, by the pair graph on the de Bruijn nodes pruned to its bi-infinite core.",
        uncallable: None,
        door: Some(door_life_reversible),
    },
    Function {
        name: "life.rule_degree",
        signature: "(rule: u8) -> i32",
        docs: "Returns the GF(2) algebraic degree of a rule, minus one for the zero rule.",
        uncallable: None,
        door: Some(door_life_rule_degree),
    },
    Function {
        name: "life.rule_name",
        signature: "(rule: u8) -> String",
        docs: "Returns the design name a rule carries, `bang dim 3, code <rule>`.\n\n# Errors\n\nErrs when the name will not print as a line of prose.",
        uncallable: None,
        door: Some(door_life_rule_name),
    },
    Function {
        name: "life.single_seed",
        signature: "(rule: u8, steps: usize) -> Tensor",
        docs: "Returns the single-seed diagram: one live cell run the given generations on a line padded by `steps` cells beyond the `2 steps + 1` window on each side, cropped back to that window.\n\n# Errors\n\nErrs when the diagram's cells do not fill its shape.",
        uncallable: None,
        door: Some(door_life_single_seed),
    },
    Function {
        name: "life.source.sequence",
        signature: "(seq: life.Source, limit: usize) -> [usize]",
        docs: "Generates the sequence's values up to the limit.\n\n# Errors\n\nErrs when a design behind the sequence will not build or count.",
        uncallable: None,
        door: Some(door_life_source_sequence),
    },
    Function {
        name: "life.step",
        signature: "(row: [u8], rule: u8, wrap: bool) -> [u8]",
        docs: "Advances one row one generation, a constant-0 boundary unless the edges wrap.\n\n# Errors\n\nErrs when the row holds a value above one.",
        uncallable: None,
        door: Some(door_life_step),
    },
    Function {
        name: "life.surjective",
        signature: "(rule: u8) -> bool",
        docs: "Returns whether a rule is surjective on bi-infinite lines, by the de Bruijn subset walk from the full node set.",
        uncallable: None,
        door: Some(door_life_surjective),
    },
    Function {
        name: "life.tessellate",
        signature: "(grids: [Cell2d], min_canvas: usize) -> [Cell2d]",
        docs: "Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there.\n\n# Errors\n\nErrs when a frame will not tile to the canvas.",
        uncallable: None,
        door: Some(door_life_tessellate),
    },
    Function {
        name: "life.wolfram_class",
        signature: "(rule: u8) -> [u8]",
        docs: "Returns the rules a rule reaches under left-right reflection and conjugation, Wolfram's equivalence, in ascending order.",
        uncallable: None,
        door: Some(door_life_wolfram_class),
    },
    Function {
        name: "math.arcs.curves",
        signature: "(code: Code, base: usize, level: usize) -> [math.arcs.Curve]",
        docs: "Walks level `level` of `bang dim 2, base b, code c` in arcs into its curves: first the strands, from each boundary midpoint in turn, across edges before upright ones, then the loops, each from its first unwalked arc.\n\nThe corner of an arc is its cell's lower-left or upper-right for a filled cell, lower-right or upper-left for a deleted one, as in `draw`.\n\n# Errors\n\nErrors on a code outside the digit space of the base, or a side past the machine word.",
        uncallable: None,
        door: Some(door_math_arcs_curves),
    },
    Function {
        name: "math.arcs.draw",
        signature: "(code: Code, base: usize, level: usize) -> math.arcs.Arcs",
        docs: "Draws level `level` of `bang dim 2, base b, code c` in arcs: cell `(x, y)` of the `b x b` mask is filled when bit `b y + x` of the code is set, level `level` is its Kronecker power built by `crate::bang::factory::create`, and level 0 is one filled cell.\n\n# Errors\n\nErrors on a code outside the digit space of the base, or a side past the machine word.",
        uncallable: None,
        door: Some(door_math_arcs_draw),
    },
    Function {
        name: "math.arcs.law",
        signature: "(code: Code, base: usize, level: usize) -> math.arcs.Law?",
        docs: "Returns the proved loop law of `bang dim 2, base b, code c` at the level, or none where no law is proved: the carpet, base 3 code 495, has `(8^n - 1)/7 - 3^n + n + 1` loops; at base 2 codes 7 and 14 have `3^(n-1) - 2^n + 1`, codes 11 and 13 have `3^(n-1) - 2^(n-1)`, both from level 1 on, code 9 has `2^n - 1`, and the other eleven codes never loop.\n\n# Errors\n\nErrors on a code outside the digit space of the base, or a count past 64 bits.",
        uncallable: None,
        door: Some(door_math_arcs_law),
    },
    Function {
        name: "math.arcs.trace",
        signature: "(side: usize, on: [bool]) -> math.arcs.Arcs",
        docs: "Draws a `side x side` grid of filled and deleted cells in arcs and counts its curves by union-find over the edge midpoints, a curve being a loop when no midpoint of it lies on the boundary.\n\n# Errors\n\nErrors on side zero or a cell list that is not `side^2` long.",
        uncallable: None,
        door: Some(door_math_arcs_trace),
    },
    Function {
        name: "math.arrays.essential",
        signature: "(sensors: [u64]) -> [bool]",
        docs: "Returns whether each sensor is essential, in the order given: whether removing it deletes a lag from the coarray, which happens exactly when some lag `t != 0` has all its pairs through it, the one pair of a lag of weight `1` or the two pairs `(s, s - t)` and `(s + t, s)` of a lag of weight `2`.",
        uncallable: None,
        door: Some(door_math_arrays_essential),
    },
    Function {
        name: "math.arrays.fractal",
        signature: "(generator: [u64], base: u64, level: u32) -> [u64]",
        docs: "Returns the fractal array of the generator at the base and level, ascending: every `sum_(i < level) g_i base^i` with each digit `g_i` in the generator.\n\nThe generator is the digit set of a dim 1 digit design, its least digit `0`; at base `2a + 1`, `a` its largest digit, this is the fractal array of Cohen and Eldar, and every base above `a` gives `L^level` distinct sensors, `L` the size of the generator.\n\n# Errors\n\nErrs when the generator is empty, misses `0`, repeats a digit or holds one at or above the base, or when the array passes `MOST` sensors.",
        uncallable: None,
        door: Some(door_math_arrays_fractal),
    },
    Function {
        name: "math.arrays.holes",
        signature: "(sensors: [u64]) -> [i64]",
        docs: "Returns the holes of the coarray, ascending: the lags between `-span` and `span` that no pair of sensors makes.",
        uncallable: None,
        door: Some(door_math_arrays_holes),
    },
    Function {
        name: "math.arrays.law",
        signature: "(generator: [u64], level: u32) -> (u64, u64)?",
        docs: "Returns the essential count and the sensor count that Theorem A of the arrays note proves for the fractal array of the generator at base `2a + 1`, `(u^level, L^level)`, `u` the count of `paired` digits; `None` off the theorem: a level below 2, a generator without `0`, or a generator whose coarray has a hole.",
        uncallable: None,
        door: Some(door_math_arrays_law),
    },
    Function {
        name: "math.arrays.lost",
        signature: "(sensors: [u64], sensor: u64) -> [i64]",
        docs: "Returns the lags that removing the sensor deletes from the coarray, ascending: those every pair of which holds it; empty when the sensor is inessential or absent.",
        uncallable: None,
        door: Some(door_math_arrays_lost),
    },
    Function {
        name: "math.arrays.paired",
        signature: "(sensors: [u64]) -> [u64]",
        docs: "Returns the paired sensors, ascending: each `g` with a partner `h` whose lag `g - h` has weight `1`, the set `U` of condition C1 of Cohen and Eldar; a paired sensor is essential.",
        uncallable: None,
        door: Some(door_math_arrays_paired),
    },
    Function {
        name: "math.arrays.weights",
        signature: "(sensors: [u64]) -> [u64]",
        docs: "Returns the coarray weights of the sensors, `w(t)` for every lag `t` from `-span` to `span` at index `t + span`, `span` the largest sensor less the least: the count of ordered pairs `(p, q)` of sensors with `p - q = t`.",
        uncallable: None,
        door: Some(door_math_arrays_weights),
    },
    Function {
        name: "math.atoms.carpet_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n carpet, on where at most one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_carpet_2d),
    },
    Function {
        name: "math.atoms.carpet_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n carpet, on where at most one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_carpet_3d),
    },
    Function {
        name: "math.atoms.carpet_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a carpet of the given side at any rank, on where at most one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_carpet_nd),
    },
    Function {
        name: "math.atoms.dust_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n dust, on where both coordinates are even.",
        uncallable: None,
        door: Some(door_math_atoms_dust_2d),
    },
    Function {
        name: "math.atoms.dust_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n dust, on where all three coordinates are even.",
        uncallable: None,
        door: Some(door_math_atoms_dust_3d),
    },
    Function {
        name: "math.atoms.dust_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a dust of the given side at any rank, on where every coordinate is even.",
        uncallable: None,
        door: Some(door_math_atoms_dust_nd),
    },
    Function {
        name: "math.atoms.hline_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n line, free on axis 1, on along the odd rows.",
        uncallable: None,
        door: Some(door_math_atoms_hline_2d),
    },
    Function {
        name: "math.atoms.htree_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n tree, free on axis 1, on along the even rows.",
        uncallable: None,
        door: Some(door_math_atoms_htree_2d),
    },
    Function {
        name: "math.atoms.line_nd",
        signature: "(n: usize, rank: usize, axis: usize) -> Tensor",
        docs: "Builds a line of the given side at any rank, odd on every axis but the free one; an axis past the rank frees none.",
        uncallable: None,
        door: Some(door_math_atoms_line_nd),
    },
    Function {
        name: "math.atoms.net_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n net, on where at least one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_net_2d),
    },
    Function {
        name: "math.atoms.net_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n net, on where at least two coordinates are odd.",
        uncallable: None,
        door: Some(door_math_atoms_net_3d),
    },
    Function {
        name: "math.atoms.net_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a net of the given side at any rank, on where the odd coordinates plus one reach the rank.",
        uncallable: None,
        door: Some(door_math_atoms_net_nd),
    },
    Function {
        name: "math.atoms.noise_2d",
        signature: "(n: usize, density: f64, rng: Rng(seed)) -> Tensor",
        docs: "Builds an n by n tensor where each cell turns on with probability density, drawn from the stream.",
        uncallable: None,
        door: Some(door_math_atoms_noise_2d),
    },
    Function {
        name: "math.atoms.noise_3d",
        signature: "(n: usize, density: f64, rng: Rng(seed)) -> Tensor",
        docs: "Builds an n by n by n tensor where each cell turns on with probability density, drawn from the stream.",
        uncallable: None,
        door: Some(door_math_atoms_noise_3d),
    },
    Function {
        name: "math.atoms.ones_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n tensor of ones.",
        uncallable: None,
        door: Some(door_math_atoms_ones_2d),
    },
    Function {
        name: "math.atoms.ones_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tensor of ones.",
        uncallable: None,
        door: Some(door_math_atoms_ones_3d),
    },
    Function {
        name: "math.atoms.point_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n point, on where both coordinates are odd.",
        uncallable: None,
        door: Some(door_math_atoms_point_2d),
    },
    Function {
        name: "math.atoms.point_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n point, on where all three coordinates are odd.",
        uncallable: None,
        door: Some(door_math_atoms_point_3d),
    },
    Function {
        name: "math.atoms.point_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a point of the given side at any rank, on where every coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_point_nd),
    },
    Function {
        name: "math.atoms.star_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n star, on where exactly one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_star_2d),
    },
    Function {
        name: "math.atoms.star_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n star, on where exactly one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_star_3d),
    },
    Function {
        name: "math.atoms.star_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a star of the given side at any rank, on where exactly one coordinate is odd.",
        uncallable: None,
        door: Some(door_math_atoms_star_nd),
    },
    Function {
        name: "math.atoms.tree_nd",
        signature: "(n: usize, rank: usize, axis: usize) -> Tensor",
        docs: "Builds a tree of the given side at any rank, even on every axis but the free one; an axis past the rank frees none.",
        uncallable: None,
        door: Some(door_math_atoms_tree_nd),
    },
    Function {
        name: "math.atoms.vline_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n line, free on axis 0, on along the odd columns.",
        uncallable: None,
        door: Some(door_math_atoms_vline_2d),
    },
    Function {
        name: "math.atoms.void_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n void, on where both coordinates share one parity.",
        uncallable: None,
        door: Some(door_math_atoms_void_2d),
    },
    Function {
        name: "math.atoms.void_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n void, on where all three coordinates share one parity.",
        uncallable: None,
        door: Some(door_math_atoms_void_3d),
    },
    Function {
        name: "math.atoms.void_nd",
        signature: "(n: usize, rank: usize) -> Tensor",
        docs: "Builds a void of the given side at any rank, on where every coordinate shares one parity.",
        uncallable: None,
        door: Some(door_math_atoms_void_nd),
    },
    Function {
        name: "math.atoms.vtree_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n tree, free on axis 0, on along the even columns.",
        uncallable: None,
        door: Some(door_math_atoms_vtree_2d),
    },
    Function {
        name: "math.atoms.xline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n line, free on axis 0, its rods running along x.",
        uncallable: None,
        door: Some(door_math_atoms_xline_3d),
    },
    Function {
        name: "math.atoms.xtree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tree, free on axis 0, its beams running along x.",
        uncallable: None,
        door: Some(door_math_atoms_xtree_3d),
    },
    Function {
        name: "math.atoms.yline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n line, free on axis 1, its rods running along y.",
        uncallable: None,
        door: Some(door_math_atoms_yline_3d),
    },
    Function {
        name: "math.atoms.ytree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tree, free on axis 1, its beams running along y.",
        uncallable: None,
        door: Some(door_math_atoms_ytree_3d),
    },
    Function {
        name: "math.atoms.zeros_2d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n tensor of zeros.",
        uncallable: None,
        door: Some(door_math_atoms_zeros_2d),
    },
    Function {
        name: "math.atoms.zeros_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tensor of zeros.",
        uncallable: None,
        door: Some(door_math_atoms_zeros_3d),
    },
    Function {
        name: "math.atoms.zline_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n line, free on axis 2, its rods running along z.",
        uncallable: None,
        door: Some(door_math_atoms_zline_3d),
    },
    Function {
        name: "math.atoms.ztree_3d",
        signature: "(n: usize) -> Tensor",
        docs: "Builds an n by n by n tree, free on axis 2, its beams running along z.",
        uncallable: None,
        door: Some(door_math_atoms_ztree_3d),
    },
    Function {
        name: "math.automaton.Automaton.abscissa",
        signature: "(self: math.automaton.Automaton) -> f64",
        docs: "Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of `crate::memory::perron`.\n\nThe series converges absolutely on `Re s > alpha`. The bracket of `Automaton::perron` is a bound and not the root, so it is not read here.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_abscissa),
    },
    Function {
        name: "math.automaton.Automaton.base",
        signature: "(self: math.automaton.Automaton) -> u64",
        docs: "Returns the base `q = 2^D`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_base),
    },
    Function {
        name: "math.automaton.Automaton.cofactor",
        signature: "(self: math.automaton.Automaton, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to.\n\nThe scalar `1 - k q^(-s)` becomes the determinant, and the cofactor is carried as `det(I - q^(-s) T) D_(P-1)(s) + 1^T adj(I - q^(-s) T) N(s)` with `N` the ladder numerator, so it is read on the whole `m = 0` pole comb where `zeta_W` itself is singular.\n\n# Errors\n\nErrs when the tolerance is out of reach.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_cofactor),
    },
    Function {
        name: "math.automaton.Automaton.denominator",
        signature: "(self: math.automaton.Automaton) -> [f64]",
        docs: "Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`.\n\nThe roots in `x` are the reciprocals of the nonzero eigenvalues of `T`, so the poles of `zeta_W` sit at `s = log_q lambda_i - m + (arg lambda_i + 2 pi j) i / log q`: one comb per distinct eigenvalue, offset by the eigenvalue's argument.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_denominator),
    },
    Function {
        name: "math.automaton.Automaton.matrix",
        signature: "(self: math.automaton.Automaton) -> [[f64]]",
        docs: "Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of `crate::memory::transfer`, entry `(u', u)` counting the letters carrying `u` to `u'`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_matrix),
    },
    Function {
        name: "math.automaton.Automaton.new",
        signature: "(rule: math.memory.Rule) -> math.automaton.Automaton",
        docs: "Builds the ladder of a rule, choosing the peel depth.\n\n# Errors\n\nErrs when the rule accepts no element with a nonzero leading digit, or when its peel overruns the exact integers.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_new),
    },
    Function {
        name: "math.automaton.Automaton.peel",
        signature: "(self: math.automaton.Automaton) -> usize",
        docs: "Returns the peel depth `P`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_peel),
    },
    Function {
        name: "math.automaton.Automaton.period",
        signature: "(self: math.automaton.Automaton) -> f64",
        docs: "Returns the pole spacing `2 pi / log q`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_period),
    },
    Function {
        name: "math.automaton.Automaton.perron",
        signature: "(self: math.automaton.Automaton) -> (f64, f64)",
        docs: "Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with.\n\nFor any nonnegative `T` and any `v > 0`, `min_u (T v)_u / v_u <= rho <= max_u (T v)_u / v_u`, so the pair brackets the root with no primitivity hypothesis; the guide is `v = (I + T)^m 1` for `m` sixty, or fewer when the entries would overrun the exponent range.\nThe ends carry a relative `1e-14` allowance of the same measured kind as `ROUNDING` and are not a rounding-certified interval, and the bracket is loose exactly when `T` is defective: at code `13` and `k = 2`, a Jordan block of root `1`, the upper end is `64/62`. Only the upper end is load bearing, as the `theta < 1` test that opens the Neumann branch; the root itself is `crate::memory::perron`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_perron),
    },
    Function {
        name: "math.automaton.Automaton.residue",
        signature: "(self: math.automaton.Automaton, w0: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to.\n\nAt such a point the resolvent is `adj(I - x T) / det(I - x T)` with `x = q^(-w)`, so the residue is `1^T adj(I - x0 T) N(w0)` over `-x0 log q det'(x0)`: the adjugate is the spectral projector in polynomial form, and no eigenvector is solved for.\nThe head term `det(I - q^(-s) T) D_(P-1)(s)` is dropped, since the determinant vanishes at the pole.\n\n# Errors\n\nErrs when the tolerance is out of reach, or when `w0` is not a simple root of `det(I - q^(-w) T)`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_residue),
    },
    Function {
        name: "math.automaton.Automaton.rule",
        signature: "(self: math.automaton.Automaton) -> math.memory.Rule",
        docs: "Returns the rule.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_rule),
    },
    Function {
        name: "math.automaton.Automaton.states",
        signature: "(self: math.automaton.Automaton) -> usize",
        docs: "Returns the state count `q^(k-1)`.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_states),
    },
    Function {
        name: "math.automaton.Automaton.with_peel",
        signature: "(rule: math.memory.Rule, peel: usize) -> math.automaton.Automaton",
        docs: "Builds the ladder at an explicit peel depth, at least the rule width and at least two.\n\n# Errors\n\nErrs when the rule accepts no such element, and at a peel depth under the rule width or two, or past the exact integers.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_with_peel),
    },
    Function {
        name: "math.automaton.Automaton.zeta",
        signature: "(self: math.automaton.Automaton, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns `zeta_W(s)` and the bound it is known to.\n\nThe bound is the propagated truncation bound, carried entrywise as a nonnegative vector through `(I - q^(-w) T)^(-1)`, plus `ROUNDING` times the scale the ladder carries, which is a measured allowance and not a proof.\nRight of the abscissa the inverse is majorised by its Neumann series, `sum_i (q^(-Re w) T)^i`, which is entrywise nonnegative and needs no norm and no primitivity; left of it the bound runs through the computed inverse certified by its own residual, `abs(C) (y + norm(y) r/(1-r) 1)` with `r = norm(I - (I - q^(-w) T) C)`, and the module raises rather than return when `r >= 1`.\n\n# Errors\n\nErrs when the tolerance is out of reach, or when a level of the walk sits on a pole of the resolvent.",
        uncallable: None,
        door: Some(door_math_automaton_automaton_zeta),
    },
    Function {
        name: "math.bang.Design.anf",
        signature: "(self: math.bang.Design) -> String",
        docs: "Returns the design's algebraic normal form as a string.",
        uncallable: None,
        door: Some(door_math_bang_design_anf),
    },
    Function {
        name: "math.bang.Design.degree",
        signature: "(self: math.bang.Design) -> i32",
        docs: "Returns the design's algebraic degree, or -1 for the zero design.",
        uncallable: None,
        door: Some(door_math_bang_design_degree),
    },
    Function {
        name: "math.bang.Design.name",
        signature: "(self: math.bang.Design) -> String",
        docs: "Returns the design's name as a line of prose, `bang dim 2, code 7`.\n\n# Errors\n\nErrors when the design's name does not read back.",
        uncallable: None,
        door: Some(door_math_bang_design_name),
    },
    Function {
        name: "math.bang.Design.rule",
        signature: "(self: math.bang.Design) -> [[u8]]",
        docs: "Returns the design's filled corners in sorted order.",
        uncallable: None,
        door: Some(door_math_bang_design_rule),
    },
    Function {
        name: "math.bang.MagicLayer.new",
        signature: "(design: math.name.Bang, number: usize) -> math.bang.MagicLayer",
        docs: "Pins a design to the side number it renders at.",
        uncallable: None,
        door: Some(door_math_bang_magiclayer_new),
    },
    Function {
        name: "math.bang.Universe.all",
        signature: "(self: math.bang.Universe) -> [math.bang.Design]",
        docs: "Returns every design in code order.",
        uncallable: None,
        door: Some(door_math_bang_universe_all),
    },
    Function {
        name: "math.bang.Universe.canonical",
        signature: "(self: math.bang.Universe) -> [math.bang.Design]",
        docs: "Returns the designs whose codes lead their orbits.",
        uncallable: None,
        door: Some(door_math_bang_universe_canonical),
    },
    Function {
        name: "math.bang.Universe.design",
        signature: "(self: math.bang.Universe, code: Code) -> math.bang.Design",
        docs: "Returns the design at a code with its precomputed orbit facts.",
        uncallable: None,
        door: Some(door_math_bang_universe_design),
    },
    Function {
        name: "math.bang.Universe.distinct",
        signature: "(self: math.bang.Universe) -> usize",
        docs: "Returns the number of distinct orbits.",
        uncallable: None,
        door: Some(door_math_bang_universe_distinct),
    },
    Function {
        name: "math.bang.Universe.new",
        signature: "(dimension: usize) -> math.bang.Universe",
        docs: "Enumerates every orbit of a dimension from 1 to 4.\n\n# Errors\n\nErrors outside dimensions one to four.",
        uncallable: None,
        door: Some(door_math_bang_universe_new),
    },
    Function {
        name: "math.bang.bang",
        signature: "(dimension: usize) -> math.bang.Universe",
        docs: "Builds the universe of a dimension.\n\n# Errors\n\nErrors outside dimensions one to four.",
        uncallable: None,
        door: Some(door_math_bang_bang),
    },
    Function {
        name: "math.bang.baseq.axis_maps",
        signature: "(base: usize) -> [[usize]]",
        docs: "Returns the distinct rotation and reflection maps of a base-q axis.",
        uncallable: None,
        door: Some(door_math_bang_baseq_axis_maps),
    },
    Function {
        name: "math.bang.baseq.bracelets",
        signature: "(max_base: usize) -> [u128]",
        docs: "Returns the distinct one-dimensional design counts for bases 1 through max_base.\n\n# Errors\n\nErrors when a Burnside average is not whole.",
        uncallable: None,
        door: Some(door_math_bang_baseq_bracelets),
    },
    Function {
        name: "math.bang.baseq.canonical",
        signature: "(group: [[usize]], code: Code) -> Code",
        docs: "Returns the least code of the design's orbit.\n\n# Errors\n\nErrors on an empty group.",
        uncallable: None,
        door: Some(door_math_bang_baseq_canonical),
    },
    Function {
        name: "math.bang.baseq.carry",
        signature: "(element: [usize], code: Code) -> Code",
        docs: "Carries a code through one group element.",
        uncallable: None,
        door: Some(door_math_bang_baseq_carry),
    },
    Function {
        name: "math.bang.baseq.class_sequence",
        signature: "(max_dimension: usize) -> [u128]",
        docs: "Returns the fill-class counts for dimensions 1 through max_dimension.",
        uncallable: None,
        door: Some(door_math_bang_baseq_class_sequence),
    },
    Function {
        name: "math.bang.baseq.classes",
        signature: "(dimension: usize) -> u128",
        docs: "Counts the fill classes of a dimension, the popcount profiles a base-2 design can have: one more than the corners of each weight, multiplied over the weights, A129824 at the dimension.",
        uncallable: None,
        door: Some(door_math_bang_baseq_classes),
    },
    Function {
        name: "math.bang.baseq.distinct_designs",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "Counts base-q designs distinct under symmetry.\n\n# Errors\n\nErrors below base one or dimension one, or when the Burnside average is not whole.",
        uncallable: None,
        door: Some(door_math_bang_baseq_distinct_designs),
    },
    Function {
        name: "math.bang.baseq.even_fill_is_balanced",
        signature: "(number: usize, dimension: usize, popcount: u128) -> u128",
        docs: "Returns the collapsed fill count at an even side number.\n\n# Errors\n\nErrors at an odd side number.",
        uncallable: None,
        door: Some(door_math_bang_baseq_even_fill_is_balanced),
    },
    Function {
        name: "math.bang.baseq.fill_from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize) -> u128",
        docs: "Returns the filled-cell count of a binary design at a side number, folded from its filled corners.",
        uncallable: None,
        door: Some(door_math_bang_baseq_fill_from_corners),
    },
    Function {
        name: "math.bang.baseq.group",
        signature: "(base: usize, dimension: usize) -> [[usize]]",
        docs: "Returns the symmetry group as cell maps, each sending the cell at index `i` to `element[i]`.",
        uncallable: None,
        door: Some(door_math_bang_baseq_group),
    },
    Function {
        name: "math.bang.baseq.group_order",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "Returns the symmetry group order counted from the enumerated axis maps.",
        uncallable: None,
        door: Some(door_math_bang_baseq_group_order),
    },
    Function {
        name: "math.bang.baseq.orbit",
        signature: "(group: [[usize]], code: Code) -> [Code]",
        docs: "Returns every code a design reaches under the group.",
        uncallable: None,
        door: Some(door_math_bang_baseq_orbit),
    },
    Function {
        name: "math.bang.baseq.predicted_group_order",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "Returns the closed-form group order the axis-map count must match.",
        uncallable: None,
        door: Some(door_math_bang_baseq_predicted_group_order),
    },
    Function {
        name: "math.bang.baseq.representatives",
        signature: "(base: usize, dimension: usize) -> [(Code, usize)]",
        docs: "Walks every code of a base and dimension and returns each orbit's least code with the orbit's size.\n\n# Errors\n\nErrors when the cell count passes the walk limit.",
        uncallable: None,
        door: Some(door_math_bang_baseq_representatives),
    },
    Function {
        name: "math.bang.baseq.sequence",
        signature: "(base: usize, max_dimension: usize) -> [u128]",
        docs: "Returns the distinct-design counts for dimensions 1 through max_dimension.\n\n# Errors\n\nErrors below base one, or when a Burnside average is not whole.",
        uncallable: None,
        door: Some(door_math_bang_baseq_sequence),
    },
    Function {
        name: "math.bang.baseq.total_designs",
        signature: "(base: usize, dimension: usize) -> u128",
        docs: "Returns the raw design count before symmetry, two to the number of cells.\n\n# Errors\n\nErrors when the cell count reaches the hundred and twenty-eight bits a u128 holds.",
        uncallable: None,
        door: Some(door_math_bang_baseq_total_designs),
    },
    Function {
        name: "math.bang.catalog.antis",
        signature: "(dimension: usize) -> [tile.recipe.Design]",
        docs: "Returns the anti designs for a dimension.",
        uncallable: None,
        door: Some(door_math_bang_catalog_antis),
    },
    Function {
        name: "math.bang.code.get",
        signature: "(self: Code) -> u128",
        docs: "Returns the bitmask the code carries.",
        uncallable: None,
        door: Some(door_math_bang_code_get),
    },
    Function {
        name: "math.bang.code_to_corners",
        signature: "(code: Code, dimension: usize, base: usize) -> [[u8]]",
        docs: "Unpacks a code into its filled residue corners.\n\n# Errors\n\nErrors when the code names a corner the dimension and base do not hold.",
        uncallable: None,
        door: Some(door_math_bang_code_to_corners),
    },
    Function {
        name: "math.bang.corners",
        signature: "(dimension: usize) -> [[u8]]",
        docs: "Returns the binary corners of a dimension in code order.",
        uncallable: None,
        door: Some(door_math_bang_corners),
    },
    Function {
        name: "math.bang.corners_to_code",
        signature: "(filled: [[u8]], dimension: usize, base: usize) -> Code",
        docs: "Packs filled residue corners back into their code.",
        uncallable: None,
        door: Some(door_math_bang_corners_to_code),
    },
    Function {
        name: "math.bang.factory.create",
        signature: "(code: Code, number: usize, dimension: usize, base: usize, level: usize) -> Tensor",
        docs: "Renders a coded design to a tensor at its side number, dimension, base and fractal level.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_bang_factory_create),
    },
    Function {
        name: "math.bang.factory.create_from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize, level: usize) -> Tensor",
        docs: "Renders a design straight from its filled residue corners.\n\n# Errors\n\nErrors when a corner does not hold one residue per axis.",
        uncallable: None,
        door: Some(door_math_bang_factory_create_from_corners),
    },
    Function {
        name: "math.bang.factory.create_named",
        signature: "(spec: str, number: usize, level: usize) -> Tensor",
        docs: "Renders a design from its canonical JSON name.\n\n# Errors\n\nErrors for JSON that is not a bang name.",
        uncallable: None,
        door: Some(door_math_bang_factory_create_named),
    },
    Function {
        name: "math.bang.factory.residue_corners",
        signature: "(dimension: usize, base: usize) -> [[u8]]",
        docs: "Returns every base-q residue corner of a dimension in row-major order.",
        uncallable: None,
        door: Some(door_math_bang_factory_residue_corners),
    },
    Function {
        name: "math.bang.factory.total_codes",
        signature: "(dimension: usize, base: usize) -> Code",
        docs: "Returns the code count of a dimension and base, two to the number of corners.\n\n# Errors\n\nErrors when the corner count reaches the hundred and twenty-eight bits a u128 code holds.",
        uncallable: None,
        door: Some(door_math_bang_factory_total_codes),
    },
    Function {
        name: "math.bang.level_cap",
        signature: "(number: usize, dimension: usize, budget: usize) -> usize",
        docs: "Returns the deepest level, never below one, whose grid of number to the dimension per level stays within the cell budget.",
        uncallable: None,
        door: Some(door_math_bang_level_cap),
    },
    Function {
        name: "math.bang.levels_code",
        signature: "(dimension: usize, base: usize, levels: [usize]) -> Code",
        docs: "Returns the code of the design filled wherever a corner's residue sum lands in the levels.",
        uncallable: None,
        door: Some(door_math_bang_levels_code),
    },
    Function {
        name: "math.bang.magic",
        signature: "(layers: [math.bang.MagicLayer]) -> Tensor",
        docs: "Composes the layers into one mixed-design cell by the ordered Kronecker product, first layer outermost.\n\nA run of one repeated layer is the ordinary self-similar fractal level.\n\n# Errors\n\nErrors below two layers, across dimensions, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_bang_magic),
    },
    Function {
        name: "math.bang.magic_named",
        signature: "(layers: [(str, usize)]) -> Tensor",
        docs: "Composes JSON-named layers in order.\n\n# Errors\n\nErrors for an object that is not a bang name, or layers that do not compose.",
        uncallable: None,
        door: Some(door_math_bang_magic_named),
    },
    Function {
        name: "math.bang.random_code",
        signature: "(dimension: usize, base: usize, rng: Rng(seed), min_fills: usize) -> Code",
        docs: "Rolls a code uniformly from the nonempty codes of a dimension and base that fill at least min_fills corners.\n\n# Errors\n\nErrors when the corners overflow a code, or min_fills is more than the corners.",
        uncallable: None,
        door: Some(door_math_bang_random_code),
    },
    Function {
        name: "math.bang.sources",
        signature: "(catalog: tile.recipe.Catalog, dimension: usize) -> [tile.recipe.Source]",
        docs: "Builds the tile sources a catalog names at a dimension.\n\n# Errors\n\nErrors outside dimensions one to four.",
        uncallable: None,
        door: Some(door_math_bang_sources),
    },
    Function {
        name: "math.bang.symmetries",
        signature: "(dimension: usize) -> [([usize], [u8])]",
        docs: "Returns the full symmetry group as axis permutations paired with flip patterns.",
        uncallable: None,
        door: Some(door_math_bang_symmetries),
    },
    Function {
        name: "math.bang.total_exposure",
        signature: "(code: Code, dimension: usize) -> bool",
        docs: "Returns whether no two filled corners of a code sit at Hamming distance one.\n\nSuch a design buries no face at any side and any level, so its surface is six per cell.",
        uncallable: None,
        door: Some(door_math_bang_total_exposure),
    },
    Function {
        name: "math.bang.touches_every_corner",
        signature: "(code: Code, dimension: usize) -> bool",
        docs: "Returns whether a code fills the all-even corner, the rule that touches every grid corner at odd side.",
        uncallable: None,
        door: Some(door_math_bang_touches_every_corner),
    },
    Function {
        name: "math.bang.universe.anf",
        signature: "(code: Code, dimension: usize) -> [u8]",
        docs: "Returns the algebraic normal form coefficients of a code, one per corner.",
        uncallable: None,
        door: Some(door_math_bang_universe_anf),
    },
    Function {
        name: "math.bang.universe.anf_string",
        signature: "(code: Code, dimension: usize) -> String",
        docs: "Formats the algebraic normal form of a code as a sum of monomials.",
        uncallable: None,
        door: Some(door_math_bang_universe_anf_string),
    },
    Function {
        name: "math.bang.universe.apply",
        signature: "(element: ([usize], [u8]), corner: [u8]) -> [u8]",
        docs: "Applies a symmetry element to a corner.",
        uncallable: None,
        door: Some(door_math_bang_universe_apply),
    },
    Function {
        name: "math.bang.universe.corner_index",
        signature: "(corner: [u8]) -> usize",
        docs: "Returns the bit position a binary corner occupies in a code.",
        uncallable: None,
        door: Some(door_math_bang_universe_corner_index),
    },
    Function {
        name: "math.bang.universe.degree",
        signature: "(code: Code, dimension: usize) -> i32",
        docs: "Returns the algebraic degree of a code, or -1 for the zero design.",
        uncallable: None,
        door: Some(door_math_bang_universe_degree),
    },
    Function {
        name: "math.bang.universe.orbit",
        signature: "(code: Code, dimension: usize) -> [Code]",
        docs: "Returns every code a design reaches under the full symmetry group.",
        uncallable: None,
        door: Some(door_math_bang_universe_orbit),
    },
    Function {
        name: "math.bang.universe.permutations",
        signature: "(n: usize) -> [[usize]]",
        docs: "Returns every permutation of 0..n in sorted order.",
        uncallable: None,
        door: Some(door_math_bang_universe_permutations),
    },
    Function {
        name: "math.bang.universe_codes",
        signature: "(dimension: usize) -> [u128]",
        docs: "Returns the canonical design codes of a dimension, computed once and cached for the process.\n\n# Errors\n\nErrors outside dimensions one to four.",
        uncallable: None,
        door: Some(door_math_bang_universe_codes),
    },
    Function {
        name: "math.bang.word.Schedule.all",
        signature: "() -> [math.bang.word.Schedule; 3]",
        docs: "Returns every Schedule in canonical order.",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_all),
    },
    Function {
        name: "math.bang.word.Schedule.frequencies",
        signature: "(self: math.bang.word.Schedule) -> (f64, f64)",
        docs: "Returns the letter frequencies the schedule tends to.",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_frequencies),
    },
    Function {
        name: "math.bang.word.Schedule.place",
        signature: "(self: math.bang.word.Schedule, index: usize) -> usize",
        docs: "Returns the letter the schedule takes at the place, zero or one.",
        uncallable: None,
        door: Some(door_math_bang_word_schedule_place),
    },
    Function {
        name: "math.bang.word.components",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "Counts the 4-connected components of a plane word without drawing it.\n\n# Errors\n\nErrors on an empty word, or when the component count passes a u128.",
        uncallable: None,
        door: Some(door_math_bang_word_components),
    },
    Function {
        name: "math.bang.word.constant_functional",
        signature: "(layers: [math.bang.MagicLayer]) -> f64",
        docs: "Returns the constant-word component functional of a plane word's letter frequencies,\nin log two units.\n\nIt reads `sum_c f_c log2 comp(A_c)`, the one linear functional exact on constant words,\nwhich on the plane alphabet at side two is `(f_6 + f_9) log 2`. It is a prediction and not\na theorem: at interior frequency it is refuted on 78 of the 105 letter pairs and exact on 27.\n\n# Errors\n\nErrors on an empty word.",
        uncallable: None,
        door: Some(door_math_bang_word_constant_functional),
    },
    Function {
        name: "math.bang.word.dimension",
        signature: "(layers: [math.bang.MagicLayer]) -> f64",
        docs: "Returns the scale dimension of a word, the sum of the log fills over the sum of the log sides.\n\n# Errors\n\nErrors on an empty word, or one whose letters are all side one.",
        uncallable: None,
        door: Some(door_math_bang_word_dimension),
    },
    Function {
        name: "math.bang.word.fill",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "Returns the filled cells of a word, the product of its letter fills.\n\n# Errors\n\nErrors when the word's fill passes a u128.",
        uncallable: None,
        door: Some(door_math_bang_word_fill),
    },
    Function {
        name: "math.bang.word.fills",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "Lists the filled cells of every letter, the product of which is the word's fill.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_bang_word_fills),
    },
    Function {
        name: "math.bang.word.letter",
        signature: "(layer: math.bang.MagicLayer) -> math.bang.word.Letter",
        docs: "Reads one plane letter: its fill, its runs, the rows and columns that wrap into a\nneighbouring copy, and its own components.\n\n# Errors\n\nErrors for a layer that is not a plane reading.",
        uncallable: None,
        door: Some(door_math_bang_word_letter),
    },
    Function {
        name: "math.bang.word.native",
        signature: "(layers: [math.bang.MagicLayer]) -> bool",
        docs: "Returns whether every letter renders at its own residue base, the native case where a\nperiodic word folds to one residue rule at the product base.",
        uncallable: None,
        door: Some(door_math_bang_word_native),
    },
    Function {
        name: "math.bang.word.period",
        signature: "(layers: [math.bang.MagicLayer]) -> usize",
        docs: "Returns the shortest whole period of the letter list, its own length when no shorter block repeats.",
        uncallable: None,
        door: Some(door_math_bang_word_period),
    },
    Function {
        name: "math.bang.word.prefixes",
        signature: "(layers: [math.bang.MagicLayer]) -> [math.bang.word.Counts]",
        docs: "Folds a plane word letter by letter and returns the counts at every prefix.\n\nEach letter folds the component count by its own geometry: a letter with no wrap-around\ncontact isolates every block, so the count becomes the outer fill times the letter's own\npieces; a connected letter with contacts on one axis merges blocks along that axis alone,\nso the count becomes the outer run count; a connected letter with both contacts leaves the\nblock graph isomorphic to the cell graph and the count unmoved. A letter that is neither\nconnected nor contact-free both splits and merges and is refused.\n\nThe list stops at the last prefix whose counts fit a u128 rather than wrapping.\n\n# Errors\n\nErrors for a letter that neither connects nor stays contact-free.",
        uncallable: None,
        door: Some(door_math_bang_word_prefixes),
    },
    Function {
        name: "math.bang.word.rates",
        signature: "(layers: [math.bang.MagicLayer]) -> [(f64, f64)]",
        docs: "Returns the prefix rates of a plane word in log two units, the component rate\n`(1/L) log2 comp` and the fill rate `(1/L) log2 fill` at every prefix length.\n\nAt interior letter frequency the two meet: the component exponent is order-blind and equals\nthe fill exponent on every one of the 105 letter pairs but the domino against the full tile.\nThe list stops at the last prefix whose counts fit a u128.\n\n# Errors\n\nErrors for a letter that neither connects nor stays contact-free.",
        uncallable: None,
        door: Some(door_math_bang_word_rates),
    },
    Function {
        name: "math.bang.word.side",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "Returns the side of a word, the product of its letter sides.\n\n# Errors\n\nErrors when the word's side passes a u128.",
        uncallable: None,
        door: Some(door_math_bang_word_side),
    },
    Function {
        name: "math.bang.word.spell",
        signature: "(schedule: math.bang.word.Schedule, pair: (math.bang.MagicLayer, math.bang.MagicLayer), length: usize) -> [math.bang.MagicLayer]",
        docs: "Spells the first letters of a schedule over an ordered pair of letters.",
        uncallable: None,
        door: Some(door_math_bang_word_spell),
    },
    Function {
        name: "math.bang.word.staircase",
        signature: "(depth: usize) -> [math.bang.MagicLayer]",
        docs: "Builds the carpet staircase word to the depth, the stacked prefixes `magic(3)`,\nthen `magic(3,5)`, then `magic(3,5,7)`, and so on.\n\nThe letter at odd side `2j + 1` occurs `depth - j + 1` times in the first `depth` blocks,\nso the word holds `depth (depth + 1) / 2` letters and its dimension is the occurrence-weighted\naverage of the per-letter dimensions.\n\n# Errors\n\nErrors on an empty block list.",
        uncallable: None,
        door: Some(door_math_bang_word_staircase),
    },
    Function {
        name: "math.bang.word.thue_morse",
        signature: "(index: usize) -> usize",
        docs: "Returns the Thue-Morse letter at the place, the parity of its binary digit sum.",
        uncallable: None,
        door: Some(door_math_bang_word_thue_morse),
    },
    Function {
        name: "math.cell.census.edges",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "Counts the distinct unit edges the filled sites carry, the edge graph's branches.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_cell_census_edges),
    },
    Function {
        name: "math.cell.census.exposure",
        signature: "(cell: Cell2d|Cell3d) -> u128",
        docs: "Counts the faces of filled sites open to emptiness or the border.",
        uncallable: None,
        door: Some(door_math_cell_census_exposure),
    },
    Function {
        name: "math.cell.census.fills",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "Counts the filled sites of the cell.",
        uncallable: None,
        door: Some(door_math_cell_census_fills),
    },
    Function {
        name: "math.cell.census.vertices",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "Counts the distinct corners the filled sites touch, the edge graph's nodes.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_cell_census_vertices),
    },
    Function {
        name: "math.cell.census.voids",
        signature: "(cell: Cell2d|Cell3d) -> usize",
        docs: "Counts the empty sites of the cell.",
        uncallable: None,
        door: Some(door_math_cell_census_voids),
    },
    Function {
        name: "math.cell.geometry.merge_reps",
        signature: "(cells: [Cell2d|Cell3d], reps: [usize]) -> Cell2d|Cell3d",
        docs: "Merges same-shaped cells into one block laid out by the per-axis repetition counts.\n\n# Errors\n\nErrors when the cells differ in shape or do not fill the arrangement.",
        uncallable: None,
        door: Some(door_math_cell_geometry_merge_reps),
    },
    Function {
        name: "math.cell.geometry.perforate",
        signature: "(mask: Tensor, cell: Cell2d|Cell3d, value: u8) -> Cell2d|Cell3d",
        docs: "Writes the value into the cell wherever the tiled mask is nonzero.\n\n# Errors\n\nErrors when the mask does not tile the cell.",
        uncallable: None,
        door: Some(door_math_cell_geometry_perforate),
    },
    Function {
        name: "math.cell.grow",
        signature: "(pattern: Tensor, level: usize) -> Cell2d|Cell3d",
        docs: "Grows a seed pattern into a cell, deepened to its fractal past level one.\n\n# Errors\n\nErrors when the pattern's rank is not N.",
        uncallable: None,
        door: Some(door_math_cell_grow),
    },
    Function {
        name: "math.cell.models.anti",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Inverts the cell.",
        uncallable: None,
        door: Some(door_math_cell_models_anti),
    },
    Function {
        name: "math.cell.models.binarize",
        signature: "(self: Cell2d|Cell3d, threshold: u8) -> Cell2d|Cell3d",
        docs: "Maps each site to one at or above the threshold, zero below.",
        uncallable: None,
        door: Some(door_math_cell_models_binarize),
    },
    Function {
        name: "math.cell.models.binarize_otsu",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Binarizes the cell at the threshold Otsu's method picks.",
        uncallable: None,
        door: Some(door_math_cell_models_binarize_otsu),
    },
    Function {
        name: "math.cell.models.blur",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, wrap: bool) -> Cell2d|Cell3d",
        docs: "Rounds each site to the mean of its masked neighborhood, wrapping on request.\n\n# Errors\n\nErrors when the mask does not match the cell's rank.",
        uncallable: None,
        door: Some(door_math_cell_models_blur),
    },
    Function {
        name: "math.cell.models.combine",
        signature: "(self: Cell2d|Cell3d, other: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Returns the Kronecker product of the two cells.",
        uncallable: None,
        door: Some(door_math_cell_models_combine),
    },
    Function {
        name: "math.cell.models.counting_dtype",
        signature: "(mask: Tensor) -> core.Dtype",
        docs: "Returns the narrowest count dtype that fits the mask's popcount.",
        uncallable: None,
        door: Some(door_math_cell_models_counting_dtype),
    },
    Function {
        name: "math.cell.models.depth",
        signature: "(self: Cell3d) -> usize",
        docs: "Returns the size of axis 0, the cube's leading axis.",
        uncallable: None,
        door: Some(door_math_cell_models_depth),
    },
    Function {
        name: "math.cell.models.dtype_for",
        signature: "(peak: i64) -> core.Dtype",
        docs: "Returns the narrowest unsigned dtype that holds the peak value.",
        uncallable: None,
        door: Some(door_math_cell_models_dtype_for),
    },
    Function {
        name: "math.cell.models.fractal",
        signature: "(self: Cell2d|Cell3d, level: usize) -> Cell2d|Cell3d",
        docs: "Deepens the cell into its level-fold fractal.\n\n# Errors\n\nErrors below level one.",
        uncallable: None,
        door: Some(door_math_cell_models_fractal),
    },
    Function {
        name: "math.cell.models.height",
        signature: "(self: Cell2d|Cell3d) -> usize",
        docs: "Returns the size of the axis before the last.",
        uncallable: None,
        door: Some(door_math_cell_models_height),
    },
    Function {
        name: "math.cell.models.invert",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Swaps filled and empty sites.",
        uncallable: None,
        door: Some(door_math_cell_models_invert),
    },
    Function {
        name: "math.cell.models.layers",
        signature: "(self: Cell2d|Cell3d) -> Cell2d|Cell3d",
        docs: "Tags each site with its ring distance from the center.",
        uncallable: None,
        door: Some(door_math_cell_models_layers),
    },
    Function {
        name: "math.cell.models.neighbors",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, target: u8, wrap: bool) -> Cell2d|Cell3d",
        docs: "Tags each site with its count of masked neighbors matching the target, wrapping on request.\n\n# Errors\n\nErrors when the mask does not match the cell's rank.",
        uncallable: None,
        door: Some(door_math_cell_models_neighbors),
    },
    Function {
        name: "math.cell.models.new",
        signature: "(types: Tensor) -> Cell2d|Cell3d",
        docs: "Builds a cell from an N-dimensional tensor of types.\n\n# Errors\n\nErrors when the tensor's rank is not N.",
        uncallable: None,
        door: Some(door_math_cell_models_new),
    },
    Function {
        name: "math.cell.models.orient",
        signature: "(self: Cell3d, index: usize) -> Cell3d",
        docs: "Turns the cell into one of the 24 cube orientations.\n\n# Errors\n\nErrors past orientation twenty-three.",
        uncallable: None,
        door: Some(door_math_cell_models_orient),
    },
    Function {
        name: "math.cell.models.pad",
        signature: "(self: Cell2d|Cell3d, count: usize, value: u8) -> Cell2d|Cell3d",
        docs: "Wraps the cell in count layers of the given value on every side.",
        uncallable: None,
        door: Some(door_math_cell_models_pad),
    },
    Function {
        name: "math.cell.models.paint",
        signature: "(self: Cell2d|Cell3d, mapping: {u8: [Color]}, mode: core.Mode, rng: Rng(seed)?) -> Cell2d|Cell3d",
        docs: "Colors each site by its type through the mapping in the given mode.\n\n# Errors\n\nErrors when the Random mode arrives without a stream to draw from.",
        uncallable: None,
        door: Some(door_math_cell_models_paint),
    },
    Function {
        name: "math.cell.models.perforate",
        signature: "(self: Cell2d|Cell3d, mask: Tensor, value: u8) -> Cell2d|Cell3d",
        docs: "Writes the value wherever the tiled mask is nonzero.\n\n# Errors\n\nErrors when the mask does not tile the cell.",
        uncallable: None,
        door: Some(door_math_cell_models_perforate),
    },
    Function {
        name: "math.cell.models.rotate",
        signature: "(self: Cell2d, k: usize) -> Cell2d | (self: Cell3d, k: usize, axes: (usize, usize)) -> Cell3d",
        docs: "Rotates the cell k quarter turns in the plane.\n\n# Errors\n\nErrors for a cell without two axes.",
        uncallable: None,
        door: Some(door_math_cell_models_rotate),
    },
    Function {
        name: "math.cell.models.tile",
        signature: "(self: Cell2d, width: usize, height: usize) -> Cell2d | (self: Cell3d, width: usize, height: usize, depth: usize) -> Cell3d",
        docs: "Repeats the cell into a width-by-height array of copies.\n\n# Errors\n\nErrors for a cell without two axes.",
        uncallable: None,
        door: Some(door_math_cell_models_tile),
    },
    Function {
        name: "math.cell.models.types",
        signature: "(self: Cell2d|Cell3d) -> Tensor",
        docs: "Returns the tensor of types.",
        uncallable: None,
        door: Some(door_math_cell_models_types),
    },
    Function {
        name: "math.cell.models.width",
        signature: "(self: Cell2d|Cell3d) -> usize",
        docs: "Returns the size of the last axis.",
        uncallable: None,
        door: Some(door_math_cell_models_width),
    },
    Function {
        name: "math.cell.paint",
        signature: "(cell: Cell2d|Cell3d, custom: {u8: [Color]}?, mode: core.Mode?, rng: Rng(seed)?) -> Cell2d|Cell3d",
        docs: "Colors the cell through the given mapping and mode, defaulting to the standard palette by type.\n\n# Errors\n\nErrors when the Random mode arrives without a stream to draw from.",
        uncallable: None,
        door: Some(door_math_cell_paint),
    },
    Function {
        name: "math.cell.serializer.byte_cube",
        signature: "(value: Json) -> [[[u8]]]",
        docs: "Reads a triply nested JSON array into layers of byte rows.\n\n# Errors\n\nErrors when the value is not a cube of bytes.",
        uncallable: None,
        door: Some(door_math_cell_serializer_byte_cube),
    },
    Function {
        name: "math.cell.serializer.byte_grid",
        signature: "(value: Json) -> [[u8]]",
        docs: "Reads a nested JSON array into rows of bytes.\n\n# Errors\n\nErrors when the value is not a grid of bytes.",
        uncallable: None,
        door: Some(door_math_cell_serializer_byte_grid),
    },
    Function {
        name: "math.cell.serializer.color_grid",
        signature: "(value: Json) -> [[[u8; 4]]]",
        docs: "Reads a nested JSON array into rows of four-channel colors.\n\n# Errors\n\nErrors when the value is not a grid of RGBA quadruples.",
        uncallable: None,
        door: Some(door_math_cell_serializer_color_grid),
    },
    Function {
        name: "math.cell.serializer.count_cube",
        signature: "(value: Json) -> [i64]",
        docs: "Reads a triply nested JSON array of counts into one flat run; a count must fit in thirty-two bits.\n\n# Errors\n\nErrors when the value is not a cube of counts.",
        uncallable: None,
        door: Some(door_math_cell_serializer_count_cube),
    },
    Function {
        name: "math.cell.serializer.count_grid",
        signature: "(value: Json) -> [i64]",
        docs: "Reads a nested JSON array of counts into one flat run; a count must fit in thirty-two bits.\n\n# Errors\n\nErrors when the value is not a grid of counts.",
        uncallable: None,
        door: Some(door_math_cell_serializer_count_grid),
    },
    Function {
        name: "math.cell.serializer.parse",
        signature: "(text: str) -> Json",
        docs: "Parses JSON text into a value tree.\n\n# Errors\n\nErrors when the text is not JSON.",
        uncallable: None,
        door: Some(door_math_cell_serializer_parse),
    },
    Function {
        name: "math.cell.serializer.tag_layer",
        signature: "(counts: [i64], shape: [usize]) -> Tensor",
        docs: "Packs a flat run of counts into a tensor of the shape, at the narrowest dtype that holds them.\n\n# Errors\n\nErrors when the counts do not match the shape.",
        uncallable: None,
        door: Some(door_math_cell_serializer_tag_layer),
    },
    Function {
        name: "math.cell.serializer.types_field",
        signature: "(data: Json) -> Json",
        docs: "Returns the types field of the data.\n\n# Errors\n\nErrors when the types field is missing.",
        uncallable: None,
        door: Some(door_math_cell_serializer_types_field),
    },
    Function {
        name: "math.counts.Exposure.at",
        signature: "(self: math.counts.Exposure, level: u32) -> u128?",
        docs: "Returns the exposed faces of the level-fold Kronecker power, or none past a u128.",
        uncallable: None,
        door: Some(door_math_counts_exposure_at),
    },
    Function {
        name: "math.counts.Exposure.from_corners",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize) -> math.counts.Exposure",
        docs: "Folds the counts from the filled residue corners at a side number, without rendering the tile.\n\nAn adjacent pair sits at positions `i, i + 1` whose residues are `r, r + 1 mod q`, and a\nspanning position pairs residue 0 with the residue of `n - 1`.",
        uncallable: None,
        door: Some(door_math_counts_exposure_from_corners),
    },
    Function {
        name: "math.counts.Exposure.of_tile",
        signature: "(tile: Tensor) -> math.counts.Exposure",
        docs: "Reads the counts off a rendered tile.",
        uncallable: None,
        door: Some(door_math_counts_exposure_of_tile),
    },
    Function {
        name: "math.counts.Exposure.recurrence",
        signature: "(self: math.counts.Exposure) -> [i128]",
        docs: "Returns the coefficients `c` of the recurrence `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...` the exposure obeys.\n\nThe roots are `occ` and the distinct nonzero spanning counts, `occ` doubled where a\nspanning count equals it.",
        uncallable: None,
        door: Some(door_math_counts_exposure_recurrence),
    },
    Function {
        name: "math.counts.centered_hexagonal",
        signature: "(m: usize) -> u128",
        docs: "Returns the centered hexagonal number at the index, the lattice points of a hexagon of side m-1.",
        uncallable: None,
        door: Some(door_math_counts_centered_hexagonal),
    },
    Function {
        name: "math.counts.cut_fills",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "Returns the filled triangle count of the code's cut section at the given level, without rendering it.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_counts_cut_fills),
    },
    Function {
        name: "math.counts.cut_voids",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "Returns the empty triangle count of the code's cut section at the given level.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_counts_cut_voids),
    },
    Function {
        name: "math.counts.dimension",
        signature: "(code: Code, number: usize, base_dimension: usize, base: usize) -> f64",
        docs: "Returns the code's fractal dimension, the log of its one-level fill over the log of number.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_dimension),
    },
    Function {
        name: "math.counts.edges_of_tile",
        signature: "(tile: Tensor, level: u32) -> u128?",
        docs: "Returns the branch count of the tile's level-fold Kronecker power, fitted to its two-term\nrecurrence over four powers, or none past a u128.",
        uncallable: None,
        door: Some(door_math_counts_edges_of_tile),
    },
    Function {
        name: "math.counts.exposure",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "Returns the exposed face count of the code's fractal in any dimension at the given level, folded from its corners.\n\n# Errors\n\nErrors when the code is out of range, or the exposure passes a hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_math_counts_exposure),
    },
    Function {
        name: "math.counts.exposure_of_tile",
        signature: "(tile: Tensor, level: u32) -> u128?",
        docs: "Returns the exposed face count of the tile's level-fold Kronecker power in closed form, or none past a u128.",
        uncallable: None,
        door: Some(door_math_counts_exposure_of_tile_2),
    },
    Function {
        name: "math.counts.exposure_recurrence",
        signature: "(tile: Tensor) -> [i128]",
        docs: "Returns the coefficients of the recurrence the tile's exposure obeys.",
        uncallable: None,
        door: Some(door_math_counts_exposure_recurrence_2),
    },
    Function {
        name: "math.counts.fill",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "Returns the filled cell count of the code's fractal at the given level, without rendering it.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_fill),
    },
    Function {
        name: "math.counts.fill_from_corners",
        signature: "(filled: [[u8]], number: usize, _dimension: usize, level: u32, base: usize) -> u128",
        docs: "Sums each corner's position products into a base fill and raises it to the level.",
        uncallable: None,
        door: Some(door_math_counts_fill_from_corners),
    },
    Function {
        name: "math.counts.grid",
        signature: "(number: usize, dimension: usize, level: u32) -> u128",
        docs: "Returns the total cells of the grid, number to the dimension, to the level.",
        uncallable: None,
        door: Some(door_math_counts_grid),
    },
    Function {
        name: "math.counts.ladder.cap",
        signature: "(base: usize) -> usize",
        docs: "The widest dimension the exact carry arithmetic reaches at the base.\n\nThe sign is the heaviest reading: it walks the characteristic polynomial at `f_D/q` with the\ndenominators cleared, so its running value climbs like `f_D^(D/2)` and a hundred and twenty-eight\nbits run out at dimension fifteen in base three and dimension eleven in base five.\n\n# Errors\n\nErrors at an even base under three, or past the dimension the ladder reaches.",
        uncallable: None,
        door: Some(door_math_counts_ladder_cap),
    },
    Function {
        name: "math.counts.ladder.carry_matrix",
        signature: "(base: usize, dimension: usize) -> [[i128]]",
        docs: "The carry matrix over the reachable carries `|c| <= (D-1)/2`, rows indexed by the carry out.\n\nA level of the design adds one base-`q` digit per coordinate, so the height of the central\ndiagonal hyperplane moves by a digit sum `s` and a carry `c -> (c + mD - s)/q` with `m` the\nmiddle digit; the map contracts onto this window from every start.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_carry_matrix),
    },
    Function {
        name: "math.counts.ladder.characteristic",
        signature: "(rows: [[i128]]) -> [i128]",
        docs: "The monic characteristic polynomial of a square integer matrix, highest power first.\n\nFaddeev-LeVerrier in exact integers: every division lands whole, so no fraction is ever needed\nand the answer is the recurrence's coefficients up to sign.\n\n# Errors\n\nErrors when the walk leaves a remainder, or a coefficient passes an i128.",
        uncallable: None,
        door: Some(door_math_counts_ladder_characteristic),
    },
    Function {
        name: "math.counts.ladder.determinant",
        signature: "(rows: [[i128]]) -> i128",
        docs: "The determinant of a square integer matrix, read off its characteristic polynomial.\n\n# Errors\n\nErrors when the walk leaves a remainder, or a coefficient passes an i128.",
        uncallable: None,
        door: Some(door_math_counts_ladder_determinant),
    },
    Function {
        name: "math.counts.ladder.digit_polynomial",
        signature: "(base: usize, dimension: usize) -> [i128]",
        docs: "The digit polynomial of the base-`q` middle-digit design in dimension `D`, lowest power first.\n\nThe design keeps the cells whose digit vector has at most one coordinate equal to the middle\ndigit, so its weight generating function is `A(t)^(D-1) (A(t) + D t^m)`, with `A` the sum of\nevery digit power but the middle one. At base three that factors as\n`(1 + t^2)^(D-1) (1 + D t + t^2)`, and the sum of the coefficients is the fill.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_digit_polynomial),
    },
    Function {
        name: "math.counts.ladder.even_block",
        signature: "(base: usize, dimension: usize) -> [[i128]]",
        docs: "The reflection-even block of the carry matrix, of size `ceil(D/2)`.\n\nThe digit polynomial is palindromic, so the reflection `c -> -c` commutes with the carry map\nand splits it; the even half carries the central count and its characteristic polynomial is\nthe recurrence the counts obey.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_even_block),
    },
    Function {
        name: "math.counts.ladder.fill",
        signature: "(base: usize, dimension: usize) -> i128",
        docs: "The count of level-one cells the design keeps, `f_D = (q - 1)^(D-1) (q - 1 + D)`.\n\n# Errors\n\nErrors at an even base under three, a dimension outside two to the ceiling, or a product past an i128.",
        uncallable: None,
        door: Some(door_math_counts_ladder_fill),
    },
    Function {
        name: "math.counts.ladder.ladder",
        signature: "(base: usize, dimension: usize, levels: usize) -> [i128]",
        docs: "The counts `a_D(L)` of level-`L` cells meeting the central diagonal hyperplane, from `L = 0`.\n\nThe count is the top-left entry of the `L`-th power of the even block. The walk stops early\nwhen the next count would pass a hundred and twenty-eight bits, so the answer runs as far as\nexact integers reach and no further.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_ladder),
    },
    Function {
        name: "math.counts.ladder.perron",
        signature: "(rows: [[i128]]) -> f64",
        docs: "The Perron root of a nonnegative square integer matrix.\n\nThe characteristic polynomial is rescaled by the largest row sum, which bounds the root above,\nthen the largest sign change on the unit interval is hunted on a grid and closed by bisection,\nthe same walk the recurrence growth reader takes.\n\n# Errors\n\nErrors when the block has no positive row sum, or no root inside its bound.",
        uncallable: None,
        door: Some(door_math_counts_ladder_perron),
    },
    Function {
        name: "math.counts.ladder.sign",
        signature: "(base: usize, dimension: usize) -> i32",
        docs: "The sign of `log_q rho_D - (log_q f_D - 1)`, the slice sign law's reading, in exact integers.\n\nThe characteristic polynomial is evaluated at `f_D/q` with the denominators cleared, so the\nanswer is a comparison of whole numbers and never a rounding. The law reads `(-1)^(D+1)`: the\nslice exponent stands above the solid's dimension less one at odd `D` and below it at even `D`.\n\n# Errors\n\nErrors at an even base under three, a dimension outside two to the ceiling, or a term past an i128.",
        uncallable: None,
        door: Some(door_math_counts_ladder_sign),
    },
    Function {
        name: "math.counts.ladder.spectral_ratio",
        signature: "(base: usize, dimension: usize) -> f64?",
        docs: "The Perron root over the modulus of the second eigenvalue, or none where the block is one wide.\n\nPower iteration finds the leading pair, Hotelling deflation takes it out and a second walk,\nreprojected every sweep so rounding cannot bring the leader back, reads the runner up. At base\nthree the ratio falls to `(D + 2)/(D - 2)`, so no fixed spectral gap survives the dimensions.\n\n# Errors\n\nErrors at an even base under three, or a dimension outside two to the ceiling.",
        uncallable: None,
        door: Some(door_math_counts_ladder_spectral_ratio),
    },
    Function {
        name: "math.counts.ladder.trace",
        signature: "(rows: [[i128]]) -> i128",
        docs: "The trace of a square integer matrix.",
        uncallable: None,
        door: Some(door_math_counts_ladder_trace),
    },
    Function {
        name: "math.counts.limit",
        signature: "(code: Code, dimension: usize, level: u32, base: usize) -> (u128, u128)",
        docs: "Returns the fill ratio the code walks toward as the side number grows, reduced.\n\nThe ratio is the corner count over the corner slots, so it is always rational: the\ncarpet holds three corners of four and the sponge four of eight.\n\n# Errors\n\nErrors when the code is out of range, or the limit overflows at this level.",
        uncallable: None,
        door: Some(door_math_counts_limit),
    },
    Function {
        name: "math.counts.pairs",
        signature: "(tile: Tensor) -> [(u128, u128)]",
        docs: "Counts, per axis, the adjacent filled pairs and the cross positions whose two end cells are both filled.\n\nA level deeper, each adjacent pair buries one face per spanning position of the block, and the\nspanning positions of the block multiply level by level, so the exposure closes.",
        uncallable: None,
        door: Some(door_math_counts_pairs),
    },
    Function {
        name: "math.counts.positions",
        signature: "(residue: usize, number: usize, base: usize) -> u128",
        docs: "Counts the indices below number that equal residue modulo base.",
        uncallable: None,
        door: Some(door_math_counts_positions),
    },
    Function {
        name: "math.counts.pro_fills",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "Returns the filled triangle count of the code's pro projection at the given level, without rendering it.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_counts_pro_fills),
    },
    Function {
        name: "math.counts.pro_voids",
        signature: "(code: Code, number: usize, level: u32) -> u128",
        docs: "Returns the empty triangle count of the code's pro projection at the given level.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_counts_pro_voids),
    },
    Function {
        name: "math.counts.profile_of_tile",
        signature: "(tile: Tensor, level: u32) -> [u128]",
        docs: "Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`.\n\nThe count is the coefficient of the digit polynomial, the level-fold product of the tile's\nweight sums, so no cell of the power is ever built; the tile must be a hypercube.\n\n# Errors\n\nErrors for a tile that is not a hypercube, a level below one, a side past the widest, or counts past a u128.",
        uncallable: None,
        door: Some(door_math_counts_profile_of_tile),
    },
    Function {
        name: "math.counts.ratio",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> f64",
        docs: "Returns the filled fraction of the grid, or 0.0 for an empty grid.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_ratio),
    },
    Function {
        name: "math.counts.rational",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> (u128, u128)",
        docs: "Returns the exact filled fraction as a fraction of fill over grid, reduced.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_rational),
    },
    Function {
        name: "math.counts.six.grid_triangles",
        signature: "(number: usize, level: u32) -> u128",
        docs: "Returns the triangles of the full hexagon with side number to the level.",
        uncallable: None,
        door: Some(door_math_counts_six_grid_triangles),
    },
    Function {
        name: "math.counts.six.solid_slice_boundary",
        signature: "(number: usize) -> u128",
        docs: "Returns the boundary edge count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_boundary),
    },
    Function {
        name: "math.counts.six.solid_slice_core_edges",
        signature: "(number: usize) -> u128",
        docs: "Returns the core edge count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_core_edges),
    },
    Function {
        name: "math.counts.six.solid_slice_core_nodes",
        signature: "(number: usize) -> u128",
        docs: "Returns the core node count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_core_nodes),
    },
    Function {
        name: "math.counts.six.solid_slice_edges",
        signature: "(number: usize) -> u128",
        docs: "Returns the distinct triangle-edge count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_edges),
    },
    Function {
        name: "math.counts.six.solid_slice_interior",
        signature: "(number: usize) -> u128",
        docs: "Returns the interior edge count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_interior),
    },
    Function {
        name: "math.counts.six.solid_slice_triangles",
        signature: "(number: usize) -> u128",
        docs: "Returns the triangle count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_triangles),
    },
    Function {
        name: "math.counts.six.solid_slice_vertices",
        signature: "(number: usize) -> u128",
        docs: "Returns the vertex count of the solid slice, defined for odd number.\n\n# Errors\n\nErrors at an even side number.",
        uncallable: None,
        door: Some(door_math_counts_six_solid_slice_vertices),
    },
    Function {
        name: "math.counts.surface",
        signature: "(code: Code, number: usize, level: u32, base: usize) -> u128",
        docs: "Returns the exposed face count of the code's 3D fractal at the given level.\n\n# Errors\n\nErrors when the code is out of range, or the exposure passes a hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_math_counts_surface),
    },
    Function {
        name: "math.counts.void",
        signature: "(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> u128",
        docs: "Returns the empty cell count, grid minus fill.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_counts_void),
    },
    Function {
        name: "math.design.digits_of",
        signature: "(mask: u32, base: u64) -> [u64]",
        docs: "Returns the digits a bitmask names inside the base, ascending.",
        uncallable: None,
        door: Some(door_math_design_digits_of),
    },
    Function {
        name: "math.design.echo_series",
        signature: "(values: [u64], log_x: [f64], exponent: f64) -> [f64]",
        docs: "Returns the density echo, the sum of mu(n) A_F(n)/n over the whole numbers up to each grid point divided by x to the exponent, sieving the Mobius values to the largest element.",
        uncallable: None,
        door: Some(door_math_design_echo_series),
    },
    Function {
        name: "math.design.elements",
        signature: "(base: u64, digits: [u64], depth: usize) -> [u64]",
        docs: "Returns the elements of the digit design below the base raised to the depth, ascending: the whole numbers of at most that many base digits, every digit drawn from the set and the leading digit nonzero.",
        uncallable: None,
        door: Some(door_math_design_elements),
    },
    Function {
        name: "math.design.log_grid",
        signature: "(values: [u64], samples: usize) -> [f64]",
        docs: "Returns the log grid uniform over the span of the elements, from the log of the first to the log of the last.",
        uncallable: None,
        door: Some(door_math_design_log_grid),
    },
    Function {
        name: "math.design.median_floor",
        signature: "(power: [f64], width: usize) -> [f64]",
        docs: "Returns the running median of the power over a window of the given width, the window clamped at the ends.",
        uncallable: None,
        door: Some(door_math_design_median_floor),
    },
    Function {
        name: "math.design.meter",
        signature: "(mu: [i8]) -> [i64]",
        docs: "Returns the running design Mobius meter, the partial sums of the Mobius values along the elements.",
        uncallable: None,
        door: Some(door_math_design_meter),
    },
    Function {
        name: "math.design.nearest",
        signature: "(value: f64, list: [f64]) -> f64",
        docs: "Returns the distance from the ordinate to the nearest entry of the list, infinite when the list is empty.",
        uncallable: None,
        door: Some(door_math_design_nearest),
    },
    Function {
        name: "math.design.peaks",
        signature: "(gamma: [f64], score: [f64], band: (f64, f64), threshold: f64) -> [usize]",
        docs: "Returns the bins inside the band that rise above both neighbours and clear the score threshold, strongest first.\n\n# Errors\n\nErrs when the axis holds fewer bins than the score.",
        uncallable: None,
        door: Some(door_math_design_peaks),
    },
    Function {
        name: "math.design.pole_lattice",
        signature: "(base: u64, top: f64) -> [f64]",
        docs: "Returns the design's pole lattice below the top, the ordinates 2 pi j over log q of the poles its Dirichlet series carries.",
        uncallable: None,
        door: Some(door_math_design_pole_lattice),
    },
    Function {
        name: "math.design.resample",
        signature: "(values: [u64], running: [i64], exponent: f64, log_x: [f64]) -> [f64]",
        docs: "Reads the running meter at every point of the log grid and divides by x to the exponent.",
        uncallable: None,
        door: Some(door_math_design_resample),
    },
    Function {
        name: "math.design.score",
        signature: "(power: [f64], width: usize) -> [f64]",
        docs: "Returns the power over its local median floor, the score a peak is read against.",
        uncallable: None,
        door: Some(door_math_design_score),
    },
    Function {
        name: "math.design.size",
        signature: "(digits: [u64], depth: usize) -> u128",
        docs: "Returns the count of elements the design holds at the depth, the length `elements` returns without building them.",
        uncallable: None,
        door: Some(door_math_design_size),
    },
    Function {
        name: "math.design.spectrum",
        signature: "(log_x: [f64], series: [f64]) -> ([f64], [f64])",
        docs: "Returns the frequency axis and the power spectrum of the series: the mean removed, a Hann window laid on, a real transform taken, and bin j read as the ordinate 2 pi j over the log range.\n\n# Errors\n\nErrs when the log grid holds fewer points than the series.",
        uncallable: None,
        door: Some(door_math_design_spectrum),
    },
    Function {
        name: "math.design.upper_rms",
        signature: "(series: [f64]) -> f64",
        docs: "Returns the root mean square of the upper half of the series, the size the echo and the meter are compared at.",
        uncallable: None,
        door: Some(door_math_design_upper_rms),
    },
    Function {
        name: "math.dissection.cap",
        signature: "(y: u64) -> u64",
        docs: "Returns `Q = floor(y^(3/5))`, the largest denominator the dissection admits, exact in integers.",
        uncallable: None,
        door: Some(door_math_dissection_cap),
    },
    Function {
        name: "math.dissection.chain_exponent",
        signature: "(base: u64) -> f64",
        docs: "Returns the chain's certificate exponent at one missing digit, `alpha_1 = log_base(z base/(base - 1))`, the same at every missing digit.",
        uncallable: None,
        door: Some(door_math_dissection_chain_exponent),
    },
    Function {
        name: "math.dissection.chain_margin",
        signature: "(base: u64) -> f64",
        docs: "Returns the chain's margin at the bar, the cubic cleared of denominators at `w = base^(1/5)(1 - 1/base)`, positive exactly when the root sits below `w` and `alpha_1 < 1/5`.",
        uncallable: None,
        door: Some(door_math_dissection_chain_margin),
    },
    Function {
        name: "math.dissection.chain_root",
        signature: "(base: u64) -> f64",
        docs: "Returns the root `z > 1` of the digit-uniform chain at one missing digit, `(z-1)^3 = (2/pi)(log base) z + gamma'(z-1) + (2/pi)(z-1)^2/(base z - 1)`, by bisection.",
        uncallable: None,
        door: Some(door_math_dissection_chain_root),
    },
    Function {
        name: "math.dissection.chain_wall",
        signature: "() -> u64",
        docs: "Returns the chain's wall, one past the last base up to `CAP_BASE` whose margin is not positive.",
        uncallable: None,
        door: Some(door_math_dissection_chain_wall),
    },
    Function {
        name: "math.dissection.consecutive",
        signature: "(digits: [u64]) -> bool",
        docs: "Returns whether the digit set keeps two consecutive digits, the hypothesis region C1 reads.",
        uncallable: None,
        door: Some(door_math_dissection_consecutive),
    },
    Function {
        name: "math.dissection.fraction",
        signature: "(a: u64, y: u64, cap: u64) -> (u64, u64)",
        docs: "Returns the last continued-fraction convergent `l/d` of `a/y` whose denominator is at most the cap, the Dirichlet fraction of the dissection.",
        uncallable: None,
        door: Some(door_math_dissection_fraction),
    },
    Function {
        name: "math.dissection.kappa",
        signature: "(base: u64, digits: [u64]) -> (u64, u64)",
        docs: "Returns `kappa_F = (base/phi(base)) #{f in F : gcd(f, base) = 1}/fill`, the main-term constant of the prime count, as a reduced fraction.",
        uncallable: None,
        door: Some(door_math_dissection_kappa),
    },
    Function {
        name: "math.dissection.masses",
        signature: "(base: u64, digits: [u64], level: u32) -> [f64]",
        docs: "Returns the unshifted masses `c_j = sum_(a < base^j) |hat F_j(a/base^j)|` for `j = 0..=level`, the `l^1` mass region A pays, `c_0 = 1`.",
        uncallable: None,
        door: Some(door_math_dissection_masses),
    },
    Function {
        name: "math.dissection.reach",
        signature: "(base: u64, missing: u64) -> str",
        docs: "Returns how the theorem reaches the set missing one digit: `proof` from the chain's wall, `certificate` at every base from `DIGIT_WALL` below it and at the `CERTIFIED` sets, `none` elsewhere.",
        uncallable: None,
        door: Some(door_math_dissection_reach),
    },
    Function {
        name: "math.dissection.reading",
        signature: "(base: u64, fill: usize, masses: [f64]) -> f64",
        docs: "Returns the `l^1` exponent the top two masses read, `log_base(c_j/(fill c_(j-1)))`: a reading of the growth region A pays, never a certificate.",
        uncallable: None,
        door: Some(door_math_dissection_reading),
    },
    Function {
        name: "math.dissection.regions",
        signature: "(base: u64, level: u32, z: u64) -> [math.dissection.Region]",
        docs: "Returns the region of every frequency `a/y`, `a < y = base^level`, at the cut `Z`, the fraction taken by `fraction` at `cap`.",
        uncallable: None,
        door: Some(door_math_dissection_regions),
    },
    Function {
        name: "math.dissection.tally",
        signature: "(base: u64, digits: [u64], level: usize, samples: usize) -> math.dissection.Tally",
        docs: "Tallies the set below `base^level` on a log grid of the given size, with the Mobius values and the primes sieved to the span.",
        uncallable: None,
        door: Some(door_math_dissection_tally),
    },
    Function {
        name: "math.dissection.weights",
        signature: "(base: u64, digits: [u64], level: u32) -> [f64]",
        docs: "Returns `|hat F_level(a/base^level)|` at every `a < base^level`, built one digit at a time from `hat F_j(t) = hat F(t) hat F_(j-1)(base t)`.",
        uncallable: None,
        door: Some(door_math_dissection_weights),
    },
    Function {
        name: "math.graph.Layout.energy",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "Returns the mean net force per node in units of `k` after the last tick.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.from_network",
        signature: "(network: math.graph.Network, seed: u64) -> math.graph.Layout",
        docs: "Starts a layout from a network's own positions and branches.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: Some("math::graph::Layout has no Serialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.ideal",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "Returns the ideal branch length `k`.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.moved",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "Returns the mean distance a node moved in the last tick.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.new",
        signature: "(positions: [f64], branches: [(usize, usize)], dim: usize, seed: u64) -> math.graph.Layout",
        docs: "Starts a layout from flat positions, `dim` floats per node, and the branch pairs.\n\n# Errors\n\nErrors when a branch points off the list, or the positions are not dim floats a node.",
        uncallable: Some("math::graph::Layout has no Serialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.nodes",
        signature: "(self: math.graph.Layout) -> usize",
        docs: "Returns the node count.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.positions",
        signature: "(self: math.graph.Layout) -> [f64]",
        docs: "Returns the positions, `dim` floats per node.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.step",
        signature: "(self: math.graph.Layout, ticks: usize) -> f64",
        docs: "Runs the ticks and returns the energy left: the mean net force per node in units of `k`.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.temperature",
        signature: "(self: math.graph.Layout) -> f64",
        docs: "Returns the cap on one node's move in the next tick.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Layout.ticks",
        signature: "(self: math.graph.Layout) -> usize",
        docs: "Returns the ticks stepped so far.",
        uncallable: Some("math::graph::Layout has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.graph.Network.add_branch",
        signature: "(self: math.graph.Network, parent: usize, child: usize, radius: f64) -> null",
        docs: "Appends a branch between two node indices.\n\n# Errors\n\nErrors when either index is out of range.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Network.add_node",
        signature: "(self: math.graph.Network, position: [f64]) -> usize",
        docs: "Appends a node at the position and returns its index.\n\n# Errors\n\nErrors on a dimension mismatch.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.graph.Network.adjacency",
        signature: "(self: math.graph.Network) -> {usize: [usize]}",
        docs: "Returns the undirected neighbor lists of every node.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_network_adjacency),
    },
    Function {
        name: "math.graph.Network.degree",
        signature: "(self: math.graph.Network) -> [usize]",
        docs: "Returns each node's branch count, indexed like the node list.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_network_degree),
    },
    Function {
        name: "math.graph.Network.new",
        signature: "(dim: usize) -> math.graph.Network",
        docs: "Builds an empty network of the given dimension.",
        uncallable: None,
        door: Some(door_math_graph_network_new),
    },
    Function {
        name: "math.graph.census",
        signature: "(network: math.graph.Network) -> math.graph.Census",
        docs: "Takes the full census of a network.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_census),
    },
    Function {
        name: "math.graph.components",
        signature: "(network: math.graph.Network) -> usize",
        docs: "Counts the connected components of the network.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_components),
    },
    Function {
        name: "math.graph.core_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "Extracts the network of filled sites joined to their axis neighbors.\n\n# Errors\n\nErrors off two and three dimensions.",
        uncallable: None,
        door: Some(door_math_graph_core_graph),
    },
    Function {
        name: "math.graph.edge_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "Extracts the network of corners and edges outlining every filled site.\n\n# Errors\n\nErrors off two and three dimensions.",
        uncallable: None,
        door: Some(door_math_graph_edge_graph),
    },
    Function {
        name: "math.graph.fractal_dimension",
        signature: "(network: math.graph.Network, samples: usize) -> f64",
        docs: "Estimates the box-counting dimension of the node cloud over a ladder of halving boxes, one rung per sample.",
        uncallable: None,
        door: Some(door_math_graph_fractal_dimension),
    },
    Function {
        name: "math.graph.junctions",
        signature: "(network: math.graph.Network) -> usize",
        docs: "Counts the nodes of degree three or more.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_junctions),
    },
    Function {
        name: "math.graph.largest_component",
        signature: "(network: math.graph.Network) -> math.graph.Network",
        docs: "Extracts the largest connected piece as a network of its own, branches re-indexed.\n\nTies go to the piece whose lowest node index comes first. An empty network comes back empty.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_largest_component),
    },
    Function {
        name: "math.graph.roles",
        signature: "(network: math.graph.Network) -> [math.graph.Role]",
        docs: "Tags every node by its degree, indexed like the node list.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_roles),
    },
    Function {
        name: "math.graph.tips",
        signature: "(network: math.graph.Network) -> usize",
        docs: "Counts the nodes of degree one.\n\n# Errors\n\nErrors when a branch names a node the network does not hold.",
        uncallable: None,
        door: Some(door_math_graph_tips),
    },
    Function {
        name: "math.graph.total_length",
        signature: "(network: math.graph.Network) -> f64",
        docs: "Sums the straight-line lengths of every branch.",
        uncallable: None,
        door: Some(door_math_graph_total_length),
    },
    Function {
        name: "math.graph.tunnel_graph",
        signature: "(grid: Tensor) -> math.graph.Network",
        docs: "Extracts the core graph of the inverted grid, joining empty sites instead.\n\n# Errors\n\nErrors off two and three dimensions.",
        uncallable: None,
        door: Some(door_math_graph_tunnel_graph),
    },
    Function {
        name: "math.ladder.Design.abscissa",
        signature: "(self: math.ladder.Design) -> f64",
        docs: "Returns the abscissa `alpha = log_q k`.",
        uncallable: None,
        door: Some(door_math_ladder_design_abscissa),
    },
    Function {
        name: "math.ladder.Design.base",
        signature: "(self: math.ladder.Design) -> u64",
        docs: "Returns the base.",
        uncallable: None,
        door: Some(door_math_ladder_design_base),
    },
    Function {
        name: "math.ladder.Design.digits",
        signature: "(self: math.ladder.Design) -> [u64]",
        docs: "Returns the digit set, ascending.",
        uncallable: None,
        door: Some(door_math_ladder_design_digits),
    },
    Function {
        name: "math.ladder.Design.new",
        signature: "(base: u64, digits: [u64]) -> math.ladder.Design",
        docs: "Builds a design on the base and the digit set, choosing the peel depth.\n\n# Errors\n\nErrs at a base under two, at fewer than two digits, at a digit outside the base, or at an all-zero digit set.",
        uncallable: None,
        door: Some(door_math_ladder_design_new),
    },
    Function {
        name: "math.ladder.Design.peel",
        signature: "(self: math.ladder.Design) -> usize",
        docs: "Returns the peel depth.",
        uncallable: None,
        door: Some(door_math_ladder_design_peel),
    },
    Function {
        name: "math.ladder.Design.period",
        signature: "(self: math.ladder.Design) -> f64",
        docs: "Returns the pole spacing `2 pi / log q`.",
        uncallable: None,
        door: Some(door_math_ladder_design_period),
    },
    Function {
        name: "math.ladder.Design.pole",
        signature: "(self: math.ladder.Design, m: usize, j: i64) -> num.zeta.Complex",
        docs: "Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`.",
        uncallable: None,
        door: Some(door_math_ladder_design_pole),
    },
    Function {
        name: "math.ladder.Design.with_peel",
        signature: "(base: u64, digits: [u64], peel: usize) -> math.ladder.Design",
        docs: "Builds a design at an explicit peel depth, at least two.\n\n# Errors\n\nErrs at a bad base or digit set, and at a peel depth under two or past the exact integers.",
        uncallable: None,
        door: Some(door_math_ladder_design_with_peel),
    },
    Function {
        name: "math.ladder.cofactor",
        signature: "(design: math.ladder.Design, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to.\n\nThe cofactor is the ladder numerator over the finite polynomial, so it is analytic on `Re s > alpha - 1` and a zero census on it needs no pole-free strip.\nThe blind lattice here is `s = alpha - m + 2 pi i j / log q` for a whole `m >= 1`: the numerator itself is read at `m = 0`, which is what makes the residue available.\n\n# Errors\n\nErrs when the tolerance is out of reach, or when the point sits on the blind lattice.",
        uncallable: None,
        door: Some(door_math_ladder_cofactor),
    },
    Function {
        name: "math.ladder.residue",
        signature: "(design: math.ladder.Design, m: usize, j: i64, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to.\n\nAt `m = 0` the factor `1 - k q^(-s)` has derivative `log q`, so the residue is the ladder numerator over `log q`; the column below comes off the same recursion, `(1 - q^m) R_m = sum_(l = 1)^m binom(-s_(m,j), l) q^(-s_(m,j)-l) gamma_l R_(m-l)`.\nThe head term `(1 - k q^(-s)) D_(P-1)(s)` is dropped, since the factor vanishes at the pole; in double precision the factor is not exactly zero, and the residue of the dropped term, about `1e-15` at base 10, is not charged separately and sits inside the carried scale.\n\n# Errors\n\nErrs when the tolerance is out of reach.",
        uncallable: None,
        door: Some(door_math_ladder_residue),
    },
    Function {
        name: "math.ladder.zeta",
        signature: "(design: math.ladder.Design, s: num.zeta.Complex, tolerance: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns `zeta_F(s)` and the bound it is known to.\n\nThe bound is the propagated truncation bound, which is proved, plus `ROUNDING` times the scale the ladder carries, which is a measured allowance; the acceptance test charges every term the returned bound carries, the Dirichlet polynomial's own scale included, so the returned bound is never above the tolerance asked.\nThe recursion walks `w = s + j` upward and divides by `1 - k q^(-w)`, so it cannot read `s = alpha - m + 2 pi i j / log q` for a whole `m >= 0`: at those points it raises and names the pole. On a full digit set that lattice is `s = 1, 0, -1, -2, ...`, where `zeta_F` is `zeta` and only `s = 1` is singular.\n\n# Errors\n\nErrs when the tolerance is out of reach, or when the point sits on the blind lattice.",
        uncallable: None,
        door: Some(door_math_ladder_zeta),
    },
    Function {
        name: "math.memory.Rule.accepts",
        signature: "(self: math.memory.Rule, word: [usize]) -> bool",
        docs: "Returns whether a word, coarsest digit first, is accepted.",
        uncallable: None,
        door: Some(door_math_memory_rule_accepts),
    },
    Function {
        name: "math.memory.Rule.allowed",
        signature: "(self: math.memory.Rule, window: usize) -> bool",
        docs: "Returns whether the window is allowed, and false for any window out of range.",
        uncallable: None,
        door: Some(door_math_memory_rule_allowed),
    },
    Function {
        name: "math.memory.Rule.alphabet",
        signature: "(self: math.memory.Rule) -> [usize]",
        docs: "Returns the letters that stand in at least one allowed window.",
        uncallable: None,
        door: Some(door_math_memory_rule_alphabet),
    },
    Function {
        name: "math.memory.Rule.codes",
        signature: "(self: math.memory.Rule) -> u128",
        docs: "Returns the count of rules of this shape, `2^(2^(k D))`.",
        uncallable: None,
        door: Some(door_math_memory_rule_codes),
    },
    Function {
        name: "math.memory.Rule.full",
        signature: "(dimension: usize, width: usize) -> math.memory.Rule",
        docs: "Returns the rule that allows every window.\n\n# Errors\n\nErrs when the dimension is not one, two or three, the width is zero, or the window count overruns.",
        uncallable: None,
        door: Some(door_math_memory_rule_full),
    },
    Function {
        name: "math.memory.Rule.letters",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "Returns the letter count `2^D`, the digit vectors of the cube's corners.",
        uncallable: None,
        door: Some(door_math_memory_rule_letters),
    },
    Function {
        name: "math.memory.Rule.new",
        signature: "(dimension: usize, width: usize, code: u64) -> math.memory.Rule",
        docs: "Builds a rule from its dimension, its width and its code.\n\n# Errors\n\nErrs when the dimension is not one, two or three, the width is zero, the window count overruns, or the code runs past it.",
        uncallable: None,
        door: Some(door_math_memory_rule_new),
    },
    Function {
        name: "math.memory.Rule.states",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on.",
        uncallable: None,
        door: Some(door_math_memory_rule_states),
    },
    Function {
        name: "math.memory.Rule.windows",
        signature: "(self: math.memory.Rule) -> usize",
        docs: "Returns the window count `2^(k D)`.",
        uncallable: None,
        door: Some(door_math_memory_rule_windows),
    },
    Function {
        name: "math.memory.allowed_windows",
        signature: "(rule: math.memory.Rule) -> usize",
        docs: "Returns the count of allowed windows `card W`, the bits the code sets inside its window range.",
        uncallable: None,
        door: Some(door_math_memory_allowed_windows),
    },
    Function {
        name: "math.memory.cells",
        signature: "(rule: math.memory.Rule, level: usize) -> [u64]",
        docs: "Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first.\n\nThe index is row major, `z side^2 + y side + x` with `side = 2^L`, so at `k = 1` the set is the level-`L` fill of the design of the same code.",
        uncallable: None,
        door: Some(door_math_memory_cells),
    },
    Function {
        name: "math.memory.counts",
        signature: "(rule: math.memory.Rule, levels: usize) -> [u64]",
        docs: "Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`.\n\nFor `L < k` every word of that length is accepted, so the count is `2^(D L)`.",
        uncallable: None,
        door: Some(door_math_memory_counts),
    },
    Function {
        name: "math.memory.exponent",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count.",
        uncallable: None,
        door: Some(door_math_memory_exponent),
    },
    Function {
        name: "math.memory.kappa",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory.\n\nThe window budget `log_2(card W) / k` is what a word of length `mk` could carry if its `m` disjoint windows were free of each other, and `log_2 rho` is what it really carries, so `kappa >= 0` and the gap is the interference the rule pays for.\nIt is zero on every product rule `W = F^k`, where `card W = card F^k` and `rho = card F`, so every `k = 1` rule reads zero; it is `f64::INFINITY` when a window is allowed and nothing survives past the window, and zero when no window is allowed.",
        uncallable: None,
        door: Some(door_math_memory_kappa),
    },
    Function {
        name: "math.memory.perron",
        signature: "(rule: math.memory.Rule) -> f64",
        docs: "Returns the Perron root of the transfer matrix, the count's growth per level.\n\nThe digraph is split into its strongly connected components first; a component carrying no cycle contributes nothing, and a rule whose components all die past the window returns exactly `0`.\nEach cyclic component is irreducible, so `I + B` is primitive and the power iteration on it converges geometrically; the iteration stops on a sustained Cauchy move of the normalised vector, never on a plateau of one scalar, and the Collatz-Wielandt ratios bracket the root.\nThe estimate is then made exact against the component's characteristic polynomial, taken by integer Faddeev-LeVerrier: an integer root is returned exactly, and otherwise the simple Perron root is bisected to the resolution of an `f64`.\nThe answer is the largest component root.",
        uncallable: None,
        door: Some(door_math_memory_perron),
    },
    Function {
        name: "math.memory.transfer",
        signature: "(rule: math.memory.Rule) -> [[u64]]",
        docs: "Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed.\n\nAt `k = 1` the one state is the empty window and the single entry counts the allowed letters.",
        uncallable: None,
        door: Some(door_math_memory_transfer),
    },
    Function {
        name: "math.moire.Field.as_f64",
        signature: "(self: math.moire.Field) -> [f64]",
        docs: "Returns the samples widened to f64.",
        uncallable: None,
        door: Some(door_math_moire_field_as_f64),
    },
    Function {
        name: "math.moire.Field.from_data",
        signature: "(data: [f32], size: usize) -> math.moire.Field",
        docs: "Wraps row-major samples of the given side.\n\n# Errors\n\nErrors when the count is not the side squared.",
        uncallable: None,
        door: Some(door_math_moire_field_from_data),
    },
    Function {
        name: "math.moire.Field.max",
        signature: "(self: math.moire.Field) -> f32",
        docs: "Returns the largest sample.",
        uncallable: None,
        door: Some(door_math_moire_field_max),
    },
    Function {
        name: "math.moire.Field.mean",
        signature: "(self: math.moire.Field) -> f64",
        docs: "Returns the mean sample, or zero for an empty field.",
        uncallable: None,
        door: Some(door_math_moire_field_mean),
    },
    Function {
        name: "math.moire.Field.min",
        signature: "(self: math.moire.Field) -> f32",
        docs: "Returns the smallest sample.",
        uncallable: None,
        door: Some(door_math_moire_field_min),
    },
    Function {
        name: "math.moire.Field.new",
        signature: "(size: usize) -> math.moire.Field",
        docs: "Builds a zeroed field of the given side.",
        uncallable: None,
        door: Some(door_math_moire_field_new),
    },
    Function {
        name: "math.moire.Field.normalized",
        signature: "(self: math.moire.Field, symmetric: bool) -> [f32]",
        docs: "Returns the samples scaled into 0..1, symmetric about zero on request.",
        uncallable: None,
        door: Some(door_math_moire_field_normalized),
    },
    Function {
        name: "math.moire.Layer.new",
        signature: "(spec: math.moire.Spec, number: usize) -> math.moire.Layer",
        docs: "Builds a layer at level 1 on a 512-pixel square lattice.",
        uncallable: None,
        door: Some(door_math_moire_layer_new),
    },
    Function {
        name: "math.moire.Preset.carpet",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "The carpet stack: every base-three corner but the centre, summed over odd scales.",
        uncallable: None,
        door: Some(door_math_moire_preset_carpet),
    },
    Function {
        name: "math.moire.Preset.field",
        signature: "(self: math.moire.Preset, size: usize) -> math.moire.Field",
        docs: "Samples the preset into a square field of the given side.\n\n# Errors\n\nErrors when the preset's code is out of range.",
        uncallable: Some("math::moire::Preset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.moire.Preset.heatmap",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "The parity heatmap: odd scales of the low corner summed on the square lattice.",
        uncallable: None,
        door: Some(door_math_moire_preset_heatmap),
    },
    Function {
        name: "math.moire.Preset.hive",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "The hive: the parity heatmap sampled on the hexagonal lattice.",
        uncallable: None,
        door: Some(door_math_moire_preset_hive),
    },
    Function {
        name: "math.moire.Preset.weave",
        signature: "(limit: usize) -> math.moire.Preset",
        docs: "The parity weave: the same odd scales folded to their parity instead of summed.",
        uncallable: None,
        door: Some(door_math_moire_preset_weave),
    },
    Function {
        name: "math.moire.Spec.new",
        signature: "(code: u128, base: usize, dimension: usize) -> math.moire.Spec",
        docs: "Builds a spec from a code, base and dimension.",
        uncallable: None,
        door: Some(door_math_moire_spec_new),
    },
    Function {
        name: "math.moire.Volume.at",
        signature: "(self: math.moire.Volume, x: usize, y: usize, z: usize) -> f32",
        docs: "Reads the sample at a voxel.",
        uncallable: None,
        door: Some(door_math_moire_volume_at),
    },
    Function {
        name: "math.moire.Volume.count",
        signature: "(self: math.moire.Volume, level: f32) -> usize",
        docs: "Counts the samples at or above the level.",
        uncallable: None,
        door: Some(door_math_moire_volume_count),
    },
    Function {
        name: "math.moire.Volume.from_data",
        signature: "(data: [f32], size: usize) -> math.moire.Volume",
        docs: "Wraps x-major samples of the side.\n\n# Errors\n\nErrors when the count is not the side cubed.",
        uncallable: None,
        door: Some(door_math_moire_volume_from_data),
    },
    Function {
        name: "math.moire.Volume.max",
        signature: "(self: math.moire.Volume) -> f32",
        docs: "Returns the largest sample.",
        uncallable: None,
        door: Some(door_math_moire_volume_max),
    },
    Function {
        name: "math.moire.Volume.min",
        signature: "(self: math.moire.Volume) -> f32",
        docs: "Returns the smallest sample.",
        uncallable: None,
        door: Some(door_math_moire_volume_min),
    },
    Function {
        name: "math.moire.Volume.new",
        signature: "(size: usize) -> math.moire.Volume",
        docs: "Builds a zeroed volume of the side.",
        uncallable: None,
        door: Some(door_math_moire_volume_new),
    },
    Function {
        name: "math.moire.Volume.plane",
        signature: "(self: math.moire.Volume, frame: math.moire.Frame, out: usize) -> ([f32], [u8])",
        docs: "Samples the plane of the frame on an out by out window: the values row by row, and one byte per pixel saying whether it lies inside the cube.",
        uncallable: None,
        door: Some(door_math_moire_volume_plane),
    },
    Function {
        name: "math.moire.Volume.sample",
        signature: "(self: math.moire.Volume, p: [f64; 3]) -> f32?",
        docs: "Reads the voxel a point of the unit cube falls in, or zero outside it.",
        uncallable: None,
        door: Some(door_math_moire_volume_sample),
    },
    Function {
        name: "math.moire.Volume.solid",
        signature: "(self: math.moire.Volume, level: f32) -> Tensor",
        docs: "Thresholds into a byte tensor: one where a sample reaches the level, zero below.",
        uncallable: None,
        door: Some(door_math_moire_volume_solid),
    },
    Function {
        name: "math.moire.all",
        signature: "(limit: usize) -> [math.moire.Preset]",
        docs: "Returns every preset stacked up to the given scale.",
        uncallable: None,
        door: Some(door_math_moire_all),
    },
    Function {
        name: "math.moire.frame",
        signature: "(normal: [f64; 3], offset: f64) -> math.moire.Frame",
        docs: "Frames the plane normal to the direction, at the offset from zero to one across the box along it; the window is the smallest square holding every section on that normal.\n\n# Errors\n\nErrors on a zero normal.",
        uncallable: None,
        door: Some(door_math_moire_frame),
    },
    Function {
        name: "math.moire.layer",
        signature: "(params: math.moire.Layer) -> [bool]",
        docs: "Samples a design over the pixel grid into a boolean mask.\n\n# Errors\n\nErrors below dimension two, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_moire_layer),
    },
    Function {
        name: "math.moire.named",
        signature: "(name: str, limit: usize) -> math.moire.Preset",
        docs: "Returns the preset the name picks.\n\n# Errors\n\nErrors for a name that is not heatmap, weave, hive or carpet.",
        uncallable: None,
        door: Some(door_math_moire_named),
    },
    Function {
        name: "math.moire.pairs.correlation",
        signature: "(m: usize, n: usize) -> f64",
        docs: "Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid.\n\nA layer at scale n is lit where the row and the column of the n by n grid are not both odd.\nThe correlation is exactly zero when the two odd scales are coprime, and zero by convention below scale two, where a layer is constant.",
        uncallable: None,
        door: Some(door_math_moire_pairs_correlation),
    },
    Function {
        name: "math.moire.pairs.sampled",
        signature: "(m: usize, n: usize) -> f64",
        docs: "Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated.\n\n# Errors\n\nErrors when either scale is zero.",
        uncallable: None,
        door: Some(door_math_moire_pairs_sampled),
    },
    Function {
        name: "math.moire.pairs.witness",
        signature: "(scale: usize) -> math.moire.pairs.Witness",
        docs: "Puts an odd scale of three or more on trial against every earlier odd scale.\n\n# Errors\n\nErrors for a scale that is not odd and three or more.",
        uncallable: None,
        door: Some(door_math_moire_pairs_witness),
    },
    Function {
        name: "math.moire.render",
        signature: "(field: math.moire.Field, colorizer: core.Colorizer, levels: usize, symmetric: bool, invert: bool, scale: usize) -> Image",
        docs: "Quantizes a field into colored levels and renders them as an image, each sample a scale by scale block.\n\n# Errors\n\nErrors at scale zero.",
        uncallable: None,
        door: Some(door_math_moire_render),
    },
    Function {
        name: "math.moire.sample.axes",
        signature: "(size: usize, lattice: math.moire.Lattice, row: usize) -> ([f64], [f64])",
        docs: "Returns the two lattice coordinates of each pixel centre along a row.",
        uncallable: None,
        door: Some(door_math_moire_sample_axes),
    },
    Function {
        name: "math.moire.sample.membership",
        signature: "(code: u128, base: usize, dimension: usize) -> [bool]",
        docs: "Unpacks a code into its residue-corner truth table.\n\n# Errors\n\nErrors when the code is out of range for the dimension and base.",
        uncallable: None,
        door: Some(door_math_moire_sample_membership),
    },
    Function {
        name: "math.moire.sample.pack",
        signature: "(residues: [usize], base: usize) -> usize",
        docs: "Folds residues into a base-q index of the truth table.",
        uncallable: None,
        door: Some(door_math_moire_sample_pack),
    },
    Function {
        name: "math.moire.stack",
        signature: "(spec: math.moire.Spec, numbers: [usize], combine: math.moire.Combine, level: usize, lattice: math.moire.Lattice, size: usize, slices: [f64]) -> math.moire.Field",
        docs: "Layers one design at several side numbers into a field under the chosen combine.\n\n# Errors\n\nErrors below dimension two, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_moire_stack),
    },
    Function {
        name: "math.moire.stack_codes",
        signature: "(specs: [math.moire.Spec], number: usize, level: usize, lattice: math.moire.Lattice, size: usize, slices: [f64]) -> math.moire.Field",
        docs: "Sums layers of several designs at one side number into a field.\n\n# Errors\n\nErrors below dimension two, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_moire_stack_codes),
    },
    Function {
        name: "math.moire.volume",
        signature: "(spec: math.moire.Spec, numbers: [usize], combine: math.moire.Combine, level: usize, size: usize) -> math.moire.Volume",
        docs: "Layers one cube design at several side numbers into a volume under the chosen combine.\n\n# Errors\n\nErrors at size zero, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_moire_volume),
    },
    Function {
        name: "math.name.Bang.cells",
        signature: "(self: math.name.Bang) -> u32",
        docs: "Returns the number of digits the code addresses.\n\n# Errors\n\nErrors below dimension one, below base two, or past the u128 code space.",
        uncallable: None,
        door: Some(door_math_name_bang_cells),
    },
    Function {
        name: "math.name.Bang.checked",
        signature: "(self: math.name.Bang) -> math.name.Bang",
        docs: "Folds a decoded value to its canonical form, or an error for one outside the kind.",
        uncallable: None,
        door: Some(door_math_name_bang_checked),
    },
    Function {
        name: "math.name.Bang.from_file",
        signature: "(text: str) -> math.name.Bang",
        docs: "Reads a filename back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_bang_from_file),
    },
    Function {
        name: "math.name.Bang.from_json",
        signature: "(text: str) -> math.name.Bang",
        docs: "Reads a JSON object into its canonical value, or an error naming the broken key.",
        uncallable: None,
        door: Some(door_math_name_bang_from_json),
    },
    Function {
        name: "math.name.Bang.from_url",
        signature: "(text: str) -> math.name.Bang",
        docs: "Reads a path and query string back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_bang_from_url),
    },
    Function {
        name: "math.name.Bang.new",
        signature: "(code: u128, dim: usize, base: usize) -> math.name.Bang",
        docs: "Pins a code to its dimension and base on the square lattice.",
        uncallable: None,
        door: Some(door_math_name_bang_new),
    },
    Function {
        name: "math.name.Bang.to_file",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_bang_to_file),
    },
    Function {
        name: "math.name.Bang.to_id",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the first eight hex digits of the sha256 of the canonical JSON.",
        uncallable: None,
        door: Some(door_math_name_bang_to_id),
    },
    Function {
        name: "math.name.Bang.to_json",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the canonical JSON object.",
        uncallable: None,
        door: Some(door_math_name_bang_to_json),
    },
    Function {
        name: "math.name.Bang.to_prose",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_bang_to_prose),
    },
    Function {
        name: "math.name.Bang.to_url",
        signature: "(self: math.name.Bang) -> String",
        docs: "Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_bang_to_url),
    },
    Function {
        name: "math.name.Lattice.default",
        signature: "() -> math.name.Lattice",
        docs: "Returns the default Lattice.",
        uncallable: None,
        door: Some(door_math_name_lattice_default),
    },
    Function {
        name: "math.name.Lattice.is_square",
        signature: "(self: math.name.Lattice) -> bool",
        docs: "Returns whether this is the square lattice.",
        uncallable: None,
        door: Some(door_math_name_lattice_is_square),
    },
    Function {
        name: "math.name.Lattice.units",
        signature: "(self: math.name.Lattice) -> usize",
        docs: "Returns the number of unit directions a twist may pick from.",
        uncallable: None,
        door: Some(door_math_name_lattice_units),
    },
    Function {
        name: "math.name.Sequence.checked",
        signature: "(self: math.name.Sequence) -> math.name.Sequence",
        docs: "Folds a decoded value to its canonical form, or an error for one outside the kind.",
        uncallable: None,
        door: Some(door_math_name_sequence_checked),
    },
    Function {
        name: "math.name.Sequence.design",
        signature: "(self: math.name.Sequence) -> math.name.Bang",
        docs: "Returns the design pinned to its dimension and base.",
        uncallable: None,
        door: Some(door_math_name_sequence_design),
    },
    Function {
        name: "math.name.Sequence.from_file",
        signature: "(text: str) -> math.name.Sequence",
        docs: "Reads a filename back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_sequence_from_file),
    },
    Function {
        name: "math.name.Sequence.from_json",
        signature: "(text: str) -> math.name.Sequence",
        docs: "Reads a JSON object into its canonical value, or an error naming the broken key.",
        uncallable: None,
        door: Some(door_math_name_sequence_from_json),
    },
    Function {
        name: "math.name.Sequence.from_url",
        signature: "(text: str) -> math.name.Sequence",
        docs: "Reads a path and query string back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_sequence_from_url),
    },
    Function {
        name: "math.name.Sequence.new",
        signature: "(code: u128, dim: usize, base: usize, measure: str, axis: str) -> math.name.Sequence",
        docs: "Pins a design's reading to its measure and axis.",
        uncallable: None,
        door: Some(door_math_name_sequence_new),
    },
    Function {
        name: "math.name.Sequence.to_file",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_file),
    },
    Function {
        name: "math.name.Sequence.to_id",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the first eight hex digits of the sha256 of the canonical JSON.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_id),
    },
    Function {
        name: "math.name.Sequence.to_json",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the canonical JSON object.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_json),
    },
    Function {
        name: "math.name.Sequence.to_prose",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_prose),
    },
    Function {
        name: "math.name.Sequence.to_url",
        signature: "(self: math.name.Sequence) -> String",
        docs: "Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_sequence_to_url),
    },
    Function {
        name: "math.name.Word.bases",
        signature: "(self: math.name.Word) -> [usize]",
        docs: "Returns the base of every letter, 2 where the name says nothing.",
        uncallable: None,
        door: Some(door_math_name_word_bases),
    },
    Function {
        name: "math.name.Word.checked",
        signature: "(self: math.name.Word) -> math.name.Word",
        docs: "Folds a decoded value to its canonical form, or an error for one outside the kind.",
        uncallable: None,
        door: Some(door_math_name_word_checked),
    },
    Function {
        name: "math.name.Word.from_file",
        signature: "(text: str) -> math.name.Word",
        docs: "Reads a filename back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_word_from_file),
    },
    Function {
        name: "math.name.Word.from_json",
        signature: "(text: str) -> math.name.Word",
        docs: "Reads a JSON object into its canonical value, or an error naming the broken key.",
        uncallable: None,
        door: Some(door_math_name_word_from_json),
    },
    Function {
        name: "math.name.Word.from_url",
        signature: "(text: str) -> math.name.Word",
        docs: "Reads a path and query string back into the value, or an error.",
        uncallable: None,
        door: Some(door_math_name_word_from_url),
    },
    Function {
        name: "math.name.Word.letters",
        signature: "(self: math.name.Word) -> [math.name.Bang]",
        docs: "Returns every letter as a design pinned to the word's dimension and its own base.",
        uncallable: None,
        door: Some(door_math_name_word_letters),
    },
    Function {
        name: "math.name.Word.new",
        signature: "(dim: usize, letters: [(u128, usize)]) -> math.name.Word",
        docs: "Pins an ordered letter list at base 2.\n\n# Errors\n\nErrors below two letters, or outside the code range.",
        uncallable: None,
        door: Some(door_math_name_word_new),
    },
    Function {
        name: "math.name.Word.to_file",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_word_to_file),
    },
    Function {
        name: "math.name.Word.to_id",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the first eight hex digits of the sha256 of the canonical JSON.",
        uncallable: None,
        door: Some(door_math_name_word_to_id),
    },
    Function {
        name: "math.name.Word.to_json",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the canonical JSON object.",
        uncallable: None,
        door: Some(door_math_name_word_to_json),
    },
    Function {
        name: "math.name.Word.to_prose",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_word_to_prose),
    },
    Function {
        name: "math.name.Word.to_url",
        signature: "(self: math.name.Word) -> String",
        docs: "Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.",
        uncallable: None,
        door: Some(door_math_name_word_to_url),
    },
    Function {
        name: "math.name.of",
        signature: "(code: Code, dim: usize, base: usize) -> String?",
        docs: "Returns the word of the first named design a code draws in the plane or the cube, or None for a code no named design draws.",
        uncallable: None,
        door: Some(door_math_name_of),
    },
    Function {
        name: "math.name.words",
        signature: "(dim: usize, base: usize) -> [(String, String)]",
        docs: "Lists every classic then anti design of the plane or the cube as its decimal code and its word, empty for any other dimension.",
        uncallable: None,
        door: Some(door_math_name_words),
    },
    Function {
        name: "math.nodal.figures",
        signature: "(code: u64, number: usize, level: usize, count: usize) -> math.nodal.Figures",
        docs: "Rings the plate cut as the design `math::two::create(code, number, level, 0, 2)`: its cells joined to their four neighbours, free at the rim and at every hole, the combinatorial Laplacian `D - A`. Returns the lowest `count` modes, `count` clamped to the cells and to 64, each in a canonical basis: eigenvalues within `1e-8` form a cluster, split by the design's transpose, left-right and up-down mirrors in that order and then by the diagonal `x + sqrt(2) y`, each figure projected exactly onto its mirror parities and signed so its first largest cell is positive. A figure's domains are the 4-connected runs of one strict sign, cells within `1e-9` of its peak left out; its bound is the 1-based index of the last member of its cluster, the last cluster closed before the cut.\n\n# Errors\n\nErrs on what `holds` refuses, or when the solver does not settle.",
        uncallable: None,
        door: Some(door_math_nodal_figures),
    },
    Function {
        name: "math.nodal.holds",
        signature: "(code: u64, number: usize, level: usize) -> bool",
        docs: "Answers whether a 2D base-2 design cuts a plate the figures can ring: one 4-connected piece of 16 to 10,000 cells on a side of at most 128. The cells are counted before the design is built.\n\n# Errors\n\nErrs when the code is not a 2D base-2 code, 0 to 15.",
        uncallable: None,
        door: Some(door_math_nodal_holds),
    },
    Function {
        name: "math.press.Press.add",
        signature: "(self: math.press.Press, number: u128, weight: i128) -> null",
        docs: "Adds a weighted number to its usage bucket.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "math.press.Press.new",
        signature: "(dimension: usize, base: usize) -> math.press.Press",
        docs: "Builds an empty press over every design of the dimension and base.\n\n# Errors\n\nErrors past twenty corners, where the bucket table would leave a million rows.",
        uncallable: None,
        door: Some(door_math_press_press_new),
    },
    Function {
        name: "math.press.Press.total",
        signature: "(self: math.press.Press, code: Code) -> i128",
        docs: "Returns the total weight the design at a code has collected.",
        uncallable: None,
        door: Some(door_math_press_press_total),
    },
    Function {
        name: "math.press.Press.totals",
        signature: "(self: math.press.Press) -> [i128]",
        docs: "Returns every design's total in code order by one subset-sum transform.",
        uncallable: None,
        door: Some(door_math_press_press_totals),
    },
    Function {
        name: "math.press.containing",
        signature: "(number: u128, dimension: usize, base: usize) -> u128",
        docs: "Returns the number of designs of the dimension and base that contain the number.\n\nA design contains the number exactly when it fills every used corner, so the count\nis two to the free corners, and the average membership over all designs is one over\ntwo to the distinct-vector count.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_containing),
    },
    Function {
        name: "math.press.coordinates",
        signature: "(number: u128, dimension: usize, base: usize) -> [u128]",
        docs: "Splits a number into its dimension coordinates, one base digit peeled per axis in parallel.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_coordinates),
    },
    Function {
        name: "math.press.count_below",
        signature: "(code: Code, dimension: usize, base: usize, limit: u128) -> u128",
        docs: "Counts the members of a design below the limit.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_count_below),
    },
    Function {
        name: "math.press.distinct",
        signature: "(number: u128, dimension: usize, base: usize) -> u32",
        docs: "Returns the count of distinct digit vectors the number uses.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_distinct),
    },
    Function {
        name: "math.press.interleave",
        signature: "(coords: [u128], base: usize) -> u128",
        docs: "Weaves dimension coordinates back into their single interleaved number.\n\n# Errors\n\nErrors on an empty coordinate list, past a hundred and twenty-seven corners, or when the woven number passes a u128.",
        uncallable: None,
        door: Some(door_math_press_interleave),
    },
    Function {
        name: "math.press.layer_table",
        signature: "(layer: math.bang.MagicLayer) -> [bool]",
        docs: "Returns the allowed digit table of one magic layer, one flag per cell of its tile.\n\nA cell is allowed when its coordinate residues form a filled corner, which is the\ntile the layer renders read as a digit alphabet.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_press_layer_table),
    },
    Function {
        name: "math.press.member",
        signature: "(code: Code, number: u128, dimension: usize, base: usize) -> bool",
        docs: "Returns whether every digit vector of the number lies in the design.\n\nThis is the scalar membership rule of the sequence press: the number's base\n`base` to the dimension digits, read as residue corners, must all be filled.\nAt dimension one it is the classic restricted-digit set.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_member),
    },
    Function {
        name: "math.press.members",
        signature: "(code: Code, dimension: usize, base: usize, count: usize) -> [u128]",
        docs: "Returns the first members of a design in ascending order.\n\nStops early where the next member would pass a hundred and twenty-eight bits.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_members),
    },
    Function {
        name: "math.press.profile",
        signature: "(code: Code, dimension: usize, base: usize, level: usize) -> [u128]",
        docs: "Returns the diagonal slice profile of one design pressed to a fractal level.\n\n# Errors\n\nErrors below level one, or on a code out of range.",
        uncallable: None,
        door: Some(door_math_press_profile),
    },
    Function {
        name: "math.press.usage",
        signature: "(number: u128, dimension: usize, base: usize) -> Code",
        docs: "Returns the corner-usage mask of a number, one bit per digit vector its expansion uses.\n\nA number is read in base `base` to the power of the dimension, so each digit is one\nresidue corner of the design cube, and zero uses exactly the zero corner.\n\n# Errors\n\nErrors when the dimension and base reach a hundred and twenty-eight corners.",
        uncallable: None,
        door: Some(door_math_press_usage),
    },
    Function {
        name: "math.press.word_count",
        signature: "(layers: [math.bang.MagicLayer]) -> u128",
        docs: "Counts the members of a magic word from its layer fills, without enumeration.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_press_word_count),
    },
    Function {
        name: "math.press.word_member",
        signature: "(layers: [math.bang.MagicLayer], number: u128) -> bool",
        docs: "Returns whether the number lies in the magic word's composed design.\n\nThe number is read in the word's mixed radix, one digit per layer with the first\nlayer most significant, and every digit must land on an allowed cell of its tile.\nA number past the word's domain is an error.\n\n# Errors\n\nErrors on a code out of range, or a number past the word's domain.",
        uncallable: None,
        door: Some(door_math_press_word_member),
    },
    Function {
        name: "math.press.word_members",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "Enumerates every member of the magic word in ascending order.\n\nThe member count is the product of the layer fills, so measure with `word_count`\nbefore pressing a word too rich to hold.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_press_word_members),
    },
    Function {
        name: "math.press.word_profile",
        signature: "(layers: [math.bang.MagicLayer]) -> [u128]",
        docs: "Returns the diagonal slice profile of a magic word by the substitution product.\n\nThe profile of a tile lists, per coordinate sum, its filled cells, and the profile\nof a Kronecker word is the product of its layer profiles with strides, so no cell\nof the composed design is ever enumerated.\n\n# Errors\n\nErrors when a layer's code is out of range.",
        uncallable: None,
        door: Some(door_math_press_word_profile),
    },
    Function {
        name: "math.radix.Base.class",
        signature: "(self: math.radix.Base, z: (i64, i64)) -> usize",
        docs: "Returns the index in the canonical residue system of the class of a point.\n\n# Errors\n\nErrs when the residue system is short, or when the point lies in no class of it.",
        uncallable: None,
        door: Some(door_math_radix_base_class),
    },
    Function {
        name: "math.radix.Base.congruent",
        signature: "(self: math.radix.Base, z: (i64, i64), w: (i64, i64)) -> bool",
        docs: "Returns whether two points are congruent modulo the base.",
        uncallable: None,
        door: Some(door_math_radix_base_congruent),
    },
    Function {
        name: "math.radix.Base.group",
        signature: "(self: math.radix.Base) -> [[usize]]",
        docs: "Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base.\n\nMultiplying every digit by a unit `v` carries the attractor of a design to `v` times that attractor over the same base, so the unit group acts; conjugation carries the base to its conjugate and joins the group exactly when that is an associate.\n\nThe list is the Burnside multiset, one entry per abstract group element, and not the order of the permutation group it induces: the action need not be faithful, so entries repeat, and the acting image is the deduplicated list.\n\n# Errors\n\nErrs when the residue system is short, or when an image lies in no class of it.",
        uncallable: None,
        door: Some(door_math_radix_base_group),
    },
    Function {
        name: "math.radix.Base.mirrored",
        signature: "(self: math.radix.Base) -> bool",
        docs: "Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group.",
        uncallable: None,
        door: Some(door_math_radix_base_mirrored),
    },
    Function {
        name: "math.radix.Base.new",
        signature: "(ring: num.gauss.Ring, value: (i64, i64)) -> math.radix.Base",
        docs: "Fixes a base in a ring.\n\n# Errors\n\nErrs below norm two.",
        uncallable: None,
        door: Some(door_math_radix_base_new),
    },
    Function {
        name: "math.radix.Base.norm",
        signature: "(self: math.radix.Base) -> u64",
        docs: "Returns the norm `q` of the base: the count of residue classes and the square of the scale.",
        uncallable: None,
        door: Some(door_math_radix_base_norm),
    },
    Function {
        name: "math.radix.Base.power",
        signature: "(self: math.radix.Base, level: usize) -> (i64, i64)",
        docs: "Returns the base raised to a level.",
        uncallable: None,
        door: Some(door_math_radix_base_power),
    },
    Function {
        name: "math.radix.Base.residues",
        signature: "(self: math.radix.Base) -> [(i64, i64)]",
        docs: "Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`.\n\nThe system is built greedily over every point of norm at most `q` sorted by norm and then by argument, which reaches every class because the covering radius of the lattice `b R` is `|b| / sqrt(2)` on the square lattice and `|b| / sqrt(3)` on the hexagonal.\n\n# Errors\n\nErrs when the greedy sweep finds fewer classes than the norm.",
        uncallable: None,
        door: Some(door_math_radix_base_residues),
    },
    Function {
        name: "math.radix.Base.ring",
        signature: "(self: math.radix.Base) -> num.gauss.Ring",
        docs: "Returns the ring.",
        uncallable: None,
        door: Some(door_math_radix_base_ring),
    },
    Function {
        name: "math.radix.Base.value",
        signature: "(self: math.radix.Base) -> (i64, i64)",
        docs: "Returns the base element.",
        uncallable: None,
        door: Some(door_math_radix_base_value),
    },
    Function {
        name: "math.radix.Radix.base",
        signature: "(self: math.radix.Radix) -> math.radix.Base",
        docs: "Returns the base.",
        uncallable: None,
        door: Some(door_math_radix_radix_base),
    },
    Function {
        name: "math.radix.Radix.canonical",
        signature: "(self: math.radix.Radix) -> bool",
        docs: "Returns whether every digit is the canonical representative of its class.\n\n# Errors\n\nErrs when the residue system is short.",
        uncallable: None,
        door: Some(door_math_radix_radix_canonical),
    },
    Function {
        name: "math.radix.Radix.code",
        signature: "(self: math.radix.Radix) -> u128",
        docs: "Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives.\n\n# Errors\n\nErrs when the residue system is short, or when a digit lies in no class of it.",
        uncallable: None,
        door: Some(door_math_radix_radix_code),
    },
    Function {
        name: "math.radix.Radix.digits",
        signature: "(self: math.radix.Radix) -> [(i64, i64)]",
        docs: "Returns the digits.",
        uncallable: None,
        door: Some(door_math_radix_radix_digits),
    },
    Function {
        name: "math.radix.Radix.dimension",
        signature: "(self: math.radix.Radix) -> f64",
        docs: "Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base.",
        uncallable: None,
        door: Some(door_math_radix_radix_dimension),
    },
    Function {
        name: "math.radix.Radix.distinct",
        signature: "(self: math.radix.Radix, level: usize) -> usize",
        docs: "Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point.",
        uncallable: None,
        door: Some(door_math_radix_radix_distinct),
    },
    Function {
        name: "math.radix.Radix.fill",
        signature: "(self: math.radix.Radix, level: usize) -> u128",
        docs: "Returns the count of words of a level, `|F|^L`.",
        uncallable: None,
        door: Some(door_math_radix_radix_fill),
    },
    Function {
        name: "math.radix.Radix.from_code",
        signature: "(base: math.radix.Base, code: u128) -> math.radix.Radix",
        docs: "Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`.\n\n# Errors\n\nErrs when the class count passes 128, and when the code runs past the classes.",
        uncallable: None,
        door: Some(door_math_radix_radix_from_code),
    },
    Function {
        name: "math.radix.Radix.new",
        signature: "(base: math.radix.Base, digits: [(i64, i64)], twists: [(i64, i64)]) -> math.radix.Radix",
        docs: "Builds a design from a base, a digit list and a unit twist per digit.\n\nPairwise incongruent digits are the hypothesis of the untwisted fill law: they are what recovers the last digit from the word read modulo the base, so a repeated class is refused here rather than silently gluing words.\n\n# Errors\n\nErrs on a length mismatch, on a twist whose norm is not one, and on two digits congruent modulo the base.",
        uncallable: None,
        door: Some(door_math_radix_radix_new),
    },
    Function {
        name: "math.radix.Radix.plane",
        signature: "(self: math.radix.Radix, level: usize) -> [(f64, f64)]",
        docs: "Returns the level-`L` points in the plane, the scaled words divided by `b^L`.",
        uncallable: None,
        door: Some(door_math_radix_radix_plane),
    },
    Function {
        name: "math.radix.Radix.ring",
        signature: "(self: math.radix.Radix) -> num.gauss.Ring",
        docs: "Returns the ring.",
        uncallable: None,
        door: Some(door_math_radix_radix_ring),
    },
    Function {
        name: "math.radix.Radix.size",
        signature: "(self: math.radix.Radix) -> usize",
        docs: "Returns the digit count `|F|`.",
        uncallable: None,
        door: Some(door_math_radix_radix_size),
    },
    Function {
        name: "math.radix.Radix.twists",
        signature: "(self: math.radix.Radix) -> [(i64, i64)]",
        docs: "Returns the twists.",
        uncallable: None,
        door: Some(door_math_radix_radix_twists),
    },
    Function {
        name: "math.radix.Radix.with_twists",
        signature: "(self: math.radix.Radix, units: [usize]) -> math.radix.Radix",
        docs: "Returns the design with the twists named by their index in the unit list, the units in turning order from one.\n\n# Errors\n\nErrs at a twist index past the unit list.",
        uncallable: None,
        door: Some(door_math_radix_radix_with_twists),
    },
    Function {
        name: "math.radix.Radix.words",
        signature: "(self: math.radix.Radix, level: usize) -> [(i64, i64)]",
        docs: "Returns the level-`L` points in exact ring coordinates scaled by `b^L`.\n\nThe place map of digit `d` is `phi_d(x) = (u_d x + d) / b`, a word `d_1 ... d_L` lands on `phi_(d_1)(... phi_(d_L)(0))`, and that point times `b^L` is `sum_(j=1..L) (prod_(i<j) u_(d_i)) d_j b^(L-j)`, which stays in the ring. Words come in digit-lexicographic order, the first digit slowest.",
        uncallable: None,
        door: Some(door_math_radix_radix_words),
    },
    Function {
        name: "math.radix.flowsnake",
        signature: "() -> math.radix.Radix",
        docs: "Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_flowsnake),
    },
    Function {
        name: "math.radix.gasket",
        signature: "() -> math.radix.Radix",
        docs: "Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_gasket),
    },
    Function {
        name: "math.radix.koch",
        signature: "() -> math.radix.Radix",
        docs: "Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`.\n\nThe digits are not the canonical residues: `2` and `-1` share a class and the canonical system holds `-1`, so the code alone does not name this design.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_koch),
    },
    Function {
        name: "math.radix.terdragon",
        signature: "() -> math.radix.Radix",
        docs: "Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`.\n\nThe untwisted code misses the curve: reading the L-system `F -> F + F - F` at `120` degrees as a turtle, three segments to a level, and normalising by the endpoint gives the segment starts word for word only under this twist.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_terdragon),
    },
    Function {
        name: "math.radix.tile",
        signature: "(m: u64, code: u128) -> math.radix.Radix",
        docs: "Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`.\n\nBit `y m + x` of the code is the cell at row `y` and column `x` of the `m` by `m` tile, the column the real part and the row the imaginary part, so the level-`L` words scaled by `m^L` are exactly the filled cells of the level-`L` tile read as `(column, row)`.\n\n# Errors\n\nErrs below a side of two, at a tile of 128 cells or more, and at a code running past the cells.",
        uncallable: None,
        door: Some(door_math_radix_tile),
    },
    Function {
        name: "math.radix.twindragon",
        signature: "() -> math.radix.Radix",
        docs: "Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`.\n\n# Errors\n\nThe fixed design is in range, so this never errs.",
        uncallable: None,
        door: Some(door_math_radix_twindragon),
    },
    Function {
        name: "math.roulette.Nodes.default",
        signature: "() -> math.roulette.Nodes",
        docs: "Returns the default Nodes.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_default),
    },
    Function {
        name: "math.roulette.Nodes.pair",
        signature: "(self: math.roulette.Nodes, i: usize, j: usize) -> usize",
        docs: "How often the curves `i` and `j` cross, either order, and zero when they are one curve.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_pair),
    },
    Function {
        name: "math.roulette.Nodes.paired",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "Every crossing of two curves.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_paired),
    },
    Function {
        name: "math.roulette.Nodes.selved",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "Every self crossing.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_selved),
    },
    Function {
        name: "math.roulette.Nodes.total",
        signature: "(self: math.roulette.Nodes) -> usize",
        docs: "Every crossing, self and pair together, which counts a node where `n` branches meet `n(n - 1)/2` times; `points` is the count of distinct nodes and the two agree exactly when `crowded` is zero.",
        uncallable: None,
        door: Some(door_math_roulette_nodes_total),
    },
    Function {
        name: "math.roulette.nodes",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], samples: usize, tol: f64) -> math.roulette.Nodes",
        docs: "Counts the nodes of the roulette the pencils draw on the track: `mrly::math::spirograph::trace` at `samples` points a pencil, every pair of polyline segments tested for a proper crossing by orientation signs on a grid of buckets, and crossings within `tol` of the picture's longer side read as one node. A pair of segments is counted in one bucket alone, the first they share, so no crossing is counted twice; the sign of an orientation is `side`, exact for any endpoints whose two differences are exact, which two `f32` endpoints are while the picture's coordinates keep their exponents within 29 of one another, as these pictures do. A seat at the wheel's centre draws one circle `b` times over and the count is meaningless there, the passes crossing one another as the sampling wanders.\n\n# Errors\n\nErrors on an empty pencil list, under three samples, a tolerance outside zero to one, or a trace with no extent.",
        uncallable: None,
        door: Some(door_math_roulette_nodes),
    },
    Function {
        name: "math.roulette.side",
        signature: "(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> i32",
        docs: "Which side of the line from `a` to `b` the point `c` lies: plus one to the left, minus one to the right, zero on it. The sign is exact whenever the two differences `b - a` and `c - a` are exact, whatever the size of the products: the determinant is taken by the fused multiply-add identity of Kahan, whose error is at most twice the rounding unit times the determinant itself, so it can neither flip a sign nor invent one.",
        uncallable: None,
        door: Some(door_math_roulette_side),
    },
    Function {
        name: "math.roulette.spread",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> [math.spirograph.Pencil]",
        docs: "One pencil for every distinct curve, the coincidence law read on the exact seats when `exact` says the seats carry no jitter: the first pencil of each family, in the order they came in. On a circle the seats fall into classes under the rotation group of order `gcd(b, 4)`, which is the clause `mrly::math::spirograph::distinct` and `mrly::math::spirograph::representatives` read; on a line and on a polygon every distinct seat draws its own curve, two seats of one radius on a line drawing translates of one shape and never one curve.",
        uncallable: None,
        door: Some(door_math_roulette_spread),
    },
    Function {
        name: "math.rules.render",
        signature: "(filled: [[u8]], number: usize, dimension: usize, base: usize) -> Tensor",
        docs: "Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list.\n\n# Errors\n\nErrors when the number, the dimension or the base is below one.",
        uncallable: None,
        door: Some(door_math_rules_render),
    },
    Function {
        name: "math.rules.tree_axes",
        signature: "(dimension: usize, free_axis: usize) -> [usize]",
        docs: "Returns every axis but the free one.",
        uncallable: None,
        door: Some(door_math_rules_tree_axes),
    },
    Function {
        name: "math.shape.Frac.minus",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "Returns the exact difference.\n\n# Errors\n\nErrors when the reduced result passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_frac_minus),
    },
    Function {
        name: "math.shape.Frac.new",
        signature: "(num: i64, den: i64) -> math.shape.Frac",
        docs: "Builds the reduced fraction num over den.\n\n# Errors\n\nErrors on a zero denominator.",
        uncallable: None,
        door: Some(door_math_shape_frac_new),
    },
    Function {
        name: "math.shape.Frac.plus",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "Returns the exact sum.\n\n# Errors\n\nErrors when the reduced result passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_frac_plus),
    },
    Function {
        name: "math.shape.Frac.times",
        signature: "(self: math.shape.Frac, other: math.shape.Frac) -> math.shape.Frac",
        docs: "Returns the exact product.\n\n# Errors\n\nErrors when the reduced result passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_frac_times),
    },
    Function {
        name: "math.shape.Frac.whole",
        signature: "(num: i64) -> math.shape.Frac",
        docs: "Wraps an integer as a fraction over one.",
        uncallable: None,
        door: Some(door_math_shape_frac_whole),
    },
    Function {
        name: "math.shape.Region.flip",
        signature: "(self: math.shape.Region) -> math.shape.Region",
        docs: "Swaps In and Out, keeping Cut.",
        uncallable: None,
        door: Some(door_math_shape_region_flip),
    },
    Function {
        name: "math.shape.census",
        signature: "(shape: math.shape.Shape, types: Tensor) -> math.shape.ShapeCensus",
        docs: "Tallies the design's cells and filled cells per region of the shape.\n\n# Errors\n\nErrors when the shape's exact scale passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_census),
    },
    Function {
        name: "math.shape.classify",
        signature: "(shape: math.shape.Shape, side: usize, index: [usize]) -> math.shape.Region",
        docs: "Places one lattice cell relative to the shape, exactly, with no floats.\n\nThe cell at the index occupies the closed box from the index to the index plus one on each axis, and the shape's unit-box coordinates are scaled by the side.\n\nA polytope is judged wall by wall: Out means some wall excludes the cell, In means every wall contains it, and Cut means neither - so a cell that touches each wall's feasible side separately reads Cut even when the wall intersection misses it, a conservative call that never mislabels In or Out.\n\n# Errors\n\nErrors when the shape's exact scale passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_classify),
    },
    Function {
        name: "math.shape.crop",
        signature: "(types: Tensor, shape: math.shape.Shape, keep_cut: bool) -> Tensor",
        docs: "Zeroes every cell of the design outside the shape, keeping Cut cells on request; anti-crop is Shape::Anti.\n\n# Errors\n\nErrors when the shape's exact scale passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_crop),
    },
    Function {
        name: "math.shape.crossing_shell",
        signature: "(radius: u64, number: u64, level: u32) -> [(u64, u64)]",
        docs: "Lists the level-`level` boxes the circle of radius `radius` crosses, in the arc's own order.\n\nA box `X` of side `number^level` is crossed when its near corner lies in the closed disc and its far corner outside it, `|X| <= radius / number^level < |X + 1|`, so the level is the whole grid's crossing shell read at that real radius. The columns telescope, the low row of one column being the high row of the next, so the walk is one pass over `floor(radius / number^level) + 1` columns and every comparison is on integers. The order runs along the arc, the first coordinate rising while the second falls, which is why a parent's children are contiguous among its own.",
        uncallable: None,
        door: Some(door_math_shape_crossing_shell),
    },
    Function {
        name: "math.shape.crossing_tree",
        signature: "(radius: u64, number: u64, keep: [bool]) -> math.shape.Shell",
        docs: "Builds the whole crossing tree of one radius, pruned by the seats the design keeps.\n\nThe depth is the least level whose grid holds the circle inside one box, so the tree is rooted there and its leaves are the `2 * radius + 1` crossed cells. `keep` reads the design's level-one tile row-major, one flag per seat, and a box is live when every seat from the root down to it is kept, so the live leaves are exactly the crossed cells the design fills.",
        uncallable: None,
        door: Some(door_math_shape_crossing_tree),
    },
    Function {
        name: "math.shape.fit",
        signature: "(shape: math.shape.Shape, extents: [usize]) -> math.shape.Shape",
        docs: "Centres a unit-box shape on the short side of a lattice of the extents and keeps it to that centred cube, so `regions`, `crop` and `census` read it on any lattice; a cube lattice keeps the shape as it is.\n\nThe cube sits half the spare cells in, rounded down, on each long axis.\n\n# Errors\n\nErrors on no extents, a zero extent, a shape of another dimension, or a scale past an i64.",
        uncallable: None,
        door: Some(door_math_shape_fit),
    },
    Function {
        name: "math.shape.named",
        signature: "(name: str, dimension: usize, radius: math.shape.Frac) -> math.shape.Shape",
        docs: "Builds a named shape of the dimension, centered at one half on every axis.\n\nRegular hexagons and equilateral triangles have irrational walls in the grid frame, so they are excluded on purpose rather than approximated.\n\n# Errors\n\nErrors for an unknown name, or one whose walls are irrational on the grid.",
        uncallable: None,
        door: Some(door_math_shape_named),
    },
    Function {
        name: "math.shape.radial_census",
        signature: "(types: Tensor, centre: [i64], r_max: u64) -> [math.shape.RadialCounts]",
        docs: "Counts a design's filled cells against every integer radius about one centre, in exact integer arithmetic.\n\nCell centres sit at the index plus one half, so every length is doubled and the centre is given in those doubled units: the lattice corner is zero on each axis and the grid centre is the side. Entry r of the result holds the filled cells whose centre lies within radius r, the filled cells whose whole cell lies within it, and the filled cells the sphere of radius r crosses; squared distances are compared as integers and never as floats.\n\nThe tallies are prefix sums over one pass of the grid: each cell lands in the smallest radius that sees it, the smallest that swallows it and the smallest that clears it, and the columns are summed afterwards.",
        uncallable: None,
        door: Some(door_math_shape_radial_census),
    },
    Function {
        name: "math.shape.reach",
        signature: "(name: str, dimension: usize, den: u32) -> u32?",
        docs: "Returns the least radius, in den-ths up to one, at which the named shape covers the whole unit box, or none when it does not by one.\n\n# Errors\n\nErrors for an unknown name or a zero den.",
        uncallable: None,
        door: Some(door_math_shape_reach),
    },
    Function {
        name: "math.shape.refine",
        signature: "(types: Tensor, shape: math.shape.Shape, base: usize, extra: usize, keep_cut: bool) -> Tensor",
        docs: "Replicates each design cell base to the extra per axis and keeps a sub-cell only where its own region passes.\n\n# Errors\n\nErrors below base one, or past the cell ceiling.",
        uncallable: None,
        door: Some(door_math_shape_refine),
    },
    Function {
        name: "math.shape.regions",
        signature: "(shape: math.shape.Shape, dims: [usize]) -> Tensor",
        docs: "Classifies every cell of the grid, packing Out, Cut and In as 0, 1 and 2; the first extent sets the lattice side.\n\n# Errors\n\nErrors when the shape's exact scale passes an i64.",
        uncallable: None,
        door: Some(door_math_shape_regions),
    },
    Function {
        name: "math.shape.shapes",
        signature: "(dimension: usize) -> [String]",
        docs: "Lists the named shapes of a dimension.",
        uncallable: None,
        door: Some(door_math_shape_shapes),
    },
    Function {
        name: "math.six.anti",
        signature: "(self: Cell6d) -> Cell6d",
        docs: "Swaps every fill triangle for a void and back.",
        uncallable: None,
        door: Some(door_math_six_anti),
    },
    Function {
        name: "math.six.binarize",
        signature: "(self: Cell6d, threshold: u8) -> Cell6d",
        docs: "Maps each triangle to one at or above the threshold, zero below.",
        uncallable: None,
        door: Some(door_math_six_binarize),
    },
    Function {
        name: "math.six.binarize_otsu",
        signature: "(self: Cell6d) -> Cell6d",
        docs: "Binarizes the triangles at the threshold Otsu's method picks.",
        uncallable: None,
        door: Some(door_math_six_binarize_otsu),
    },
    Function {
        name: "math.six.blank",
        signature: "(radius: usize, orient: math.six.Orientation, fill: u8, void: u8) -> Cell2d",
        docs: "Builds a hexagon of the given radius, fill inside and void outside.\n\n# Errors\n\nErrors below radius one.",
        uncallable: None,
        door: Some(door_math_six_blank),
    },
    Function {
        name: "math.six.blur",
        signature: "(self: Cell6d, mask: Tensor, wrap: bool) -> Cell6d",
        docs: "Rounds each triangle to the mean of its masked neighborhood, wrapping on request.\n\n# Errors\n\nErrors when the mask does not match the cell's rank.",
        uncallable: None,
        door: Some(door_math_six_blur),
    },
    Function {
        name: "math.six.census",
        signature: "(cell: Cell6d, include_grid: bool) -> math.six.Census",
        docs: "Tallies a cell's triangles, corners and edges, counting the backdrop only on request.",
        uncallable: None,
        door: Some(door_math_six_census),
    },
    Function {
        name: "math.six.components",
        signature: "(cell: Cell6d) -> usize",
        docs: "Counts the connected pieces of the fill, triangles joined across shared edges.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_components),
    },
    Function {
        name: "math.six.copies",
        signature: "(rings: usize) -> usize",
        docs: "Returns the hexagons in a disc of the given rings about one centre: 1, 7, 19, 37 and on.\n\n# Errors\n\nErrors when the count passes the machine word.",
        uncallable: None,
        door: Some(door_math_six_copies),
    },
    Function {
        name: "math.six.cut",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "Slices a cube through its center across the main diagonal into a hexagon.\n\n# Errors\n\nErrors for a cell that is not a cube.",
        uncallable: None,
        door: Some(door_math_six_cut),
    },
    Function {
        name: "math.six.cut_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "Builds the coded 3d design and slices its central hexagon.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_six_cut_design),
    },
    Function {
        name: "math.six.disc",
        signature: "(cell: Cell6d, rings: usize) -> Cell6d",
        docs: "Tessellates a hexagonal cell over the disc of the given rings about one centre, keeping its projection and orientation and taking the start parity of the uncropped sheet the disc sits in; no rings is the cell itself.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_disc),
    },
    Function {
        name: "math.six.east",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "The three corners of the east-pointing triangle at the grid column and row.",
        uncallable: None,
        door: Some(door_math_six_east),
    },
    Function {
        name: "math.six.euler",
        signature: "(cell: Cell6d, include_grid: bool) -> i64",
        docs: "Returns the Euler characteristic of the cell's mesh, counting the backdrop only on request.",
        uncallable: None,
        door: Some(door_math_six_euler),
    },
    Function {
        name: "math.six.fills",
        signature: "(cell: Cell6d) -> usize",
        docs: "Counts the filled triangles of the cell.",
        uncallable: None,
        door: Some(door_math_six_fills),
    },
    Function {
        name: "math.six.fills_only",
        signature: "(cell: Cell6d) -> math.six.Census",
        docs: "Tallies only the filled triangles, leaving the voids and the backdrop out of the mesh.",
        uncallable: None,
        door: Some(door_math_six_fills_only),
    },
    Function {
        name: "math.six.framed",
        signature: "(cell: Cell6d) -> Cell6d",
        docs: "Backs a cell onto a backdrop whose longer axis matches its orientation, leaving every triangle where it stood.\n\nA renderer reads a sheet's orientation off its frame, so a tall sheet of wide hexagons would draw every triangle on its side; the spare columns or rows are backdrop and reach neither the census nor the picture.\n\n# Errors\n\nErrors when the backdrop does not take the cell's triangles.",
        uncallable: None,
        door: Some(door_math_six_framed),
    },
    Function {
        name: "math.six.from_json",
        signature: "(text: str) -> Cell6d",
        docs: "Parses a cell from JSON, defaulting any missing projection metadata.\n\n# Errors\n\nErrors when the text is not a hex cell.",
        uncallable: None,
        door: Some(door_math_six_from_json),
    },
    Function {
        name: "math.six.giant",
        signature: "(cell: Cell6d) -> usize",
        docs: "Returns the triangle count of the fill's largest connected piece.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_giant),
    },
    Function {
        name: "math.six.giant_network",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "Returns the largest connected piece of the filled-triangle network as a network of its own.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_giant_network),
    },
    Function {
        name: "math.six.height",
        signature: "(self: Cell6d) -> usize",
        docs: "Returns the grid height in triangles.",
        uncallable: None,
        door: Some(door_math_six_height),
    },
    Function {
        name: "math.six.holes",
        signature: "(cell: Cell6d) -> usize",
        docs: "Counts the holes of the fill, its piece count less the Euler number of the filled sub-mesh.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_holes),
    },
    Function {
        name: "math.six.image",
        signature: "(cell: Cell6d, scale: usize, outline: Color?, width: usize) -> Image",
        docs: "Rasters a cell's triangles to an image at the given scale, stroked and padded when an outline is given.\n\n# Errors\n\nErrors when nothing renders or when the scale is zero.",
        uncallable: None,
        door: Some(door_math_six_image),
    },
    Function {
        name: "math.six.is_cube",
        signature: "(cell: Cell3d) -> bool",
        docs: "Returns whether the cell's three sides are equal.",
        uncallable: None,
        door: Some(door_math_six_is_cube),
    },
    Function {
        name: "math.six.is_hex",
        signature: "(cell: Cell2d) -> bool",
        docs: "Returns whether the cell's width, height and parity frame a hexagon.",
        uncallable: None,
        door: Some(door_math_six_is_hex),
    },
    Function {
        name: "math.six.iso",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "Projects a cube into the isometric hexagon of top, left and right faces.\n\n# Errors\n\nErrors for a cell that is not a cube.",
        uncallable: None,
        door: Some(door_math_six_iso),
    },
    Function {
        name: "math.six.iso_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "Builds the coded 3d design and projects it isometrically.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_six_iso_design),
    },
    Function {
        name: "math.six.new",
        signature: "(cell: Cell2d, projection: math.six.Projection, orientation: math.six.Orientation, start: u8) -> Cell6d",
        docs: "Builds a cell from its four parts.",
        uncallable: None,
        door: Some(door_math_six_new),
    },
    Function {
        name: "math.six.north",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "The three corners of the north-pointing triangle at the grid column and row.",
        uncallable: None,
        door: Some(door_math_six_north),
    },
    Function {
        name: "math.six.orientation",
        signature: "(width: usize, height: usize) -> math.six.Orientation",
        docs: "Returns the orientation a hexagon's width and height imply.\n\n# Errors\n\nErrors when the width and the height are equal.",
        uncallable: None,
        door: Some(door_math_six_orientation),
    },
    Function {
        name: "math.six.pad",
        signature: "(cell: Cell6d, k: usize, value: u8) -> Cell6d",
        docs: "Wraps a hexagonal cell in k rings of the given value, carrying colors and tags along.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_pad),
    },
    Function {
        name: "math.six.paint",
        signature: "(cell: Cell6d, custom: {u8: [Color]}?, mode: core.Mode?, rng: Rng(seed)?) -> Cell6d",
        docs: "Colors each triangle by its type through the custom or default mapping in the given or type mode.\n\n# Errors\n\nErrors when the Random mode arrives without a stream to draw from.",
        uncallable: None,
        door: Some(door_math_six_paint),
    },
    Function {
        name: "math.six.perforate",
        signature: "(self: Cell6d, mask: Tensor, value: u8) -> Cell6d",
        docs: "Writes the value wherever the tiled mask is nonzero.\n\n# Errors\n\nErrors when the mask does not tile the cell.",
        uncallable: None,
        door: Some(door_math_six_perforate),
    },
    Function {
        name: "math.six.pro",
        signature: "(cell: Cell3d) -> Cell6d",
        docs: "Projects a cube's three facing sides into a hexagon of fills and voids.\n\n# Errors\n\nErrors for a cell that is not a cube.",
        uncallable: None,
        door: Some(door_math_six_pro),
    },
    Function {
        name: "math.six.pro_design",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell6d",
        docs: "Builds the coded 3d design and projects its facing sides.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_six_pro_design),
    },
    Function {
        name: "math.six.radial",
        signature: "(cell: Cell6d, radius: usize) -> Cell2d",
        docs: "Tessellates a hexagonal cell over the disc mask of the given radius.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_radial),
    },
    Function {
        name: "math.six.radial_crop",
        signature: "(cell: Cell2d, radius: usize, size: (usize, usize)) -> Cell2d",
        docs: "Crops the interlocking overhang off a disc tiled at the given radius and tile size.\n\n# Errors\n\nErrors when the width and the height are equal.",
        uncallable: None,
        door: Some(door_math_six_radial_crop),
    },
    Function {
        name: "math.six.radial_mask",
        signature: "(radius: usize, orient: math.six.Orientation) -> Tensor",
        docs: "Builds the disc mask of cells within hex distance radius of the center.",
        uncallable: None,
        door: Some(door_math_six_radial_mask),
    },
    Function {
        name: "math.six.raster",
        signature: "(cell: Cell6d, size: usize) -> [f32]",
        docs: "Rasterizes a hex cell's fills on a square of the side at the true hex aspect, one for a fill triangle and zero elsewhere.\n\n# Errors\n\nErrors at size zero, for a cell that is not a hexagon, or when nothing is filled.",
        uncallable: None,
        door: Some(door_math_six_raster),
    },
    Function {
        name: "math.six.rect_image",
        signature: "(cell: Cell6d, scale: usize, start: usize?) -> Image",
        docs: "Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to an image.\n\n# Errors\n\nErrors for a cell that is not a hexagon or when the scale is zero.",
        uncallable: None,
        door: Some(door_math_six_rect_image),
    },
    Function {
        name: "math.six.rect_svg",
        signature: "(cell: Cell6d, scale: usize, start: usize?) -> String",
        docs: "Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string.\n\n# Errors\n\nErrors for a cell that is not a hexagon, or when nothing renders.",
        uncallable: None,
        door: Some(door_math_six_rect_svg),
    },
    Function {
        name: "math.six.rim_holes",
        signature: "(cell: Cell6d) -> usize",
        docs: "Counts the void regions the rim never reaches, the second route to the hole count.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_rim_holes),
    },
    Function {
        name: "math.six.skin",
        signature: "(cell: Cell6d) -> Cell6d",
        docs: "Recodes an isometric projection's top, left and right faces as plain fills, so a census reads its visible skin as one figure.\n\nThe other two projections already speak in fills and voids and come back untouched.",
        uncallable: None,
        door: Some(door_math_six_skin),
    },
    Function {
        name: "math.six.slice_core_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "Builds the network of filled triangles joined by shared edges.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_slice_core_graph),
    },
    Function {
        name: "math.six.slice_dual_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "Builds the network of fill and void triangles joined by shared edges.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_slice_dual_graph),
    },
    Function {
        name: "math.six.slice_edge_graph",
        signature: "(cell: Cell6d, value: u8?) -> math.graph.Network",
        docs: "Builds the corner-and-edge network of the triangles matching the value, or of every fill and void.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_slice_edge_graph),
    },
    Function {
        name: "math.six.slice_tunnel_graph",
        signature: "(cell: Cell6d) -> math.graph.Network",
        docs: "Builds the network of void triangles joined by shared edges, the pore network of the slice.\n\n# Errors\n\nErrors when a triangle index falls outside the sheet.",
        uncallable: None,
        door: Some(door_math_six_slice_tunnel_graph),
    },
    Function {
        name: "math.six.south",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "The three corners of the south-pointing triangle at the grid column and row.",
        uncallable: None,
        door: Some(door_math_six_south),
    },
    Function {
        name: "math.six.spectral_exponent",
        signature: "(cell: Cell6d, window: f64) -> f64",
        docs: "Reads the spectral dimension of the giant piece: twice the low-window log-log slope of the normalised Laplacian's integrated density of states.\n\n# Errors\n\nErrors when the giant piece is too small to fit an exponent.",
        uncallable: None,
        door: Some(door_math_six_spectral_exponent),
    },
    Function {
        name: "math.six.star.Branch.constant",
        signature: "(self: math.six.star.Branch) -> f64",
        docs: "The constant the ladder converges on, `C` at even `L` and `C + 1/8` at odd `L`.",
        uncallable: None,
        door: Some(door_math_six_star_branch_constant),
    },
    Function {
        name: "math.six.star.Branch.name",
        signature: "(self: math.six.star.Branch) -> String",
        docs: "The name of the branch.",
        uncallable: None,
        door: Some(door_math_six_star_branch_name),
    },
    Function {
        name: "math.six.star.Branch.of",
        signature: "(layers: usize) -> math.six.star.Branch",
        docs: "The branch of a layer count.",
        uncallable: None,
        door: Some(door_math_six_star_branch_of),
    },
    Function {
        name: "math.six.star.Branch.residual",
        signature: "(self: math.six.star.Branch) -> f64?",
        docs: "The exact `1/L^2` coefficient at even `L`, absent at odd `L`.",
        uncallable: None,
        door: Some(door_math_six_star_branch_residual),
    },
    Function {
        name: "math.six.star.Share.reduced",
        signature: "(self: math.six.star.Share) -> (i64, i64)",
        docs: "The share in lowest terms, numerator then denominator.",
        uncallable: None,
        door: Some(door_math_six_star_share_reduced),
    },
    Function {
        name: "math.six.star.Share.value",
        signature: "(self: math.six.star.Share) -> f64",
        docs: "The share as a real number.",
        uncallable: None,
        door: Some(door_math_six_star_share_value),
    },
    Function {
        name: "math.six.star.Star.arm",
        signature: "(self: math.six.star.Star, number: usize, half: usize) -> math.six.star.Share",
        docs: "The exact ink share of the band of half-width `W` cells about the arm `x = y` at odd `n`.\n\nAt `W = 0` the band is the arm itself, the `2n` cells of the diameter, whose share is the\nclosed form of `arm_law`.\n\n# Errors\n\nErrors at an even layer number.",
        uncallable: None,
        door: Some(door_math_six_star_star_arm),
    },
    Function {
        name: "math.six.star.Star.cell",
        signature: "(self: math.six.star.Star, number: usize, x: i64, z: i64) -> bool?",
        docs: "The ink of the cut cell at column `x` and even height `z` of the layer at odd `n`.\n\nThe answer is absent where the plane `x + y + z = 6n - 2` leaves the cube, which is what draws\nthe hexagon's own edge.",
        uncallable: None,
        door: Some(door_math_six_star_star_cell),
    },
    Function {
        name: "math.six.star.Star.excesses",
        signature: "(self: math.six.star.Star, layers: usize, half: usize) -> [f64]",
        docs: "The per-layer excess of the star band over the hexagon across the first `L` odd layers.\n\n# Errors\n\nErrors below one layer, or at an even layer number.",
        uncallable: None,
        door: Some(door_math_six_star_star_excesses),
    },
    Function {
        name: "math.six.star.Star.hexagon",
        signature: "(self: math.six.star.Star, number: usize) -> math.six.star.Share",
        docs: "The exact ink share of the whole hexagonal cut at odd `n`, the background the star is read against.\n\nThe rows are the `2n` even heights `z`, and each runs over the `x` the plane leaves inside the\ncube. The fill depends on `x` only through `x mod 8`, so a row is counted in eight steps and\nthe whole layer in `O(n)`.\n\n# Errors\n\nErrors at an even layer number.",
        uncallable: None,
        door: Some(door_math_six_star_star_hexagon),
    },
    Function {
        name: "math.six.star.Star.new",
        signature: "(code: u128) -> math.six.star.Star",
        docs: "Reads the star of a base-2 space code, the carpet being `23`.\n\n# Errors\n\nErrors when the code is out of range for a base-2 cube.",
        uncallable: None,
        door: Some(door_math_six_star_star_new),
    },
    Function {
        name: "math.six.star.arm_law",
        signature: "(number: usize) -> math.six.star.Share",
        docs: "The closed form of the star arm's ink at odd `n`, `1/2 + chi_8(n)/(2n)`, as `n + chi_8(n)` cells of `2n`.\n\nThe arm `x = y` of the cut is a diameter of `2n` cells with no width to choose, and on it the two\nblock parities agree, so the space rule collapses to `floor(x/4)` even and the inked count is\n`f(3n) - f(n) = n + chi_8(n)` with `f(m) = 4 floor(m/8) + min(m mod 8, 4)`.\n\n# Errors\n\nErrors at an even layer number.",
        uncallable: None,
        door: Some(door_math_six_star_arm_law),
    },
    Function {
        name: "math.six.star.chi8",
        signature: "(number: usize) -> i64",
        docs: "The real character mod 8 of `Q(sqrt 2)`: `+1` at `n = 1, 7`, `-1` at `n = 3, 5`, zero at even `n`.",
        uncallable: None,
        door: Some(door_math_six_star_chi8),
    },
    Function {
        name: "math.six.star.constant",
        signature: "() -> f64",
        docs: "The constant beside the decay, `ln(1 + sqrt 2)/(2 sqrt 2) - G/8 - gamma/4 - (ln 2)/2`.\n\nEvery limb is a closed form the subtraction of the two ink laws hands over: the character part is\n`L(1, chi_8)/2` by the class number formula for `Q(sqrt 2)`, the square tail is `-G/8` with `G`\nCatalan's constant, and the harmonic tail carries `-gamma/4 - (ln 2)/2`.",
        uncallable: None,
        door: Some(door_math_six_star_constant),
    },
    Function {
        name: "math.six.star.decay",
        signature: "(excesses: [f64], layers: usize) -> math.six.star.Decay",
        docs: "The decay read off the per-layer excesses at a layer count, the slope taken from `L/2` to `L`.\n\nAt `W = 0` and even `L` the reading is `L * excess_L = -(ln L)/4 + C + O(1/L^2)`, so the slope\nsettles on `-1/4` and the residual on the branch's exact `1/L^2` coefficient. The sliding window\ncarries a residue trap: the per-layer excess opens on a term in the ink law's character whose sum\nvanishes at even `L` alone, so a window from `L/2` to `L` cancels it only when `L` is divisible by\nfour. At `L = 2 mod 4` the half is odd and the window converges somewhere else entirely, so the\nslope is refused there rather than reported wrong.\n\n# Errors\n\nErrors below two layers, or past the excesses read.",
        uncallable: None,
        door: Some(door_math_six_star_decay),
    },
    Function {
        name: "math.six.star.width_law",
        signature: "(half: usize) -> f64",
        docs: "The cell-frame decay coefficient of a band of half-width `W` cells, `-(K + b)/(4(2K + 1))` for `K = floor(W/2)`.\n\n`b` is the block parity of the band edge, one when `floor(K/2)` is even and zero when it is odd.\nThe band's per-layer excess is `kappa chi + (m + q chi_8(n))/n + chi/(8 n^2)`, and summing the\n`1/n` term over the first `L` odd layers hands this coefficient as `m/2`, so it is a theorem and\nnot a fit. At even `W` it reads `-(W + 2b)/(8(W + 1))` and at odd `W` it reads\n`-(W - 1 + 2b)/(8W)`, since `x - y` is even on the cut and an odd width is never a new band:\n`W = 1` returns the arm's `-1/4` and `W = 3` returns `-1/6`. As `W` grows it tends to `-1/8`.",
        uncallable: None,
        door: Some(door_math_six_star_width_law),
    },
    Function {
        name: "math.six.svg",
        signature: "(cell: Cell6d, scale: usize, outline: Color?, width: usize, start: usize?) -> String",
        docs: "Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given.\n\n# Errors\n\nErrors when nothing renders.",
        uncallable: None,
        door: Some(door_math_six_svg),
    },
    Function {
        name: "math.six.tessellate",
        signature: "(cell: Cell6d, mask: Tensor) -> Cell2d",
        docs: "Stamps a hexagonal cell at every set mask entry into one interlocking sheet, colors and tags included.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_tessellate),
    },
    Function {
        name: "math.six.tile",
        signature: "(cell: Cell6d, width: usize, height: usize) -> Cell2d",
        docs: "Tessellates a hexagonal cell over a full width-by-height mask.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_tile),
    },
    Function {
        name: "math.six.tile_cell",
        signature: "(cell: Cell6d, width: usize, height: usize, crop: bool) -> Cell6d",
        docs: "Tessellates a hexagon over a full width-by-height mask and returns the sheet as a projected cell, cropped to the interlocking rectangle on request.\n\nThe sheet keeps the tile's projection and orientation. A crop slides the triangle grid by the interlocking step on both axes, so the start parity flips whenever that step is odd; without the flip every triangle in the sheet points the wrong way.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_tile_cell),
    },
    Function {
        name: "math.six.tile_crop",
        signature: "(cell: Cell2d, size: (usize, usize)) -> Cell2d",
        docs: "Crops one interlocking step off each side of a sheet tiled at the given size.\n\n# Errors\n\nErrors when the width and the height are equal.",
        uncallable: None,
        door: Some(door_math_six_tile_crop),
    },
    Function {
        name: "math.six.tile_step",
        signature: "(size: (usize, usize)) -> (usize, usize)",
        docs: "Returns the interlocking step, in triangle columns and rows, that a sheet of hexagons of the given width and height loses off each side when cropped.\n\n# Errors\n\nErrors when the width and the height are equal.",
        uncallable: None,
        door: Some(door_math_six_tile_step),
    },
    Function {
        name: "math.six.to_json",
        signature: "(cell: Cell6d) -> String",
        docs: "Serializes a cell and its projection metadata to JSON.",
        uncallable: None,
        door: Some(door_math_six_to_json),
    },
    Function {
        name: "math.six.triangles",
        signature: "(cell: Cell6d, start: usize?) -> [([(i64, i64); 3], [u8; 4])]",
        docs: "Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override.\n\n# Errors\n\nErrors for a cell that is not a hexagon.",
        uncallable: None,
        door: Some(door_math_six_triangles),
    },
    Function {
        name: "math.six.west",
        signature: "(x: i64, y: i64) -> [(i64, i64); 3]",
        docs: "The three corners of the west-pointing triangle at the grid column and row.",
        uncallable: None,
        door: Some(door_math_six_west),
    },
    Function {
        name: "math.six.width",
        signature: "(self: Cell6d) -> usize",
        docs: "Returns the grid width in triangles.",
        uncallable: None,
        door: Some(door_math_six_width),
    },
    Function {
        name: "math.spectrum.clusters",
        signature: "(eigenvalues: [f64], tolerance: f64) -> [(f64, usize)]",
        docs: "Groups eigenvalues into runs split by consecutive gaps above the tolerance, each run its mean and its size.\n\n# Errors\n\nErrors on a NaN.",
        uncallable: None,
        door: Some(door_math_spectrum_clusters),
    },
    Function {
        name: "math.spectrum.laplacian",
        signature: "(network: math.graph.Network, normalised: bool) -> [[f64]]",
        docs: "Builds the Laplacian of a network, the combinatorial `D - A` or the normalised `I - D^-1/2 A D^-1/2`.\n\nSelf-loops are ignored and the degrees are the adjacency row sums. Errors when a\nnode carries no branch and the normalised form is asked for.\n\n# Errors\n\nErrors when a node carries no branch under the normalised reading.",
        uncallable: None,
        door: Some(door_math_spectrum_laplacian),
    },
    Function {
        name: "math.spectrum.laplacian_spectrum",
        signature: "(network: math.graph.Network, normalised: bool) -> [f64]",
        docs: "Returns the ascending Laplacian spectrum of a network, combinatorial or normalised.\n\n# Errors\n\nErrors when a node carries no branch, or the spectrum is not a number.",
        uncallable: None,
        door: Some(door_math_spectrum_laplacian_spectrum),
    },
    Function {
        name: "math.spectrum.multiplicity",
        signature: "(eigenvalues: [f64], value: f64, tolerance: f64) -> usize",
        docs: "Counts the eigenvalues within the tolerance of a value.",
        uncallable: None,
        door: Some(door_math_spectrum_multiplicity),
    },
    Function {
        name: "math.spectrum.spectral_exponent",
        signature: "(eigenvalues: [f64], window: f64) -> f64?",
        docs: "Reads the spectral exponent: twice the log-log slope of the integrated density of states over its low window.",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_exponent),
    },
    Function {
        name: "math.spectrum.spectral_fit",
        signature: "(eigenvalues: [f64], window: f64) -> (f64, f64, usize)?",
        docs: "Fits the low window of the integrated density of states in log-log: the intercept, the slope and the fitted count.\n\nThe points come from `spectral_points` and the first `max(floor(window * total), 3)` of\nthem are fitted by least squares. Returns `None` when fewer than two points remain.",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_fit),
    },
    Function {
        name: "math.spectrum.spectral_points",
        signature: "(eigenvalues: [f64]) -> [(f64, f64)]",
        docs: "Builds the integrated density of states as points, each an eigenvalue and its rank fraction.\n\nThe eigenvalues are clamped at zero and sorted, exactly equal neighbours collapse to one\npoint at the run's last index over the total count, and points at or below `1e-12` drop.",
        uncallable: None,
        door: Some(door_math_spectrum_spectral_points),
    },
    Function {
        name: "math.spectrum.symmetric_eigenvalues",
        signature: "(matrix: [[f64]]) -> [f64]",
        docs: "Returns the eigenvalues of a dense real symmetric matrix in ascending order.\n\nThe matrix is reduced to tridiagonal form by Householder reflections and then\ndiagonalised by implicit QL with Wilkinson shifts. Errors on a matrix that is\nnot square or whose entries disagree across the diagonal by more than `1e-12`.\n\n# Errors\n\nErrors on a matrix that is not square or not symmetric, or a spectrum that is not a number.",
        uncallable: None,
        door: Some(door_math_spectrum_symmetric_eigenvalues),
    },
    Function {
        name: "math.spin.Blend.fold",
        signature: "(self: math.spin.Blend, values: [f32]) -> f32",
        docs: "Merges one site's copies into the blended value.",
        uncallable: None,
        door: Some(door_math_spin_blend_fold),
    },
    Function {
        name: "math.spin.Blend.named",
        signature: "(name: str) -> math.spin.Blend?",
        docs: "Reads a blend by name: mean, sum, union, meet, parity or difference.",
        uncallable: None,
        door: Some(door_math_spin_blend_named),
    },
    Function {
        name: "math.spin.arcs",
        signature: "(data: [f32], size: usize, radius: f64) -> [(f64, f64, f32)]",
        docs: "The arcs of the circle of the radius about the raster's centre: each as its start angle, end angle and the value of the one cell it lies in, zero outside.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_arcs),
    },
    Function {
        name: "math.spin.harmonics",
        signature: "(data: [f32], size: usize, rings: usize, orders: usize) -> [f64]",
        docs: "The circular-harmonic power of a raster: for every order `m` up to the last, the energy `sum |c_m(r)|^2 2 pi r dr` of its `m`-th harmonic over rings radii, each ring's coefficient exact from its arcs.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_harmonics),
    },
    Function {
        name: "math.spin.mass",
        signature: "(profile: [f32], size: usize) -> f64",
        docs: "The mass a profile carries, the trapezoid integral of `2 pi r F(r)` in cells of the raster it came from.",
        uncallable: None,
        door: Some(door_math_spin_mass),
    },
    Function {
        name: "math.spin.mass_within",
        signature: "(profile: [f32], size: usize, radius: f64) -> f64",
        docs: "The mass a profile carries inside the radius, the trapezoid integral of `2 pi r F(r)` from the centre out, in cells of the raster it came from.",
        uncallable: None,
        door: Some(door_math_spin_mass_within),
    },
    Function {
        name: "math.spin.petals",
        signature: "(copies: usize, order: usize) -> usize",
        docs: "The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple.",
        uncallable: None,
        door: Some(door_math_spin_petals),
    },
    Function {
        name: "math.spin.profile",
        signature: "(data: [f32], size: usize, steps: usize) -> [f32]",
        docs: "The ring profile: the circle means at steps radii spaced evenly from the centre to the corner circle.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_profile),
    },
    Function {
        name: "math.spin.radial",
        signature: "(data: [f32], size: usize, out: usize, copies: usize, step: f64, blend: math.spin.Blend, samples: usize) -> [f32]",
        docs: "Stacks a raster radially: copies turned by multiples of the step, in turns, about the centre and merged by the blend, on an output raster of the side whose inscribed circle is the source's corner circle, every pixel the mean of samples by samples points.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_radial),
    },
    Function {
        name: "math.spin.reach",
        signature: "(size: usize) -> f64",
        docs: "The radius of the corner circle of a square raster of the side, the last radius a profile reads.",
        uncallable: None,
        door: Some(door_math_spin_reach),
    },
    Function {
        name: "math.spin.ring",
        signature: "(data: [f32], size: usize, radius: f64) -> f64",
        docs: "The exact mean of a square raster over the circle of the radius about its centre, each cell read as a constant and the outside as zero.\n\n# Errors\n\nErrors when the sample count is not the side squared.",
        uncallable: None,
        door: Some(door_math_spin_ring),
    },
    Function {
        name: "math.spin.turns",
        signature: "(power: [f64]) -> usize",
        docs: "The rotation order a harmonic power spectrum reveals: the gcd of the orders carrying more than a ten-thousandth of the power, the share pixel aliasing stays under, or zero when none does.",
        uncallable: None,
        door: Some(door_math_spin_turns),
    },
    Function {
        name: "math.spin.wheel",
        signature: "(profile: [f32], size: usize) -> [f32]",
        docs: "The wheel: a profile spread over a square raster of the side, the corner circle it ends on drawn as the inscribed circle, every pixel reading the profile at its own radius.",
        uncallable: None,
        door: Some(door_math_spin_wheel),
    },
    Function {
        name: "math.spirograph.Seats.default",
        signature: "() -> math.spirograph.Seats",
        docs: "Returns the default Seats.",
        uncallable: None,
        door: Some(door_math_spirograph_seats_default),
    },
    Function {
        name: "math.spirograph.cell",
        signature: "(width: usize, height: usize, reach: f64) -> f64",
        docs: "The side of one cell in wheel radii at a reach, the number the page needs to draw the tile on the wheel.",
        uncallable: None,
        door: Some(door_math_spirograph_cell),
    },
    Function {
        name: "math.spirograph.cover",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool, samples: usize, side: usize) -> math.spirograph.Cover",
        docs: "The shape between the walls of a circle roulette, on a raster of `side` by `side` pixels over the disc, row zero at the top and the ordinate falling down the rows. Every distinct curve under the coincidence law is drawn once as a polyline of at least `samples` points, and of enough points that consecutive points land in one pixel or in two of the eight that touch, so the polylines make a wall no four-connected flood crosses. One flood starts from every pixel of the raster's edge, the fluid poured from outside; one starts from the centre pixel, the fluid poured at the centre, and is empty when the centre is a wall or the outside already reached it; the shape is the rest of the disc, pockets included. `covered` is the shape's share of the disc's pixels, the wall's own pixels counted in and reported apart as `wall`, and `hole` is the centre flood's share. `winding` is the mean signed winding number of the disc's pixel centres, read off crossings of the same polylines by scanline and never off a flood, and `areas` is the closed form it converges to, the distinct curves' `signed_area` summed over the disc's area: the pair checks the polylines and the raster against Green's theorem and never the floods, which are guarded instead by the sample spacing of at most half a pixel, which makes the wall eight-connected and a four-connected flood unable to cross it. Every share carries a boundary error of the order of the polylines' length times the pixel side over the disc's area.\n\n# Errors\n\nErrors on a line or a polygon track, a raster side out of range, under two samples, or past the point cap.",
        uncallable: None,
        door: Some(door_math_spirograph_cover),
    },
    Function {
        name: "math.spirograph.disc",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil]) -> math.spirograph.Disc",
        docs: "The disc a circle roulette sits in: the wheel's centre turns on a circle of radius `rho`, and a seat `d` from the wheel's centre puts the pencil at `|z|^2 = rho^2 + d^2 + 2 rho d cos(a t / b -+ arg p)`, whose phase runs over `a` full turns, so that curve lies in the closed annulus from `abs(rho - d)` to `rho + d` and attains both bounds. The whole roulette therefore never leaves the disc of radius `rho + max d` and enters no disc of radius under `min abs(rho - d)`, the least over the seats and not the outermost seat's own, since seats on both sides of `rho` each keep their own inner radius. Refuses a line or a polygon track, whose roulette need not close and has no wall.\n\n# Errors\n\nErrors on a line or a polygon track.",
        uncallable: None,
        door: Some(door_math_spirograph_disc),
    },
    Function {
        name: "math.spirograph.distinct",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> usize",
        docs: "How many classes `representatives` finds: the distinct curves on a circle track, the shapes up to a shift along a line track, one class per pencil on a polygon and under jitter.",
        uncallable: None,
        door: Some(door_math_spirograph_distinct),
    },
    Function {
        name: "math.spirograph.frame",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil]) -> [f64; 4]",
        docs: "The box the whole picture sits in: the centre path and the track, padded by the wheel's radius or the farthest seat, whichever reaches further.",
        uncallable: None,
        door: Some(door_math_spirograph_frame),
    },
    Function {
        name: "math.spirograph.nodes",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> u64?",
        docs: "The crossings of the whole roulette on a circle track, the generic count, with `R/r = a/b` in lowest terms. Write `|p|` for a seat's distance from the wheel's centre in wheel radii and `A` for the centre path's radius in the same units, `(a - b)/b` inside and `(a + b)/b` outside. Every seat must lie strictly inside the window `0 < |p| < min(1, A)`, which three hypotheses cut: `|p| > 0`, since a seat at the wheel's centre draws the centre circle `b` times over and never crosses; `|p| < 1`, the loop threshold, past which a curve loops; and `|p| < A`, the seat threshold, where the seat reaches the centre path, which comes before the loop threshold on every inside track with `a < 2b` and never bites outside. Inside that window two distinct curves cross exactly `2ab` times, one curve crosses itself `a(b - 1)` times, and `k` distinct curves cross `2ab k(k - 1) / 2 + k a (b - 1)` times, the design entering only through `k`. `exact` reads the coincidence law on the seats, as `distinct` does. `None` on a line or a polygon track, and `None` when any seat leaves the window, where neither count is the law's. At isolated reaches some crossings merge, so the count holds for the generic reach.",
        uncallable: None,
        door: Some(door_math_spirograph_nodes),
    },
    Function {
        name: "math.spirograph.pencils",
        signature: "(types: [u8], width: usize, height: usize, mode: str, reach: f64, jitter: f64, seed: u32) -> [math.spirograph.Pencil]",
        docs: "Seats one pencil per chosen site of a byte grid: `fill` the filled cells, `void` the empty ones, `both`, or `corners` the corners of the filled cells, each once. The tile is scaled so its circumradius is `reach` wheel radii, and `jitter` moves every seat by up to that fraction of a cell each way, seeded.\n\n# Errors\n\nErrors on a grid that is not width by height, a reach or jitter out of range, or a mode that is not fill, void, both or corners.",
        uncallable: None,
        door: Some(door_math_spirograph_pencils),
    },
    Function {
        name: "math.spirograph.point",
        signature: "(track: math.spirograph.Track, pencil: math.spirograph.Pencil, s: f64) -> (f64, f64)",
        docs: "Where a pencil is after `s` of path length: the centre plus the seat turned with the wheel.",
        uncallable: None,
        door: Some(door_math_spirograph_point),
    },
    Function {
        name: "math.spirograph.pose",
        signature: "(track: math.spirograph.Track, s: f64) -> (f64, f64)",
        docs: "The wheel's centre after `s` of path length.",
        uncallable: None,
        door: Some(door_math_spirograph_pose),
    },
    Function {
        name: "math.spirograph.representatives",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], exact: bool) -> [usize]",
        docs: "One pencil per class, the first index of every class in the order the pencils were seated. On a circle track the classes are the distinct curves, by the coincidence law on exact seats: two pencils draw one curve iff a rotation of a full turn over the ratio's denominator carries one seat to the other, which the square lattice allows only by half turns when the denominator is even and by quarter turns when four divides it. On a line track the classes are the seat radii, and those are shapes up to a shift, not curves: turning a seat by `gamma` slides its whole ribbon `gamma` wheel radii along the line while the ribbon's period is a full turn of the wheel, so two seats of one radius draw translates of one shape and share no point unless the seats are equal. On a polygon every pencil is its own class, and so is every pencil under jitter.",
        uncallable: None,
        door: Some(door_math_spirograph_representatives),
    },
    Function {
        name: "math.spirograph.seats",
        signature: "(pencils: [math.spirograph.Pencil]) -> math.spirograph.Seats",
        docs: "Counts the pencils by kind.",
        uncallable: None,
        door: Some(door_math_spirograph_seats),
    },
    Function {
        name: "math.spirograph.signed_area",
        signature: "(track: math.spirograph.Track, pencil: math.spirograph.Pencil) -> f64?",
        docs: "The signed area one pencil's closed trochoid sweeps over the whole track, counterclockwise positive and counted with multiplicity, so it is the winding number integrated over the plane: `pi b rho (rho - d^2/r)` inside and `pi b rho (rho + d^2/r)` outside, with `rho` the centre circle's radius `R -+ r`, `d = r |p|` the seat's distance from the wheel's centre and `R/r = a/b` in lowest terms. Green's theorem on `z(t) = rho e^(i t) + p r e^(-+ i (rho/r) t)` gives it, and the cross terms carry `e^(-+ i a t / b)` over `b` centre turns and integrate to zero. No hypothesis on the seat: loops are counted with their sign. `None` off a circle track, where the roulette need not close.",
        uncallable: None,
        door: Some(door_math_spirograph_signed_area),
    },
    Function {
        name: "math.spirograph.trace",
        signature: "(track: math.spirograph.Track, pencils: [math.spirograph.Pencil], samples: usize) -> [f32]",
        docs: "Traces every pencil along the whole track at `samples` evenly spaced path lengths, first and last included: pencil by pencil, sample by sample, x then y.\n\n# Errors\n\nErrors under two samples, or past the point cap.",
        uncallable: None,
        door: Some(door_math_spirograph_trace),
    },
    Function {
        name: "math.spirograph.track",
        signature: "(kind: str, ring: usize, wheel: usize, sides: usize, laps: usize) -> math.spirograph.Track",
        docs: "Lays a track: `line` a straight line under the wheel for `laps` turns; `in` and `out` a circle of radius `ring` with the wheel inside or outside, closing after the reduced denominator of `ring` over `wheel` orbits; `polyin` and `polyout` a regular polygon of `sides` sides and circumradius `ring` for `laps` laps.\n\n# Errors\n\nErrors on a radius or lap count out of range, a wheel that does not fit its ring, or a kind that is not line, in, out, polyin or polyout.",
        uncallable: None,
        door: Some(door_math_spirograph_track),
    },
    Function {
        name: "math.spirograph.turn",
        signature: "(track: math.spirograph.Track, s: f64) -> f64",
        docs: "The wheel's turn after `s` of path length, in radians: `side` times `s` over the wheel's radius.",
        uncallable: None,
        door: Some(door_math_spirograph_turn),
    },
    Function {
        name: "math.sumset.Pair.clean",
        signature: "(self: math.sumset.Pair) -> bool",
        docs: "Whether the pair is clean, `3^k > d(k, m)` and `4^m > d(k, m)`, so that `S meet [0, d] = A_k + B_m`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_clean),
    },
    Function {
        name: "math.sumset.Pair.copy",
        signature: "(self: math.sumset.Pair) -> bool",
        docs: "Whether the pair is a gap copy, `2 4^m < 3^k + 5`: `A_k + B_m` is then two disjoint translates of `A_(k-1) + B_m` and its energy is twice theirs.",
        uncallable: None,
        door: Some(door_math_sumset_pair_copy),
    },
    Function {
        name: "math.sumset.Pair.energy",
        signature: "(self: math.sumset.Pair) -> u128",
        docs: "The additive energy `E(k, m) = sum_x r(x)^2`, `r(x)` the number of ways `x = a + b` with `a` in `A_k` and `b` in `B_m`.\n\nSummed as `sum_t 2^(z_3(t) + z_4(t))` over the `3^m` integers `t` with an `m`-digit base-4 string in `{-1, 0, 1}`, `z_4(t)` its zero digits and `z_3(t)` the zero digits of the `k`-digit balanced ternary expansion of `t`, a `t` past `(3^k - 1)/2` adding nothing: a difference of two digit strings from `{0, 1}` is a digit string from `{-1, 0, 1}`, which fixes `t` in either base, and each zero difference arises twice. The cost is `3^m` strings.",
        uncallable: None,
        door: Some(door_math_sumset_pair_energy),
    },
    Function {
        name: "math.sumset.Pair.four",
        signature: "(self: math.sumset.Pair) -> u32",
        docs: "The base-4 level `m`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_four),
    },
    Function {
        name: "math.sumset.Pair.gap",
        signature: "(self: math.sumset.Pair) -> (u64, u64)?",
        docs: "The first and the last integer of the open interval `(d(k, m), min(3^k, 4^m))`, which `S` misses, or `None` when it holds none.\n\nA sum with `a < 3^k` and `b < 4^m` is at most `d(k, m)`, and a sum with `a >= 3^k` or `b >= 4^m` is at least `min(3^k, 4^m)`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_gap),
    },
    Function {
        name: "math.sumset.Pair.largest",
        signature: "(self: math.sumset.Pair) -> u64",
        docs: "The largest element `d(k, m) = (3^k - 1)/2 + (4^m - 1)/3` of `A_k + B_m`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_largest),
    },
    Function {
        name: "math.sumset.Pair.new",
        signature: "(three: u32, four: u32) -> math.sumset.Pair",
        docs: "Names the pair `(k, m)`.\n\n# Errors\n\nErrs when either level is zero or past `WIDEST`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_new),
    },
    Function {
        name: "math.sumset.Pair.ratio",
        signature: "(self: math.sumset.Pair, energy: u128) -> f64",
        docs: "The energy ratio `Q(k, m) = E(k, m) (d + 1)/4^(k+m)` of the energy `Pair::energy` returns, the energy against its flat value, at least `1`; `card(A_k + B_m) >= (d + 1)/Q` by Cauchy-Schwarz.",
        uncallable: None,
        door: Some(door_math_sumset_pair_ratio),
    },
    Function {
        name: "math.sumset.Pair.scale",
        signature: "(self: math.sumset.Pair) -> f64",
        docs: "The scaling `tau = 4^m/3^k`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_scale),
    },
    Function {
        name: "math.sumset.Pair.three",
        signature: "(self: math.sumset.Pair) -> u32",
        docs: "The base-3 level `k`.",
        uncallable: None,
        door: Some(door_math_sumset_pair_three),
    },
    Function {
        name: "math.sumset.Sumset.contains",
        signature: "(self: math.sumset.Sumset, x: u64) -> bool?",
        docs: "Whether `x` is in `S`, or `None` past the top.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.count",
        signature: "(self: math.sumset.Sumset, x: u64) -> u64?",
        docs: "Counts `card(S meet [1, x])`, or `None` past the top.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.density",
        signature: "(self: math.sumset.Sumset, x: u64) -> f64?",
        docs: "Reads the density `D(x) = card(S meet [1, x])/x`, or `None` at zero and past the top.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.extremes",
        signature: "(self: math.sumset.Sumset, edges: [u64]) -> [(f64, f64)?]",
        docs: "Reads the least and the greatest `D(x)` over each window `[edges[i], edges[i + 1])`, `None` for an empty window.\n\n# Errors\n\nErrs when the edges decrease, start at zero or run past `top + 1`.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.fills",
        signature: "(self: math.sumset.Sumset, low: u64, high: u64, cells: usize) -> [f64]",
        docs: "Reads the share of members in each of `cells` equal runs of the integers `[low, high)`, the strip of `S` a page draws.\n\nRun `i` is `[low + floor(i n/cells), low + floor((i + 1) n/cells))` with `n = high - low`.\n\n# Errors\n\nErrs when the window is empty or runs past the top, or when `cells` is zero or more than the integers in the window.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.level",
        signature: "(self: math.sumset.Sumset) -> u32",
        docs: "The level the array was built to.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.new",
        signature: "(level: u32) -> math.sumset.Sumset",
        docs: "Builds `S meet [0, 3^level]`.\n\n# Errors\n\nErrs when the level is zero or past `DEEPEST`.",
        uncallable: Some("math::sumset::Sumset has no Serialize"),
        door: None,
    },
    Function {
        name: "math.sumset.Sumset.top",
        signature: "(self: math.sumset.Sumset) -> u64",
        docs: "The largest integer the array holds, `3^level`.",
        uncallable: Some("math::sumset::Sumset has no Deserialize"),
        door: None,
    },
    Function {
        name: "math.sumset.pairs",
        signature: "(level: u32) -> [math.sumset.Pair]",
        docs: "Lists the pairs of the census: every `(k, m)` with `4^m` within a factor `3` of `3^k` and `d(k, m) <= 3^level`, by `d`.\n\n# Errors\n\nErrs when the level is zero or past `WIDEST`.",
        uncallable: None,
        door: Some(door_math_sumset_pairs),
    },
    Function {
        name: "math.three.Vec3.cross",
        signature: "(self: math.three.Vec3, o: math.three.Vec3) -> math.three.Vec3",
        docs: "Returns the cross product, perpendicular to both vectors.",
        uncallable: None,
        door: Some(door_math_three_vec3_cross),
    },
    Function {
        name: "math.three.Vec3.dot",
        signature: "(self: math.three.Vec3, o: math.three.Vec3) -> f32",
        docs: "Returns the dot product of the two vectors.",
        uncallable: None,
        door: Some(door_math_three_vec3_dot),
    },
    Function {
        name: "math.three.Vec3.new",
        signature: "(x: f32, y: f32, z: f32) -> math.three.Vec3",
        docs: "Builds a vector from its components.",
        uncallable: None,
        door: Some(door_math_three_vec3_new),
    },
    Function {
        name: "math.three.Vec3.scale",
        signature: "(self: math.three.Vec3, s: f32) -> math.three.Vec3",
        docs: "Multiplies every component by the scalar.",
        uncallable: None,
        door: Some(door_math_three_vec3_scale),
    },
    Function {
        name: "math.three.carpet",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the Menger sponge, filled where at most one coordinate is odd, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_carpet),
    },
    Function {
        name: "math.three.census",
        signature: "(cell: Cell3d) -> math.three.Census",
        docs: "Tallies a cell's sites, its exposed surface and its Euler characteristic in one reading.\n\n# Errors\n\nErrors when the edge network cannot be lifted from the cube.",
        uncallable: None,
        door: Some(door_math_three_census),
    },
    Function {
        name: "math.three.core_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "Extracts the network of filled sites joined to their axis neighbors.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_three_core_graph),
    },
    Function {
        name: "math.three.create",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> Cell3d",
        docs: "Builds the cube the universe code names, deepened to the given fractal level.\n\n# Errors\n\nErrors when the code is out of range.",
        uncallable: None,
        door: Some(door_math_three_create),
    },
    Function {
        name: "math.three.diagonal_slice",
        signature: "(code: Code, number: usize, level: usize, base: usize, height: usize) -> [[u32; 3]]",
        docs: "Lists the filled cells on the diagonal plane `x + y + z = height`, as `x, y, z` triples.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_three_diagonal_slice),
    },
    Function {
        name: "math.three.diagonal_svg",
        signature: "(code: Code, number: usize, level: usize, base: usize, heights: [usize], scale: usize) -> String",
        docs: "Draws the given diagonal slices as one circle per cell, coloured by height slot and top-scale corner.\n\nThe frame is tight around the projected points and carries no background, so the drawing sits on\nwhatever the page is.\n\n# Errors\n\nErrors when the code is out of range, the level is below one, or nothing renders.",
        uncallable: None,
        door: Some(door_math_three_diagonal_svg),
    },
    Function {
        name: "math.three.dust",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the dust cube, filled where every coordinate is even, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_dust),
    },
    Function {
        name: "math.three.edge_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "Extracts the network of corners and edges outlining every filled site.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_three_edge_graph),
    },
    Function {
        name: "math.three.euler",
        signature: "(cell: Cell3d) -> i64",
        docs: "Returns the Euler characteristic of the filled complex, vertices less edges plus faces less sites.\n\nOne for a solid block, and one less than twice the genus below zero for a tunnelled one.\n\n# Errors\n\nErrors when the edge network cannot be lifted from the cube.",
        uncallable: None,
        door: Some(door_math_three_euler),
    },
    Function {
        name: "math.three.extrude",
        signature: "(cell: Cell2d, axis: usize, depth: usize) -> Cell3d",
        docs: "Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it.\n\nThis is the inverse of slice: every slice of the lift on that axis is the flat cell again.\n\n# Errors\n\nErrors when the axis is not 0, 1 or 2, or the depth is below one.",
        uncallable: None,
        door: Some(door_math_three_extrude),
    },
    Function {
        name: "math.three.extrude_cube",
        signature: "(cell: Cell3d, depth: usize) -> Cell3d",
        docs: "Repeats every plane of a cube depth times along its leading axis, colors and tags with it.\n\n# Errors\n\nErrors when the depth is below one.",
        uncallable: None,
        door: Some(door_math_three_extrude_cube),
    },
    Function {
        name: "math.three.faces",
        signature: "(cell: Cell3d) -> usize",
        docs: "Returns the count of unit faces the filled sites touch, a face shared by two sites counted once.\n\nSurface counts only the faces open to void; this counts every face of the complex.",
        uncallable: None,
        door: Some(door_math_three_faces),
    },
    Function {
        name: "math.three.fills",
        signature: "(cell: Cell3d) -> usize",
        docs: "Returns the count of filled sites.",
        uncallable: None,
        door: Some(door_math_three_fills),
    },
    Function {
        name: "math.three.from_corners",
        signature: "(corners: [[u8]], number: usize, level: usize, base: usize) -> Cell3d",
        docs: "Builds a cube from its corner patterns, deepened to the given fractal level.\n\n# Errors\n\nErrors when a corner does not fit the dimension and base.",
        uncallable: None,
        door: Some(door_math_three_from_corners),
    },
    Function {
        name: "math.three.from_json",
        signature: "(text: str) -> Cell3d",
        docs: "Parses a cell from its JSON, colors and tags included.\n\n# Errors\n\nErrors when the text is not a cube.",
        uncallable: None,
        door: Some(door_math_three_from_json),
    },
    Function {
        name: "math.three.from_strings",
        signature: "(data: [[String]]) -> Cell3d",
        docs: "Builds a cube from one string of digits per row, grouped plane by plane.\n\n# Errors\n\nErrors when the planes are empty, ragged, or hold anything but digits.",
        uncallable: None,
        door: Some(door_math_three_from_strings),
    },
    Function {
        name: "math.three.hidden",
        signature: "(cell: Cell3d) -> u128",
        docs: "Returns the count of faces buried between two filled sites, six per site less the exposed surface.",
        uncallable: None,
        door: Some(door_math_three_hidden),
    },
    Function {
        name: "math.three.level_set",
        signature: "(number: usize, levels: [usize], level: usize, base: usize) -> Cell3d",
        docs: "Builds the cube filled wherever the residue sum lands in the levels, at the given level.\n\nCarpet, net and void are the three presets of this one engine: the levels are all a\nsymmetric cube design ever names.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_level_set),
    },
    Function {
        name: "math.three.magic",
        signature: "(cells: [Cell2d|Cell3d]) -> Cell2d|Cell3d",
        docs: "Folds two or more cells into one by chained Kronecker combination.\n\n# Errors\n\nErrors when the cells differ in shape.",
        uncallable: None,
        door: Some(door_math_three_magic),
    },
    Function {
        name: "math.three.manhattan_layers",
        signature: "(cell: Cell3d) -> Cell3d",
        docs: "Tags every site with its Manhattan distance from the cube's center, the diamond shells.\n\nThe cell's own layers count Chebyshev shells, which are boxes; these are octahedra.",
        uncallable: None,
        door: Some(door_math_three_manhattan_layers),
    },
    Function {
        name: "math.three.merge",
        signature: "(cells: [Cell3d], width: usize, height: usize, depth: usize) -> Cell3d",
        docs: "Merges the cells into one cube arranged width by height by depth.\n\n# Errors\n\nErrors when the cells differ in shape or do not fill the arrangement.",
        uncallable: None,
        door: Some(door_math_three_merge),
    },
    Function {
        name: "math.three.mosaic",
        signature: "(mask: Tensor, cells: [Cell2d|Cell3d]) -> Cell2d|Cell3d",
        docs: "Builds a cell by placing at each mask site the cell its value indexes.\n\n# Errors\n\nErrors when the mask and the cells do not fit one another.",
        uncallable: None,
        door: Some(door_math_three_mosaic),
    },
    Function {
        name: "math.three.named",
        signature: "(design: tile.recipe.Design, number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube the name picks, deepened to the given fractal level.\n\n# Errors\n\nErrors for a design that is not 3d, or a level below one.",
        uncallable: None,
        door: Some(door_math_three_named),
    },
    Function {
        name: "math.three.net",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the net cube, filled where at least two coordinates are odd, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_net),
    },
    Function {
        name: "math.three.noise",
        signature: "(number: usize, level: usize, density: f64, rng: Rng(seed)) -> Cell3d",
        docs: "Builds a cube whose every site turns on with probability density, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_noise),
    },
    Function {
        name: "math.three.ones",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the solid cube at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_ones),
    },
    Function {
        name: "math.three.orientations",
        signature: "() -> [(usize, usize, usize)]",
        docs: "Returns the 24 rotation triples that reach each distinct cube orientation.",
        uncallable: None,
        door: Some(door_math_three_orientations),
    },
    Function {
        name: "math.three.point",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the point cube, filled where every coordinate is odd, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_point),
    },
    Function {
        name: "math.three.profile",
        signature: "(code: Code, number: usize, level: usize, base: usize) -> [u128]",
        docs: "Counts the filled cells on every diagonal plane `x + y + z = s`, for `s` in `0..=3*(side - 1)`.\n\nThe count is the coefficient of the digit polynomial, the product over the scales of the\npattern's weight sums, so no cell of the solid is ever built.\n\n# Errors\n\nErrors when the code is out of range, or the level is below one.",
        uncallable: None,
        door: Some(door_math_three_profile),
    },
    Function {
        name: "math.three.project",
        signature: "(point: [u32; 3]) -> (f64, f64)",
        docs: "Projects a cell down the `(1,1,1)` axis: `u = (x - y)/sqrt 2`, `v = (x + y - 2z)/sqrt 6`.",
        uncallable: None,
        door: Some(door_math_three_project),
    },
    Function {
        name: "math.three.quads",
        signature: "(cell: Cell3d) -> [math.three.Quad]",
        docs: "Returns one outward quad per exposed face, scaled into the unit box.",
        uncallable: None,
        door: Some(door_math_three_quads),
    },
    Function {
        name: "math.three.shadow",
        signature: "(point: [u32; 3]) -> (i64, i64)",
        docs: "Returns the integer shadow `(x - y, x + y - 2z)`, the projection with its irrational scales dropped.",
        uncallable: None,
        door: Some(door_math_three_shadow),
    },
    Function {
        name: "math.three.slice",
        signature: "(cell: Cell3d, axis: usize, index: usize) -> Cell2d",
        docs: "Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it.\n\n# Errors\n\nErrors when the axis or the index is past the cube.",
        uncallable: None,
        door: Some(door_math_three_slice),
    },
    Function {
        name: "math.three.special",
        signature: "(mask: Tensor, cell: Cell3d) -> Cell3d",
        docs: "Orients a copy of the cell by each mask value and merges them in the mask's shape.\n\n# Errors\n\nErrors when the mask is not 3d, or holds a value past twenty-three.",
        uncallable: None,
        door: Some(door_math_three_special),
    },
    Function {
        name: "math.three.sponge.deep",
        signature: "(radius: f64) -> f64",
        docs: "The volume `Deep(radius)` of the points of one arm's quarter beyond the tubes of both walls it touches, at every radius, to double precision.\n\nWith `line = min(radius, 1/6)`, the quarter is `[0, line]^2` across the arm times its length,\nand every point of `Deep` sits, through both of its wall coordinates, in one and the same hole\nof the wall carpet that the line `across = line` cuts: a crossing hole. The volume is the sum\nover crossing holes of one arcsine integral, `2^(m-1)` holes of side `3^(-m-1)` at every level\n`m` for `line = 1/6`, none at all for `1/12`, whose triple `1/4 = 0.0202...` has no digit `1`.\nIt vanishes from `sqrt(10)/18` on, where the arms are swallowed. Each integral is elementary, but its\nexpanded arcsine form cancels at deep levels, so it is summed by 16-point Gauss-Legendre on each smooth\npiece in the angle `asin(R/radius)`, `R` the depth that a wall point needs.",
        uncallable: None,
        door: Some(door_math_three_sponge_deep),
    },
    Function {
        name: "math.three.sponge.dimension",
        signature: "() -> f64",
        docs: "The Minkowski dimension of the Menger sponge, `log(20)/log(3)`, the similarity dimension of its 20 maps of ratio `1/3`.",
        uncallable: None,
        door: Some(door_math_three_sponge_dimension),
    },
    Function {
        name: "math.three.sponge.distance",
        signature: "(point: [f64; 3]) -> f64",
        docs: "The Euclidean distance from a point of the unit cube to the Menger sponge, exact to the last binary place.\n\nThe point descends the base-3 digits while its digit triple holds at most one `1`, each step\ndividing the distance by 3, since inside a retained subcube the distance to the sponge is the\ndistance to that subcube's copy of it. Once it lands in the plus of seven removed cubes, the\ndistance is the one to the twelve edges of the centre cube or to the four carpets walling the\narm it sits in. Coordinates outside `[0, 1]` are clamped; the window centre `(1/2, 1/2, 0)`\nreads `1/6` and the centre of the cube `sqrt(2)/6`.",
        uncallable: None,
        door: Some(door_math_three_sponge_distance),
    },
    Function {
        name: "math.three.sponge.exact",
        signature: "(radius: f64) -> f64",
        docs: "The volume `T(radius)` of the points of the plus within `radius` of the sponge at every radius, `Deep` included, to double precision.\n\nOn `(0, 1/6]` it is the closed form of `tube` less `24 Deep`. On `[1/6, sqrt(2)/6]` the arms\nhold `2/9 - 24 Deep` and the centre cube `pi r^2 - 8 sqrt(2) r^3 - 4 r^2 acos(1/(6r)) +\n(2/3 + 16 c^2) c` with `c = sqrt(r^2 - 1/36)`, the edge cylinders overlapping along the faces;\nfrom `sqrt(2)/6` on the plus is swallowed, `7/27`.",
        uncallable: None,
        door: Some(door_math_three_sponge_exact),
    },
    Function {
        name: "math.three.sponge.profile",
        signature: "(radius: f64) -> f64?",
        docs: "The periodic function `p` of Kombrink, Pearse and Winter at `radius`: the reading's limit profile, unchanged when the radius is multiplied by 3, or `None` on the phases `(1/6, sqrt(2)/6]` where `T` has no closed form.\n\nThe radius is first carried by powers of 3 into `(sqrt(2)/18, sqrt(2)/6]`; on\n`(sqrt(2)/18, 1/6]` it is `r^(D-3) (20/27 + sum_(l = 0..40) (27/20)^l T(r/3^l))`, and the\nsponge is Minkowski measurable exactly when this is constant. It is not: the certificate puts\n`p(1/12)` in `[2.122718, 2.122723]` and `p(1/6)` in `[2.135019, 2.136794]`.",
        uncallable: None,
        door: Some(door_math_three_sponge_profile),
    },
    Function {
        name: "math.three.sponge.reading",
        signature: "(radius: f64) -> f64?",
        docs: "The Minkowski reading `radius^(D-3)` times the volume inside the cube, the number whose limit as the radius shrinks would be the sponge's Minkowski content.",
        uncallable: None,
        door: Some(door_math_three_sponge_reading),
    },
    Function {
        name: "math.three.sponge.tube",
        signature: "(radius: f64) -> f64?",
        docs: "The volume `T(radius)` of the points of the plus of seven removed level-1 cubes within `radius` of the sponge, or `None` on `(1/6, sqrt(2)/6)`, where no closed form is known.\n\nOn `(0, 1/6]` it is `(pi + 8) r^2 - 8 sqrt(2) r^3 + 48 (V1 - A1)`, the centre cube's edge\ncylinders plus the arcsine hole sums of the 24 wall carpets, `V1` over half a wall and `A1`\nover the strip along a shared edge, both summed level by level and closed by their geometric\ntails. The one term left out is `24 Deep`, `Deep` the corner volume beyond both wall tubes,\nat most `24 x 3.842e-5 < 9.23e-4` at `1/6` and `5.81e-9` at `1/12` by the paper's bound on\n`Deep`, so this is the upper edge of the certified enclosure:\n`T(1/6) = (pi + 8)/36 - sqrt(2)/27`. From `sqrt(2)/6` on the plus is swallowed, `7/27`.",
        uncallable: None,
        door: Some(door_math_three_sponge_tube),
    },
    Function {
        name: "math.three.sponge.volume",
        signature: "(radius: f64) -> f64?",
        docs: "The volume of the points of the unit cube within `radius` of the sponge, `sum_k (20/27)^k T(3^k radius)`, or `None` when some `3^k radius` falls where `T` has no closed form.\n\nThe cube is the plus and 20 retained subcubes, each holding a copy of the whole picture at a\nthird of the scale, so the volume is `T(radius)` plus `20/27` of the volume at three times the\nradius, and the sum stops at the first radius past `sqrt(2)/6`, where the volume is the cube.",
        uncallable: None,
        door: Some(door_math_three_sponge_volume),
    },
    Function {
        name: "math.three.star",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the star cube, filled where exactly one coordinate is odd, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_star),
    },
    Function {
        name: "math.three.support",
        signature: "(counts: [u128]) -> (usize, usize)?",
        docs: "Returns the first and last height a profile fills, or none when the design is empty.",
        uncallable: None,
        door: Some(door_math_three_support),
    },
    Function {
        name: "math.three.surface",
        signature: "(cell: Cell3d) -> u128",
        docs: "Returns the count of filled faces exposed to void or the outside.",
        uncallable: None,
        door: Some(door_math_three_surface),
    },
    Function {
        name: "math.three.text",
        signature: "(cell: Cell3d, glyphs: {u8: String}?) -> [String]",
        docs: "Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped.",
        uncallable: None,
        door: Some(door_math_three_text),
    },
    Function {
        name: "math.three.to_json",
        signature: "(cell: Cell3d) -> String",
        docs: "Serializes the cell's shape and types to JSON, with colors and tags when present.",
        uncallable: None,
        door: Some(door_math_three_to_json),
    },
    Function {
        name: "math.three.to_obj",
        signature: "(cell: Cell3d) -> String",
        docs: "Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner.",
        uncallable: None,
        door: Some(door_math_three_to_obj),
    },
    Function {
        name: "math.three.to_strings",
        signature: "(cell: Cell3d) -> [[String]]",
        docs: "Unrolls the cube into one string of digits per row, grouped plane by plane.",
        uncallable: None,
        door: Some(door_math_three_to_strings),
    },
    Function {
        name: "math.three.tunnel_graph",
        signature: "(cell: Cell2d|Cell3d) -> math.graph.Network",
        docs: "Extracts the network of empty sites joined to their axis neighbors.\n\n# Errors\n\nErrors when the cell is neither two- nor three-dimensional.",
        uncallable: None,
        door: Some(door_math_three_tunnel_graph),
    },
    Function {
        name: "math.three.void",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the checkerboard cube, filled where all coordinate parities agree, at the given level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_void),
    },
    Function {
        name: "math.three.voids",
        signature: "(cell: Cell3d) -> usize",
        docs: "Returns the count of empty sites.",
        uncallable: None,
        door: Some(door_math_three_voids),
    },
    Function {
        name: "math.three.volume",
        signature: "(cell: Cell3d) -> usize",
        docs: "Returns the filled-site count, the cube's volume.",
        uncallable: None,
        door: Some(door_math_three_volume),
    },
    Function {
        name: "math.three.wires",
        signature: "(cell: Cell3d) -> [[math.three.Vec3; 2]]",
        docs: "Returns the cell's edge-graph segments, scaled into the unit box.",
        uncallable: None,
        door: Some(door_math_three_wires),
    },
    Function {
        name: "math.three.xline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of rods along the x axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_xline),
    },
    Function {
        name: "math.three.xtree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of beams along the x axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_xtree),
    },
    Function {
        name: "math.three.yline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of rods along the y axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_yline),
    },
    Function {
        name: "math.three.ytree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of beams along the y axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_ytree),
    },
    Function {
        name: "math.three.zeros",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the all-void cube at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_zeros),
    },
    Function {
        name: "math.three.zline",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of rods along the z axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_zline),
    },
    Function {
        name: "math.three.ztree",
        signature: "(number: usize, level: usize) -> Cell3d",
        docs: "Builds the cube of beams along the z axis at the given size and level.\n\n# Errors\n\nErrors when the seed is not a cube.",
        uncallable: None,
        door: Some(door_math_three_ztree),
    },
    Function {
        name: "math.tourbillon.eyes",
        signature: "(qmax: usize) -> [math.tourbillon.Eye]",
        docs: "The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle.\n\nThe odd carpet is unchanged by a quarter turn, so two layers whose angles agree modulo ninety stand on one exact lattice of nodes and the stack stops moving there.",
        uncallable: None,
        door: Some(door_math_tourbillon_eyes),
    },
    Function {
        name: "math.tourbillon.field",
        signature: "(top: usize, size: usize, schedule: str, increment: f64, set: str, weights: str, mode: str, blend: str, seed: u32) -> [f32]",
        docs: "Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.\n\nThe schedule is unspun, degrees, golden, primes, random or gaussian, the layer set is odd, primes, squarefree or prime powers, the weights are plain, mobius or harmonic, the render mode is cells, edges or corners and the blend is mean, sum, union, meet, parity or difference. The weights are scaled to average magnitude one and apply only under the linear blends, so mean is paper coverage and sum is the inked layer count. Sites outside the disc are NaN.\n\n# Errors\n\nErrors at a raster or top out of range, or for a schedule, set, weights or blend name the stack does not know.",
        uncallable: None,
        door: Some(door_math_tourbillon_field),
    },
    Function {
        name: "math.tourbillon.layers",
        signature: "(top: usize, schedule: str, increment: f64, set: str, weights: str, seed: u32) -> [math.tourbillon.Layer]",
        docs: "The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle.\n\nThe schedule is unspun, degrees, golden, primes, random or gaussian, the layer set is odd, primes, squarefree or prime powers and the weights are plain, mobius or harmonic. The weights are rescaled so their magnitudes sum to the layer count, so the mean blend stays paper coverage and the sum blend stays the inked layer count.\n\n# Errors\n\nErrors at an even top or one out of range, or for a schedule, set or weights name the stack does not know.",
        uncallable: None,
        door: Some(door_math_tourbillon_layers),
    },
    Function {
        name: "math.tourbillon.period",
        signature: "(increment: f64) -> usize?",
        docs: "The least whole number of increments that closes a quarter turn, none once the count passes the cap.",
        uncallable: None,
        door: Some(door_math_tourbillon_period),
    },
    Function {
        name: "math.tourbillon.sharing",
        signature: "(list: [math.tourbillon.Layer]) -> (usize, usize)",
        docs: "The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one.",
        uncallable: None,
        door: Some(door_math_tourbillon_sharing),
    },
    Function {
        name: "math.tourbillon.stack",
        signature: "(list: [math.tourbillon.Layer], size: usize, mode: str, blend: math.spin.Blend) -> [f32]",
        docs: "Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site.\n\nThe render mode is cells, the lit squares of the carpet, edges, the boundaries between its cells, or corners, the vertices of its lit squares. The weights ride on the layers under the linear blends only. Sites outside the disc are NaN.\n\n# Errors\n\nErrors for a mode that is not cells, edges or corners, or a raster past the site budget.",
        uncallable: None,
        door: Some(door_math_tourbillon_stack),
    },
    Function {
        name: "math.tourbillon.stats",
        signature: "(field: [f32], size: usize, top: usize, schedule: str, increment: f64, set: str, weights: str, blend: str, seed: u32) -> math.tourbillon.Stats",
        docs: "Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites.\n\nThe centre is computed from the layers at the exact half-half point, never sampled off the raster, so no turn of the layers moves it.\n\n# Errors\n\nErrors when the field is not size by size, or a name the stack does not know.",
        uncallable: None,
        door: Some(door_math_tourbillon_stats),
    },
    Function {
        name: "math.two.capacity",
        signature: "(cell: Cell2d) -> usize",
        docs: "Returns the payload bytes the cell's filled sites can hold, its length header paid for.",
        uncallable: None,
        door: Some(door_math_two_capacity),
    },
    Function {
        name: "math.two.carpet",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the carpet fractal, its seed pierced at every odd-odd site, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_carpet),
    },
    Function {
        name: "math.two.census",
        signature: "(cell: Cell2d) -> math.two.Census",
        docs: "Takes the cell's full census in one reading.\n\n# Errors\n\nErrors when the edge network cannot be lifted from the cell.",
        uncallable: None,
        door: Some(door_math_two_census),
    },
    Function {
        name: "math.two.create",
        signature: "(code: Code, number: usize, level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "Builds the design a universe code names, deepened to the level and rotated by quarter-turns.\n\n# Errors\n\nErrors when the code is out of range.",
        uncallable: None,
        door: Some(door_math_two_create),
    },
    Function {
        name: "math.two.dust",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the dust fractal, its seed on at every even-even site, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_dust),
    },
    Function {
        name: "math.two.embed",
        signature: "(cell: Cell2d, payload: [u8]) -> Cell2d",
        docs: "Writes the payload over the cell's filled sites, repeating it until every site is spoken for.\n\n# Errors\n\nErrors when the payload needs more sites than the cell carries.",
        uncallable: None,
        door: Some(door_math_two_embed),
    },
    Function {
        name: "math.two.euler",
        signature: "(cell: Cell2d) -> i64",
        docs: "Returns the Euler characteristic of the filled sites, vertices less edges plus faces.\n\n# Errors\n\nErrors when the edge network cannot be lifted from the cell.",
        uncallable: None,
        door: Some(door_math_two_euler),
    },
    Function {
        name: "math.two.extract",
        signature: "(carrier: Cell2d, carried: Cell2d) -> [u8]",
        docs: "Reads the payload back, the plain cell naming the sites the carried one wrote over.\n\n# Errors\n\nErrors when the two cells differ in shape, or the header does not fit.",
        uncallable: None,
        door: Some(door_math_two_extract),
    },
    Function {
        name: "math.two.fills",
        signature: "(cell: Cell2d) -> usize",
        docs: "Counts the filled sites of the cell.",
        uncallable: None,
        door: Some(door_math_two_fills),
    },
    Function {
        name: "math.two.from_corners",
        signature: "(corners: [[u8]], number: usize, level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "Builds the design straight from its filled residue corners, deepened to the level and rotated by quarter-turns.\n\n# Errors\n\nErrors when a corner does not fit the dimension and base.",
        uncallable: None,
        door: Some(door_math_two_from_corners),
    },
    Function {
        name: "math.two.from_json",
        signature: "(text: str) -> Cell2d",
        docs: "Restores a cell from its JSON string, colors and tags included.\n\n# Errors\n\nErrors when the text is not a flat cell.",
        uncallable: None,
        door: Some(door_math_two_from_json),
    },
    Function {
        name: "math.two.from_strings",
        signature: "(rows: [String]) -> Cell2d",
        docs: "Builds a cell from rows of digits, the inverse of the text rendering.\n\n# Errors\n\nErrors when a row is empty, ragged, or holds anything but decimal digits.",
        uncallable: None,
        door: Some(door_math_two_from_strings),
    },
    Function {
        name: "math.two.hline",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the hline fractal, its seed striped along odd rows, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_hline),
    },
    Function {
        name: "math.two.htree",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the htree fractal, its seed striped along even rows, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_htree),
    },
    Function {
        name: "math.two.image",
        signature: "(cell: Cell2d, scale: usize, outline: Color?, width: usize, shape: math.two.Shape) -> Image",
        docs: "Renders the cell to an image at the given pixel scale, stroked and padded when an outline is given.\n\n# Errors\n\nErrs on a scale below one.",
        uncallable: None,
        door: Some(door_math_two_image),
    },
    Function {
        name: "math.two.level_set",
        signature: "(number: usize, levels: [usize], level: usize, rotation: usize, base: usize) -> Cell2d",
        docs: "Builds the level-set design, filling every residue corner whose digits sum to a named level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_level_set),
    },
    Function {
        name: "math.two.mask",
        signature: "(mask: Tensor, shape: [usize]) -> Tensor",
        docs: "Tiles the mask over the shape and crops it, the perforation pattern itself.\n\n# Errors\n\nErrors when the mask or the shape is not two-axis, or the mask is empty.",
        uncallable: None,
        door: Some(door_math_two_mask),
    },
    Function {
        name: "math.two.merge",
        signature: "(cells: [Cell2d], width: usize, height: usize) -> Cell2d",
        docs: "Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept.\n\n# Errors\n\nErrors on an empty list, a wrong cell count, or cells of different shapes.",
        uncallable: None,
        door: Some(door_math_two_merge),
    },
    Function {
        name: "math.two.named",
        signature: "(design: tile.recipe.Design, number: usize, level: usize, rotation: usize) -> Cell2d",
        docs: "Builds the design the name picks, deepened to the level and rotated by quarter-turns.\n\n# Errors\n\nErrors for a design that is not 2d, or a level below one.",
        uncallable: None,
        door: Some(door_math_two_named),
    },
    Function {
        name: "math.two.net",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the net fractal, its seed on wherever a coordinate is odd, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_net),
    },
    Function {
        name: "math.two.noise",
        signature: "(number: usize, level: usize, density: f64, rng: Rng(seed)) -> Cell2d",
        docs: "Builds a random cell, each seed site drawn on with probability density, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_noise),
    },
    Function {
        name: "math.two.ones",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds an all-filled cell of the given size and level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_ones),
    },
    Function {
        name: "math.two.payload.frame",
        signature: "() -> Tensor",
        docs: "The five by five mask a carried mosaic lays its four tiles out under.",
        uncallable: None,
        door: Some(door_math_two_payload_frame),
    },
    Function {
        name: "math.two.perimeter",
        signature: "(cell: Cell2d) -> u128",
        docs: "Counts the faces of filled sites open to emptiness or the border.",
        uncallable: None,
        door: Some(door_math_two_perimeter),
    },
    Function {
        name: "math.two.point",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the point fractal, its seed on at every odd-odd site, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_point),
    },
    Function {
        name: "math.two.read",
        signature: "(sheet: Cell2d, carrier: Cell2d) -> [u8]",
        docs: "Reads the payload back from a framed sheet, the plain fourth cell naming the sites.\n\n# Errors\n\nErrors when the sheet is not five carriers across and down.",
        uncallable: None,
        door: Some(door_math_two_read),
    },
    Function {
        name: "math.two.sheet",
        signature: "(cells: [Cell2d; 4], payload: [u8]) -> Cell2d",
        docs: "Builds the framed sheet of four same-sized cells, the fourth carrying the payload.\n\n# Errors\n\nErrors when the payload needs more sites than the carrier holds.",
        uncallable: None,
        door: Some(door_math_two_sheet),
    },
    Function {
        name: "math.two.special",
        signature: "(mask: Tensor, cell: Cell2d) -> Cell2d",
        docs: "Tiles quarter-turned copies of the cell as the 2d mask directs.\n\n# Errors\n\nErrors when the mask is not 2d, or holds a rotation past three.",
        uncallable: None,
        door: Some(door_math_two_special),
    },
    Function {
        name: "math.two.star",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the star fractal, its seed on where exactly one coordinate is odd, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_star),
    },
    Function {
        name: "math.two.svg",
        signature: "(cell: Cell2d, scale: usize, outline: Color?, width: usize, shape: math.two.Shape) -> String",
        docs: "Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given.",
        uncallable: None,
        door: Some(door_math_two_svg),
    },
    Function {
        name: "math.two.text",
        signature: "(cell: Cell2d, glyphs: {u8: String}?) -> [String]",
        docs: "Renders the cell as rows of glyphs, or of digits where no glyph is mapped.",
        uncallable: None,
        door: Some(door_math_two_text),
    },
    Function {
        name: "math.two.to_3d",
        signature: "(cell: Cell2d) -> Cell3d",
        docs: "Lifts the flat cell into a cube one site deep, colors and tags with it.",
        uncallable: None,
        door: Some(door_math_two_to_3d),
    },
    Function {
        name: "math.two.to_json",
        signature: "(cell: Cell2d) -> String",
        docs: "Serializes the cell to a JSON string of its types, with colors and tags when present.",
        uncallable: None,
        door: Some(door_math_two_to_json),
    },
    Function {
        name: "math.two.vline",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the vline fractal, its seed striped along odd columns, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_vline),
    },
    Function {
        name: "math.two.void",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the void fractal, its seed a checkerboard on even parity, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_void),
    },
    Function {
        name: "math.two.voids",
        signature: "(cell: Cell2d) -> usize",
        docs: "Counts the empty sites of the cell.",
        uncallable: None,
        door: Some(door_math_two_voids),
    },
    Function {
        name: "math.two.vtree",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds the vtree fractal, its seed striped along even columns, deepened to the level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_vtree),
    },
    Function {
        name: "math.two.zeros",
        signature: "(number: usize, level: usize) -> Cell2d",
        docs: "Builds an all-empty cell of the given size and level.\n\n# Errors\n\nErrors when the seed is not a plane.",
        uncallable: None,
        door: Some(door_math_two_zeros),
    },
    Function {
        name: "num.apollonian.Circle.centre",
        signature: "(self: num.apollonian.Circle) -> (f64, f64)?",
        docs: "The centre, none on a line.",
        uncallable: None,
        door: Some(door_num_apollonian_circle_centre),
    },
    Function {
        name: "num.apollonian.Circle.is_line",
        signature: "(self: num.apollonian.Circle) -> bool",
        docs: "Whether the circle is a line.",
        uncallable: None,
        door: Some(door_num_apollonian_circle_is_line),
    },
    Function {
        name: "num.apollonian.Circle.radius",
        signature: "(self: num.apollonian.Circle) -> f64?",
        docs: "The radius, none on a line.",
        uncallable: None,
        door: Some(door_num_apollonian_circle_radius),
    },
    Function {
        name: "num.apollonian.flat",
        signature: "(name: str, cap: i64) -> [f64]",
        docs: "Grows the named packing and lays every circle of positive curvature as `x, y, r, k` in one flat run: the root's own circles first in root order, then the grown ones in curvature order.\n\n# Errors\n\nErrs as `grow` does.",
        uncallable: None,
        door: Some(door_num_apollonian_flat),
    },
    Function {
        name: "num.apollonian.form",
        signature: "(u: [i64; 4], v: [i64; 4]) -> i128",
        docs: "The bilinear form `B(u, v) = (sum u)(sum v) - 2 sum u v` that the reflection preserves.",
        uncallable: None,
        door: Some(door_num_apollonian_form),
    },
    Function {
        name: "num.apollonian.frame",
        signature: "(p: num.apollonian.Packing) -> [f64; 4]",
        docs: "The box the packing is drawn in: one period of the strip, or the box of the circle that contains a bounded packing.",
        uncallable: None,
        door: Some(door_num_apollonian_frame),
    },
    Function {
        name: "num.apollonian.grow",
        signature: "(name: str, cap: i64) -> num.apollonian.Packing",
        docs: "Grows the named packing to the curvature cap, one circle per node of the reflection tree and the root quadruple excluded, so `circles.len()` is the census `N(T)`. On the strip only the two root swaps that replace a line are taken, which are exactly the two that stay inside one period.\n\n# Errors\n\nErrs at a name outside `ROOTS`, and at a curvature cap outside two through `CURVATURE_CAP`.",
        uncallable: None,
        door: Some(door_num_apollonian_grow),
    },
    Function {
        name: "num.apollonian.is_ford",
        signature: "(c: num.apollonian.Circle) -> bool",
        docs: "Whether the circle is the Ford circle over its own tangency point: curvature `2 b^2` and abscissa `2 a b` at the reduced `a/b`.",
        uncallable: None,
        door: Some(door_num_apollonian_is_ford),
    },
    Function {
        name: "num.apollonian.on_line",
        signature: "(c: num.apollonian.Circle) -> bool",
        docs: "Whether the circle has positive curvature and is tangent to the line `y = 0`, which in these coordinates reads `k > 0` and `k y = 1`: the curvature guard is what excludes the line `y = 1`, which is `(0, 0, 1)`.",
        uncallable: None,
        door: Some(door_num_apollonian_on_line),
    },
    Function {
        name: "num.apollonian.reflect",
        signature: "(q: [num.apollonian.Circle; 4], at: usize) -> num.apollonian.Circle",
        docs: "Reflects the circle at the seat through the other three, `v' = 2(v_1 + v_2 + v_3) - v` on all three coordinates at once, which is the second root of the Descartes quadratic and needs no square root.",
        uncallable: None,
        door: Some(door_num_apollonian_reflect),
    },
    Function {
        name: "num.apollonian.root",
        signature: "(name: str) -> [num.apollonian.Circle; 4]",
        docs: "The named root quadruple: `strip` is the two lines a unit apart holding the circles at `0` and `1`, and the rest are bounded packings named by their four curvatures.\n\n# Errors\n\nErrs at a name outside `ROOTS`.",
        uncallable: None,
        door: Some(door_num_apollonian_root),
    },
    Function {
        name: "num.apollonian.shadow",
        signature: "(p: num.apollonian.Packing, order: usize) -> num.apollonian.Shadow",
        docs: "Reads the Farey stack of the order against the packing: the nodes lit inside the open period against the tangency points of the line-tangent circles of curvature at most `2 Q^2`, and the brightness `floor(Q/b)` summed on the nodes against `Q(Q + 1)/2`. Off the strip there is no line and every count is zero.\n\n# Errors\n\nErrs at an order outside one through `ORDER_CAP`.",
        uncallable: None,
        door: Some(door_num_apollonian_shadow),
    },
    Function {
        name: "num.apollonian.sound",
        signature: "(q: [num.apollonian.Circle; 4]) -> bool",
        docs: "Whether the quadruple carries all six exact invariants: Descartes `B(k, k) = 0`, the position half `B(k, kx) = B(k, ky) = B(kx, ky) = 0`, and the frame `B(kx, kx) = B(ky, ky) = -4`.",
        uncallable: None,
        door: Some(door_num_apollonian_sound),
    },
    Function {
        name: "num.apollonian.swap",
        signature: "(q: [num.apollonian.Circle; 4], at: usize) -> [num.apollonian.Circle; 4]",
        docs: "The quadruple with the circle at the seat replaced by its reflection.",
        uncallable: None,
        door: Some(door_num_apollonian_swap),
    },
    Function {
        name: "num.apollonian.touches",
        signature: "(p: num.apollonian.Packing) -> [num.apollonian.Touch]",
        docs: "The tangency points on the line `y = 0`, ascending: one per circle of the packing with `k y = 1`, the root excluded. Empty off the strip.",
        uncallable: None,
        door: Some(door_num_apollonian_touches),
    },
    Function {
        name: "num.blend.add",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "Adds two sequences term by term over their shared length.",
        uncallable: None,
        door: Some(door_num_blend_add),
    },
    Function {
        name: "num.blend.cauchy",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "Convolves two sequences, keeping the exact prefix their shared length affords.\n\n# Errors\n\nErrs when a term passes a signed hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_num_blend_cauchy),
    },
    Function {
        name: "num.blend.characteristic",
        signature: "(coefficients: [(i128, i128)]) -> [(i128, i128)]",
        docs: "Returns the monic characteristic polynomial of a recurrence, highest power first.",
        uncallable: None,
        door: Some(door_num_blend_characteristic),
    },
    Function {
        name: "num.blend.decimate",
        signature: "(a: [i128], step: usize, offset: usize) -> [i128]",
        docs: "Keeps every step-th term from the offset onward.\n\n# Errors\n\nErrs at a step of zero.",
        uncallable: None,
        door: Some(door_num_blend_decimate),
    },
    Function {
        name: "num.blend.delta",
        signature: "(a: [i128]) -> [i128]",
        docs: "Returns the first differences of a sequence, one term shorter.",
        uncallable: None,
        door: Some(door_num_blend_delta),
    },
    Function {
        name: "num.blend.growth",
        signature: "(coefficients: [(i128, i128)]) -> f64",
        docs: "Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands.\n\nA simple root lands at machine precision; a repeated root lands to a few decimals only.",
        uncallable: None,
        door: Some(door_num_blend_growth),
    },
    Function {
        name: "num.blend.hadamard",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "Multiplies two sequences term by term over their shared length.",
        uncallable: None,
        door: Some(door_num_blend_hadamard),
    },
    Function {
        name: "num.blend.recurrence",
        signature: "(terms: [i128]) -> [(i128, i128)]?",
        docs: "Finds the smallest linear constant-coefficient recurrence that fits every supplied term.\n\nThe coefficients come back as reduced fractions with the newest term first, so a\nresult of one and one twice is the Fibonacci rule. The hunt runs modulo three\nprimes, rebuilds the rationals by remainder reconstruction, and verifies the rule\non every term before answering; too few terms to trust an order returns nothing,\nand the zero sequence returns the empty rule.",
        uncallable: None,
        door: Some(door_num_blend_recurrence),
    },
    Function {
        name: "num.blend.scale",
        signature: "(a: [i128], factor: i128) -> [i128]",
        docs: "Multiplies every term of a sequence by the factor.",
        uncallable: None,
        door: Some(door_num_blend_scale),
    },
    Function {
        name: "num.blend.shift",
        signature: "(a: [i128], count: usize) -> [i128]",
        docs: "Drops the first terms of a sequence.",
        uncallable: None,
        door: Some(door_num_blend_shift),
    },
    Function {
        name: "num.blend.sigma",
        signature: "(a: [i128]) -> [i128]",
        docs: "Returns the partial sums of a sequence.\n\n# Errors\n\nErrs when a partial sum passes a signed hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_num_blend_sigma),
    },
    Function {
        name: "num.blend.sub",
        signature: "(a: [i128], b: [i128]) -> [i128]",
        docs: "Subtracts the second sequence from the first over their shared length.",
        uncallable: None,
        door: Some(door_num_blend_sub),
    },
    Function {
        name: "num.boolean.is_balanced",
        signature: "(code: u128, n: usize) -> bool",
        docs: "Reports whether the packed function outputs one on exactly half of its inputs.",
        uncallable: None,
        door: Some(door_num_boolean_is_balanced),
    },
    Function {
        name: "num.boolean.nonlinearity",
        signature: "(code: u128, n: usize) -> i64",
        docs: "Returns how far the packed function sits from every affine function, zero when it is one.",
        uncallable: None,
        door: Some(door_num_boolean_nonlinearity),
    },
    Function {
        name: "num.boolean.sac",
        signature: "(code: u128, n: usize) -> f64",
        docs: "Returns the mean chance that flipping one input bit flips the output, 0.5 at full avalanche.",
        uncallable: None,
        door: Some(door_num_boolean_sac),
    },
    Function {
        name: "num.boolean.walsh_spectrum",
        signature: "(code: u128, n: usize) -> [i64]",
        docs: "Returns the Walsh spectrum of an n-input boolean function packed as a truth-table code.",
        uncallable: None,
        door: Some(door_num_boolean_walsh_spectrum),
    },
    Function {
        name: "num.chladni.ladder",
        signature: "(cap: usize) -> [num.chladni.Rung]",
        docs: "Lists every distinct plate figure with `0 <= n <= m <= cap` but `(0, 0)`, both signs when `n < m` and the sign 1 when `n == m`, by pitch, then `n`, then the sign 1 before -1.\n\n# Errors\n\nErrs when the cap is over 64.",
        uncallable: None,
        door: Some(door_num_chladni_ladder),
    },
    Function {
        name: "num.chladni.lines",
        signature: "(field: [f64], side: usize) -> [f64]",
        docs: "Traces the zero set of a plate raster by marching squares on the lattice of cell centres, its holes and its rim ghosted to the mean of their plate neighbours so the lines run to every free edge: segments `x0, y0, x1, y1` in `[0, 1]` back to back, pieces inside a NaN cell dropped, a saddle joined by the mean of its four corners.\n\n# Errors\n\nErrs when the side is 0, or the field is not the side squared or holds an infinity.",
        uncallable: None,
        door: Some(door_num_chladni_lines),
    },
    Function {
        name: "num.chladni.plate",
        signature: "(n: usize, m: usize, sign: i32, size: usize) -> [f64]",
        docs: "Samples the Chladni figure `cos(n pi x) cos(m pi y) + sign cos(m pi x) cos(n pi y)` of a free square plate at the cell centres `(i + 1/2) / size` of a row-major `size` by `size` raster, `x` along a row, scaled so the largest magnitude is 1, or all zero when no centre rises past `1e-12`; when `n == m` it is the single product for either sign.\n\n# Errors\n\nErrs when the sign is not 1 or -1, the size is 0 or over 1024, or a wave number is over 64.",
        uncallable: None,
        door: Some(door_num_chladni_plate),
    },
    Function {
        name: "num.chladni.shake",
        signature: "(field: [f64], side: usize, grains: [f32], from: u64, steps: usize, seed: u64) -> [f32]",
        docs: "Shakes sand on a plate raster: grains are `x, y` pairs in `[0, 1)` over the row-major `side` by `side` field, a NaN pair an empty slot that stays NaN, a NaN cell a hole. Each step moves a grain a random length `kick |f| + floor` in a random direction plus a drift `-0.3 f grad f / max|grad f|^2`, with `kick = 0.6 / max|grad f|` read off the raster by finite differences and a fixed kick on a flat field, the field read bilinearly with its holes and its rim ghosted to the mean of their plate neighbours. The rim reflects; a grain on a hole falls through and becomes NaN. Step `from + i` of grain `j` draws from a counter keyed by `(seed, from + i, j)`, so six steps are three and three.\n\n# Errors\n\nErrs when the side is 0, the field is not the side squared or holds an infinity, the grains hold an odd count or over 400,000 pairs, or the steps are over 64.",
        uncallable: None,
        door: Some(door_num_chladni_shake),
    },
    Function {
        name: "num.factor.aliquot",
        signature: "(number: usize) -> usize",
        docs: "Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one.",
        uncallable: None,
        door: Some(door_num_factor_aliquot),
    },
    Function {
        name: "num.factor.coprime",
        signature: "(a: usize, b: usize) -> bool",
        docs: "Returns whether two numbers share no divisor above one.",
        uncallable: None,
        door: Some(door_num_factor_coprime),
    },
    Function {
        name: "num.factor.divisors",
        signature: "(number: u64) -> [u64]",
        docs: "Builds every divisor of a wide number from its factorization, ascending, empty for zero.\n\nThe width is the one the factorization needs, so a caller on the pointer width casts at its own call.",
        uncallable: None,
        door: Some(door_num_factor_divisors),
    },
    Function {
        name: "num.factor.factorial",
        signature: "(number: usize) -> u128",
        docs: "Returns the factorial of the number, the product of one through it, erring past thirty-four.",
        uncallable: None,
        door: Some(door_num_factor_factorial),
    },
    Function {
        name: "num.factor.factorize",
        signature: "(number: usize) -> [(usize, u32)]",
        docs: "Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel.",
        uncallable: None,
        door: Some(door_num_factor_factorize),
    },
    Function {
        name: "num.factor.factorize_wide",
        signature: "(number: u64) -> [(u64, u32)]",
        docs: "Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel.",
        uncallable: None,
        door: Some(door_num_factor_factorize_wide),
    },
    Function {
        name: "num.factor.gcd",
        signature: "(a: u128, b: u128) -> u128",
        docs: "Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes.",
        uncallable: None,
        door: Some(door_num_factor_gcd),
    },
    Function {
        name: "num.factor.lcm",
        signature: "(a: usize, b: usize) -> usize",
        docs: "Returns the least common multiple of two numbers, zero when either side is zero.\n\nWraps silently once the multiple passes the pointer width.",
        uncallable: None,
        door: Some(door_num_factor_lcm),
    },
    Function {
        name: "num.factor.mobius",
        signature: "(number: usize) -> i8",
        docs: "Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes.\n\nThe argument is the pointer width, so a wide value narrows on wasm32 above two to the thirty-two.",
        uncallable: None,
        door: Some(door_num_factor_mobius),
    },
    Function {
        name: "num.factor.mobius_sieve",
        signature: "(limit: usize) -> [i8]",
        docs: "Sieves the Mobius values of zero through the limit in one pass.",
        uncallable: None,
        door: Some(door_num_factor_mobius_sieve),
    },
    Function {
        name: "num.factor.radical",
        signature: "(number: usize) -> usize",
        docs: "Returns the radical of the number, the product of its distinct primes, zero for zero and one for one.",
        uncallable: None,
        door: Some(door_num_factor_radical),
    },
    Function {
        name: "num.factor.reduce",
        signature: "(numerator: u128, denominator: u128) -> (u128, u128)",
        docs: "Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one.",
        uncallable: None,
        door: Some(door_num_factor_reduce),
    },
    Function {
        name: "num.factor.sigma",
        signature: "(number: usize, power: u32) -> u128",
        docs: "Returns the sum of every divisor of the number raised to the power, so power zero counts them.\n\nStays exact for a power of two or below, above which the sum can wrap past a hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_num_factor_sigma),
    },
    Function {
        name: "num.factor.squarefree",
        signature: "(number: usize) -> bool",
        docs: "Returns whether no prime squares into the number, true for one and false for zero.",
        uncallable: None,
        door: Some(door_num_factor_squarefree),
    },
    Function {
        name: "num.factor.totient",
        signature: "(number: usize) -> usize",
        docs: "Returns the Euler totient of the number from its factorization, zero for zero and one for one.",
        uncallable: None,
        door: Some(door_num_factor_totient),
    },
    Function {
        name: "num.factor.totients",
        signature: "(n: usize) -> [u64]",
        docs: "Sieves the Euler totients of zero through n in one pass, the run beside the single value.",
        uncallable: None,
        door: Some(door_num_factor_totients),
    },
    Function {
        name: "num.factor.twisted",
        signature: "(number: usize, rhythm: [i8]) -> i64",
        docs: "Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm.",
        uncallable: None,
        door: Some(door_num_factor_twisted),
    },
    Function {
        name: "num.fft.convolve",
        signature: "(field: [f64], kernel: [f64], size: usize) -> [f64]",
        docs: "Circularly convolves a size-square field on the torus by a kernel of the same shape through fft2 both ways.\n\n# Errors\n\nErrs when the field or the kernel is not the size squared, or when the size is no power of two.",
        uncallable: None,
        door: Some(door_num_fft_convolve),
    },
    Function {
        name: "num.fft.convolve_with",
        signature: "(field: [f64], kernel_re: [f64], kernel_im: [f64], size: usize) -> [f64]",
        docs: "Convolves a size-square field on the torus by a kernel already transformed by fft2, the inverse scaled back by size squared.",
        uncallable: None,
        door: Some(door_num_fft_convolve_with),
    },
    Function {
        name: "num.fft.embed_kernel",
        signature: "(mask: [u8], side: usize, size: usize) -> [f64]",
        docs: "Lays an odd-side mask into a size-square kernel with the mask centre at index (0, 0) and negative offsets wrapped; the cell at offset (dr, dc) lands at (-dr, -dc) modulo size, so convolving a field by the kernel reads at every site the mask-weighted sum over its neighbours, the neighbour count the life step counts.\n\n# Errors\n\nErrs at an even mask side, at a side past the field, and when the mask is not the side squared.",
        uncallable: None,
        door: Some(door_num_fft_embed_kernel),
    },
    Function {
        name: "num.fft.log_spectrum",
        signature: "(field: [f64], size: usize) -> [f64]",
        docs: "Returns the centred magnitude spectrum of a size-square field through log(1 + magnitude), the DC bin included at the centre.\n\n# Errors\n\nErrs when the field is not the size squared, or when the size is no power of two.",
        uncallable: None,
        door: Some(door_num_fft_log_spectrum),
    },
    Function {
        name: "num.fft.magnitude_spectrum",
        signature: "(field: [f64], size: usize) -> [f64]",
        docs: "Returns the magnitudes of a square field's transform, shifted so zero frequency sits at the centre.\n\n# Errors\n\nErrs when the field is not the size squared, or when the size is no power of two.",
        uncallable: None,
        door: Some(door_num_fft_magnitude_spectrum),
    },
    Function {
        name: "num.fft.peak_ring",
        signature: "(profile: [f64]) -> usize",
        docs: "Finds the ring past the centre where a radial profile peaks, a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.",
        uncallable: None,
        door: Some(door_num_fft_peak_ring),
    },
    Function {
        name: "num.fft.peak_wavelength",
        signature: "(profile: [f64], size: usize) -> f64",
        docs: "Reads the wavelength in cells at a radial profile's peak, size over the peak ring with a tie broken at the smaller ring; zero when the profile holds no ring past ring 0.",
        uncallable: None,
        door: Some(door_num_fft_peak_wavelength),
    },
    Function {
        name: "num.fft.radial_profile",
        signature: "(spectrum: [f64], size: usize) -> [f64]",
        docs: "Averages a centred size-square spectrum over rings of integer radius from the centre bin, a bin joining the ring its distance rounds to, rings 0 through size over two; ring k holds the frequencies near k cycles per field.\n\n# Errors\n\nErrs when the spectrum is not the size squared.",
        uncallable: None,
        door: Some(door_num_fft_radial_profile),
    },
    Function {
        name: "num.fft.transform",
        signature: "(field: [f64], size: usize) -> ([f64], [f64])",
        docs: "Transforms a real size-square field forward by fft2, returning the real and imaginary parts.\n\n# Errors\n\nErrs when the field is not the size squared, or when the size is no power of two.",
        uncallable: None,
        door: Some(door_num_fft_transform),
    },
    Function {
        name: "num.gauss.Class.prime",
        signature: "(self: num.gauss.Class) -> bool",
        docs: "Returns whether the class is prime.",
        uncallable: None,
        door: Some(door_num_gauss_class_prime),
    },
    Function {
        name: "num.gauss.Class.word",
        signature: "(self: num.gauss.Class) -> String",
        docs: "Returns the class as a word.",
        uncallable: None,
        door: Some(door_num_gauss_class_word),
    },
    Function {
        name: "num.gauss.Ring.associates",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> [(i64, i64)]",
        docs: "Returns the unit multiples of a point, the point first, turning anticlockwise.",
        uncallable: None,
        door: Some(door_num_gauss_ring_associates),
    },
    Function {
        name: "num.gauss.Ring.canon",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "Returns the canonical associate of a point: the one with `a > 0` and `b >= 0` on the square lattice, the one with `a > 0` and `0 <= b < a` on the hexagonal, the origin for the origin.",
        uncallable: None,
        door: Some(door_num_gauss_ring_canon),
    },
    Function {
        name: "num.gauss.Ring.conjugate",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "Returns the conjugate: the mirror image in the real axis.",
        uncallable: None,
        door: Some(door_num_gauss_ring_conjugate),
    },
    Function {
        name: "num.gauss.Ring.count",
        signature: "(self: num.gauss.Ring, radius: u64) -> usize",
        docs: "Returns the count of points within the reach: the square or the hexagon.",
        uncallable: None,
        door: Some(door_num_gauss_ring_count),
    },
    Function {
        name: "num.gauss.Ring.div_rem",
        signature: "(self: num.gauss.Ring, z: (i64, i64), w: (i64, i64)) -> ((i64, i64), (i64, i64))",
        docs: "Returns the quotient and the remainder of a point by a nonzero point: `z = q w + r` with the norm of `r` below the norm of `w`.",
        uncallable: None,
        door: Some(door_num_gauss_ring_div_rem),
    },
    Function {
        name: "num.gauss.Ring.fate",
        signature: "(self: num.gauss.Ring, n: u64) -> num.gauss.Class",
        docs: "Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise.",
        uncallable: None,
        door: Some(door_num_gauss_ring_fate),
    },
    Function {
        name: "num.gauss.Ring.gaussian_gcd",
        signature: "(self: num.gauss.Ring, z: (i64, i64), w: (i64, i64)) -> (i64, i64)",
        docs: "Returns the greatest common divisor of two points as its canonical associate, by the nearest-point Euclidean algorithm, the origin for two origins.",
        uncallable: None,
        door: Some(door_num_gauss_ring_gaussian_gcd),
    },
    Function {
        name: "num.gauss.Ring.inert",
        signature: "(self: num.gauss.Ring, p: u64) -> bool",
        docs: "Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3.",
        uncallable: None,
        door: Some(door_num_gauss_ring_inert),
    },
    Function {
        name: "num.gauss.Ring.mul",
        signature: "(self: num.gauss.Ring, arg1: (i64, i64), arg2: (i64, i64)) -> (i64, i64)",
        docs: "Returns the product of two points.",
        uncallable: None,
        door: Some(door_num_gauss_ring_mul),
    },
    Function {
        name: "num.gauss.Ring.named",
        signature: "(name: str) -> num.gauss.Ring?",
        docs: "Reads a ring from its name.",
        uncallable: None,
        door: Some(door_num_gauss_ring_named),
    },
    Function {
        name: "num.gauss.Ring.nearest",
        signature: "(self: num.gauss.Ring, x: f64, y: f64) -> (i64, i64)",
        docs: "Returns the point nearest a place in the plane.",
        uncallable: None,
        door: Some(door_num_gauss_ring_nearest),
    },
    Function {
        name: "num.gauss.Ring.norm",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64",
        docs: "Returns the norm of a point: its squared length.",
        uncallable: None,
        door: Some(door_num_gauss_ring_norm),
    },
    Function {
        name: "num.gauss.Ring.place",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (f64, f64)",
        docs: "Returns the place of a point in the plane, x right and y up, one unit between neighbours.",
        uncallable: None,
        door: Some(door_num_gauss_ring_place),
    },
    Function {
        name: "num.gauss.Ring.ramified",
        signature: "(self: num.gauss.Ring) -> u64",
        docs: "Returns the one rational prime that ramifies: 2 or 3.",
        uncallable: None,
        door: Some(door_num_gauss_ring_ramified),
    },
    Function {
        name: "num.gauss.Ring.reach",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64",
        docs: "Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance.",
        uncallable: None,
        door: Some(door_num_gauss_ring_reach),
    },
    Function {
        name: "num.gauss.Ring.symmetry",
        signature: "(self: num.gauss.Ring) -> usize",
        docs: "Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12.",
        uncallable: None,
        door: Some(door_num_gauss_ring_symmetry),
    },
    Function {
        name: "num.gauss.Ring.top",
        signature: "(self: num.gauss.Ring, radius: u64) -> u64",
        docs: "Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's.",
        uncallable: None,
        door: Some(door_num_gauss_ring_top),
    },
    Function {
        name: "num.gauss.Ring.turn",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> (i64, i64)",
        docs: "Returns the point turned anticlockwise by one unit: a quarter turn or a sixth.",
        uncallable: None,
        door: Some(door_num_gauss_ring_turn),
    },
    Function {
        name: "num.gauss.Ring.units",
        signature: "(self: num.gauss.Ring) -> usize",
        docs: "Returns the count of units: 4 or 6.",
        uncallable: None,
        door: Some(door_num_gauss_ring_units),
    },
    Function {
        name: "num.gauss.Ring.whole",
        signature: "(self: num.gauss.Ring, a: i64, b: i64) -> u64?",
        docs: "Returns the whole number an associate of the point lies on, when one lies on the positive real axis.",
        uncallable: None,
        door: Some(door_num_gauss_ring_whole),
    },
    Function {
        name: "num.gauss.Window.census",
        signature: "(self: num.gauss.Window) -> num.gauss.Census",
        docs: "Counts every class inside.",
        uncallable: None,
        door: Some(door_num_gauss_window_census),
    },
    Function {
        name: "num.gauss.Window.class",
        signature: "(self: num.gauss.Window, a: i64, b: i64) -> num.gauss.Class",
        docs: "Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime.",
        uncallable: None,
        door: Some(door_num_gauss_window_class),
    },
    Function {
        name: "num.gauss.Window.holds",
        signature: "(self: num.gauss.Window, a: i64, b: i64) -> bool",
        docs: "Returns whether a point lies inside.",
        uncallable: None,
        door: Some(door_num_gauss_window_holds),
    },
    Function {
        name: "num.gauss.Window.new",
        signature: "(ring: num.gauss.Ring, radius: u64) -> num.gauss.Window",
        docs: "Opens the window of a ring out to a reach, sieving every norm inside it.",
        uncallable: None,
        door: Some(door_num_gauss_window_new),
    },
    Function {
        name: "num.gauss.Window.points",
        signature: "(self: num.gauss.Window) -> [(i64, i64)]",
        docs: "Lists every point inside, row by row from the bottom left of the bounding square.",
        uncallable: None,
        door: Some(door_num_gauss_window_points),
    },
    Function {
        name: "num.gauss.Window.radius",
        signature: "(self: num.gauss.Window) -> u64",
        docs: "Returns the reach.",
        uncallable: None,
        door: Some(door_num_gauss_window_radius),
    },
    Function {
        name: "num.gauss.Window.ring",
        signature: "(self: num.gauss.Window) -> num.gauss.Ring",
        docs: "Returns the ring.",
        uncallable: None,
        door: Some(door_num_gauss_window_ring),
    },
    Function {
        name: "num.gauss.classes",
        signature: "(ring: num.gauss.Ring, bound: u64) -> [(i64, i64)]",
        docs: "Lists one point per associate class of the nonzero points of norm at most the bound: canonical associates, in order of norm and then of coordinates.",
        uncallable: None,
        door: Some(door_num_gauss_classes),
    },
    Function {
        name: "num.gauss.peak",
        signature: "(ring: num.gauss.Ring, limit: usize) -> (usize, u32)",
        docs: "Returns the norm from one through the limit with the most points and that count, the earliest on a tie.",
        uncallable: None,
        door: Some(door_num_gauss_peak),
    },
    Function {
        name: "num.gauss.shells",
        signature: "(ring: num.gauss.Ring, limit: usize) -> [u32]",
        docs: "Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice.",
        uncallable: None,
        door: Some(door_num_gauss_shells),
    },
    Function {
        name: "num.kummer.carry",
        signature: "(prime: u64, top: u64, bottom: u64) -> num.kummer.Carry",
        docs: "Builds the carry automaton of `K_(a,b)` at the prime, `a` the top and `b` the bottom.\n\nAt `(3, 1)` and `p = 7` state `0` keeps the digits `0..2` to itself and `4` to state `1`, and state `1` keeps `0, 1` to state `0` and `3, 4` to itself.\n\n# Errors\n\nErrs when `p` is not a prime, when `1 <= b < a <= MOST` fails, or when the table passes `TABLE` moves.",
        uncallable: None,
        door: Some(door_num_kummer_carry),
    },
    Function {
        name: "num.kummer.columns",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> [num.kummer.Column]",
        docs: "Returns the columns the automaton reads for `k`, units first: one per digit of `k`, then the zeros that spend the carries, ending at the first column that carries or once every digit is read and the state is `(0, 0)` again.\n\n# Errors\n\nErrs as `carry` does on the prime and the pair.",
        uncallable: None,
        door: Some(door_num_kummer_columns),
    },
    Function {
        name: "num.kummer.count",
        signature: "(prime: u64, top: u64, bottom: u64, level: u32) -> u128",
        docs: "Returns the count of `k < p^level` in `K_(a,b)`: the words of length `level` the automaton allows, summed by the state they end in over the states that close.\n\nAt every prime `p > a` every state keeps `(p + 1)/2` digits and every state closes, so the count is `((p + 1)/2)^level`; at `p <= a` it can fail, as at `(3, 1)` and `p = 3`.\n\n# Errors\n\nErrs as `carry` does, and when `p^level` passes a `u128`.",
        uncallable: None,
        door: Some(door_num_kummer_count),
    },
    Function {
        name: "num.kummer.member",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> bool",
        docs: "Returns whether `k` lies in `K_(a,b)`, read by the carry automaton: whether the prime does not divide `C(a k, b k)`.\n\n# Errors\n\nErrs as `carry` does on the prime and the pair.",
        uncallable: None,
        door: Some(door_num_kummer_member),
    },
    Function {
        name: "num.kummer.valuation",
        signature: "(k: u64, prime: u64, top: u64, bottom: u64) -> u32",
        docs: "Returns `v_p(C(a k, b k))` by Legendre's digit sums, `(s_p(b k) + s_p((a - b) k) - s_p(a k))/(p - 1)` with `s_p` the digit sum in base `p`: the carries of `b k + (a - b) k`.\n\n# Errors\n\nErrs as `carry` does on the prime and the pair.",
        uncallable: None,
        door: Some(door_num_kummer_valuation),
    },
    Function {
        name: "num.kummer.witness",
        signature: "(prime: u64, top: u64, bottom: u64) -> (u64, u64)?",
        docs: "Returns the least witness that `K_(a,b)` is no digit design, as the pair `(u, v p)` of elements: digits `u, v < p` with `u` and `v p` in the set and `u + v p` outside it, least `u + v p` first; a digit design holding `u` and `v p` holds `u + v p`. `None` when no two such digits exist, as on the half interval `K_(2,1)`.\n\n# Errors\n\nErrs as `carry` does.",
        uncallable: None,
        door: Some(door_num_kummer_witness),
    },
    Function {
        name: "num.lattice.coprime_pairs",
        signature: "(n: usize) -> u64",
        docs: "Counts the ordered pairs of coprime coordinates between one and n: twice the totient sum less one.",
        uncallable: None,
        door: Some(door_num_lattice_coprime_pairs),
    },
    Function {
        name: "num.lattice.farey",
        signature: "(order: usize) -> [num.lattice.Node]",
        docs: "Walks the Farey sequence of the order by the Stern-Brocot mediant recurrence from zero over one to one over one: every reduced fraction with denominator at most the order, ascending.",
        uncallable: None,
        door: Some(door_num_lattice_farey),
    },
    Function {
        name: "num.lattice.grid",
        signature: "(n: usize) -> [num.lattice.Node2d]",
        docs: "Lists the grid crossings of a window's nodes, row-major over the ascending axis nodes.",
        uncallable: None,
        door: Some(door_num_lattice_grid),
    },
    Function {
        name: "num.lattice.new_nodes",
        signature: "(n: usize) -> u64",
        docs: "Counts the nodes window n lights that window n minus one lacked: two at window one, phi of n after.",
        uncallable: None,
        door: Some(door_num_lattice_new_nodes),
    },
    Function {
        name: "num.lattice.pi_estimate",
        signature: "(n: usize) -> f64",
        docs: "Estimates pi from visibility: the density of coprime pairs in the n-by-n window tends to six over pi squared.",
        uncallable: None,
        door: Some(door_num_lattice_pi_estimate),
    },
    Function {
        name: "num.lattice.recovered",
        signature: "(n: usize, dimension: u32) -> f64",
        docs: "Recovers the constant the dimension hides from the visible count of the window, pi at an even dimension and zeta of the dimension at an odd one.\n\n# Errors\n\nErrs at a zero dimension.",
        uncallable: None,
        door: Some(door_num_lattice_recovered),
    },
    Function {
        name: "num.lattice.visible_density",
        signature: "(dimension: u32) -> f64",
        docs: "The density the visible count of a window in the dimension walks to, one over zeta of the dimension.\n\n# Errors\n\nErrs at a dimension of one or below.",
        uncallable: None,
        door: Some(door_num_lattice_visible_density),
    },
    Function {
        name: "num.lattice.zeta_factor",
        signature: "(dimension: u32) -> f64?",
        docs: "The rational factor r with zeta of the dimension equal to r times pi to the dimension, read off the Bernoulli fraction; none at an odd dimension or past twelve.",
        uncallable: None,
        door: Some(door_num_lattice_zeta_factor),
    },
    Function {
        name: "num.lattice.zeta_whole",
        signature: "(s: u32) -> f64",
        docs: "The value zeta takes at a whole argument above one, the exact Bernoulli form at an even one and the Euler-Maclaurin sum at an odd one.\n\n# Errors\n\nErrs at a whole argument of one or below, where the sum does not converge.",
        uncallable: None,
        door: Some(door_num_lattice_zeta_whole),
    },
    Function {
        name: "num.morse.Lift.all",
        signature: "() -> [num.morse.Lift; 4]",
        docs: "Returns every Lift in canonical order.",
        uncallable: None,
        door: Some(door_num_morse_lift_all),
    },
    Function {
        name: "num.morse.Lift.at",
        signature: "(self: num.morse.Lift, i: u64, j: u64) -> u8",
        docs: "Returns the sign at a site, zero for plus one and one for minus one.",
        uncallable: None,
        door: Some(door_num_morse_lift_at),
    },
    Function {
        name: "num.morse.Lift.formula",
        signature: "(self: num.morse.Lift) -> String",
        docs: "Returns the lift's formula, written the way the page prints it.",
        uncallable: None,
        door: Some(door_num_morse_lift_formula),
    },
    Function {
        name: "num.morse.boundary",
        signature: "(word: [u8]) -> [u8]",
        docs: "Returns the run-boundary word, one wherever a letter differs from the next.",
        uncallable: None,
        door: Some(door_num_morse_boundary),
    },
    Function {
        name: "num.morse.difference",
        signature: "(a: [u8], b: [u8]) -> [u8]",
        docs: "Exclusive-ors two grids of the same length, site by site.",
        uncallable: None,
        door: Some(door_num_morse_difference),
    },
    Function {
        name: "num.morse.digits",
        signature: "(length: usize) -> [u8]",
        docs: "Builds the first letters of the Thue-Morse word by the digit rule.",
        uncallable: None,
        door: Some(door_num_morse_digits),
    },
    Function {
        name: "num.morse.doubling",
        signature: "(length: usize) -> [u8]",
        docs: "Builds the period-doubling word by the substitution `1 -> 10`, `0 -> 11`, from the seed 1.",
        uncallable: None,
        door: Some(door_num_morse_doubling),
    },
    Function {
        name: "num.morse.faults",
        signature: "(a: [u8], b: [u8]) -> usize",
        docs: "Counts the sites where two grids of the same length differ.",
        uncallable: None,
        door: Some(door_num_morse_faults),
    },
    Function {
        name: "num.morse.fold",
        signature: "(grid: [u8], side: usize, number: usize) -> num.morse.Fold",
        docs: "Tests a grid against the Kronecker power of its own corner tile.\n\nThe corner tile is the only candidate worth testing. If the grid is `T` folded `L` times then\nits corner block is `T` with every bit flipped by `(L - 1) t00`, and folding that block `L`\ntimes flips the grid by `L (L - 1) t00`, which is even. So a grid folds if and only if it\nfolds from its corner block, and no search over tiles is needed.\n\n# Errors\n\nErrs when the grid is not the side squared, below a tile side of two, and when the side is no power of the tile side.",
        uncallable: None,
        door: Some(door_num_morse_fold),
    },
    Function {
        name: "num.morse.letter",
        signature: "(place: u64) -> u8",
        docs: "Returns the Thue-Morse letter at the place, the parity of its binary digit sum.",
        uncallable: None,
        door: Some(door_num_morse_letter),
    },
    Function {
        name: "num.morse.lift",
        signature: "(kind: num.morse.Lift, side: usize) -> [u8]",
        docs: "Builds a lift as a row-major sign grid of the side, zero for plus one and one for minus one.",
        uncallable: None,
        door: Some(door_num_morse_lift),
    },
    Function {
        name: "num.morse.mask",
        signature: "(a: [u8], b: [u8]) -> [u8]",
        docs: "Keeps the sites lit in both grids of the same length, site by site.",
        uncallable: None,
        door: Some(door_num_morse_mask),
    },
    Function {
        name: "num.morse.power",
        signature: "(tile: [u8], number: usize, level: usize) -> [u8]",
        docs: "Folds a tile of the side into its Kronecker power at the level, one bit per site.\n\nThe bits ride the exclusive or, so the power is the digit rule of the tile: a site takes the\nexclusive or of the tile's bits at its base-side digit pairs.\n\n# Errors\n\nErrs when the tile is not the side squared, or when the level passes what a machine holds.",
        uncallable: None,
        door: Some(door_num_morse_power),
    },
    Function {
        name: "num.morse.repeat",
        signature: "(tile: [u8], number: usize, side: usize) -> [u8]",
        docs: "Repeats a tile until it fills a grid of the side.",
        uncallable: None,
        door: Some(door_num_morse_repeat),
    },
    Function {
        name: "num.morse.runs",
        signature: "(word: [u8]) -> [usize]",
        docs: "Returns the lengths of the maximal blocks of one repeated letter, in order.",
        uncallable: None,
        door: Some(door_num_morse_runs),
    },
    Function {
        name: "num.morse.signs",
        signature: "(types: [u8]) -> [u8]",
        docs: "Reads a design plus-minus: a filled cell is plus, zero, and a void is minus, one.",
        uncallable: None,
        door: Some(door_num_morse_signs),
    },
    Function {
        name: "num.morse.stage",
        signature: "(rounds: usize) -> [u8]",
        docs: "Returns the substitution stage after the rounds, a word of length two to the rounds.",
        uncallable: None,
        door: Some(door_num_morse_stage),
    },
    Function {
        name: "num.morse.substitution",
        signature: "(length: usize) -> [u8]",
        docs: "Builds the first letters of the Thue-Morse word by the substitution `0 -> 01`, `1 -> 10`.\n\nThe seed is the single letter 0, and the rounds double the length until it covers the ask.",
        uncallable: None,
        door: Some(door_num_morse_substitution),
    },
    Function {
        name: "num.morse.upsample",
        signature: "(grid: [u8], side: usize, scale: usize) -> [u8]",
        docs: "Blows a grid up by the scale, every site becoming a scale-by-scale block.",
        uncallable: None,
        door: Some(door_num_morse_upsample),
    },
    Function {
        name: "num.prime.Sieve.count",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "Returns the count of numbers marked prime so far.",
        uncallable: None,
        door: Some(door_num_prime_sieve_count),
    },
    Function {
        name: "num.prime.Sieve.done",
        signature: "(self: num.prime.Sieve) -> bool",
        docs: "Returns whether every number is settled.",
        uncallable: None,
        door: Some(door_num_prime_sieve_done),
    },
    Function {
        name: "num.prime.Sieve.finish",
        signature: "(self: num.prime.Sieve) -> null",
        docs: "Runs the sieve to the end.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "num.prime.Sieve.new",
        signature: "(limit: usize) -> num.prime.Sieve",
        docs: "Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside.",
        uncallable: None,
        door: Some(door_num_prime_sieve_new),
    },
    Function {
        name: "num.prime.Sieve.rank",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "Returns the count of primes used so far.",
        uncallable: None,
        door: Some(door_num_prime_sieve_rank),
    },
    Function {
        name: "num.prime.Sieve.step",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "Uses the next prime: marks it prime, strikes its untouched multiples from its square with its rank plus one, and returns it; zero once done.\n\nThe strike mark saturates at 255, so it is exact through the 254th prime.",
        uncallable: Some("mutates its argument in place"),
        door: None,
    },
    Function {
        name: "num.prime.Sieve.struck",
        signature: "(self: num.prime.Sieve) -> usize",
        docs: "Returns the count of numbers the last step struck.",
        uncallable: None,
        door: Some(door_num_prime_sieve_struck),
    },
    Function {
        name: "num.prime.Sieve.types",
        signature: "(self: num.prime.Sieve) -> [u8]",
        docs: "Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it.",
        uncallable: None,
        door: Some(door_num_prime_sieve_types),
    },
    Function {
        name: "num.prime.chart",
        signature: "(top: usize, bins: usize) -> [num.prime.Reading]",
        docs: "Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last.",
        uncallable: None,
        door: Some(door_num_prime_chart),
    },
    Function {
        name: "num.prime.flags",
        signature: "(limit: usize) -> [bool]",
        docs: "Returns whether every number from zero through the limit is prime, the finished sieve read flag by flag.\n\nThe flag is the prime mark, never the strike rank, so it is exact at any limit.",
        uncallable: None,
        door: Some(door_num_prime_flags),
    },
    Function {
        name: "num.prime.goldbach",
        signature: "(number: usize) -> usize",
        docs: "Returns the count of unordered pairs of primes summing to the number, zero below four.",
        uncallable: None,
        door: Some(door_num_prime_goldbach),
    },
    Function {
        name: "num.prime.goldbach_record",
        signature: "(top: usize) -> [usize]",
        docs: "Returns the count of prime pairs at every even number from four up to the top, one entry per even number.",
        uncallable: None,
        door: Some(door_num_prime_goldbach_record),
    },
    Function {
        name: "num.prime.is_prime",
        signature: "(number: usize) -> bool",
        docs: "Returns whether the number is prime, by trial division on the six-step wheel.",
        uncallable: None,
        door: Some(door_num_prime_is_prime),
    },
    Function {
        name: "num.prime.pile",
        signature: "(number: u64) -> num.prime.Pile",
        docs: "Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization.",
        uncallable: None,
        door: Some(door_num_prime_pile),
    },
    Function {
        name: "num.prime.prime_count",
        signature: "(n: usize) -> usize",
        docs: "Returns the count of primes at or below n.",
        uncallable: None,
        door: Some(door_num_prime_prime_count),
    },
    Function {
        name: "num.prime.prime_from",
        signature: "(number: usize) -> usize",
        docs: "Returns the smallest prime at or above the number.",
        uncallable: None,
        door: Some(door_num_prime_prime_from),
    },
    Function {
        name: "num.prime.primes",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the primes up to the limit, the finished sieve read as a list.",
        uncallable: None,
        door: Some(door_num_prime_primes),
    },
    Function {
        name: "num.prime.rectangles",
        signature: "(number: usize) -> [(usize, usize)]",
        docs: "Returns every rectangle of the number as a pair of sides, the shorter first, ascending: the divisors at or below the root.",
        uncallable: None,
        door: Some(door_num_prime_rectangles),
    },
    Function {
        name: "num.prime.splits",
        signature: "(number: usize) -> [(usize, usize)]",
        docs: "Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending.",
        uncallable: None,
        door: Some(door_num_prime_splits),
    },
    Function {
        name: "num.prime.squares",
        signature: "(number: usize) -> (usize, usize)?",
        docs: "Returns the smallest pair of positive sides whose squares sum to the number, when one exists.",
        uncallable: None,
        door: Some(door_num_prime_squares),
    },
    Function {
        name: "num.prime.study",
        signature: "(limit: usize) -> [num.prime.Prime]",
        docs: "Returns one prime object for every prime up to and including the limit.",
        uncallable: None,
        door: Some(door_num_prime_study),
    },
    Function {
        name: "num.series.basel",
        signature: "(n: usize) -> f64",
        docs: "Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six.",
        uncallable: None,
        door: Some(door_num_series_basel),
    },
    Function {
        name: "num.series.bernoulli",
        signature: "(count: usize) -> [(i128, i128)]",
        docs: "Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention.\n\n# Errors\n\nErrs past a count of thirty-two, where the fractions overrun a signed hundred and twenty-eight bits.",
        uncallable: None,
        door: Some(door_num_series_bernoulli),
    },
    Function {
        name: "num.series.beta",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums.",
        uncallable: None,
        door: Some(door_num_series_beta),
    },
    Function {
        name: "num.series.binary",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the powers of two up to the limit.",
        uncallable: None,
        door: Some(door_num_series_binary),
    },
    Function {
        name: "num.series.catalan",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the distinct Catalan numbers up to the limit.",
        uncallable: None,
        door: Some(door_num_series_catalan),
    },
    Function {
        name: "num.series.chi3",
        signature: "(number: usize) -> i8",
        docs: "Returns the mod-three rhythm of the number: zero, one, minus one.",
        uncallable: None,
        door: Some(door_num_series_chi3),
    },
    Function {
        name: "num.series.chi4",
        signature: "(number: usize) -> i8",
        docs: "Returns the mod-four rhythm of the number: zero, one, zero, minus one.",
        uncallable: None,
        door: Some(door_num_series_chi4),
    },
    Function {
        name: "num.series.chi8",
        signature: "(number: usize) -> i8",
        docs: "Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.",
        uncallable: None,
        door: Some(door_num_series_chi8),
    },
    Function {
        name: "num.series.dirichlet",
        signature: "(s: f64, rhythm: [i8], terms: usize) -> f64",
        docs: "Returns the L-series partial sum with a periodic rhythm painted on the terms.",
        uncallable: None,
        door: Some(door_num_series_dirichlet),
    },
    Function {
        name: "num.series.e_partial",
        signature: "(n: usize) -> f64",
        docs: "Returns one plus one over n raised to the n, walking to the natural base.",
        uncallable: None,
        door: Some(door_num_series_e_partial),
    },
    Function {
        name: "num.series.euler_gamma_partial",
        signature: "(n: usize) -> f64",
        docs: "Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.",
        uncallable: None,
        door: Some(door_num_series_euler_gamma_partial),
    },
    Function {
        name: "num.series.euler_product",
        signature: "(s: f64, limit: usize) -> f64",
        docs: "Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit.",
        uncallable: None,
        door: Some(door_num_series_euler_product),
    },
    Function {
        name: "num.series.evens",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the even numbers up to the limit.",
        uncallable: None,
        door: Some(door_num_series_evens),
    },
    Function {
        name: "num.series.fibonacci",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the distinct Fibonacci numbers up to the limit.",
        uncallable: None,
        door: Some(door_num_series_fibonacci),
    },
    Function {
        name: "num.series.harmonic",
        signature: "(terms: usize) -> f64",
        docs: "Returns the partial harmonic sum, the reciprocals of one through the term count.",
        uncallable: None,
        door: Some(door_num_series_harmonic),
    },
    Function {
        name: "num.series.lambda",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "Returns the Dirichlet lambda value, one minus two to the minus s times zeta.\n\n# Errors\n\nErrs at an s of one or below, and at a NaN, where zeta does not converge.",
        uncallable: None,
        door: Some(door_num_series_lambda),
    },
    Function {
        name: "num.series.leibniz",
        signature: "(n: usize) -> f64",
        docs: "Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four.",
        uncallable: None,
        door: Some(door_num_series_leibniz),
    },
    Function {
        name: "num.series.li",
        signature: "(x: f64) -> f64",
        docs: "Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x.",
        uncallable: None,
        door: Some(door_num_series_li),
    },
    Function {
        name: "num.series.mertens",
        signature: "(n: usize) -> i64",
        docs: "Returns the Mertens function at n, the Mobius values of one through n summed.",
        uncallable: None,
        door: Some(door_num_series_mertens),
    },
    Function {
        name: "num.series.odds",
        signature: "(limit: usize) -> [usize]",
        docs: "Returns the odd numbers up to the limit.",
        uncallable: None,
        door: Some(door_num_series_odds),
    },
    Function {
        name: "num.series.visible",
        signature: "(limit: usize, dimension: u32) -> u128",
        docs: "Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion.\n\nThe count wraps once the limit to the dimension passes a signed hundred and twenty-eight bits.\n\n# Errors\n\nErrs at a zero dimension.",
        uncallable: None,
        door: Some(door_num_series_visible),
    },
    Function {
        name: "num.series.wallis_half_pi",
        signature: "(n: usize) -> f64",
        docs: "Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.",
        uncallable: None,
        door: Some(door_num_series_wallis_half_pi),
    },
    Function {
        name: "num.series.wallis_quarter_pi",
        signature: "(factors: usize) -> f64",
        docs: "Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four.",
        uncallable: None,
        door: Some(door_num_series_wallis_quarter_pi),
    },
    Function {
        name: "num.series.zeta",
        signature: "(s: f64, terms: usize) -> f64",
        docs: "Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail.\n\n# Errors\n\nErrs at an s of one or below, and at a NaN, where the sum does not converge.",
        uncallable: None,
        door: Some(door_num_series_zeta),
    },
    Function {
        name: "num.sieve.cells",
        signature: "(word: [u64], dimension: u32) -> u128",
        docs: "Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.\n\n# Errors\n\nErrs when a letter is not an odd side from three, or when the product overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_cells),
    },
    Function {
        name: "num.sieve.exponent",
        signature: "(word: [u64], dimension: u32) -> f64",
        docs: "Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.\n\nChanging the letter is not enough: the alternating word 3, 5, 3, 5 changes at every step and freezes at log 192 / log 15, its ratio falling to nothing like a fixed-ratio schedule. The hypothesis that buys a positive area is strictly increasing odd letters, under which sum s_k^(-d) converges.\n\n# Errors\n\nErrs when a letter is not an odd side from three.",
        uncallable: None,
        door: Some(door_num_sieve_exponent),
    },
    Function {
        name: "num.sieve.flat_word",
        signature: "(side: u64, levels: usize) -> [u64]",
        docs: "Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.",
        uncallable: None,
        door: Some(door_num_sieve_flat_word),
    },
    Function {
        name: "num.sieve.holes",
        signature: "(word: [u64], dimension: u32) -> u128",
        docs: "Returns the punctures the word makes, one per surviving cell at every level.\n\n# Errors\n\nErrs when a letter is not an odd side from three, or when the count overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_holes),
    },
    Function {
        name: "num.sieve.limit",
        signature: "(word: [u64], dimension: u32) -> f64?",
        docs: "Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.\n\nA run of consecutive odd letters from a, of two letters or more, names the schedule a, a + 2, a + 4 and on: its letters are strictly increasing, sum s_k^(-d) converges, and the limit is positive, pi over four in the plane and the closed form above in the cube divided by the head the run skips. Two or more copies of one letter name the fixed-ratio schedule, whose limit is zero; that is what repetition costs, and an alternating word such as 3, 5, 3, 5 changes at every step and still loses the whole measure. Every other word, and every word of one letter, names no schedule and reads None.\n\n# Errors\n\nErrs when a letter is not an odd side from three.",
        uncallable: None,
        door: Some(door_num_sieve_limit),
    },
    Function {
        name: "num.sieve.odd_word",
        signature: "(levels: usize) -> [u64]",
        docs: "Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.",
        uncallable: None,
        door: Some(door_num_sieve_odd_word),
    },
    Function {
        name: "num.sieve.parity_product",
        signature: "(dimension: u32, odd: bool) -> f64",
        docs: "Returns the Wallis sieve product of a parity design, `prod_(N odd >= 3) (1 - N^-dim)` for the corners with an odd count of odd coordinates and `prod (1 + N^-dim)` for the even count, at `dim >= 2`.\n\nAt odd `N` the sum of `(-1)^(x_1 + ... + x_dim)` over the box is 1, so the two designs fill `(N^dim -+ 1)/2` and the row word's constant is this product. The logarithm is summed over odd `N` from 3 to 999, smallest term first, and the tail past 1001 is the series `sum_j (-+1)^j / j sum N^(-dim j)`, each inner sum by Euler-Maclaurin through the fourth Bernoulli term, as for the solid limit.\n\n# Errors\n\nErrs below two dimensions, where the product runs off to 0 or to infinity.",
        uncallable: None,
        door: Some(door_num_sieve_parity_product),
    },
    Function {
        name: "num.sieve.punctures",
        signature: "(word: [u64], dimension: u32) -> [u64]",
        docs: "Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.\n\n# Errors\n\nErrs when a letter is not an odd side from three, or when the word's side overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_punctures),
    },
    Function {
        name: "num.sieve.raster",
        signature: "(word: [u64]) -> (usize, [u8])",
        docs: "Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.\n\n# Errors\n\nErrs when a letter is not an odd side from three.",
        uncallable: None,
        door: Some(door_num_sieve_raster),
    },
    Function {
        name: "num.sieve.ratio",
        signature: "(word: [u64], dimension: u32) -> f64",
        docs: "Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.\n\n# Errors\n\nErrs when a letter is not an odd side from three.",
        uncallable: None,
        door: Some(door_num_sieve_ratio),
    },
    Function {
        name: "num.sieve.row_fill",
        signature: "(profile: [u64], side: u64) -> u128",
        docs: "Returns the cells a design of this profile fills at one side, `sum_j a_j E^(dim-j) O^j` with `E` and `O` the even and the odd positions an axis holds: `P_F(n)` at side `2n - 1` and `w n^dim` at side `2n`.\n\n# Errors\n\nErrs on a profile no design in dim 1 to 3 holds, or when the fill overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_row_fill),
    },
    Function {
        name: "num.sieve.row_law",
        signature: "(profile: [u64]) -> num.sieve.Row",
        docs: "Reads the design of this profile along the row word: its drift, its first correction, the roots of its fill polynomial, its constant from the Gamma form, the constant spelled, and the constant again by reflection, against the mirror and as a Wallis sieve product wherever those apply.\n\nThe fill polynomial factors as `w n^(m_0) (n-1)^(m_1)` times a core with integer coefficients; its rational roots are found exactly and divided out, and what is left is a constant, an irreducible quadratic or, at dim 3, an irreducible cubic, whose roots are taken in floating point. Each `Gamma(2 - r)` is read against one `Gamma(3/2)` as a ratio, by the recurrence up past 16 and Stirling's series there, so the large terms cancel before they round.\n\n# Errors\n\nErrs on a profile no design in dim 1 to 3 holds.",
        uncallable: None,
        door: Some(door_num_sieve_row_law),
    },
    Function {
        name: "num.sieve.row_profile",
        signature: "(code: u128, dimension: usize) -> [u64]",
        docs: "Counts a base-2 design's corners by how many odd coordinates each holds, the profile the row word reads: the code is a bitmask over the corners, corner `i` is the binary digits of `i` as `math::bang::code_to_corners` reads it, so its odd coordinates are the ones of `i`.\n\n# Errors\n\nErrs outside dim 1 to 3, or on a code out of range for the dimension.",
        uncallable: None,
        door: Some(door_num_sieve_row_profile),
    },
    Function {
        name: "num.sieve.row_settle",
        signature: "(profile: [u64], stops: [usize], even: bool) -> [f64]",
        docs: "Walks the renormalised fill `R_L (2^dim/w)^L / L^drift` of the design of this profile to every stop, `R_L` the product of the letters' fill ratios, on the odd sides `3, 5, ..., 2L+1` or on the even sides `2, 4, ..., 2L`.\n\nEach letter adds `log(1 + (2^dim fill - w side^dim) / (w side^dim))`, the excess taken in exact integers; at an even side the excess is zero and the even word carries no drift, so its walk stands at 1.\n\n# Errors\n\nErrs on a profile no design in dim 1 to 3 holds, or on stops that are not rising whole numbers from 1.",
        uncallable: None,
        door: Some(door_num_sieve_row_settle),
    },
    Function {
        name: "num.sieve.side",
        signature: "(word: [u64]) -> u128",
        docs: "Returns the side of the word, the product of its letters' sides.\n\n# Errors\n\nErrs when a letter is not an odd side from three, or when the product overruns a u128.",
        uncallable: None,
        door: Some(door_num_sieve_side),
    },
    Function {
        name: "num.sieve.solid_limit",
        signature: "() -> f64",
        docs: "Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.\n\nWrite m = 2k + 1 and factor m^3 - 1 = (m - 1)(m - w)(m - w^2) at w = exp(2 pi i / 3), the three cube roots of one; dividing by m^3 = 8 (k + 1/2)^3 turns the k-th factor into k (k + (1 - w)/2) (k + (1 - w^2)/2) / (k + 1/2)^3, so the product is a ratio of three rising shifts over one repeated three times.\n\nThe three shifts a1 = 0, a2 = (1 - w)/2 and a3 = (1 - w^2)/2 sum to (2 - w - w^2)/2 = 3/2, exactly three times the shift b = 1/2 below, and prod_{k=1..N} (k + a) = Gamma(N + 1 + a) / Gamma(1 + a) with Gamma(N + 1 + a) / Gamma(N + 1 + b) ~ N^(a - b), so the N-dependent factor tends to N^(a1 + a2 + a3 - 3b) = 1 and the limit is Gamma(1 + b)^3 / (Gamma(1 + a1) Gamma(1 + a2) Gamma(1 + a3)).\n\nThat is Gamma(3/2)^3 / (Gamma(1) Gamma((3 - w)/2) Gamma((3 - w^2)/2)) = pi^(3/2) / (8 |Gamma(7/4 - i sqrt(3)/4)|^2), the two gamma values being conjugates; the same Weierstrass product gives the identity prod_{k >= 1} (1 - a^3/k^3) = 1 / (Gamma(1 - a) Gamma(1 - a w) Gamma(1 - a w^2)) the ratio form of this limit leans on, at a = 1/2 against prod_{n >= 2} (1 - n^-3) = cosh(pi sqrt(3)/2) / (3 pi).\n\nThe value is read from the logarithm rather than from that gamma ratio, which cancels four digits away: log P = sum over odd n from 3 to 999 of log(1 - n^-3), taken from its smallest term, minus sum_{j = 1..3} (1/j) sum_{n odd >= 1001} n^(-3j), each inner sum by Euler-Maclaurin through the fourth Bernoulli term. Every piece carries one sign, so nothing cancels and the exponential lands within a few ulps.",
        uncallable: None,
        door: Some(door_num_sieve_solid_limit),
    },
    Function {
        name: "num.spiral.Growth.all",
        signature: "() -> [num.spiral.Growth; 2]",
        docs: "Returns every Growth in canonical order.",
        uncallable: None,
        door: Some(door_num_spiral_growth_all),
    },
    Function {
        name: "num.spiral.Lattice.all",
        signature: "() -> [num.spiral.Lattice; 2]",
        docs: "Returns every Lattice in canonical order.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_all),
    },
    Function {
        name: "num.spiral.Lattice.count",
        signature: "(self: num.spiral.Lattice, side: usize) -> usize",
        docs: "Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_count),
    },
    Function {
        name: "num.spiral.Lattice.n",
        signature: "(self: num.spiral.Lattice, x: i64, y: i64) -> u64",
        docs: "Returns the number at a cell, one at the origin.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_n),
    },
    Function {
        name: "num.spiral.Lattice.radius",
        signature: "(self: num.spiral.Lattice, side: usize) -> usize",
        docs: "Returns the outermost ring of a sheet the odd side wide, half the side rounded down.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_radius),
    },
    Function {
        name: "num.spiral.Lattice.ring",
        signature: "(self: num.spiral.Lattice, n: u64) -> u64",
        docs: "Returns the ring a number sits on, zero for one.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_ring),
    },
    Function {
        name: "num.spiral.Lattice.ring_of",
        signature: "(self: num.spiral.Lattice, x: i64, y: i64) -> u64",
        docs: "Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_ring_of),
    },
    Function {
        name: "num.spiral.Lattice.xy",
        signature: "(self: num.spiral.Lattice, n: u64) -> (i64, i64)",
        docs: "Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon.",
        uncallable: None,
        door: Some(door_num_spiral_lattice_xy),
    },
    Function {
        name: "num.spiral.Mark.all",
        signature: "() -> [num.spiral.Mark; 4]",
        docs: "Returns every Mark in canonical order.",
        uncallable: None,
        door: Some(door_num_spiral_mark_all),
    },
    Function {
        name: "num.spiral.diagonal",
        signature: "(lattice: num.spiral.Lattice, side: usize, a: i64, b: i64, c: i64) -> num.spiral.Diagonal",
        docs: "Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak.",
        uncallable: None,
        door: Some(door_num_spiral_diagonal),
    },
    Function {
        name: "num.spiral.level_of",
        signature: "(n: u64, base: u64) -> u32",
        docs: "Returns the level of a number in a base, the count of its digits less one, so zero below the base and one at the base itself.",
        uncallable: None,
        door: Some(door_num_spiral_level_of),
    },
    Function {
        name: "num.spiral.marks",
        signature: "(mark: num.spiral.Mark, limit: usize) -> [i8]",
        docs: "Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero.",
        uncallable: None,
        door: Some(door_num_spiral_marks),
    },
    Function {
        name: "num.spiral.snail",
        signature: "(base: u64, top: u64, growth: num.spiral.Growth) -> num.spiral.Snail",
        docs: "Winds one to the top on the square spiral and lays a square tile on every cell, the snail.\n\nThe tile of n has side base to the level of n when n grows and side one when it does not, and its lower-left corner is the corner of the tile before it plus one unit step of the winding scaled by the side of that earlier tile. Tiles overlap wherever the growth outruns the winding; a base below two is read as two and a top below one as one.",
        uncallable: None,
        door: Some(door_num_spiral_snail),
    },
    Function {
        name: "num.zeta.Complex.abs",
        signature: "(z: num.zeta.Complex) -> f64",
        docs: "Returns the modulus.",
        uncallable: None,
        door: Some(door_num_zeta_complex_abs),
    },
    Function {
        name: "num.zeta.Complex.arg",
        signature: "(z: num.zeta.Complex) -> f64",
        docs: "Returns the principal argument.",
        uncallable: None,
        door: Some(door_num_zeta_complex_arg),
    },
    Function {
        name: "num.zeta.Complex.default",
        signature: "() -> num.zeta.Complex",
        docs: "Returns the default Complex.",
        uncallable: None,
        door: Some(door_num_zeta_complex_default),
    },
    Function {
        name: "num.zeta.Complex.exp",
        signature: "(z: num.zeta.Complex) -> num.zeta.Complex",
        docs: "Returns the exponential.",
        uncallable: None,
        door: Some(door_num_zeta_complex_exp),
    },
    Function {
        name: "num.zeta.Complex.ln",
        signature: "(z: num.zeta.Complex) -> num.zeta.Complex",
        docs: "Returns the principal logarithm.",
        uncallable: None,
        door: Some(door_num_zeta_complex_ln),
    },
    Function {
        name: "num.zeta.Complex.new",
        signature: "(re: f64, im: f64) -> num.zeta.Complex",
        docs: "Builds a complex number from its parts.",
        uncallable: None,
        door: Some(door_num_zeta_complex_new),
    },
    Function {
        name: "num.zeta.Complex.turn",
        signature: "(angle: f64) -> num.zeta.Complex",
        docs: "Returns a unit complex number at the given angle.",
        uncallable: None,
        door: Some(door_num_zeta_complex_turn),
    },
    Function {
        name: "num.zeta.Line.count",
        signature: "(self: num.zeta.Line, t: f64) -> usize",
        docs: "Counts the zeros on the line below t: the sign changes of Z on ten steps a Gram interval, the Riemann-Siegel main sum turned from step to step and its remainder read only where it could flip the sign.",
        uncallable: None,
        door: Some(door_num_zeta_line_count),
    },
    Function {
        name: "num.zeta.Line.default",
        signature: "() -> num.zeta.Line",
        docs: "Returns the default Line.",
        uncallable: None,
        door: Some(door_num_zeta_line_default),
    },
    Function {
        name: "num.zeta.Line.exact",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "Returns Z(t) from the Euler-Maclaurin value turned onto the real axis.",
        uncallable: None,
        door: Some(door_num_zeta_line_exact),
    },
    Function {
        name: "num.zeta.Line.gram",
        signature: "(self: num.zeta.Line, n: i64) -> f64",
        docs: "Returns the n-th Gram point, where theta is n pi, by Newton from the right.",
        uncallable: None,
        door: Some(door_num_zeta_line_gram),
    },
    Function {
        name: "num.zeta.Line.maclaurin",
        signature: "(self: num.zeta.Line, t: f64) -> num.zeta.Complex",
        docs: "Returns zeta at one half plus i t by the complex Euler-Maclaurin sum: t plus ten terms and seven Bernoulli corrections.",
        uncallable: None,
        door: Some(door_num_zeta_line_maclaurin),
    },
    Function {
        name: "num.zeta.Line.new",
        signature: "() -> num.zeta.Line",
        docs: "Builds the line: the even Bernoulli numbers through the fourteenth and their Euler-Maclaurin weights.",
        uncallable: None,
        door: Some(door_num_zeta_line_new),
    },
    Function {
        name: "num.zeta.Line.novelty_coefficients",
        signature: "(self: num.zeta.Line, gammas: [f64]) -> [num.zeta.Complex]",
        docs: "Returns the wave coefficient of every zero at the given ordinates: F(rho) zeta(rho - 1) over zeta'(rho) at rho one half plus i gamma, F the Mellin transform of the bump.",
        uncallable: None,
        door: Some(door_num_zeta_line_novelty_coefficients),
    },
    Function {
        name: "num.zeta.Line.pair",
        signature: "(self: num.zeta.Line, s: num.zeta.Complex) -> (num.zeta.Complex, num.zeta.Complex)",
        docs: "Returns zeta and its derivative together at any complex s but one, by the same Euler-Maclaurin sum: the modulus of t plus ten terms and seven Bernoulli corrections, each term differentiated in s.",
        uncallable: None,
        door: Some(door_num_zeta_line_pair),
    },
    Function {
        name: "num.zeta.Line.point",
        signature: "(self: num.zeta.Line, t: f64) -> (num.zeta.Complex, f64)",
        docs: "Returns zeta on the line and Z(t) together, from the engine that serves the t.",
        uncallable: None,
        door: Some(door_num_zeta_line_point),
    },
    Function {
        name: "num.zeta.Line.seam",
        signature: "(self: num.zeta.Line, t0: f64, t1: f64, steps: usize) -> f64",
        docs: "Returns the largest gap between the two engines over the t range on a grid.",
        uncallable: None,
        door: Some(door_num_zeta_line_seam),
    },
    Function {
        name: "num.zeta.Line.siegel",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "Returns Z(t) by the Riemann-Siegel formula: the main sum and the first four corrections.",
        uncallable: None,
        door: Some(door_num_zeta_line_siegel),
    },
    Function {
        name: "num.zeta.Line.theta",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "Returns the Riemann-Siegel theta: the argument of gamma at one quarter plus i t over two, less t ln pi over two, by Stirling's series after a shift of ten.",
        uncallable: None,
        door: Some(door_num_zeta_line_theta),
    },
    Function {
        name: "num.zeta.Line.z",
        signature: "(self: num.zeta.Line, t: f64) -> f64",
        docs: "Returns Z(t): Euler-Maclaurin below the join, Riemann-Siegel above.",
        uncallable: None,
        door: Some(door_num_zeta_line_z),
    },
    Function {
        name: "num.zeta.Line.zeros",
        signature: "(self: num.zeta.Line, count: usize) -> [f64]",
        docs: "Returns the first zeros on the line: sign changes of Z between Gram points, refined by bisection on Euler-Maclaurin to a billionth.",
        uncallable: None,
        door: Some(door_num_zeta_line_zeros),
    },
    Function {
        name: "num.zeta.bump",
        signature: "(u: f64) -> f64",
        docs: "The smooth window on [1, 2]: exp(4 - 1/((u - 1)(2 - u))) inside, zero outside, every derivative vanishing at the ends and a peak of one at u = 3/2.",
        uncallable: None,
        door: Some(door_num_zeta_bump),
    },
    Function {
        name: "num.zeta.corrections",
        signature: "(p: f64) -> [f64; 4]",
        docs: "Returns the first four Riemann-Siegel corrections at the fractional part p: the kernel and its derivatives by central differences with one Richardson step.",
        uncallable: None,
        door: Some(door_num_zeta_corrections),
    },
    Function {
        name: "num.zeta.kernel",
        signature: "(p: f64) -> f64",
        docs: "Returns the Riemann-Siegel kernel, the cosine ratio that leads the remainder, in the form that stays finite at its removable points.",
        uncallable: None,
        door: Some(door_num_zeta_kernel),
    },
    Function {
        name: "num.zeta.mellin",
        signature: "(s: num.zeta.Complex) -> num.zeta.Complex",
        docs: "Returns the Mellin transform of the bump at a complex s, the integral of bump(u) u^(s - 1) over [1, 2], by a 4096-node midpoint rule.",
        uncallable: None,
        door: Some(door_num_zeta_mellin),
    },
    Function {
        name: "num.zeta.novelty_main",
        signature: "() -> f64",
        docs: "Returns the main term of the smoothed novelty: six over pi squared times the bump's transform at two.",
        uncallable: None,
        door: Some(door_num_zeta_novelty_main),
    },
    Function {
        name: "num.zeta.novelty_wave",
        signature: "(gammas: [f64], coef: [num.zeta.Complex], log_y: f64) -> f64",
        docs: "Sums the waves of the zeros at log y: twice the real part of the coefficients times y to the minus i gamma, the smoothed error over y to the three halves that the zeros predict.",
        uncallable: None,
        door: Some(door_num_zeta_novelty_wave),
    },
    Function {
        name: "num.zeta.psi_curve",
        signature: "(gammas: [f64], samples: [f64]) -> [f64]",
        docs: "Returns the explicit formula over the zeros at the given ordinates at every sample x.",
        uncallable: None,
        door: Some(door_num_zeta_psi_curve),
    },
    Function {
        name: "num.zeta.psi_formula",
        signature: "(x: f64, gammas: [f64]) -> f64",
        docs: "Returns the von Mangoldt explicit formula at x over the zeros at the given ordinates and their mirrors: x less the sum of x to the rho over rho, less ln two pi, less half the ln of one minus x to the minus two.",
        uncallable: None,
        door: Some(door_num_zeta_psi_formula),
    },
    Function {
        name: "num.zeta.psi_gap",
        signature: "(x: usize, gammas: [f64]) -> f64",
        docs: "Returns how far the explicit formula over the zeros at the given ordinates lands from the staircase at the whole number x.",
        uncallable: None,
        door: Some(door_num_zeta_psi_gap),
    },
    Function {
        name: "num.zeta.psi_stair",
        signature: "(x: usize) -> [f64]",
        docs: "Returns the Chebyshev staircase at every whole number from one to x: the sum of ln p over the prime powers up to each.",
        uncallable: None,
        door: Some(door_num_zeta_psi_stair),
    },
    Function {
        name: "num.zeta.raise",
        signature: "(base: f64, exponent: num.zeta.Complex) -> num.zeta.Complex",
        docs: "Returns a positive real base raised to a complex exponent.",
        uncallable: None,
        door: Some(door_num_zeta_raise),
    },
    Function {
        name: "num.zeta.sharp_novelty",
        signature: "(prefix: [u64], y: f64) -> f64",
        docs: "Returns the sharp novelty error at y: y squared times the totient sum over the scales from 1 over y to 2 over y, both ends in, less nine over pi squared, from the prefix sums of the totients, which must reach 2 over y.",
        uncallable: None,
        door: Some(door_num_zeta_sharp_novelty),
    },
    Function {
        name: "num.zeta.smoothed_novelty",
        signature: "(phi: [u64], y: f64, main: f64) -> f64",
        docs: "Returns the smoothed novelty error at y: y squared times the totients weighed by the bump at n y, less the main term given; the totients must reach 2 over y.",
        uncallable: None,
        door: Some(door_num_zeta_smoothed_novelty),
    },
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

fn door_core_colorizer_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::Colorizer::default()))
}

fn door_core_colorizer_diverge(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::Colorizer::diverge()))
}

fn door_core_colorizer_fire(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::Colorizer::fire()))
}

fn door_core_colorizer_gradient_bins(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Color = color(name, 0, &args[0])?;
    let a1: Vec<mrly::core::Color> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(color(name, 1, item0)?);
        }
        list0
    };
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::core::Colorizer::gradient_bins(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colorizer_heat(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::Colorizer::heat()))
}

fn door_core_dtype_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Dtype = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Dtype::max(a0)))
}

fn door_core_cell_anti(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Cell::anti(a0)))
}

fn door_core_cell_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Cell::binarize(a0, a1)))
}

fn door_core_cell_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Cell::binarize_otsu(a0)))
}

fn door_core_cell_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly::core::Cell::blur(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_color_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Cell::color_at(&a0, a1)))
}

fn door_core_cell_combine(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: mrly::core::Cell = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Cell::combine(&a0, &a1)))
}

fn door_core_cell_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::core::Cell::fractal(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Cell::invert(a0)))
}

fn door_core_cell_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: mrly::core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Cell::layers(a0, a1)))
}

fn door_core_cell_magic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::core::Cell> = take!(name, 0, &args[0]);
    match mrly::core::cell::magic(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_mapping(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok({
        let mut pairs0 = serde_json::Map::new();
        for (key0, item0) in mrly::core::cell::mapping() {
            pairs0.insert(key0.to_string(), {
                let mut list1 = Vec::new();
                for item1 in item0 {
                    list1.push(tint(item1));
                }
                Value::Array(list1)
            });
        }
        Value::Object(pairs0)
    })
}

fn door_core_cell_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly::core::Cell> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::cell::merge(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_moore(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::cell::moore(a0)))
}

fn door_core_cell_mosaic(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<mrly::core::Cell> = take!(name, 1, &args[1]);
    match mrly::core::cell::mosaic(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_neighbors(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: mrly::core::Dtype = take!(name, 4, &args[4]);
    match mrly::core::Cell::neighbors(a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Cell::new(a0)))
}

fn door_core_cell_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(give!(mrly::core::Cell::pad(a0, a1, a2)))
}

fn door_core_cell_paint(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1 = {
        let mut pairs0: Vec<(u8, Vec<mrly::core::Color>)> = Vec::new();
        for (text0, item0) in fields(name, 1, &args[1])? {
            let key0 = match text0.parse() {
                Ok(key) => key,
                Err(_) => return Err(bad(name, 1, "an object key")),
            };
            pairs0.push((key0, {
                let mut list1 = Vec::new();
                for item1 in items(name, 1, item0)? {
                    list1.push(color(name, 1, item1)?);
                }
                list1
            }));
        }
        pairs0
    }
    .into_iter()
    .collect();
    let a2: mrly::core::Mode = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() {
        None
    } else {
        Some(mrly::core::Rng::new(seed(name, 3, &args[3])?))
    };
    match mrly::core::Cell::paint(a0, &a1, a2, a3.as_mut()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly::core::Cell::perforate(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_remap(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    match mrly::core::cell::remap(&a0, &a1, &a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_rgba(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Cell::rgba(&a0)))
}

fn door_core_cell_rot90_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly::core::cell::rot90_map(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_rotate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly::core::Cell::rotate(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_shape(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Cell::shape(&a0)))
}

fn door_core_cell_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Cell::size(&a0)))
}

fn door_core_cell_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Cell = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::Cell::tile(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_tile_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::cell::tile_map(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_theme_hues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::colors::Theme = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::core::colors::Theme::hues(&a0) {
            list0.push(tint(item0));
        }
        Value::Array(list0)
    })
}

fn door_core_colors_theme_inks(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::colors::Theme = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::core::colors::Theme::inks(&a0) {
            list0.push(tint(item0));
        }
        Value::Array(list0)
    })
}

fn door_core_colors_alpha(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Color = color(name, 0, &args[0])?;
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(tint(mrly::core::Color::alpha(&a0, a1)))
}

fn door_core_colors_board(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: bool = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::colors::board(a0)))
}

fn door_core_colors_css(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Color = color(name, 0, &args[0])?;
    Ok(give!(mrly::core::Color::css(&a0)))
}

fn door_core_colors_from_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::core::Color::from_hex(a0.as_str()) {
        Ok(value) => Ok(tint(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_gradient(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly::core::Color> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(color(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::core::colors::gradient(&a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(tint(item0));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_ink(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: bool = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::colors::ink(a0)))
}

fn door_core_colors_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Color = color(name, 0, &args[0])?;
    Ok(tint(mrly::core::Color::invert(&a0)))
}

fn door_core_colors_lightness(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Color = color(name, 0, &args[0])?;
    let a1: u8 = take!(name, 1, &args[1]);
    match mrly::core::Color::lightness(&a0, a1) {
        Ok(value) => Ok(tint(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_luma_types(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(give!(mrly::core::colors::luma_types(&a0, a1, a2, a3)))
}

fn door_core_colors_mix(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Color = color(name, 0, &args[0])?;
    let a1: mrly::core::Color = color(name, 1, &args[1])?;
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly::core::colors::mix(a0, a1, a2) {
        Ok(value) => Ok(tint(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::core::colors::named(a0.as_str()) {
        Ok(value) => Ok(tint(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_random(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: bool = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(tint(mrly::core::Color::random(a0, &mut a1)))
}

fn door_core_colors_rgb(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(tint(mrly::core::Color::rgb(a0, a1, a2)))
}

fn door_core_colors_rgba(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(tint(mrly::core::Color::rgba(a0, a1, a2, a3)))
}

fn door_core_colors_shades(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Color = color(name, 0, &args[0])?;
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::core::colors::shades(a0) {
            list0.push(tint(item0));
        }
        Value::Array(list0)
    })
}

fn door_core_colors_snap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: Vec<mrly::core::Color> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(color(name, 1, item0)?);
        }
        list0
    };
    Ok(give!(mrly::core::colors::snap(&a0, &a1)))
}

fn door_core_colors_to_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Color = color(name, 0, &args[0])?;
    Ok(give!(mrly::core::Color::to_hex(&a0)))
}

fn door_core_error_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::core::error::parse(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_hex_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: mrly::core::Filter = take!(name, 4, &args[4]);
    match mrly::core::hex_fit(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_hex_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly::core::hex_size(a0, a1, a2)))
}

fn door_core_image_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly::core::image::blur(&a0, a1, a2, a3)))
}

fn door_core_image_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<[u8; 4]> = take!(name, 2, &args[2]);
    match mrly::core::Image::new(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_image_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::core::Image = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrly::core::Filter = take!(name, 3, &args[3]);
    match mrly::core::Image::resample(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_image_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Image = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::core::Image::scale(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_name_sha_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::name::sha::hex(&a0)))
}

fn door_core_name_sha_sha256(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::name::sha::sha256(&a0)))
}

fn door_core_name_sha_short(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::name::sha::short(&a0)))
}

fn door_core_paint_config_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::paint::Config::default()))
}

fn door_core_paint_edition_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::paint::Edition::all()))
}

fn door_core_paint_edition_mode(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::paint::Edition = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::paint::Edition::mode(a0)))
}

fn door_core_paint_ink_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::paint::Ink::all()))
}

fn door_core_paint_ink_color(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::paint::Ink = take!(name, 0, &args[0]);
    Ok(tint(mrly::core::paint::Ink::color(a0)))
}

fn door_core_paint_paint_is_simple(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::paint::Paint = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::paint::Paint::is_simple(&a0)))
}

fn door_core_paint_paint_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::paint::Edition = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::paint::Paint::new(a0)))
}

fn door_core_paint_scheme_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::paint::Scheme::all()))
}

fn door_core_paint_target_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::core::paint::Target::all()))
}

fn door_core_paint_random_edition(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Option<Vec<mrly::core::paint::Edition>> = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrly::core::paint::random_edition(
        a0.as_deref(),
        &mut a1
    )))
}

fn door_core_paint_reroll(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::paint::Paint = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrly::core::paint::reroll(a0, &mut a1)))
}

fn door_core_paint_setup(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::paint::Paint = take!(name, 0, &args[0]);
    let a1: mrly::core::paint::Config = take!(name, 1, &args[1]);
    let mut a2 = mrly::core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrly::core::paint::setup(a0, &a1, &mut a2)))
}

fn door_core_ramp_color(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Colorizer = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(tint(mrly::core::ramp::color(&a0, a1, a2)))
}

fn door_core_ramp_colors(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Colorizer = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::core::ramp::colors(&a0, &a1, a2)))
}

fn door_core_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: mrly::core::Filter = take!(name, 5, &args[5]);
    match mrly::core::resample(&a0, a1, a2, a3, a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_rng_below(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let mut a0 = mrly::core::Rng::new(seed(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Rng::below(&mut a0, a1)))
}

fn door_core_rng_boolean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrly::core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrly::core::Rng::boolean(&mut a0)))
}

fn door_core_rng_chance(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let mut a0 = mrly::core::Rng::new(seed(name, 0, &args[0])?);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Rng::chance(&mut a0, a1)))
}

fn door_core_rng_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Rng::new(a0)))
}

fn door_core_rng_range(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let mut a0 = mrly::core::Rng::new(seed(name, 0, &args[0])?);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::core::Rng::range(&mut a0, a1, a2)))
}

fn door_core_rng_sample_indices(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let mut a0 = mrly::core::Rng::new(seed(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::core::Rng::sample_indices(&mut a0, a1, a2)))
}

fn door_core_rng_unit(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrly::core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrly::core::Rng::unit(&mut a0)))
}

fn door_core_tensor_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::at(&a0, a1)))
}

fn door_core_tensor_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::binarize(&a0, a1)))
}

fn door_core_tensor_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Tensor::binarize_otsu(&a0)))
}

fn door_core_tensor_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly::core::Tensor::blur(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_bytes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    match mrly::core::Tensor::bytes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::count(&a0, a1)))
}

fn door_core_tensor_dtype(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Tensor::dtype(&a0)))
}

fn door_core_tensor_exposed(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(Value::String(mrly::core::Tensor::exposed(&a0).to_string()))
}

fn door_core_tensor_filled(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: mrly::core::Dtype = take!(name, 2, &args[2]);
    Ok(give!(mrly::core::Tensor::filled(a0, a1, a2)))
}

fn door_core_tensor_flip(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::core::Tensor::flip(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::fractal(&a0, a1)))
}

fn door_core_tensor_full(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::full(a0, a1)))
}

fn door_core_tensor_get(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::Tensor::get(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_i32(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i32> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::Tensor::i32(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_i32s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    match mrly::core::Tensor::i32s(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::index(&a0, &a1)))
}

fn door_core_tensor_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Tensor::invert(&a0)))
}

fn door_core_tensor_kron(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::kron(&a0, &a1)))
}

fn door_core_tensor_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::layers(&a0, a1)))
}

fn door_core_tensor_neighbors(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: mrly::core::Dtype = take!(name, 4, &args[4]);
    match mrly::core::Tensor::neighbors(&a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Tensor::new(a0)))
}

fn door_core_tensor_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::Tensor::of(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_otsu_threshold(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Tensor::otsu_threshold(&a0)))
}

fn door_core_tensor_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(give!(mrly::core::Tensor::pad(&a0, a1, a2)))
}

fn door_core_tensor_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly::core::Tensor::perforate(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_rot90(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly::core::Tensor::rot90(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Tensor::size(&a0)))
}

fn door_core_tensor_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::core::Tensor::slice(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_sum(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::core::Tensor::sum(&a0)))
}

fn door_core_tensor_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::Tensor::tile(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_transpose(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::core::Tensor::transpose(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_typed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: mrly::core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrly::core::Tensor::typed(a0, a1)))
}

fn door_core_tensor_u16(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u16> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::Tensor::u16(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_u16s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    match mrly::core::Tensor::u16s(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_u32(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u32> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::Tensor::u32(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_u32s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    match mrly::core::Tensor::u32s(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_u8(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::core::Tensor::u8(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_font_glyph_height(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::font::Glyph = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::Glyph::height(&a0)))
}

fn door_font_glyph_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: char = take!(name, 0, &args[0]);
    let a1: Vec<String> = take!(name, 1, &args[1]);
    Ok(give!(mrly::font::Glyph::new(a0, a1)))
}

fn door_font_glyph_width(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::font::Glyph = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::Glyph::width(&a0)))
}

fn door_font_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::all()))
}

fn door_font_animate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::font::animate(a0.as_str(), a1)))
}

fn door_font_cycle(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::font::Anim = take!(name, 0, &args[0]);
    let a1: Vec<Vec<usize>> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::font::cycle(&a0, &a1, a2)))
}

fn door_font_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::digits()))
}

fn door_font_draft(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::draft(&a0)))
}

fn door_font_extras(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::extras()))
}

fn door_font_floor(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::floor(&a0)))
}

fn door_font_glyph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::glyph(a0)))
}

fn door_font_lower(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    let b0: Vec<&str> = a0.iter().map(|item| item.as_str()).collect();
    Ok(give!(mrly::font::lower(&b0)))
}

fn door_font_lowers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::lowers()))
}

fn door_font_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::map()))
}

fn door_font_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::font::merge(a0.as_str(), a1)))
}

fn door_font_name_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::name_of(a0)))
}

fn door_font_path(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::path(a0)))
}

fn door_font_paths_penned(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::paths::penned(a0)))
}

fn door_font_pens_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::pens::all()))
}

fn door_font_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::raster(a0.as_str())))
}

fn door_font_specials(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::specials()))
}

fn door_font_strokes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: char = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::strokes(a0)))
}

fn door_font_supported(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::supported()))
}

fn door_font_trim(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    Ok(give!(mrly::font::trim(&a0)))
}

fn door_font_uppers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::font::uppers()))
}

fn door_life_boundary_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::life::Boundary::all()))
}

fn door_life_boundary_wrap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Boundary = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Boundary::wrap(a0)))
}

fn door_life_config_budget(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Config = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Config::budget(&a0)))
}

fn door_life_config_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Config = take!(name, 0, &args[0]);
    match mrly::life::Config::counts(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_config_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly::life::Counts = take!(name, 1, &args[1]);
    let a2: mrly::life::Counts = take!(name, 2, &args[2]);
    Ok(give!(mrly::life::Config::new(a0, a1, a2)))
}

fn door_life_counts_drawn(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::life::Source = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly::life::Counts::drawn(a0, a1, a2)))
}

fn door_life_counts_list(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Counts::list(a0)))
}

fn door_life_counts_values(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::life::Counts = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::life::Counts::values(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_fate_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::life::Fate::all()))
}

fn door_life_life_last(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Life = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Life::last(&a0)))
}

fn door_life_rule_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Rule::boundary(&a0)))
}

fn door_life_rule_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Rule = take!(name, 0, &args[0]);
    match <mrly::life::Rule as mrly::core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_config(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::life::Rule = take!(name, 0, &args[0]);
    let a1: mrly::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    Ok(give!(mrly::life::Rule::config(&a0, a1)))
}

fn door_life_rule_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::life::Rule as mrly::core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::life::Rule as mrly::core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::life::Rule as mrly::core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::life::Counts = take!(name, 0, &args[0]);
    let a1: mrly::life::Counts = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly::life::Rule::new(a0, a1, a2)))
}

fn door_life_rule_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Config = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Rule::of(&a0)))
}

fn door_life_rule_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Rule = take!(name, 0, &args[0]);
    match <mrly::life::Rule as mrly::core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Rule = take!(name, 0, &args[0]);
    Ok(give!(<mrly::life::Rule as mrly::core::name::Named>::to_id(
        &a0
    )))
}

fn door_life_rule_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Rule = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::life::Rule as mrly::core::name::Named>::to_json(&a0)
    ))
}

fn door_life_rule_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Rule = take!(name, 0, &args[0]);
    match <mrly::life::Rule as mrly::core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_rule_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Rule = take!(name, 0, &args[0]);
    match <mrly::life::Rule as mrly::core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_source_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::life::Source::all()))
}

fn door_life_source_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::life::Source::designs()))
}

fn door_life_source_is_random(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Source::is_random(a0)))
}

fn door_life_source_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Source::name(a0)))
}

fn door_life_source_numbers(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::life::Source::numbers()))
}

fn door_life_source_oeis(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::life::Source = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Source::oeis(a0)))
}

fn door_life_source_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::life::Source::parse(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_source_read(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::Source::read(a0.as_str())))
}

fn door_life_affine(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::affine(a0)))
}

fn door_life_animate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly::life::Config = take!(name, 1, &args[1]);
    match mrly::life::animate(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_churn(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::churn(&a0)))
}

fn door_life_corner_bits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::corner_bits(a0)))
}

fn door_life_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::life::Source = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrly::life::counts(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    match mrly::life::crop(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_cube_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::cube_orbit(a0)))
}

fn door_life_design_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 1, &args[1])?);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::life::design_mask(a0, a1, a2, a3) {
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
    Ok(give!(mrly::life::elementary::output(a0, a1, a2, a3)))
}

fn door_life_entropy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::entropy(&a0)))
}

fn door_life_frames(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::life::frames(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::gasket(a0)))
}

fn door_life_genus(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::genus(a0)))
}

fn door_life_heatmap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::life::heatmap(&a0, a1) {
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
    match mrly::life::history(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_lambda(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::lambda(a0)))
}

fn door_life_lattice_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::lattice_index(&a0)))
}

fn door_life_mask_offsets(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::mask_offsets(&a0)))
}

fn door_life_moore(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly::life::moore() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_next_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    let a3: mrly::core::Tensor = take!(name, 3, &args[3]);
    let a4: mrly::life::Boundary = take!(name, 4, &args[4]);
    match mrly::life::next_grid(&a0, &a1, &a2, &a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_npn_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::npn_class(a0)))
}

fn door_life_outer_totalistic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::outer_totalistic(a0)))
}

fn door_life_popcount(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::popcount(a0)))
}

fn door_life_render_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::life::render::frame(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_reversible(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::reversible(a0)))
}

fn door_life_rule_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::rule_degree(a0)))
}

fn door_life_rule_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    match mrly::life::rule_name(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_single_seed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::life::single_seed(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_source_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::life::Source = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::life::source::sequence(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_step(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly::life::step(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_surjective(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::surjective(a0)))
}

fn door_life_tessellate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::life::tessellate(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_life_wolfram_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u8 = take!(name, 0, &args[0]);
    Ok(give!(mrly::life::wolfram_class(a0)))
}

fn door_math_arcs_curves(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::arcs::curves(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_draw(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::arcs::draw(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::arcs::law(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arcs_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<bool> = take!(name, 1, &args[1]);
    match mrly::math::arcs::trace(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arrays_essential(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::arrays::essential(&a0)))
}

fn door_math_arrays_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly::math::arrays::fractal(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_arrays_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::arrays::holes(&a0)))
}

fn door_math_arrays_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::arrays::law(&a0, a1)))
}

fn door_math_arrays_lost(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::arrays::lost(&a0, a1)))
}

fn door_math_arrays_paired(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::arrays::paired(&a0)))
}

fn door_math_arrays_weights(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::arrays::weights(&a0)))
}

fn door_math_atoms_carpet_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::carpet_2d(a0)))
}

fn door_math_atoms_carpet_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::carpet_3d(a0)))
}

fn door_math_atoms_carpet_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::atoms::carpet_nd(a0, a1)))
}

fn door_math_atoms_dust_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::dust_2d(a0)))
}

fn door_math_atoms_dust_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::dust_3d(a0)))
}

fn door_math_atoms_dust_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::atoms::dust_nd(a0, a1)))
}

fn door_math_atoms_hline_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::hline_2d(a0)))
}

fn door_math_atoms_htree_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::htree_2d(a0)))
}

fn door_math_atoms_line_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::atoms::line_nd(a0, a1, a2)))
}

fn door_math_atoms_net_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::net_2d(a0)))
}

fn door_math_atoms_net_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::net_3d(a0)))
}

fn door_math_atoms_net_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::atoms::net_nd(a0, a1)))
}

fn door_math_atoms_noise_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let mut a2 = mrly::core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrly::math::atoms::noise_2d(a0, a1, &mut a2)))
}

fn door_math_atoms_noise_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let mut a2 = mrly::core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrly::math::atoms::noise_3d(a0, a1, &mut a2)))
}

fn door_math_atoms_ones_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::ones_2d(a0)))
}

fn door_math_atoms_ones_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::ones_3d(a0)))
}

fn door_math_atoms_point_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::point_2d(a0)))
}

fn door_math_atoms_point_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::point_3d(a0)))
}

fn door_math_atoms_point_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::atoms::point_nd(a0, a1)))
}

fn door_math_atoms_star_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::star_2d(a0)))
}

fn door_math_atoms_star_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::star_3d(a0)))
}

fn door_math_atoms_star_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::atoms::star_nd(a0, a1)))
}

fn door_math_atoms_tree_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::atoms::tree_nd(a0, a1, a2)))
}

fn door_math_atoms_vline_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::vline_2d(a0)))
}

fn door_math_atoms_void_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::void_2d(a0)))
}

fn door_math_atoms_void_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::void_3d(a0)))
}

fn door_math_atoms_void_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::atoms::void_nd(a0, a1)))
}

fn door_math_atoms_vtree_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::vtree_2d(a0)))
}

fn door_math_atoms_xline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::xline_3d(a0)))
}

fn door_math_atoms_xtree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::xtree_3d(a0)))
}

fn door_math_atoms_yline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::yline_3d(a0)))
}

fn door_math_atoms_ytree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::ytree_3d(a0)))
}

fn door_math_atoms_zeros_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::zeros_2d(a0)))
}

fn door_math_atoms_zeros_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::zeros_3d(a0)))
}

fn door_math_atoms_zline_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::zline_3d(a0)))
}

fn door_math_atoms_ztree_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::atoms::ztree_3d(a0)))
}

fn door_math_automaton_automaton_abscissa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::abscissa(&a0)))
}

fn door_math_automaton_automaton_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::base(&a0)))
}

fn door_math_automaton_automaton_cofactor(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly::math::automaton::Automaton::cofactor(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_denominator(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::denominator(&a0)))
}

fn door_math_automaton_automaton_matrix(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::matrix(&a0)))
}

fn door_math_automaton_automaton_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    match mrly::math::automaton::Automaton::new(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::peel(&a0)))
}

fn door_math_automaton_automaton_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::period(&a0)))
}

fn door_math_automaton_automaton_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::perron(&a0)))
}

fn door_math_automaton_automaton_residue(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly::math::automaton::Automaton::residue(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_rule(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::rule(&a0)))
}

fn door_math_automaton_automaton_states(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::automaton::Automaton::states(&a0)))
}

fn door_math_automaton_automaton_with_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::automaton::Automaton::with_peel(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_automaton_automaton_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::automaton::Automaton = take!(name, 0, &args[0]);
    let a1: mrly::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly::math::automaton::Automaton::zeta(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_design_anf(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::Design::anf(&a0)))
}

fn door_math_bang_design_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::Design::degree(&a0)))
}

fn door_math_bang_design_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::Design = take!(name, 0, &args[0]);
    match mrly::math::bang::Design::name(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_design_rule(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::Design::rule(&a0)))
}

fn door_math_bang_magiclayer_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::name::Bang = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::MagicLayer::new(a0, a1)))
}

fn door_math_bang_universe_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::Universe::all(&a0)))
}

fn door_math_bang_universe_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::Universe::canonical(&a0)))
}

fn door_math_bang_universe_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::bang::Universe = take!(name, 0, &args[0]);
    let a1: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(give!(mrly::math::bang::Universe::design(&a0, a1)))
}

fn door_math_bang_universe_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::Universe = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::Universe::distinct(&a0)))
}

fn door_math_bang_universe_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::bang::Universe::new(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_bang(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::bang::bang(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_axis_maps(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::baseq::axis_maps(a0)))
}

fn door_math_bang_baseq_bracelets(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::bang::baseq::bracelets(a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<usize>> = take!(name, 0, &args[0]);
    let a1: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 1, &args[1])?);
    match mrly::math::bang::baseq::canonical(&a0, a1) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_carry(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(Value::String(
        mrly::math::bang::baseq::carry(&a0, a1).get().to_string(),
    ))
}

fn door_math_bang_baseq_class_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::math::bang::baseq::class_sequence(a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_baseq_classes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly::math::bang::baseq::classes(a0).to_string(),
    ))
}

fn door_math_bang_baseq_distinct_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::bang::baseq::distinct_designs(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_even_fill_is_balanced(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u128 = big(name, 2, &args[2])?;
    match mrly::math::bang::baseq::even_fill_is_balanced(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_fill_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly::math::bang::baseq::fill_from_corners(&a0, a1, a2).to_string(),
    ))
}

fn door_math_bang_baseq_group(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::baseq::group(a0, a1)))
}

fn door_math_bang_baseq_group_order(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::bang::baseq::group_order(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<Vec<usize>> = take!(name, 0, &args[0]);
    let a1: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 1, &args[1])?);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::math::bang::baseq::orbit(&a0, a1) {
            list0.push(Value::String(item0.get().to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_baseq_predicted_group_order(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::bang::baseq::predicted_group_order(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_representatives(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::bang::baseq::representatives(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let parts1 = item0;
                    Value::Array(vec![
                        Value::String(parts1.0.get().to_string()),
                        give!(parts1.1),
                    ])
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_sequence(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::bang::baseq::sequence(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_baseq_total_designs(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::bang::baseq::total_designs(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_catalog_antis(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::catalog::antis(a0)))
}

fn door_math_bang_code_get(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::Code = take!(name, 0, &args[0]);
    Ok(Value::String(mrly::math::bang::Code::get(a0).to_string()))
}

fn door_math_bang_code_to_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::bang::code_to_corners(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::corners(a0)))
}

fn door_math_bang_corners_to_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly::math::bang::corners_to_code(&a0, a1, a2)
            .get()
            .to_string(),
    ))
}

fn door_math_bang_factory_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::bang::factory::create(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_create_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::bang::factory::create_from_corners(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_create_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::bang::factory::create_named(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_factory_residue_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::factory::residue_corners(a0, a1)))
}

fn door_math_bang_factory_total_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::bang::factory::total_codes(a0, a1) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_level_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::bang::level_cap(a0, a1, a2)))
}

fn door_math_bang_levels_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly::math::bang::levels_code(a0, a1, &a2).get().to_string(),
    ))
}

fn door_math_bang_magic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::magic(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_magic_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(String, usize)> = take!(name, 0, &args[0]);
    let b0: Vec<(&str, usize)> = a0.iter().map(|item| (item.0.as_str(), item.1)).collect();
    match mrly::math::bang::magic_named(&b0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_random_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let mut a2 = mrly::core::Rng::new(seed(name, 2, &args[2])?);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::bang::random_code(a0, a1, &mut a2, a3) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_sources(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::tile::recipe::Catalog = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::bang::sources(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_symmetries(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::symmetries(a0)))
}

fn door_math_bang_total_exposure(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::total_exposure(a0, a1)))
}

fn door_math_bang_touches_every_corner(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::touches_every_corner(a0, a1)))
}

fn door_math_bang_universe_anf(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::universe::anf(a0, a1)))
}

fn door_math_bang_universe_anf_string(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::universe::anf_string(a0, a1)))
}

fn door_math_bang_universe_apply(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: (Vec<usize>, Vec<u8>) = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::universe::apply(&a0, &a1)))
}

fn door_math_bang_universe_corner_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::universe::corner_index(&a0)))
}

fn door_math_bang_universe_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::universe::degree(a0, a1)))
}

fn door_math_bang_universe_orbit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::math::bang::universe::orbit(a0, a1) {
            list0.push(Value::String(item0.get().to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_bang_universe_permutations(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::universe::permutations(a0)))
}

fn door_math_bang_universe_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::bang::universe_codes(a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_schedule_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::bang::word::Schedule::all()))
}

fn door_math_bang_word_schedule_frequencies(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::word::Schedule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::word::Schedule::frequencies(a0)))
}

fn door_math_bang_word_schedule_place(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::bang::word::Schedule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::bang::word::Schedule::place(a0, a1)))
}

fn door_math_bang_word_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::word::components(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_constant_functional(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::word::constant_functional(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::word::dimension(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::word::fill(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::word::fills(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_letter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::MagicLayer = take!(name, 0, &args[0]);
    match mrly::math::bang::word::letter(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_native(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::word::native(&a0)))
}

fn door_math_bang_word_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::word::period(&a0)))
}

fn door_math_bang_word_prefixes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::word::prefixes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_rates(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::word::rates(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::bang::word::side(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_spell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::word::Schedule = take!(name, 0, &args[0]);
    let a1: (mrly::math::bang::MagicLayer, mrly::math::bang::MagicLayer) = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::bang::word::spell(a0, a1, a2)))
}

fn door_math_bang_word_staircase(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::bang::word::staircase(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_bang_word_thue_morse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::bang::word::thue_morse(a0)))
}

fn door_math_cell_census_edges(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly::math::cell::census::edges::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly::math::cell::census::edges::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_exposure(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(Value::String(
                mrly::math::cell::census::exposure::<2>(&a0).to_string(),
            ))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(Value::String(
                mrly::math::cell::census::exposure::<3>(&a0).to_string(),
            ))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_fills(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::census::fills::<2>(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::census::fills::<3>(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_vertices(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly::math::cell::census::vertices::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly::math::cell::census::vertices::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_census_voids(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::census::voids::<2>(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::census::voids::<3>(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_geometry_merge_reps(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
            let a1: Vec<usize> = take!(name, 1, &args[1]);
            match mrly::math::cell::geometry::merge_reps::<2>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: Vec<mrly::math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
            let a1: Vec<usize> = take!(name, 1, &args[1]);
            match mrly::math::cell::geometry::merge_reps::<3>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_geometry_perforate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 1)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
            let a1: mrly::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly::math::cell::geometry::perforate::<2>(&a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
            let a1: mrly::math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly::math::cell::geometry::perforate::<3>(&a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_grow(name: &str, args: &[Value]) -> Done {
    match tensor_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly::math::cell::grow::<2>(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly::math::cell::grow::<3>(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_anti(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::anti(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::anti(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_binarize(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: u8 = take!(name, 1, &args[1]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::binarize(
                a0, a1
            )))
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: u8 = take!(name, 1, &args[1]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::binarize(
                a0, a1
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_binarize_otsu(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::binarize_otsu(
                a0
            )))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::binarize_otsu(
                a0
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_blur(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
            let a2: bool = take!(name, 2, &args[2]);
            match mrly::math::cell::models::CellNd::<2>::blur(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
            let a2: bool = take!(name, 2, &args[2]);
            match mrly::math::cell::models::CellNd::<3>::blur(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_combine(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::combine(
                &a0, &a1
            )))
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly::math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::combine(
                &a0, &a1
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_counting_dtype(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::cell::models::counting_dtype(&a0)))
}

fn door_math_cell_models_depth(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::cell::models::CellNd::<3>::depth(&a0)))
}

fn door_math_cell_models_dtype_for(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: i64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::cell::models::dtype_for(a0)))
}

fn door_math_cell_models_fractal(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly::math::cell::models::CellNd::<2>::fractal(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly::math::cell::models::CellNd::<3>::fractal(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_height(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::height(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::height(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_invert(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::invert(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::invert(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_layers(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::layers(a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::layers(a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_neighbors(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            let a3: bool = take!(name, 3, &args[3]);
            match mrly::math::cell::models::CellNd::<2>::neighbors(a0, &a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            let a3: bool = take!(name, 3, &args[3]);
            match mrly::math::cell::models::CellNd::<3>::neighbors(a0, &a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_new(name: &str, args: &[Value]) -> Done {
    match tensor_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
            match mrly::math::cell::models::CellNd::<2>::new(a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
            match mrly::math::cell::models::CellNd::<3>::new(a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_orient(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::cell::models::CellNd::<3>::orient(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_models_pad(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::pad(
                a0, a1, a2
            )))
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::pad(
                a0, a1, a2
            )))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_paint(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1 = {
                let mut pairs0: Vec<(u8, Vec<mrly::core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect();
            let a2: mrly::core::Mode = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly::core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly::math::cell::models::CellNd::<2>::paint(a0, &a1, a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1 = {
                let mut pairs0: Vec<(u8, Vec<mrly::core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect();
            let a2: mrly::core::Mode = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly::core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly::math::cell::models::CellNd::<3>::paint(a0, &a1, a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_perforate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly::math::cell::models::CellNd::<2>::perforate(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
            let a2: u8 = take!(name, 2, &args[2]);
            match mrly::math::cell::models::CellNd::<3>::perforate(a0, &a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_rotate(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            match mrly::math::cell::models::CellNd::<2>::rotate(a0, a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 3)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: (usize, usize) = take!(name, 2, &args[2]);
            match mrly::math::cell::models::CellNd::<3>::rotate(a0, a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_tile(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 3)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: usize = take!(name, 2, &args[2]);
            match mrly::math::cell::models::CellNd::<2>::tile(a0, a1, a2) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: usize = take!(name, 1, &args[1]);
            let a2: usize = take!(name, 2, &args[2]);
            let a3: usize = take!(name, 3, &args[3]);
            match mrly::math::cell::models::CellNd::<3>::tile(a0, a1, a2, a3) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_types(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::types(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::types(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_models_width(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<2>::width(&a0)))
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            Ok(give!(mrly::math::cell::models::CellNd::<3>::width(&a0)))
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_paint(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 4)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            let a1: Option<_> = if Value::is_null(&args[1]) {
                None
            } else {
                Some(
                    {
                        let mut pairs0: Vec<(u8, Vec<mrly::core::Color>)> = Vec::new();
                        for (text0, item0) in fields(name, 1, &args[1])? {
                            let key0 = match text0.parse() {
                                Ok(key) => key,
                                Err(_) => return Err(bad(name, 1, "an object key")),
                            };
                            pairs0.push((key0, {
                                let mut list1 = Vec::new();
                                for item1 in items(name, 1, item0)? {
                                    list1.push(color(name, 1, item1)?);
                                }
                                list1
                            }));
                        }
                        pairs0
                    }
                    .into_iter()
                    .collect(),
                )
            };
            let a2: Option<mrly::core::Mode> = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly::core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly::math::cell::paint::<2>(a0, a1.as_ref(), a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 4)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            let a1: Option<_> = if Value::is_null(&args[1]) {
                None
            } else {
                Some(
                    {
                        let mut pairs0: Vec<(u8, Vec<mrly::core::Color>)> = Vec::new();
                        for (text0, item0) in fields(name, 1, &args[1])? {
                            let key0 = match text0.parse() {
                                Ok(key) => key,
                                Err(_) => return Err(bad(name, 1, "an object key")),
                            };
                            pairs0.push((key0, {
                                let mut list1 = Vec::new();
                                for item1 in items(name, 1, item0)? {
                                    list1.push(color(name, 1, item1)?);
                                }
                                list1
                            }));
                        }
                        pairs0
                    }
                    .into_iter()
                    .collect(),
                )
            };
            let a2: Option<mrly::core::Mode> = take!(name, 2, &args[2]);
            let mut a3 = if args[3].is_null() {
                None
            } else {
                Some(mrly::core::Rng::new(seed(name, 3, &args[3])?))
            };
            match mrly::math::cell::paint::<3>(a0, a1.as_ref(), a2, a3.as_mut()) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_cell_serializer_byte_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly::math::cell::serializer::byte_cube(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_byte_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly::math::cell::serializer::byte_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_color_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly::math::cell::serializer::color_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_count_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly::math::cell::serializer::count_cube(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_count_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly::math::cell::serializer::count_grid(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::math::cell::serializer::parse(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_tag_layer(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i64> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::math::cell::serializer::tag_layer(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_cell_serializer_types_field(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: serde_json::Value = take!(name, 0, &args[0]);
    match mrly::math::cell::serializer::types_field(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_exposure_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::counts::Exposure = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly::math::counts::Exposure::at(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly::math::counts::Exposure::from_corners(
        &a0, a1, a2, a3
    )))
}

fn door_math_counts_exposure_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::counts::Exposure::of_tile(&a0)))
}

fn door_math_counts_exposure_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::counts::Exposure = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::math::counts::Exposure::recurrence(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_centered_hexagonal(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly::math::counts::centered_hexagonal(a0).to_string(),
    ))
}

fn door_math_counts_cut_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly::math::counts::cut_fills(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_cut_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly::math::counts::cut_voids(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::counts::dimension(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_edges_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly::math::counts::edges_of_tile(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::counts::exposure(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_exposure_of_tile_2(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(match mrly::math::counts::exposure_of_tile(&a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_math_counts_exposure_recurrence_2(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::math::counts::exposure_recurrence(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_counts_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::counts::fill(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_fill_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    Ok(Value::String(
        mrly::math::counts::fill_from_corners(&a0, a1, a2, a3, a4).to_string(),
    ))
}

fn door_math_counts_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly::math::counts::grid(a0, a1, a2).to_string(),
    ))
}

fn door_math_counts_ladder_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::counts::ladder::cap(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_carry_matrix(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::counts::ladder::carry_matrix(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let mut list1 = Vec::new();
                    for item1 in item0 {
                        list1.push(Value::String(item1.to_string()));
                    }
                    Value::Array(list1)
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_characteristic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly::math::counts::ladder::characteristic(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_determinant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly::math::counts::ladder::determinant(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_digit_polynomial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::counts::ladder::digit_polynomial(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_even_block(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::counts::ladder::even_block(a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let mut list1 = Vec::new();
                    for item1 in item0 {
                        list1.push(Value::String(item1.to_string()));
                    }
                    Value::Array(list1)
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::counts::ladder::fill(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_ladder(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::counts::ladder::ladder(a0, a1, a2) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    match mrly::math::counts::ladder::perron(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_sign(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::counts::ladder::sign(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_spectral_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::counts::ladder::spectral_ratio(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ladder_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<i128>> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let mut list1 = Vec::new();
                for item1 in items(name, 0, item0)? {
                    list1.push(small(name, 0, item1)?);
                }
                list1
            });
        }
        list0
    };
    Ok(Value::String(
        mrly::math::counts::ladder::trace(&a0).to_string(),
    ))
}

fn door_math_counts_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::counts::limit(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let parts0 = value;
            Value::Array(vec![
                Value::String(parts0.0.to_string()),
                Value::String(parts0.1.to_string()),
            ])
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::math::counts::pairs(&a0) {
            list0.push({
                let parts1 = item0;
                Value::Array(vec![
                    Value::String(parts1.0.to_string()),
                    Value::String(parts1.1.to_string()),
                ])
            });
        }
        Value::Array(list0)
    })
}

fn door_math_counts_positions(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(Value::String(
        mrly::math::counts::positions(a0, a1, a2).to_string(),
    ))
}

fn door_math_counts_pro_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly::math::counts::pro_fills(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_pro_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly::math::counts::pro_voids(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_profile_of_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::math::counts::profile_of_tile(&a0, a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::counts::ratio(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_rational(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::counts::rational(a0, a1, a2, a3, a4) {
        Ok(value) => Ok({
            let parts0 = value;
            Value::Array(vec![
                Value::String(parts0.0.to_string()),
                Value::String(parts0.1.to_string()),
            ])
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_grid_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly::math::counts::six::grid_triangles(a0, a1).to_string(),
    ))
}

fn door_math_counts_six_solid_slice_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::counts::six::solid_slice_boundary(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_core_edges(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::counts::six::solid_slice_core_edges(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_core_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::counts::six::solid_slice_core_nodes(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_edges(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::counts::six::solid_slice_edges(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_interior(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::counts::six::solid_slice_interior(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::counts::six::solid_slice_triangles(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_six_solid_slice_vertices(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::counts::six::solid_slice_vertices(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_surface(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::counts::surface(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_counts_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::counts::void(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_digits_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::design::digits_of(a0, a1)))
}

fn door_math_design_echo_series(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::design::echo_series(&a0, &a1, a2)))
}

fn door_math_design_elements(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::design::elements(a0, &a1, a2)))
}

fn door_math_design_log_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::design::log_grid(&a0, a1)))
}

fn door_math_design_median_floor(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::design::median_floor(&a0, a1)))
}

fn door_math_design_meter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i8> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::design::meter(&a0)))
}

fn door_math_design_nearest(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::design::nearest(a0, &a1)))
}

fn door_math_design_peaks(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: (f64, f64) = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly::math::design::peaks(&a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_pole_lattice(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::design::pole_lattice(a0, a1)))
}

fn door_math_design_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<i64> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: Vec<f64> = take!(name, 3, &args[3]);
    Ok(give!(mrly::math::design::resample(&a0, &a1, a2, &a3)))
}

fn door_math_design_score(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::design::score(&a0, a1)))
}

fn door_math_design_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(Value::String(mrly::math::design::size(&a0, a1).to_string()))
}

fn door_math_design_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    match mrly::math::design::spectrum(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_design_upper_rms(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::design::upper_rms(&a0)))
}

fn door_math_dissection_cap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::dissection::cap(a0)))
}

fn door_math_dissection_chain_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::dissection::chain_exponent(a0)))
}

fn door_math_dissection_chain_margin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::dissection::chain_margin(a0)))
}

fn door_math_dissection_chain_root(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::dissection::chain_root(a0)))
}

fn door_math_dissection_chain_wall(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::dissection::chain_wall()))
}

fn door_math_dissection_consecutive(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::dissection::consecutive(&a0)))
}

fn door_math_dissection_fraction(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::dissection::fraction(a0, a1, a2)))
}

fn door_math_dissection_kappa(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::dissection::kappa(a0, &a1)))
}

fn door_math_dissection_masses(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::dissection::masses(a0, &a1, a2)))
}

fn door_math_dissection_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::dissection::reach(a0, a1)))
}

fn door_math_dissection_reading(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<f64> = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::dissection::reading(a0, a1, &a2)))
}

fn door_math_dissection_regions(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::dissection::regions(a0, a1, a2)))
}

fn door_math_dissection_tally(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly::math::dissection::tally(a0, &a1, a2, a3)))
}

fn door_math_dissection_weights(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::dissection::weights(a0, &a1, a2)))
}

fn door_math_graph_network_adjacency(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    match mrly::math::graph::Network::adjacency(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_network_degree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    match mrly::math::graph::Network::degree(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_network_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::graph::Network::new(a0)))
}

fn door_math_graph_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    match mrly::math::graph::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    match mrly::math::graph::components(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_core_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    match mrly::math::graph::core_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_edge_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    match mrly::math::graph::edge_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_fractal_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::graph::fractal_dimension(&a0, a1)))
}

fn door_math_graph_junctions(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    match mrly::math::graph::junctions(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_largest_component(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    match mrly::math::graph::largest_component(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_roles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    match mrly::math::graph::roles(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_tips(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    match mrly::math::graph::tips(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_graph_total_length(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::graph::total_length(&a0)))
}

fn door_math_graph_tunnel_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    match mrly::math::graph::tunnel_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_design_abscissa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::ladder::Design::abscissa(&a0)))
}

fn door_math_ladder_design_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::ladder::Design::base(&a0)))
}

fn door_math_ladder_design_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::ladder::Design::digits(&a0)))
}

fn door_math_ladder_design_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    match mrly::math::ladder::Design::new(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_design_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::ladder::Design::peel(&a0)))
}

fn door_math_ladder_design_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::ladder::Design::period(&a0)))
}

fn door_math_ladder_design_pole(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::ladder::Design::pole(&a0, a1, a2)))
}

fn door_math_ladder_design_with_peel(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: Vec<u64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::ladder::Design::with_peel(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_cofactor(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    let a1: mrly::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly::math::ladder::cofactor(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_residue(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly::math::ladder::residue(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_ladder_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::ladder::Design = take!(name, 0, &args[0]);
    let a1: mrly::num::zeta::Complex = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly::math::ladder::zeta(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_accepts(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::memory::Rule::accepts(&a0, &a1)))
}

fn door_math_memory_rule_allowed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::memory::Rule::allowed(&a0, a1)))
}

fn door_math_memory_rule_alphabet(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::Rule::alphabet(&a0)))
}

fn door_math_memory_rule_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly::math::memory::Rule::codes(&a0).to_string(),
    ))
}

fn door_math_memory_rule_full(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::memory::Rule::full(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_letters(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::Rule::letters(&a0)))
}

fn door_math_memory_rule_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly::math::memory::Rule::new(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_memory_rule_states(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::Rule::states(&a0)))
}

fn door_math_memory_rule_windows(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::Rule::windows(&a0)))
}

fn door_math_memory_allowed_windows(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::allowed_windows(&a0)))
}

fn door_math_memory_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::memory::cells(&a0, a1)))
}

fn door_math_memory_counts(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::memory::counts(&a0, a1)))
}

fn door_math_memory_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::exponent(&a0)))
}

fn door_math_memory_kappa(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::kappa(&a0)))
}

fn door_math_memory_perron(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::perron(&a0)))
}

fn door_math_memory_transfer(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::memory::Rule = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::memory::transfer(&a0)))
}

fn door_math_moire_field_as_f64(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Field::as_f64(&a0)))
}

fn door_math_moire_field_from_data(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::moire::Field::from_data(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_field_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Field::max(&a0)))
}

fn door_math_moire_field_mean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Field::mean(&a0)))
}

fn door_math_moire_field_min(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::moire::Field = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Field::min(&a0)))
}

fn door_math_moire_field_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Field::new(a0)))
}

fn door_math_moire_field_normalized(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::moire::Field = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::moire::Field::normalized(&a0, a1)))
}

fn door_math_moire_layer_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::moire::Spec = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::moire::Layer::new(a0, a1)))
}

fn door_math_moire_preset_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Preset::carpet(a0)))
}

fn door_math_moire_preset_heatmap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Preset::heatmap(a0)))
}

fn door_math_moire_preset_hive(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Preset::hive(a0)))
}

fn door_math_moire_preset_weave(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Preset::weave(a0)))
}

fn door_math_moire_spec_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::moire::Spec::new(a0, a1, a2)))
}

fn door_math_moire_volume_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly::math::moire::Volume::at(&a0, a1, a2, a3)))
}

fn door_math_moire_volume_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::moire::Volume::count(&a0, a1)))
}

fn door_math_moire_volume_from_data(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::moire::Volume::from_data(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_volume_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::moire::Volume = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Volume::max(&a0)))
}

fn door_math_moire_volume_min(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::moire::Volume = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Volume::min(&a0)))
}

fn door_math_moire_volume_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::Volume::new(a0)))
}

fn door_math_moire_volume_plane(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: mrly::math::moire::Frame = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::moire::Volume::plane(&a0, &a1, a2)))
}

fn door_math_moire_volume_sample(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: [f64; 3] = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::moire::Volume::sample(&a0, a1)))
}

fn door_math_moire_volume_solid(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::moire::Volume = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::moire::Volume::solid(&a0, a1)))
}

fn door_math_moire_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::moire::all(a0)))
}

fn door_math_moire_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [f64; 3] = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly::math::moire::frame(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_layer(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::moire::Layer = take!(name, 0, &args[0]);
    match mrly::math::moire::layer(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::moire::named(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_pairs_correlation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::moire::pairs::correlation(a0, a1)))
}

fn door_math_moire_pairs_sampled(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::moire::pairs::sampled(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_pairs_witness(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::moire::pairs::witness(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: mrly::math::moire::Field = take!(name, 0, &args[0]);
    let a1: mrly::core::Colorizer = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: bool = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrly::math::moire::render(&a0, &a1, a2, a3, a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_sample_axes(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly::math::moire::Lattice = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::moire::sample::axes(a0, a1, a2)))
}

fn door_math_moire_sample_membership(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::moire::sample::membership(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_sample_pack(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::moire::sample::pack(&a0, a1)))
}

fn door_math_moire_stack(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: mrly::math::moire::Spec = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: mrly::math::moire::Combine = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly::math::moire::Lattice = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    let a6: Vec<f64> = take!(name, 6, &args[6]);
    match mrly::math::moire::stack(a0, &a1, a2, a3, a4, a5, &a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_stack_codes(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<mrly::math::moire::Spec> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrly::math::moire::Lattice = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: Vec<f64> = take!(name, 5, &args[5]);
    match mrly::math::moire::stack_codes(&a0, a1, a2, a3, a4, &a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_moire_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::moire::Spec = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: mrly::math::moire::Combine = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::moire::volume(a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Bang = take!(name, 0, &args[0]);
    match mrly::math::name::Bang::cells(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Bang = take!(name, 0, &args[0]);
    match <mrly::math::name::Bang as mrly::core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Bang as mrly::core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Bang as mrly::core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Bang as mrly::core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::name::Bang::new(a0, a1, a2)))
}

fn door_math_name_bang_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Bang = take!(name, 0, &args[0]);
    match <mrly::math::name::Bang as mrly::core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Bang = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::math::name::Bang as mrly::core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_bang_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Bang = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::math::name::Bang as mrly::core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_bang_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Bang = take!(name, 0, &args[0]);
    match <mrly::math::name::Bang as mrly::core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_bang_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Bang = take!(name, 0, &args[0]);
    match <mrly::math::name::Bang as mrly::core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_lattice_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::name::Lattice::default()))
}

fn door_math_name_lattice_is_square(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Lattice = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::name::Lattice::is_square(&a0)))
}

fn door_math_name_lattice_units(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Lattice = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::name::Lattice::units(a0)))
}

fn door_math_name_sequence_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly::math::name::Sequence as mrly::core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::name::Sequence::design(&a0)))
}

fn door_math_name_sequence_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Sequence as mrly::core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Sequence as mrly::core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Sequence as mrly::core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    Ok(give!(mrly::math::name::Sequence::new(
        a0,
        a1,
        a2,
        a3.as_str(),
        a4.as_str()
    )))
}

fn door_math_name_sequence_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly::math::name::Sequence as mrly::core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::math::name::Sequence as mrly::core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_sequence_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Sequence = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::math::name::Sequence as mrly::core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_sequence_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly::math::name::Sequence as mrly::core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_sequence_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Sequence = take!(name, 0, &args[0]);
    match <mrly::math::name::Sequence as mrly::core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_bases(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::name::Word::bases(&a0)))
}

fn door_math_name_word_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Word = take!(name, 0, &args[0]);
    match <mrly::math::name::Word as mrly::core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Word as mrly::core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Word as mrly::core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::math::name::Word as mrly::core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_letters(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::name::Word::letters(&a0)))
}

fn door_math_name_word_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<(u128, usize)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push({
                let parts1 = parts(name, 1, item0, 2)?;
                (big(name, 1, &parts1[0])?, take!(name, 1, &parts1[1]))
            });
        }
        list0
    };
    match mrly::math::name::Word::new(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Word = take!(name, 0, &args[0]);
    match <mrly::math::name::Word as mrly::core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::math::name::Word as mrly::core::name::Named>::to_id(&a0)
    ))
}

fn door_math_name_word_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Word = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::math::name::Word as mrly::core::name::Named>::to_json(&a0)
    ))
}

fn door_math_name_word_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Word = take!(name, 0, &args[0]);
    match <mrly::math::name::Word as mrly::core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_word_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::name::Word = take!(name, 0, &args[0]);
    match <mrly::math::name::Word as mrly::core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_name_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::name::of(a0, a1, a2)))
}

fn door_math_name_words(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::name::words(a0, a1)))
}

fn door_math_nodal_figures(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::nodal::figures(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_nodal_holds(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::nodal::holds(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_press_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::press::Press::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_press_total(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::press::Press = take!(name, 0, &args[0]);
    let a1: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 1, &args[1])?);
    Ok(Value::String(
        mrly::math::press::Press::total(&a0, a1).to_string(),
    ))
}

fn door_math_press_press_totals(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::press::Press = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::math::press::Press::totals(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_math_press_containing(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::press::containing(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_coordinates(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::press::coordinates(a0, a1, a2) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_count_below(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: u128 = big(name, 3, &args[3])?;
    match mrly::math::press::count_below(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::press::distinct(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_interleave(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(big(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::press::interleave(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_layer_table(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::bang::MagicLayer = take!(name, 0, &args[0]);
    match mrly::math::press::layer_table(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: u128 = big(name, 1, &args[1])?;
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::press::member(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_members(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::press::members(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::press::profile(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_usage(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::press::usage(a0, a1, a2) {
        Ok(value) => Ok(Value::String(value.get().to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::press::word_count(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly::math::press::word_member(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_members(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::press::word_members(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_press_word_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::bang::MagicLayer> = take!(name, 0, &args[0]);
    match mrly::math::press::word_profile(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    match mrly::math::radix::Base::class(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_congruent(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::radix::Base::congruent(a0, a1, a2)))
}

fn door_math_radix_base_group(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    match mrly::math::radix::Base::group(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_mirrored(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Base::mirrored(a0)))
}

fn door_math_radix_base_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    match mrly::math::radix::Base::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_norm(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Base::norm(a0)))
}

fn door_math_radix_base_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::radix::Base::power(a0, a1)))
}

fn door_math_radix_base_residues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    match mrly::math::radix::Base::residues(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_base_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Base::ring(a0)))
}

fn door_math_radix_base_value(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Base::value(a0)))
}

fn door_math_radix_radix_base(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Radix::base(&a0)))
}

fn door_math_radix_radix_canonical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    match mrly::math::radix::Radix::canonical(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    match mrly::math::radix::Radix::code(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Radix::digits(&a0)))
}

fn door_math_radix_radix_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Radix::dimension(&a0)))
}

fn door_math_radix_radix_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::radix::Radix::distinct(&a0, a1)))
}

fn door_math_radix_radix_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly::math::radix::Radix::fill(&a0, a1).to_string(),
    ))
}

fn door_math_radix_radix_from_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly::math::radix::Radix::from_code(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::radix::Base = take!(name, 0, &args[0]);
    let a1: Vec<(i64, i64)> = take!(name, 1, &args[1]);
    let a2: Vec<(i64, i64)> = take!(name, 2, &args[2]);
    match mrly::math::radix::Radix::new(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_plane(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::radix::Radix::plane(&a0, a1)))
}

fn door_math_radix_radix_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Radix::ring(&a0)))
}

fn door_math_radix_radix_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Radix::size(&a0)))
}

fn door_math_radix_radix_twists(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::radix::Radix::twists(&a0)))
}

fn door_math_radix_radix_with_twists(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::math::radix::Radix::with_twists(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_radix_words(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::radix::Radix = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::radix::Radix::words(&a0, a1)))
}

fn door_math_radix_flowsnake(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly::math::radix::flowsnake() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_gasket(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly::math::radix::gasket() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_koch(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly::math::radix::koch() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_terdragon(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly::math::radix::terdragon() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    match mrly::math::radix::tile(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_radix_twindragon(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    match mrly::math::radix::twindragon() {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_roulette_nodes_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::roulette::Nodes::default()))
}

fn door_math_roulette_nodes_pair(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::roulette::Nodes = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::roulette::Nodes::pair(&a0, a1, a2)))
}

fn door_math_roulette_nodes_paired(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::roulette::Nodes::paired(&a0)))
}

fn door_math_roulette_nodes_selved(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::roulette::Nodes::selved(&a0)))
}

fn door_math_roulette_nodes_total(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::roulette::Nodes = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::roulette::Nodes::total(&a0)))
}

fn door_math_roulette_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: f64 = take!(name, 3, &args[3]);
    match mrly::math::roulette::nodes(&a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_roulette_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: [f64; 2] = take!(name, 0, &args[0]);
    let a1: [f64; 2] = take!(name, 1, &args[1]);
    let a2: [f64; 2] = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::roulette::side(a0, a1, a2)))
}

fn door_math_roulette_spread(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::roulette::spread(&a0, &a1, a2)))
}

fn door_math_rules_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::rules::render(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_rules_tree_axes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::rules::tree_axes(a0, a1)))
}

fn door_math_shape_frac_minus(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly::math::shape::Frac = take!(name, 1, &args[1]);
    match mrly::math::shape::Frac::minus(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly::math::shape::Frac::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_plus(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly::math::shape::Frac = take!(name, 1, &args[1]);
    match mrly::math::shape::Frac::plus(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_times(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::shape::Frac = take!(name, 0, &args[0]);
    let a1: mrly::math::shape::Frac = take!(name, 1, &args[1]);
    match mrly::math::shape::Frac::times(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_frac_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: i64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::shape::Frac::whole(a0)))
}

fn door_math_shape_region_flip(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::shape::Region = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::shape::Region::flip(a0)))
}

fn door_math_shape_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::shape::Shape = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    match mrly::math::shape::census(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_classify(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::shape::Shape = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    match mrly::math::shape::classify(&a0, a1, &a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::math::shape::Shape = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly::math::shape::crop(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_crossing_shell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::shape::crossing_shell(a0, a1, a2)))
}

fn door_math_shape_crossing_tree(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: Vec<bool> = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::shape::crossing_tree(a0, a1, &a2)))
}

fn door_math_shape_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::shape::Shape = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::math::shape::fit(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly::math::shape::Frac = take!(name, 2, &args[2]);
    match mrly::math::shape::named(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_radial_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<i64> = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::shape::radial_census(&a0, &a1, a2)))
}

fn door_math_shape_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u32 = take!(name, 2, &args[2]);
    match mrly::math::shape::reach(a0.as_str(), a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_refine(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::math::shape::Shape = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: bool = take!(name, 4, &args[4]);
    match mrly::math::shape::refine(&a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_regions(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::shape::Shape = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::math::shape::regions(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_shape_shapes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::shape::shapes(a0)))
}

fn door_math_six_anti(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::Cell6d::anti(a0)))
}

fn door_math_six_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::six::Cell6d::binarize(a0, a1)))
}

fn door_math_six_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::Cell6d::binarize_otsu(a0)))
}

fn door_math_six_blank(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly::math::six::Orientation = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    match mrly::math::six::blank(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly::math::six::Cell6d::blur(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::six::census(&a0, a1)))
}

fn door_math_six_components(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::components(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_copies(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::six::copies(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_cut(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly::math::six::cut(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_cut_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::six::cut_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_disc(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::six::disc(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_east(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::six::east(a0, a1)))
}

fn door_math_six_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::six::euler(&a0, a1)))
}

fn door_math_six_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::fills(&a0)))
}

fn door_math_six_fills_only(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::fills_only(&a0)))
}

fn door_math_six_framed(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::framed(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::math::six::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_giant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::giant(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_giant_network(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::giant_network(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_height(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::Cell6d::height(&a0)))
}

fn door_math_six_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::holes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly::core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::six::image(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_is_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::is_cube(&a0)))
}

fn door_math_six_is_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::is_hex(&a0)))
}

fn door_math_six_iso(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly::math::six::iso(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_iso_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::six::iso_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly::math::six::Projection = take!(name, 1, &args[1]);
    let a2: mrly::math::six::Orientation = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(give!(mrly::math::six::Cell6d::new(a0, a1, a2, a3)))
}

fn door_math_six_north(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::six::north(a0, a1)))
}

fn door_math_six_orientation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::six::orientation(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly::math::six::pad(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_paint(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<_> = if Value::is_null(&args[1]) {
        None
    } else {
        Some(
            {
                let mut pairs0: Vec<(u8, Vec<mrly::core::Color>)> = Vec::new();
                for (text0, item0) in fields(name, 1, &args[1])? {
                    let key0 = match text0.parse() {
                        Ok(key) => key,
                        Err(_) => return Err(bad(name, 1, "an object key")),
                    };
                    pairs0.push((key0, {
                        let mut list1 = Vec::new();
                        for item1 in items(name, 1, item0)? {
                            list1.push(color(name, 1, item1)?);
                        }
                        list1
                    }));
                }
                pairs0
            }
            .into_iter()
            .collect(),
        )
    };
    let a2: Option<mrly::core::Mode> = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() {
        None
    } else {
        Some(mrly::core::Rng::new(seed(name, 3, &args[3])?))
    };
    match mrly::math::six::paint(a0, a1.as_ref(), a2, a3.as_mut()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly::math::six::Cell6d::perforate(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pro(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly::math::six::pro(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_pro_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::six::pro_design(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::six::radial(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly::math::six::radial_crop(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_radial_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: mrly::math::six::Orientation = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::six::radial_mask(a0, a1)))
}

fn door_math_six_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::six::raster(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rect_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<usize> = take!(name, 2, &args[2]);
    match mrly::math::six::rect_image(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rect_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<usize> = take!(name, 2, &args[2]);
    match mrly::math::six::rect_svg(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_rim_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::rim_holes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_skin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::skin(&a0)))
}

fn door_math_six_slice_core_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::slice_core_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_dual_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::slice_dual_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_edge_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<u8> = take!(name, 1, &args[1]);
    match mrly::math::six::slice_edge_graph(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_slice_tunnel_graph(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    match mrly::math::six::slice_tunnel_graph(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_south(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::six::south(a0, a1)))
}

fn door_math_six_spectral_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly::math::six::spectral_exponent(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_branch_constant(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::star::Branch::constant(a0)))
}

fn door_math_six_star_branch_name(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::star::Branch::name(a0)))
}

fn door_math_six_star_branch_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::star::Branch::of(a0)))
}

fn door_math_six_star_branch_residual(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::star::Branch = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::star::Branch::residual(a0)))
}

fn door_math_six_star_share_reduced(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::star::Share = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::star::Share::reduced(a0)))
}

fn door_math_six_star_share_value(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::star::Share = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::star::Share::value(a0)))
}

fn door_math_six_star_star_arm(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::six::star::Star::arm(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    Ok(give!(mrly::math::six::star::Star::cell(&a0, a1, a2, a3)))
}

fn door_math_six_star_star_excesses(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::six::star::Star::excesses(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_hexagon(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::star::Star = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::six::star::Star::hexagon(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_star_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u128 = big(name, 0, &args[0])?;
    match mrly::math::six::star::Star::new(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_arm_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::math::six::star::arm_law(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_chi8(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::star::chi8(a0)))
}

fn door_math_six_star_constant(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::six::star::constant()))
}

fn door_math_six_star_decay(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::six::star::decay(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_star_width_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::star::width_law(a0)))
}

fn door_math_six_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly::core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Option<usize> = take!(name, 4, &args[4]);
    match mrly::math::six::svg(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tessellate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: mrly::core::Tensor = take!(name, 1, &args[1]);
    match mrly::math::six::tessellate(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::six::tile(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    match mrly::math::six::tile_cell(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_crop(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: (usize, usize) = take!(name, 1, &args[1]);
    match mrly::math::six::tile_crop(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_tile_step(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: (usize, usize) = take!(name, 0, &args[0]);
    match mrly::math::six::tile_step(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::to_json(&a0)))
}

fn door_math_six_triangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    let a1: Option<usize> = take!(name, 1, &args[1]);
    match mrly::math::six::triangles(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_six_west(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::six::west(a0, a1)))
}

fn door_math_six_width(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::six::Cell6d = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::six::Cell6d::width(&a0)))
}

fn door_math_spectrum_clusters(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    match mrly::math::spectrum::clusters(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_laplacian(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly::math::spectrum::laplacian(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_laplacian_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::graph::Network = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly::math::spectrum::laplacian_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spectrum_multiplicity(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::spectrum::multiplicity(&a0, a1, a2)))
}

fn door_math_spectrum_spectral_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spectrum::spectral_exponent(&a0, a1)))
}

fn door_math_spectrum_spectral_fit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spectrum::spectral_fit(&a0, a1)))
}

fn door_math_spectrum_spectral_points(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::spectrum::spectral_points(&a0)))
}

fn door_math_spectrum_symmetric_eigenvalues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<f64>> = take!(name, 0, &args[0]);
    match mrly::math::spectrum::symmetric_eigenvalues(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_blend_fold(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::spin::Blend = take!(name, 0, &args[0]);
    let a1: Vec<f32> = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spin::Blend::fold(a0, &a1)))
}

fn door_math_spin_blend_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::spin::Blend::named(a0.as_str())))
}

fn door_math_spin_arcs(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly::math::spin::arcs(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_harmonics(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::spin::harmonics(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_mass(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spin::mass(&a0, a1)))
}

fn door_math_spin_mass_within(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::spin::mass_within(&a0, a1, a2)))
}

fn door_math_spin_petals(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spin::petals(a0, a1)))
}

fn door_math_spin_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::spin::profile(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_radial(name: &str, args: &[Value]) -> Done {
    count(name, args, 7)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: f64 = take!(name, 4, &args[4]);
    let a5: mrly::math::spin::Blend = take!(name, 5, &args[5]);
    let a6: usize = take!(name, 6, &args[6]);
    match mrly::math::spin::radial(&a0, a1, a2, a3, a4, a5, a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::spin::reach(a0)))
}

fn door_math_spin_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly::math::spin::ring(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spin_turns(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::spin::turns(&a0)))
}

fn door_math_spin_wheel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f32> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spin::wheel(&a0, a1)))
}

fn door_math_spirograph_seats_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::spirograph::Seats::default()))
}

fn door_math_spirograph_cell(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::spirograph::cell(a0, a1, a2)))
}

fn door_math_spirograph_cover(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::spirograph::cover(&a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_disc(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    match mrly::math::spirograph::disc(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_distinct(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::spirograph::distinct(&a0, &a1, a2)))
}

fn door_math_spirograph_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spirograph::frame(&a0, &a1)))
}

fn door_math_spirograph_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::spirograph::nodes(&a0, &a1, a2)))
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
    match mrly::math::spirograph::pencils(&a0, a1, a2, a3.as_str(), a4, a5, a6) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: mrly::math::spirograph::Pencil = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::spirograph::point(&a0, &a1, a2)))
}

fn door_math_spirograph_pose(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spirograph::pose(&a0, a1)))
}

fn door_math_spirograph_representatives(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::spirograph::representatives(&a0, &a1, a2)))
}

fn door_math_spirograph_seats(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::spirograph::Pencil> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::spirograph::seats(&a0)))
}

fn door_math_spirograph_signed_area(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: mrly::math::spirograph::Pencil = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spirograph::signed_area(&a0, &a1)))
}

fn door_math_spirograph_trace(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: Vec<mrly::math::spirograph::Pencil> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::spirograph::trace(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_track(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::spirograph::track(a0.as_str(), a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_spirograph_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::spirograph::Track = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::spirograph::turn(&a0, a1)))
}

fn door_math_sumset_pair_clean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::sumset::Pair::clean(&a0)))
}

fn door_math_sumset_pair_copy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::sumset::Pair::copy(&a0)))
}

fn door_math_sumset_pair_energy(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(Value::String(
        mrly::math::sumset::Pair::energy(&a0).to_string(),
    ))
}

fn door_math_sumset_pair_four(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::sumset::Pair::four(&a0)))
}

fn door_math_sumset_pair_gap(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::sumset::Pair::gap(&a0)))
}

fn door_math_sumset_pair_largest(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::sumset::Pair::largest(&a0)))
}

fn door_math_sumset_pair_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::math::sumset::Pair::new(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_sumset_pair_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    let a1: u128 = big(name, 1, &args[1])?;
    Ok(give!(mrly::math::sumset::Pair::ratio(&a0, a1)))
}

fn door_math_sumset_pair_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::sumset::Pair::scale(&a0)))
}

fn door_math_sumset_pair_three(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::sumset::Pair = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::sumset::Pair::three(&a0)))
}

fn door_math_sumset_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly::math::sumset::pairs(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_vec3_cross(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: mrly::math::three::Vec3 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::three::Vec3::cross(a0, a1)))
}

fn door_math_three_vec3_dot(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: mrly::math::three::Vec3 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::three::Vec3::dot(a0, a1)))
}

fn door_math_three_vec3_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: f32 = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    let a2: f32 = take!(name, 2, &args[2]);
    Ok(give!(mrly::math::three::Vec3::new(a0, a1, a2)))
}

fn door_math_three_vec3_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::three::Vec3 = take!(name, 0, &args[0]);
    let a1: f32 = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::three::Vec3::scale(a0, a1)))
}

fn door_math_three_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::carpet(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly::math::three::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_core_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly::math::three::core_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly::math::three::core_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::three::create(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_diagonal_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::three::diagonal_slice(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_diagonal_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: Vec<usize> = take!(name, 4, &args[4]);
    let a5: usize = take!(name, 5, &args[5]);
    match mrly::math::three::diagonal_svg(a0, a1, a2, a3, &a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_dust(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::dust(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_edge_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly::math::three::edge_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly::math::three::edge_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    match mrly::math::three::euler(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_extrude(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::three::extrude(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_extrude_cube(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::extrude_cube(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_faces(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::faces(&a0)))
}

fn door_math_three_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::fills(&a0)))
}

fn door_math_three_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::three::from_corners(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::math::three::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_from_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<Vec<String>> = take!(name, 0, &args[0]);
    match mrly::math::three::from_strings(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_hidden(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly::math::three::hidden(&a0).to_string()))
}

fn door_math_three_level_set(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::three::level_set(a0, &a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_magic(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
            match mrly::math::three::magic::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: Vec<mrly::math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
            match mrly::math::three::magic::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_manhattan_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::manhattan_layers(a0)))
}

fn door_math_three_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<mrly::math::cell::models::CellNd<3>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::three::merge(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_mosaic(name: &str, args: &[Value]) -> Done {
    match cells_rank(name, args, 1)? {
        2 => {
            count(name, args, 2)?;
            let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
            let a1: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 1, &args[1]);
            match mrly::math::three::mosaic::<2>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 2)?;
            let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
            let a1: Vec<mrly::math::cell::models::CellNd<3>> = take!(name, 1, &args[1]);
            match mrly::math::three::mosaic::<3>(&a0, &a1) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::tile::recipe::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::three::named(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_net(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::net(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_noise(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let mut a3 = mrly::core::Rng::new(seed(name, 3, &args[3])?);
    match mrly::math::three::noise(a0, a1, a2, &mut a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ones(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::ones(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_orientations(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::three::orientations()))
}

fn door_math_three_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::point(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::three::profile(a0, a1, a2, a3) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_project(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [u32; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::project(a0)))
}

fn door_math_three_quads(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::quads(&a0)))
}

fn door_math_three_shadow(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [u32; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::shadow(a0)))
}

fn door_math_three_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::three::slice(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_special(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::math::cell::models::CellNd<3> = take!(name, 1, &args[1]);
    match mrly::math::three::special(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_sponge_deep(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::sponge::deep(a0)))
}

fn door_math_three_sponge_dimension(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::three::sponge::dimension()))
}

fn door_math_three_sponge_distance(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [f64; 3] = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::sponge::distance(a0)))
}

fn door_math_three_sponge_exact(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::sponge::exact(a0)))
}

fn door_math_three_sponge_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::sponge::profile(a0)))
}

fn door_math_three_sponge_reading(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::sponge::reading(a0)))
}

fn door_math_three_sponge_tube(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::sponge::tube(a0)))
}

fn door_math_three_sponge_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::sponge::volume(a0)))
}

fn door_math_three_star(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::star(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_support(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(big(name, 0, item0)?);
        }
        list0
    };
    Ok(give!(mrly::math::three::support(&a0)))
}

fn door_math_three_surface(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly::math::three::surface(&a0).to_string()))
}

fn door_math_three_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    let a1: Option<_> = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::three::text(&a0, a1.as_ref())))
}

fn door_math_three_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::to_json(&a0)))
}

fn door_math_three_to_obj(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::to_obj(&a0)))
}

fn door_math_three_to_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::to_strings(&a0)))
}

fn door_math_three_tunnel_graph(name: &str, args: &[Value]) -> Done {
    match cell_rank(name, args, 0)? {
        2 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
            match mrly::math::three::tunnel_graph::<2>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        3 => {
            count(name, args, 1)?;
            let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
            match mrly::math::three::tunnel_graph::<3>(&a0) {
                Ok(value) => Ok(give!(value)),
                Err(error) => Err(Error::Library(error.to_string())),
            }
        }
        other => Err(usage(format!(
            "{name} wants a 2d or 3d argument, got {other}d"
        ))),
    }
}

fn door_math_three_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::void(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::voids(&a0)))
}

fn door_math_three_volume(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::volume(&a0)))
}

fn door_math_three_wires(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<3> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::three::wires(&a0)))
}

fn door_math_three_xline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::xline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_xtree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::xtree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_yline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::yline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ytree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::ytree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::zeros(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_zline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::zline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_three_ztree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::three::ztree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_eyes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::tourbillon::eyes(a0)))
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
    match mrly::math::tourbillon::field(
        a0,
        a1,
        a2.as_str(),
        a3,
        a4.as_str(),
        a5.as_str(),
        a6.as_str(),
        a7.as_str(),
        a8,
    ) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: String = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: String = take!(name, 3, &args[3]);
    let a4: String = take!(name, 4, &args[4]);
    let a5: u32 = take!(name, 5, &args[5]);
    match mrly::math::tourbillon::layers(a0, a1.as_str(), a2, a3.as_str(), a4.as_str(), a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_tourbillon_period(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::tourbillon::period(a0)))
}

fn door_math_tourbillon_sharing(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly::math::tourbillon::Layer> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::tourbillon::sharing(&a0)))
}

fn door_math_tourbillon_stack(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<mrly::math::tourbillon::Layer> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: String = take!(name, 2, &args[2]);
    let a3: mrly::math::spin::Blend = take!(name, 3, &args[3]);
    match mrly::math::tourbillon::stack(&a0, a1, a2.as_str(), a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
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
    match mrly::math::tourbillon::stats(
        &a0,
        a1,
        a2,
        a3.as_str(),
        a4,
        a5.as_str(),
        a6.as_str(),
        a7.as_str(),
        a8,
    ) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_capacity(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::two::capacity(&a0)))
}

fn door_math_two_carpet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::carpet(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrly::math::two::census(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::bang::Code = mrly::math::bang::Code::from(big(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::two::create(a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_dust(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::dust(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_embed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    match mrly::math::two::embed(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_euler(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    match mrly::math::two::euler(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_extract(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly::math::two::extract(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_fills(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::two::fills(&a0)))
}

fn door_math_two_from_corners(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: Vec<Vec<u8>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::two::from_corners(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::math::two::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_from_strings(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<String> = take!(name, 0, &args[0]);
    match mrly::math::two::from_strings(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_hline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::hline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_htree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::htree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_image(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly::core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly::math::two::Shape = take!(name, 4, &args[4]);
    match mrly::math::two::image(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_level_set(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    match mrly::math::two::level_set(a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly::math::two::mask(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_merge(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<mrly::math::cell::models::CellNd<2>> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::math::two::merge(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::tile::recipe::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::math::two::named(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_net(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::net(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_noise(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let mut a3 = mrly::core::Rng::new(seed(name, 3, &args[3])?);
    match mrly::math::two::noise(a0, a1, a2, &mut a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_ones(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::ones(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_payload_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::math::two::payload::frame()))
}

fn door_math_two_perimeter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(Value::String(mrly::math::two::perimeter(&a0).to_string()))
}

fn door_math_two_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::point(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_read(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: mrly::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly::math::two::read(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_sheet(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly::math::cell::models::CellNd<2>; 4] = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    match mrly::math::two::sheet(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_special(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly::math::cell::models::CellNd<2> = take!(name, 1, &args[1]);
    match mrly::math::two::special(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_star(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::star(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_svg(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Option<mrly::core::Color> = if Value::is_null(&args[2]) {
        None
    } else {
        Some(color(name, 2, &args[2])?)
    };
    let a3: usize = take!(name, 3, &args[3]);
    let a4: mrly::math::two::Shape = take!(name, 4, &args[4]);
    Ok(give!(mrly::math::two::svg(&a0, a1, a2, a3, a4)))
}

fn door_math_two_text(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    let a1: Option<_> = take!(name, 1, &args[1]);
    Ok(give!(mrly::math::two::text(&a0, a1.as_ref())))
}

fn door_math_two_to_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::two::to_3d(&a0)))
}

fn door_math_two_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::two::to_json(&a0)))
}

fn door_math_two_vline(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::vline(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_void(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::void(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_voids(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::math::cell::models::CellNd<2> = take!(name, 0, &args[0]);
    Ok(give!(mrly::math::two::voids(&a0)))
}

fn door_math_two_vtree(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::vtree(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_math_two_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::math::two::zeros(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_circle_centre(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::apollonian::Circle::centre(a0)))
}

fn door_num_apollonian_circle_is_line(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::apollonian::Circle::is_line(a0)))
}

fn door_num_apollonian_circle_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::apollonian::Circle::radius(a0)))
}

fn door_num_apollonian_flat(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly::num::apollonian::flat(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_form(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [i64; 4] = take!(name, 0, &args[0]);
    let a1: [i64; 4] = take!(name, 1, &args[1]);
    Ok(Value::String(
        mrly::num::apollonian::form(a0, a1).to_string(),
    ))
}

fn door_num_apollonian_frame(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::apollonian::Packing = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::apollonian::frame(&a0)))
}

fn door_num_apollonian_grow(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: String = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    match mrly::num::apollonian::grow(a0.as_str(), a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_is_ford(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::apollonian::is_ford(a0)))
}

fn door_num_apollonian_on_line(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::apollonian::Circle = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::apollonian::on_line(a0)))
}

fn door_num_apollonian_reflect(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly::num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::apollonian::reflect(&a0, a1)))
}

fn door_num_apollonian_root(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly::num::apollonian::root(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_shadow(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::apollonian::Packing = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::apollonian::shadow(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_apollonian_sound(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: [mrly::num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::apollonian::sound(&a0)))
}

fn door_num_apollonian_swap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: [mrly::num::apollonian::Circle; 4] = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::apollonian::swap(&a0, a1)))
}

fn door_num_apollonian_touches(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::apollonian::Packing = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::apollonian::touches(&a0)))
}

fn door_num_blend_add(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::num::blend::add(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_cauchy(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    match mrly::num::blend::cauchy(&a0, &a1) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_blend_characteristic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(i128, i128)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let parts1 = parts(name, 0, item0, 2)?;
                (small(name, 0, &parts1[0])?, small(name, 0, &parts1[1])?)
            });
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::num::blend::characteristic(&a0) {
            list0.push({
                let parts1 = item0;
                Value::Array(vec![
                    Value::String(parts1.0.to_string()),
                    Value::String(parts1.1.to_string()),
                ])
            });
        }
        Value::Array(list0)
    })
}

fn door_num_blend_decimate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::num::blend::decimate(&a0, a1, a2) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_blend_delta(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::num::blend::delta(&a0) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_growth(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<(i128, i128)> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push({
                let parts1 = parts(name, 0, item0, 2)?;
                (small(name, 0, &parts1[0])?, small(name, 0, &parts1[1])?)
            });
        }
        list0
    };
    Ok(give!(mrly::num::blend::growth(&a0)))
}

fn door_num_blend_hadamard(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::num::blend::hadamard(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_recurrence(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    Ok(match mrly::num::blend::recurrence(&a0) {
        Some(item0) => {
            let mut list1 = Vec::new();
            for item1 in item0 {
                list1.push({
                    let parts2 = item1;
                    Value::Array(vec![
                        Value::String(parts2.0.to_string()),
                        Value::String(parts2.1.to_string()),
                    ])
                });
            }
            Value::Array(list1)
        }
        None => Value::Null,
    })
}

fn door_num_blend_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: i128 = small(name, 1, &args[1])?;
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::num::blend::scale(&a0, a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_shift(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::num::blend::shift(&a0, a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_blend_sigma(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    match mrly::num::blend::sigma(&a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push(Value::String(item0.to_string()));
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_blend_sub(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(small(name, 0, item0)?);
        }
        list0
    };
    let a1: Vec<i128> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(small(name, 1, item0)?);
        }
        list0
    };
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly::num::blend::sub(&a0, &a1) {
            list0.push(Value::String(item0.to_string()));
        }
        Value::Array(list0)
    })
}

fn door_num_boolean_is_balanced(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::boolean::is_balanced(a0, a1)))
}

fn door_num_boolean_nonlinearity(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::boolean::nonlinearity(a0, a1)))
}

fn door_num_boolean_sac(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::boolean::sac(a0, a1)))
}

fn door_num_boolean_walsh_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::boolean::walsh_spectrum(a0, a1)))
}

fn door_num_chladni_ladder(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::num::chladni::ladder(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_lines(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::chladni::lines(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_plate(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i32 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::num::chladni::plate(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_chladni_shake(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<f32> = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: u64 = take!(name, 5, &args[5]);
    match mrly::num::chladni::shake(&a0, a1, &a2, a3, a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_factor_aliquot(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::aliquot(a0)))
}

fn door_num_factor_coprime(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::factor::coprime(a0, a1)))
}

fn door_num_factor_divisors(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::divisors(a0)))
}

fn door_num_factor_factorial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::num::factor::factorial(a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_factor_factorize(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::factorize(a0)))
}

fn door_num_factor_factorize_wide(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::factorize_wide(a0)))
}

fn door_num_factor_gcd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: u128 = big(name, 1, &args[1])?;
    Ok(Value::String(mrly::num::factor::gcd(a0, a1).to_string()))
}

fn door_num_factor_lcm(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::factor::lcm(a0, a1)))
}

fn door_num_factor_mobius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::mobius(a0)))
}

fn door_num_factor_mobius_sieve(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::mobius_sieve(a0)))
}

fn door_num_factor_radical(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::radical(a0)))
}

fn door_num_factor_reduce(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: u128 = big(name, 1, &args[1])?;
    Ok({
        let parts0 = mrly::num::factor::reduce(a0, a1);
        Value::Array(vec![
            Value::String(parts0.0.to_string()),
            Value::String(parts0.1.to_string()),
        ])
    })
}

fn door_num_factor_sigma(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    Ok(Value::String(mrly::num::factor::sigma(a0, a1).to_string()))
}

fn door_num_factor_squarefree(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::squarefree(a0)))
}

fn door_num_factor_totient(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::totient(a0)))
}

fn door_num_factor_totients(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::factor::totients(a0)))
}

fn door_num_factor_twisted(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<i8> = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::factor::twisted(a0, &a1)))
}

fn door_num_fft_convolve(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::num::fft::convolve(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_convolve_with(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    let a2: Vec<f64> = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    match mrly::num::fft::convolve_with(&a0, &a1, &a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_embed_kernel(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::num::fft::embed_kernel(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_log_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::fft::log_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_magnitude_spectrum(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::fft::magnitude_spectrum(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_peak_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::fft::peak_ring(&a0)))
}

fn door_num_fft_peak_wavelength(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::fft::peak_wavelength(&a0, a1)))
}

fn door_num_fft_radial_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::fft::radial_profile(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_fft_transform(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::fft::transform(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_gauss_class_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Class = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Class::prime(a0)))
}

fn door_num_gauss_class_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Class = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Class::word(a0)))
}

fn door_num_gauss_ring_associates(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::associates(a0, a1, a2)))
}

fn door_num_gauss_ring_canon(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::canon(a0, a1, a2)))
}

fn door_num_gauss_ring_conjugate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::conjugate(a0, a1, a2)))
}

fn door_num_gauss_ring_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::gauss::Ring::count(a0, a1)))
}

fn door_num_gauss_ring_div_rem(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::div_rem(a0, a1, a2)))
}

fn door_num_gauss_ring_fate(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::gauss::Ring::fate(a0, a1)))
}

fn door_num_gauss_ring_gaussian_gcd(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::gaussian_gcd(a0, a1, a2)))
}

fn door_num_gauss_ring_inert(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::gauss::Ring::inert(a0, a1)))
}

fn door_num_gauss_ring_mul(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: (i64, i64) = take!(name, 1, &args[1]);
    let a2: (i64, i64) = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::mul(a0, a1, a2)))
}

fn door_num_gauss_ring_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Ring::named(a0.as_str())))
}

fn door_num_gauss_ring_nearest(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::nearest(a0, a1, a2)))
}

fn door_num_gauss_ring_norm(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::norm(a0, a1, a2)))
}

fn door_num_gauss_ring_place(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::place(a0, a1, a2)))
}

fn door_num_gauss_ring_ramified(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Ring::ramified(a0)))
}

fn door_num_gauss_ring_reach(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::reach(a0, a1, a2)))
}

fn door_num_gauss_ring_symmetry(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Ring::symmetry(a0)))
}

fn door_num_gauss_ring_top(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::gauss::Ring::top(a0, a1)))
}

fn door_num_gauss_ring_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::turn(a0, a1, a2)))
}

fn door_num_gauss_ring_units(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Ring::units(a0)))
}

fn door_num_gauss_ring_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Ring::whole(a0, a1, a2)))
}

fn door_num_gauss_window_census(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Window::census(&a0)))
}

fn door_num_gauss_window_class(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Window = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Window::class(&a0, a1, a2)))
}

fn door_num_gauss_window_holds(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::gauss::Window = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::gauss::Window::holds(&a0, a1, a2)))
}

fn door_num_gauss_window_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::gauss::Window::new(a0, a1)))
}

fn door_num_gauss_window_points(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Window::points(&a0)))
}

fn door_num_gauss_window_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Window::radius(&a0)))
}

fn door_num_gauss_window_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::gauss::Window = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::gauss::Window::ring(&a0)))
}

fn door_num_gauss_classes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::gauss::classes(a0, a1)))
}

fn door_num_gauss_peak(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::gauss::peak(a0, a1)))
}

fn door_num_gauss_shells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::gauss::Ring = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::gauss::shells(a0, a1)))
}

fn door_num_kummer_carry(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly::num::kummer::carry(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_columns(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly::num::kummer::columns(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u32 = take!(name, 3, &args[3]);
    match mrly::num::kummer::count(a0, a1, a2, a3) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_member(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly::num::kummer::member(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_valuation(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    let a3: u64 = take!(name, 3, &args[3]);
    match mrly::num::kummer::valuation(a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_kummer_witness(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    match mrly::num::kummer::witness(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_coprime_pairs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::lattice::coprime_pairs(a0)))
}

fn door_num_lattice_farey(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::lattice::farey(a0)))
}

fn door_num_lattice_grid(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::lattice::grid(a0)))
}

fn door_num_lattice_new_nodes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::lattice::new_nodes(a0)))
}

fn door_num_lattice_pi_estimate(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::lattice::pi_estimate(a0)))
}

fn door_num_lattice_recovered(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::num::lattice::recovered(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_visible_density(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly::num::lattice::visible_density(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_lattice_zeta_factor(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::lattice::zeta_factor(a0)))
}

fn door_num_lattice_zeta_whole(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u32 = take!(name, 0, &args[0]);
    match mrly::num::lattice::zeta_whole(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_lift_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::morse::Lift::all()))
}

fn door_num_morse_lift_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::morse::Lift = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: u64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::morse::Lift::at(a0, a1, a2)))
}

fn door_num_morse_lift_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::morse::Lift = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::Lift::formula(a0)))
}

fn door_num_morse_boundary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::boundary(&a0)))
}

fn door_num_morse_difference(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::morse::difference(&a0, &a1)))
}

fn door_num_morse_digits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::digits(a0)))
}

fn door_num_morse_doubling(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::doubling(a0)))
}

fn door_num_morse_faults(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::morse::faults(&a0, &a1)))
}

fn door_num_morse_fold(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::num::morse::fold(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_letter(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::letter(a0)))
}

fn door_num_morse_lift(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::morse::Lift = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::morse::lift(a0, a1)))
}

fn door_num_morse_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<u8> = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::morse::mask(&a0, &a1)))
}

fn door_num_morse_power(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::num::morse::power(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_morse_repeat(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::morse::repeat(&a0, a1, a2)))
}

fn door_num_morse_runs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::runs(&a0)))
}

fn door_num_morse_signs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::signs(&a0)))
}

fn door_num_morse_stage(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::stage(a0)))
}

fn door_num_morse_substitution(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::morse::substitution(a0)))
}

fn door_num_morse_upsample(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::morse::upsample(&a0, a1, a2)))
}

fn door_num_prime_sieve_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::Sieve::count(&a0)))
}

fn door_num_prime_sieve_done(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::Sieve::done(&a0)))
}

fn door_num_prime_sieve_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::Sieve::new(a0)))
}

fn door_num_prime_sieve_rank(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::Sieve::rank(&a0)))
}

fn door_num_prime_sieve_struck(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::Sieve::struck(&a0)))
}

fn door_num_prime_sieve_types(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::prime::Sieve = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::Sieve::types(&a0)))
}

fn door_num_prime_chart(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::prime::chart(a0, a1)))
}

fn door_num_prime_flags(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::flags(a0)))
}

fn door_num_prime_goldbach(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::goldbach(a0)))
}

fn door_num_prime_goldbach_record(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::goldbach_record(a0)))
}

fn door_num_prime_is_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::is_prime(a0)))
}

fn door_num_prime_pile(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::pile(a0)))
}

fn door_num_prime_prime_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::prime_count(a0)))
}

fn door_num_prime_prime_from(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::prime_from(a0)))
}

fn door_num_prime_primes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::primes(a0)))
}

fn door_num_prime_rectangles(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::rectangles(a0)))
}

fn door_num_prime_splits(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::splits(a0)))
}

fn door_num_prime_squares(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::squares(a0)))
}

fn door_num_prime_study(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::prime::study(a0)))
}

fn door_num_series_basel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::basel(a0)))
}

fn door_num_series_bernoulli(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::num::series::bernoulli(a0) {
        Ok(value) => Ok({
            let mut list0 = Vec::new();
            for item0 in value {
                list0.push({
                    let parts1 = item0;
                    Value::Array(vec![
                        Value::String(parts1.0.to_string()),
                        Value::String(parts1.1.to_string()),
                    ])
                });
            }
            Value::Array(list0)
        }),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_beta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::series::beta(a0, a1)))
}

fn door_num_series_binary(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::binary(a0)))
}

fn door_num_series_catalan(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::catalan(a0)))
}

fn door_num_series_chi3(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::chi3(a0)))
}

fn door_num_series_chi4(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::chi4(a0)))
}

fn door_num_series_chi8(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::chi8(a0)))
}

fn door_num_series_dirichlet(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<i8> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::series::dirichlet(a0, &a1, a2)))
}

fn door_num_series_e_partial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::e_partial(a0)))
}

fn door_num_series_euler_gamma_partial(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::euler_gamma_partial(a0)))
}

fn door_num_series_euler_product(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::series::euler_product(a0, a1)))
}

fn door_num_series_evens(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::evens(a0)))
}

fn door_num_series_fibonacci(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::fibonacci(a0)))
}

fn door_num_series_harmonic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::harmonic(a0)))
}

fn door_num_series_lambda(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::series::lambda(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_leibniz(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::leibniz(a0)))
}

fn door_num_series_li(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::li(a0)))
}

fn door_num_series_mertens(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::mertens(a0)))
}

fn door_num_series_odds(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::odds(a0)))
}

fn door_num_series_visible(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::num::series::visible(a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_series_wallis_half_pi(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::wallis_half_pi(a0)))
}

fn door_num_series_wallis_quarter_pi(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::series::wallis_quarter_pi(a0)))
}

fn door_num_series_zeta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::series::zeta(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_cells(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::num::sieve::cells(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_exponent(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::num::sieve::exponent(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_flat_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::sieve::flat_word(a0, a1)))
}

fn door_num_sieve_holes(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::num::sieve::holes(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::num::sieve::limit(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_odd_word(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::sieve::odd_word(a0)))
}

fn door_num_sieve_parity_product(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u32 = take!(name, 0, &args[0]);
    let a1: bool = take!(name, 1, &args[1]);
    match mrly::num::sieve::parity_product(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_punctures(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::num::sieve::punctures(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_raster(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly::num::sieve::raster(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_ratio(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u32 = take!(name, 1, &args[1]);
    match mrly::num::sieve::ratio(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_fill(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    match mrly::num::sieve::row_fill(&a0, a1) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_law(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly::num::sieve::row_law(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_profile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u128 = big(name, 0, &args[0])?;
    let a1: usize = take!(name, 1, &args[1]);
    match mrly::num::sieve::row_profile(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_row_settle(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly::num::sieve::row_settle(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_side(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    match mrly::num::sieve::side(&a0) {
        Ok(value) => Ok(Value::String(value.to_string())),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_num_sieve_solid_limit(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::sieve::solid_limit()))
}

fn door_num_spiral_growth_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::spiral::Growth::all()))
}

fn door_num_spiral_lattice_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::spiral::Lattice::all()))
}

fn door_num_spiral_lattice_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::spiral::Lattice::count(a0, a1)))
}

fn door_num_spiral_lattice_n(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::spiral::Lattice::n(a0, a1, a2)))
}

fn door_num_spiral_lattice_radius(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::spiral::Lattice::radius(a0, a1)))
}

fn door_num_spiral_lattice_ring(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::spiral::Lattice::ring(a0, a1)))
}

fn door_num_spiral_lattice_ring_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::spiral::Lattice::ring_of(a0, a1, a2)))
}

fn door_num_spiral_lattice_xy(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::spiral::Lattice::xy(a0, a1)))
}

fn door_num_spiral_mark_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::spiral::Mark::all()))
}

fn door_num_spiral_diagonal(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly::num::spiral::Lattice = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    let a3: i64 = take!(name, 3, &args[3]);
    let a4: i64 = take!(name, 4, &args[4]);
    Ok(give!(mrly::num::spiral::diagonal(a0, a1, a2, a3, a4)))
}

fn door_num_spiral_level_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::spiral::level_of(a0, a1)))
}

fn door_num_spiral_marks(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::spiral::Mark = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::spiral::marks(a0, a1)))
}

fn door_num_spiral_snail(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: u64 = take!(name, 1, &args[1]);
    let a2: mrly::num::spiral::Growth = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::spiral::snail(a0, a1, a2)))
}

fn door_num_zeta_complex_abs(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::Complex::abs(a0)))
}

fn door_num_zeta_complex_arg(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::Complex::arg(a0)))
}

fn door_num_zeta_complex_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::zeta::Complex::default()))
}

fn door_num_zeta_complex_exp(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::Complex::exp(a0)))
}

fn door_num_zeta_complex_ln(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::Complex::ln(a0)))
}

fn door_num_zeta_complex_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Complex::new(a0, a1)))
}

fn door_num_zeta_complex_turn(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::Complex::turn(a0)))
}

fn door_num_zeta_line_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::count(&a0, a1)))
}

fn door_num_zeta_line_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::zeta::Line::default()))
}

fn door_num_zeta_line_exact(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::exact(&a0, a1)))
}

fn door_num_zeta_line_gram(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::gram(&a0, a1)))
}

fn door_num_zeta_line_maclaurin(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::maclaurin(&a0, a1)))
}

fn door_num_zeta_line_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::zeta::Line::new()))
}

fn door_num_zeta_line_novelty_coefficients(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::novelty_coefficients(&a0, &a1)))
}

fn door_num_zeta_line_pair(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: mrly::num::zeta::Complex = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::pair(&a0, a1)))
}

fn door_num_zeta_line_point(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::point(&a0, a1)))
}

fn door_num_zeta_line_seam(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly::num::zeta::Line::seam(&a0, a1, a2, a3)))
}

fn door_num_zeta_line_siegel(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::siegel(&a0, a1)))
}

fn door_num_zeta_line_theta(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::theta(&a0, a1)))
}

fn door_num_zeta_line_z(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::z(&a0, a1)))
}

fn door_num_zeta_line_zeros(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::num::zeta::Line = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::Line::zeros(&a0, a1)))
}

fn door_num_zeta_bump(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::bump(a0)))
}

fn door_num_zeta_corrections(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::corrections(a0)))
}

fn door_num_zeta_kernel(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: f64 = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::kernel(a0)))
}

fn door_num_zeta_mellin(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::num::zeta::Complex = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::mellin(a0)))
}

fn door_num_zeta_novelty_main(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::num::zeta::novelty_main()))
}

fn door_num_zeta_novelty_wave(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<mrly::num::zeta::Complex> = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::zeta::novelty_wave(&a0, &a1, a2)))
}

fn door_num_zeta_psi_curve(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<f64> = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::psi_curve(&a0, &a1)))
}

fn door_num_zeta_psi_formula(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::psi_formula(a0, &a1)))
}

fn door_num_zeta_psi_gap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: Vec<f64> = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::psi_gap(a0, &a1)))
}

fn door_num_zeta_psi_stair(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::num::zeta::psi_stair(a0)))
}

fn door_num_zeta_raise(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: f64 = take!(name, 0, &args[0]);
    let a1: mrly::num::zeta::Complex = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::raise(a0, a1)))
}

fn door_num_zeta_sharp_novelty(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::num::zeta::sharp_novelty(&a0, a1)))
}

fn door_num_zeta_smoothed_novelty(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<u64> = take!(name, 0, &args[0]);
    let a1: f64 = take!(name, 1, &args[1]);
    let a2: f64 = take!(name, 2, &args[2]);
    Ok(give!(mrly::num::zeta::smoothed_novelty(&a0, a1, a2)))
}

fn door_tile_group_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::tile::Group::all()))
}

fn door_tile_parity_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::tile::Parity::all()))
}

fn door_tile_parity_keep(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::tile::Parity = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::tile::Parity::keep(a0, a1)))
}

fn door_tile_tile_check(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::Tile = take!(name, 0, &args[0]);
    match mrly::tile::Tile::check(&a0) {
        Ok(()) => Ok(Value::Null),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_tile_degenerate(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::Tile = take!(name, 0, &args[0]);
    Ok(give!(mrly::tile::Tile::degenerate(&a0)))
}

fn door_tile_tile_max_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::Tile = take!(name, 0, &args[0]);
    Ok(give!(mrly::tile::Tile::max_size(&a0)))
}

fn door_tile_tile_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::Group = take!(name, 0, &args[0]);
    Ok(give!(mrly::tile::Tile::new(a0)))
}

fn door_tile_tile_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::tile::Tile = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly::tile::Tile::size(a0, a1, a2)))
}

fn door_tile_background(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u64 = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly::tile::background(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_config2d_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::tile::build::Config2d::default()))
}

fn door_tile_build_config3d_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::tile::build::Config3d::default()))
}

fn door_tile_build_build_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::Tile = take!(name, 0, &args[0]);
    match mrly::tile::build::build_2d(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_build_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::Tile = take!(name, 0, &args[0]);
    match mrly::tile::build::build_3d(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_build_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::build::HexTile = take!(name, 0, &args[0]);
    match mrly::tile::build::build_6d(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_create_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    match mrly::tile::build::create_2d(&a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_create_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    match mrly::tile::build::create_3d(&a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_create_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0 = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    match mrly::tile::build::create_6d(&a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_random_tile_2d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    match mrly::tile::build::random_tile_2d(a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_random_tile_3d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    match mrly::tile::build::random_tile_3d(a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_build_random_tile_6d(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    match mrly::tile::build::random_tile_6d(a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_classic_code(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::recipe::Design = take!(name, 0, &args[0]);
    Ok(match mrly::tile::classic_code(a0) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_tile_classic_code_nd(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::tile::recipe::Design = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(match mrly::tile::classic_code_nd(a0, a1) {
        Some(item0) => Value::String(item0.to_string()),
        None => Value::Null,
    })
}

fn door_tile_hex_key(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrly::tile::hex_key(a0, &mut a1)))
}

fn door_tile_name_slots_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::tile::name::Slots::default()))
}

fn door_tile_name_tile_checked(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::name::Tile = take!(name, 0, &args[0]);
    match <mrly::tile::name::Tile as mrly::core::name::Named>::checked(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_from_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::tile::name::Tile as mrly::core::name::Named>::from_file(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_from_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::tile::name::Tile as mrly::core::name::Named>::from_json(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_from_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match <mrly::tile::name::Tile as mrly::core::name::Named>::from_url(a0.as_str()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::Tile = take!(name, 0, &args[0]);
    match mrly::tile::name::Tile::of(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_recipe(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::name::Tile = take!(name, 0, &args[0]);
    match mrly::tile::name::Tile::recipe(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_to_file(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::name::Tile = take!(name, 0, &args[0]);
    match <mrly::tile::name::Tile as mrly::core::name::Named>::to_file(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_to_id(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::name::Tile = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::tile::name::Tile as mrly::core::name::Named>::to_id(&a0)
    ))
}

fn door_tile_name_tile_to_json(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::name::Tile = take!(name, 0, &args[0]);
    Ok(give!(
        <mrly::tile::name::Tile as mrly::core::name::Named>::to_json(&a0)
    ))
}

fn door_tile_name_tile_to_prose(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::name::Tile = take!(name, 0, &args[0]);
    match <mrly::tile::name::Tile as mrly::core::name::Named>::to_prose(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_name_tile_to_url(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::name::Tile = take!(name, 0, &args[0]);
    match <mrly::tile::name::Tile as mrly::core::name::Named>::to_url(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_random_design(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrly::core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrly::tile::random_design(&mut a0)))
}

fn door_tile_random_rotation(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::tile::recipe::Design = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrly::tile::random_rotation(a0, &mut a1)))
}

fn door_tile_recipe_design_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::tile::recipe::Design::all()))
}

fn door_tile_recipe_classics(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly::tile::recipe::classics(a0)))
}

fn door_tile_recipe_generals(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly::tile::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrly::tile::recipe::generals(a0, a1, a2)))
}

fn door_tile_recipe_nestings(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly::tile::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrly::tile::recipe::nestings(a0, a1, a2)))
}

fn door_tile_recipe_powers(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: mrly::tile::Parity = take!(name, 2, &args[2]);
    Ok(give!(mrly::tile::recipe::powers(a0, a1, a2)))
}

fn door_tile_recipe_products(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrly::tile::Parity = take!(name, 3, &args[3]);
    Ok(give!(mrly::tile::recipe::products(a0, a1, a2, a3)))
}

fn door_tile_recipe_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: i64 = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    Ok(give!(mrly::tile::recipe::size(a0, a1)))
}

fn door_tile_tree_mask(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    match mrly::tile::tree_mask(a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_variation_config_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly::tile::variation::Config::default()))
}

fn door_tile_variation_file_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly::tile::variation::File::new(a0, a1)))
}

fn door_tile_variation_variation_is_cover(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::variation::Variation = take!(name, 0, &args[0]);
    Ok(give!(mrly::tile::variation::Variation::is_cover(&a0)))
}

fn door_tile_variation_variation_is_prime(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly::tile::variation::Variation = take!(name, 0, &args[0]);
    Ok(give!(mrly::tile::variation::Variation::is_prime(&a0)))
}

fn door_tile_variation_create(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly::tile::variation::Config = take!(name, 0, &args[0]);
    let mut a1 = mrly::core::Rng::new(seed(name, 1, &args[1])?);
    match mrly::tile::variation::create(&a0, &mut a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_variation_generate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::tile::variation::Variation = take!(name, 0, &args[0]);
    let a1: mrly::tile::variation::Config = take!(name, 1, &args[1]);
    let mut a2 = mrly::core::Rng::new(seed(name, 2, &args[2])?);
    match mrly::tile::variation::generate(a0, &a1, &mut a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_tile_variation_render(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly::tile::variation::Variation = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let mut a2 = mrly::core::Rng::new(seed(name, 2, &args[2])?);
    match mrly::tile::variation::render(a0, a1, &mut a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}
