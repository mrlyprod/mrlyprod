use mrlyrs::core::rng::Rng;
use mrlyrs::core::tensor::Tensor;
use mrlyrs::core::{png, Color, Colorizer};
use mrlyrs::font;
use mrlyrs::gen;
use mrlyrs::life::{self, Config, Source};
use mrlyrs::math::bang::Code;
use mrlyrs::math::graph::Network;
use mrlyrs::math::name::Named;
use mrlyrs::math::two::Cell2d;
use mrlyrs::math::{atoms, bang, counts, spectrum, three, two};
use mrlyrs::num::{factor, prime, series};
use serde_json::{json, Map, Value};

#[path = "sha256.rs"]
pub mod sha256;

pub const MODULES: [&str; 6] = ["core", "num", "math", "gen", "life", "font"];

const SEED: u64 = 2026;

const CARPET_NAME: &str = r#"{"kind":"tile","code":7,"side":3,"level":2}"#;

pub fn rows(module: &str) -> Vec<Value> {
    match module {
        "core" => core_rows(),
        "num" => num_rows(),
        "math" => math_rows(),
        "gen" => gen_rows(),
        "life" => life_rows(),
        "font" => font_rows(),
        other => panic!("no fixture module {other:?}"),
    }
}

// SHAPES

fn row(path: &str, input: Value, out: Value, note: &str) -> Value {
    let mut map = Map::new();
    map.insert("fn".to_string(), json!(path));
    map.insert("in".to_string(), input);
    map.insert("out".to_string(), out);
    map.insert("note".to_string(), json!(note));
    Value::Object(map)
}

fn tensor(grid: &Tensor) -> Value {
    json!({"shape": grid.shape, "data": grid.bytes().unwrap()})
}

fn cell(flat: &Cell2d) -> Value {
    serde_json::from_str(&two::to_json(flat)).unwrap()
}

fn big(number: u128) -> Value {
    json!(number.to_string())
}

fn f12(value: f64) -> Value {
    json!(format!("{value:.12}").parse::<f64>().unwrap() + 0.0)
}

fn sha(bytes: &[u8]) -> Value {
    json!(sha256::hex(bytes))
}

// CORE

fn core_rows() -> Vec<Value> {
    let source: Vec<u8> = (0..12).collect();
    let turned = Tensor::of(source.clone(), vec![3, 4])
        .unwrap()
        .rot90(1, (0, 1))
        .unwrap();
    let red = Color::from_hex("#ff3d40").unwrap();
    let corners = [
        [255u8, 0, 0, 255],
        [0, 255, 0, 255],
        [0, 0, 255, 255],
        [255, 255, 255, 255],
    ];
    let bytes = png(&corners, 2, 2, 3).unwrap();
    let ramp = Colorizer::gradient_bins(
        Color::from_hex("#000000").unwrap(),
        &[
            Color::from_hex("#ff3d40").unwrap(),
            Color::from_hex("#3d40ff").unwrap(),
        ],
        4,
    )
    .unwrap();
    let swatches: Vec<String> = (0..4)
        .map(|value| mrlyrs::core::ramp::color(&ramp, value, 9).to_hex())
        .collect();
    vec![
        row(
            "core::Tensor::rot90",
            json!({"data": source, "shape": [3, 4], "k": 1, "axes": [0, 1]}),
            tensor(&turned),
            "one quarter turn in the (0, 1) plane, the numpy law",
        ),
        row(
            "core::Color::from_hex",
            json!({"hex": "#ff3d40"}),
            json!({"rgba": [red.r, red.g, red.b, red.a], "hex": red.to_hex()}),
            "the house red parsed, then printed back by to_hex",
        ),
        row(
            "core::png",
            json!({"colors": corners, "width": 2, "height": 2, "scale": 3}),
            sha(&bytes),
            "sha256 of the png bytes, which pins the encoder and the scale",
        ),
        row(
            "core::Colorizer::color",
            json!({
                "background": "#000000",
                "ramp": ["#ff3d40", "#3d40ff"],
                "shades": 4,
                "values": [0, 1, 2, 3],
                "max": 9,
            }),
            json!(swatches),
            "gradient_bins first; four shades against a max of 9 bin every live value at the ramp head",
        ),
    ]
}

// NUM

