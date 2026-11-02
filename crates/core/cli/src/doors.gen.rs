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
];

fn door_core_colorizer_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::Colorizer::default()))
}

fn door_core_colorizer_diverge(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::Colorizer::diverge()))
}

fn door_core_colorizer_fire(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::Colorizer::fire()))
}

fn door_core_colorizer_gradient_bins(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Color = color(name, 0, &args[0])?;
    let a1: Vec<mrly_core::Color> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(color(name, 1, item0)?);
        }
        list0
    };
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_core::Colorizer::gradient_bins(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colorizer_heat(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::Colorizer::heat()))
}

fn door_core_dtype_max(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Dtype = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Dtype::max(a0)))
}

fn door_core_cell_anti(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Cell::anti(a0)))
}

fn door_core_cell_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Cell::binarize(a0, a1)))
}

fn door_core_cell_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Cell::binarize_otsu(a0)))
}

fn door_core_cell_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_core::Cell::blur(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_color_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Cell::color_at(&a0, a1)))
}

fn door_core_cell_combine(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: mrly_core::Cell = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Cell::combine(&a0, &a1)))
}

fn door_core_cell_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_core::Cell::fractal(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Cell::invert(a0)))
}

fn door_core_cell_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: mrly_core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Cell::layers(a0, a1)))
}

fn door_core_cell_magic(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<mrly_core::Cell> = take!(name, 0, &args[0]);
    match mrly_core::cell::magic(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_mapping(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok({
        let mut pairs0 = serde_json::Map::new();
        for (key0, item0) in mrly_core::cell::mapping() {
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
    let a0: Vec<mrly_core::Cell> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::cell::merge(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_moore(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: usize = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::cell::moore(a0)))
}

fn door_core_cell_mosaic(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<mrly_core::Cell> = take!(name, 1, &args[1]);
    match mrly_core::cell::mosaic(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_neighbors(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: mrly_core::Dtype = take!(name, 4, &args[4]);
    match mrly_core::Cell::neighbors(a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Cell::new(a0)))
}

fn door_core_cell_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(give!(mrly_core::Cell::pad(a0, a1, a2)))
}

fn door_core_cell_paint(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1 = {
        let mut pairs0: Vec<(u8, Vec<mrly_core::Color>)> = Vec::new();
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
    let a2: mrly_core::Mode = take!(name, 2, &args[2]);
    let mut a3 = if args[3].is_null() {
        None
    } else {
        Some(mrly_core::Rng::new(seed(name, 3, &args[3])?))
    };
    match mrly_core::Cell::paint(a0, &a1, a2, a3.as_mut()) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly_core::Cell::perforate(a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_remap(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: Vec<usize> = take!(name, 2, &args[2]);
    match mrly_core::cell::remap(&a0, &a1, &a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_rgba(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Cell::rgba(&a0)))
}

fn door_core_cell_rot90_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly_core::cell::rot90_map(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_rotate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly_core::Cell::rotate(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_shape(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Cell::shape(&a0)))
}

fn door_core_cell_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Cell::size(&a0)))
}

fn door_core_cell_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Cell = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::Cell::tile(a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_cell_tile_map(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::cell::tile_map(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_theme_hues(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::colors::Theme = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_core::colors::Theme::hues(&a0) {
            list0.push(tint(item0));
        }
        Value::Array(list0)
    })
}

fn door_core_colors_theme_inks(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::colors::Theme = take!(name, 0, &args[0]);
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_core::colors::Theme::inks(&a0) {
            list0.push(tint(item0));
        }
        Value::Array(list0)
    })
}

fn door_core_colors_alpha(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Color = color(name, 0, &args[0])?;
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(tint(mrly_core::Color::alpha(&a0, a1)))
}

fn door_core_colors_board(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: bool = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::colors::board(a0)))
}

fn door_core_colors_css(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Color = color(name, 0, &args[0])?;
    Ok(give!(mrly_core::Color::css(&a0)))
}

fn door_core_colors_from_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_core::Color::from_hex(a0.as_str()) {
        Ok(value) => Ok(tint(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_gradient(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<mrly_core::Color> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 0, &args[0])? {
            list0.push(color(name, 0, item0)?);
        }
        list0
    };
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_core::colors::gradient(&a0, a1) {
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
    Ok(give!(mrly_core::colors::ink(a0)))
}

fn door_core_colors_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Color = color(name, 0, &args[0])?;
    Ok(tint(mrly_core::Color::invert(&a0)))
}

fn door_core_colors_lightness(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Color = color(name, 0, &args[0])?;
    let a1: u8 = take!(name, 1, &args[1]);
    match mrly_core::Color::lightness(&a0, a1) {
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
    Ok(give!(mrly_core::colors::luma_types(&a0, a1, a2, a3)))
}

fn door_core_colors_mix(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Color = color(name, 0, &args[0])?;
    let a1: mrly_core::Color = color(name, 1, &args[1])?;
    let a2: f64 = take!(name, 2, &args[2]);
    match mrly_core::colors::mix(a0, a1, a2) {
        Ok(value) => Ok(tint(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_named(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_core::colors::named(a0.as_str()) {
        Ok(value) => Ok(tint(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_colors_random(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: bool = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    Ok(tint(mrly_core::Color::random(a0, &mut a1)))
}

fn door_core_colors_rgb(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(tint(mrly_core::Color::rgb(a0, a1, a2)))
}

fn door_core_colors_rgba(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: u8 = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: u8 = take!(name, 3, &args[3]);
    Ok(tint(mrly_core::Color::rgba(a0, a1, a2, a3)))
}

fn door_core_colors_shades(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Color = color(name, 0, &args[0])?;
    Ok({
        let mut list0 = Vec::new();
        for item0 in mrly_core::colors::shades(a0) {
            list0.push(tint(item0));
        }
        Value::Array(list0)
    })
}

fn door_core_colors_snap(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: Vec<mrly_core::Color> = {
        let mut list0 = Vec::new();
        for item0 in items(name, 1, &args[1])? {
            list0.push(color(name, 1, item0)?);
        }
        list0
    };
    Ok(give!(mrly_core::colors::snap(&a0, &a1)))
}

fn door_core_colors_to_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Color = color(name, 0, &args[0])?;
    Ok(give!(mrly_core::Color::to_hex(&a0)))
}

fn door_core_error_parse(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: String = take!(name, 0, &args[0]);
    match mrly_core::error::parse(a0.as_str()) {
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
    let a4: mrly_core::Filter = take!(name, 4, &args[4]);
    match mrly_core::hex_fit(&a0, a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_hex_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    Ok(give!(mrly_core::hex_size(a0, a1, a2)))
}

fn door_core_image_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    Ok(give!(mrly_core::image::blur(&a0, a1, a2, a3)))
}

fn door_core_image_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: usize = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: Vec<[u8; 4]> = take!(name, 2, &args[2]);
    match mrly_core::Image::new(a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_image_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 4)?;
    let a0: mrly_core::Image = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: mrly_core::Filter = take!(name, 3, &args[3]);
    match mrly_core::Image::resample(&a0, a1, a2, a3) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_image_scale(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Image = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_core::Image::scale(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_name_sha_hex(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::name::sha::hex(&a0)))
}

fn door_core_name_sha_sha256(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::name::sha::sha256(&a0)))
}

fn door_core_name_sha_short(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::name::sha::short(&a0)))
}

fn door_core_paint_config_default(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::paint::Config::default()))
}

fn door_core_paint_edition_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::paint::Edition::all()))
}

fn door_core_paint_edition_mode(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::paint::Edition = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::paint::Edition::mode(a0)))
}

fn door_core_paint_ink_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::paint::Ink::all()))
}

fn door_core_paint_ink_color(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::paint::Ink = take!(name, 0, &args[0]);
    Ok(tint(mrly_core::paint::Ink::color(a0)))
}

fn door_core_paint_paint_is_simple(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::paint::Paint = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::paint::Paint::is_simple(&a0)))
}

fn door_core_paint_paint_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::paint::Edition = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::paint::Paint::new(a0)))
}

fn door_core_paint_scheme_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::paint::Scheme::all()))
}

fn door_core_paint_target_all(name: &str, args: &[Value]) -> Done {
    count(name, args, 0)?;
    Ok(give!(mrly_core::paint::Target::all()))
}

fn door_core_paint_random_edition(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Option<Vec<mrly_core::paint::Edition>> = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrly_core::paint::random_edition(
        a0.as_deref(),
        &mut a1
    )))
}

fn door_core_paint_reroll(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::paint::Paint = take!(name, 0, &args[0]);
    let mut a1 = mrly_core::Rng::new(seed(name, 1, &args[1])?);
    Ok(give!(mrly_core::paint::reroll(a0, &mut a1)))
}

fn door_core_paint_setup(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::paint::Paint = take!(name, 0, &args[0]);
    let a1: mrly_core::paint::Config = take!(name, 1, &args[1]);
    let mut a2 = mrly_core::Rng::new(seed(name, 2, &args[2])?);
    Ok(give!(mrly_core::paint::setup(a0, &a1, &mut a2)))
}

fn door_core_ramp_color(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Colorizer = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(tint(mrly_core::ramp::color(&a0, a1, a2)))
}

fn door_core_ramp_colors(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Colorizer = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_core::ramp::colors(&a0, &a1, a2)))
}

fn door_core_resample(name: &str, args: &[Value]) -> Done {
    count(name, args, 6)?;
    let a0: Vec<[u8; 4]> = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    let a3: usize = take!(name, 3, &args[3]);
    let a4: usize = take!(name, 4, &args[4]);
    let a5: mrly_core::Filter = take!(name, 5, &args[5]);
    match mrly_core::resample(&a0, a1, a2, a3, a4, a5) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_rng_below(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let mut a0 = mrly_core::Rng::new(seed(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Rng::below(&mut a0, a1)))
}

fn door_core_rng_boolean(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrly_core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrly_core::Rng::boolean(&mut a0)))
}