fn num_rows() -> Vec<Value> {
    let pairs = [(1071u128, 462u128), (1u128 << 100, 1u128 << 60)];
    let named: Vec<Vec<String>> = pairs
        .iter()
        .map(|&(a, b)| vec![a.to_string(), b.to_string()])
        .collect();
    let gcds: Vec<String> = pairs
        .iter()
        .map(|&(a, b)| factor::gcd(a, b).to_string())
        .collect();
    let primes: Vec<usize> = (1..=100).filter(|&n| prime::is_prime(n)).collect();
    vec![
        row(
            "num::factor::factorial",
            json!({"number": 25}),
            big(factor::factorial(25).unwrap()),
            "the u128 crossing: 25! as a decimal string",
        ),
        row(
            "num::factor::gcd",
            json!({"pairs": named}),
            json!(gcds),
            "two calls, (1071, 462) and (2^100, 2^60), u128 in and out as decimal strings",
        ),
        row(
            "num::factor::divisors",
            json!({"number": 360}),
            json!(factor::divisors(360)),
            "the 24 divisors of 360 in ascending order",
        ),
        row(
            "num::factor::mobius_sieve",
            json!({"limit": 30}),
            json!(factor::mobius_sieve(30)),
            "OEIS A008683 with a leading pad: 31 entries, mu(n) at index n",
        ),
        row(
            "num::prime::is_prime",
            json!({"from": 1, "to": 100}),
            json!(primes),
            "the 25 numbers in 1..=100 the test keeps",
        ),
        row(
            "num::series::zeta",
            json!({"s": 2.0, "terms": 1000}),
            f12(series::zeta(2.0, 1000).unwrap()),
            "the f64 crossing: pi squared over six, rounded to 12 decimals",
        ),
    ]
}

// MATH

fn math_rows() -> Vec<Value> {
    let atom = atoms::carpet_2d(3);
    let flat = two::carpet(3, 2).unwrap();
    let survey = two::census::census(&flat).unwrap();
    let filled = counts::fill(Code::from(23u64), 3, 3, 2, 2).unwrap();
    let universe = bang::bang(3).unwrap();
    let cube = three::carpet(3, 1).unwrap();
    let solid = three::census::census(&cube).unwrap();
    let mut network = Network::new(1);
    for node in 0..5 {
        network.add_node(vec![node as f64]).unwrap();
    }
    for branch in 0..4 {
        network.add_branch(branch, branch + 1, 1.0).unwrap();
    }
    let spread = spectrum::laplacian_spectrum(&network, false).unwrap();
    vec![
        row(
            "math::atoms::carpet_2d",
            json!({"n": 3}),
            tensor(&atom),
            "the 3 by 3 seed every carpet design rests on",
        ),
        row(
            "math::two::carpet",
            json!({"number": 3, "level": 2}),
            cell(&flat),
            "the byte-grid crossing: the cell as math::two::to_json writes it",
        ),
        row(
            "math::two::census::census",
            json!({"cell": {"fn": "math::two::carpet", "in": {"number": 3, "level": 2}}}),
            serde_json::to_value(&survey).unwrap(),
            "a struct crossing: the Census through its serde derive",
        ),
        row(
            "math::counts::fill",
            json!({"code": "23", "number": 3, "dimension": 3, "level": 2, "base": 2}),
            big(filled),
            "the sponge's filled cells in closed form, u128 as a decimal string",
        ),
        row(
            "math::bang::bang",
            json!({"dimension": 3}),
            json!(universe.distinct()),
            "Universe::distinct() of the dimension-3 universe",
        ),
        row(
            "math::three::census::census",
            json!({"cell": {"fn": "math::three::carpet", "in": {"number": 3, "level": 1}}}),
            big(solid.surface),
            "the .surface field only, u128 as a decimal string",
        ),
        row(
            "math::spectrum::laplacian_spectrum",
            json!({
                "network": {
                    "dim": 1,
                    "nodes": [[0.0], [1.0], [2.0], [3.0], [4.0]],
                    "branches": [[0, 1, 1.0], [1, 2, 1.0], [2, 3, 1.0], [3, 4, 1.0]],
                },
                "normalised": false,
            }),
            Value::Array(spread.iter().map(|&value| f12(value)).collect()),
            "the five eigenvalues of a 5-node path, ascending, to 12 decimals",
        ),
    ]
}

// GEN

fn gen_rows() -> Vec<Value> {
    let name = gen::name::Tile::from_json(CARPET_NAME).unwrap();
    let recipe = name.recipe().unwrap();
    let folded = gen::name::Tile::of(&recipe).unwrap();
    let built = gen::build::build_2d(&recipe).unwrap();
    let mut drawing = Rng::new(SEED);
    let drawn = gen::draw::create(
        &gen::draw::ConfigNd::<2>::default(),
        |_, rng| rng.below(4),
        &mut drawing,
    )
    .unwrap();
    let mut varying = Rng::new(SEED);
    let made = gen::variation::create(&gen::variation::Config::default(), &mut varying).unwrap();
    vec![
        row(
            "gen::name::Tile::recipe",
            json!({"name": CARPET_NAME}),
            json!({
                "recipe": serde_json::to_value(&recipe).unwrap(),
                "json": folded.to_json(),
                "id": folded.to_id(),
            }),
            "the name read, unfolded to a recipe, folded back and hashed",
        ),
        row(
            "gen::draw::create",
            json!({"config": "ConfigNd::<2>::default()", "rotation": "rng.below(4)", "seed": SEED}),
            json!(gen::name::Tile::of(&drawn).unwrap().to_json()),
            "the drawn recipe as the canonical json of its name",
        ),
        row(
            "gen::build::build_2d",
            json!({"tile": {"fn": "gen::name::Tile::recipe", "in": {"name": CARPET_NAME}}}),
            cell(&built),
            "the classic carpet recipe built into its 9 by 9 cell",
        ),
        row(
            "gen::variation::create",
            json!({"config": "variation::Config::default()", "seed": SEED}),
            json!({
                "key": made.key,
                "seed": made.seed.to_string(),
                "edition": made.edition.name(),
                "tile": gen::name::Tile::of(&made.tile).unwrap().to_json(),
            }),
            "seed is a u64 drawn from the stream, so it crosses as a decimal string",
        ),
        row(
            "gen::background",
            json!({"seed": 1, "width": 2, "height": 2}),
            sha(&gen::background(1, 2, 2).unwrap()),
            "sha256 of the png bytes of a whole seeded artwork",
        ),
    ]
}