fn door_core_rng_chance(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let mut a0 = mrly_core::Rng::new(seed(name, 0, &args[0])?);
    let a1: f64 = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Rng::chance(&mut a0, a1)))
}

fn door_core_rng_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: u64 = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Rng::new(a0)))
}

fn door_core_rng_range(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let mut a0 = mrly_core::Rng::new(seed(name, 0, &args[0])?);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: i64 = take!(name, 2, &args[2]);
    Ok(give!(mrly_core::Rng::range(&mut a0, a1, a2)))
}

fn door_core_rng_sample_indices(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let mut a0 = mrly_core::Rng::new(seed(name, 0, &args[0])?);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    Ok(give!(mrly_core::Rng::sample_indices(&mut a0, a1, a2)))
}

fn door_core_rng_unit(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let mut a0 = mrly_core::Rng::new(seed(name, 0, &args[0])?);
    Ok(give!(mrly_core::Rng::unit(&mut a0)))
}

fn door_core_tensor_at(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::at(&a0, a1)))
}

fn door_core_tensor_binarize(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::binarize(&a0, a1)))
}

fn door_core_tensor_binarize_otsu(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Tensor::binarize_otsu(&a0)))
}

fn door_core_tensor_blur(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: bool = take!(name, 2, &args[2]);
    match mrly_core::Tensor::blur(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_bytes(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_core::Tensor::bytes(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_count(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::count(&a0, a1)))
}

fn door_core_tensor_dtype(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Tensor::dtype(&a0)))
}

fn door_core_tensor_exposed(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(Value::String(mrly_core::Tensor::exposed(&a0).to_string()))
}

fn door_core_tensor_filled(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: i64 = take!(name, 1, &args[1]);
    let a2: mrly_core::Dtype = take!(name, 2, &args[2]);
    Ok(give!(mrly_core::Tensor::filled(a0, a1, a2)))
}

fn door_core_tensor_flip(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    match mrly_core::Tensor::flip(&a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_fractal(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::fractal(&a0, a1)))
}

fn door_core_tensor_full(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: u8 = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::full(a0, a1)))
}

fn door_core_tensor_get(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::Tensor::get(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_i32(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<i32> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::Tensor::i32(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_i32s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_core::Tensor::i32s(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_index(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::index(&a0, &a1)))
}

fn door_core_tensor_invert(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Tensor::invert(&a0)))
}

fn door_core_tensor_kron(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::kron(&a0, &a1)))
}

fn door_core_tensor_layers(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::layers(&a0, a1)))
}

fn door_core_tensor_neighbors(name: &str, args: &[Value]) -> Done {
    count(name, args, 5)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    let a3: bool = take!(name, 3, &args[3]);
    let a4: mrly_core::Dtype = take!(name, 4, &args[4]);
    match mrly_core::Tensor::neighbors(&a0, &a1, a2, a3, a4) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_new(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Tensor::new(a0)))
}

fn door_core_tensor_of(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::Tensor::of(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_otsu_threshold(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Tensor::otsu_threshold(&a0)))
}

fn door_core_tensor_pad(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    Ok(give!(mrly_core::Tensor::pad(&a0, a1, a2)))
}

fn door_core_tensor_perforate(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: mrly_core::Tensor = take!(name, 1, &args[1]);
    let a2: u8 = take!(name, 2, &args[2]);
    match mrly_core::Tensor::perforate(&a0, &a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_rot90(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: (usize, usize) = take!(name, 2, &args[2]);
    match mrly_core::Tensor::rot90(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_size(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Tensor::size(&a0)))
}

fn door_core_tensor_slice(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_core::Tensor::slice(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_sum(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    Ok(give!(mrly_core::Tensor::sum(&a0)))
}

fn door_core_tensor_tile(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::Tensor::tile(&a0, &a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_transpose(name: &str, args: &[Value]) -> Done {
    count(name, args, 3)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    let a1: usize = take!(name, 1, &args[1]);
    let a2: usize = take!(name, 2, &args[2]);
    match mrly_core::Tensor::transpose(&a0, a1, a2) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_typed(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<usize> = take!(name, 0, &args[0]);
    let a1: mrly_core::Dtype = take!(name, 1, &args[1]);
    Ok(give!(mrly_core::Tensor::typed(a0, a1)))
}

fn door_core_tensor_u16(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u16> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::Tensor::u16(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_u16s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_core::Tensor::u16s(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_u32(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u32> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::Tensor::u32(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_u32s(name: &str, args: &[Value]) -> Done {
    count(name, args, 1)?;
    let a0: mrly_core::Tensor = take!(name, 0, &args[0]);
    match mrly_core::Tensor::u32s(&a0) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}

fn door_core_tensor_u8(name: &str, args: &[Value]) -> Done {
    count(name, args, 2)?;
    let a0: Vec<u8> = take!(name, 0, &args[0]);
    let a1: Vec<usize> = take!(name, 1, &args[1]);
    match mrly_core::Tensor::u8(a0, a1) {
        Ok(value) => Ok(give!(value)),
        Err(error) => Err(Error::Library(error.to_string())),
    }
}