// LIFE

fn glider() -> Cell2d {
    let mut grid = Tensor::new(vec![8, 8]);
    for (y, x) in [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)] {
        grid.set(&[y, x], 1).unwrap();
    }
    Cell2d::new(grid).unwrap()
}

fn glider_in() -> Value {
    json!({
        "seed": tensor(glider().types()),
        "config": {
            "mask": "life::moore",
            "birth": [3],
            "survive": [2, 3],
            "boundary": "Constant",
            "max_generations": 64,
            "grid_size": 1,
            "padding": 0,
        },
    })
}

fn life_rows() -> Vec<Value> {
    let mut seed = vec![0u8; 31];
    seed[15] = 1;
    let space = life::history(&seed, 30, 16, true).unwrap();
    let config = Config::new(life::moore().unwrap(), vec![3].into(), vec![2, 3].into());
    let run = life::animate::animate(&glider(), &config).unwrap();
    let small = two::carpet(2, 2).unwrap();
    let blank = Cell2d::new(Tensor::new(vec![4, 4])).unwrap();
    let pair = [blank, small.clone()];
    let heat = life::heatmap(&run.grids, 1).unwrap();
    vec![
        row(
            "life::history",
            json!({"row": seed, "rule": 30, "steps": 16, "wrap": true}),
            tensor(&space),
            "the space-time diagram, row 0 the seed and one row per step",
        ),
        row(
            "life::animate::animate",
            glider_in(),
            json!({
                "fate": run.fate.name(),
                "count": run.count,
                "loop_length": run.loop_length,
                "last": cell(run.last().unwrap()),
            }),
            "the glider walks into the dead border and settles",
        ),
        row(
            "life::entropy",
            json!({"grid": {"fn": "math::two::carpet", "in": {"number": 2, "level": 2}}}),
            json!(life::entropy(&small)),
            "the binary Shannon entropy of the grid, in millibits",
        ),
        row(
            "life::churn",
            json!({
                "grids": [
                    {"zeros": {"shape": [4, 4]}},
                    {"fn": "math::two::carpet", "in": {"number": 2, "level": 2}},
                ],
            }),
            f12(life::churn(&pair)),
            "the mean fraction of sites changed, to 12 decimals",
        ),
        row(
            "life::counts",
            json!({
                "seq": "Primes",
                "max_neighbors": 8,
                "include_zeros": false,
                "include_ones": false,
            }),
            json!(life::counts(Source::Primes, 8, false, false).unwrap()),
            "the primes up to eight, laid down as neighbor counts",
        ),
        row(
            "life::heatmap",
            json!({"grids": {"fn": "life::animate::animate", "in": glider_in()}, "scale": 1}),
            json!(heat),
            "the row crossing: the raw png bytes of one heat frame per generation",
        ),
    ]
}

// FONT

fn font_rows() -> Vec<Value> {
    let a = font::glyph('a').unwrap();
    let write = font::animate::animate("MRLY", 1);
    let book = font::map();
    let text = serde_json::to_string(&book).unwrap();
    vec![
        row(
            "font::glyph",
            json!({"c": "a"}),
            json!(a.rows),
            "the five bitmap rows of the rounded lowercase a",
        ),
        row(
            "font::raster::raster",
            json!({"text": "42"}),
            json!(font::raster::raster("42")),
            "the text as one 0/1 grid, its trimmed glyphs a blank column apart",
        ),
        row(
            "font::path",
            json!({"c": "M"}),
            json!(font::path('M')),
            "the tuple-list crossing: the stroke order as (row, col) pairs",
        ),
        row(
            "font::animate::animate",
            json!({"text": "MRLY", "pad": 1}),
            json!({
                "rows": write.rows,
                "cols": write.cols,
                "fps": write.fps,
                "frames": write.frames.len(),
                "last": write.frames.last().unwrap(),
            }),
            "the board, the rate, the frame count and the finished frame",
        ),
        row(
            "font::map",
            json!({}),
            json!({"sha256": sha256::hex(text.as_bytes()), "len": book.len()}),
            "sha256 of serde's json of the whole table, which pins all 108 glyphs",
        ),
    ]
}
