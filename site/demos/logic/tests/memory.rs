use demos::memory::*;
use demos::two::two_grid;
use mrlyrs::core::error::parse;

fn read(dimension: usize, width: usize, code: &str, levels: usize) -> mrlyrs::core::Json {
    parse(&memory_read(dimension, width, code, levels).unwrap()).unwrap()
}

#[test]
fn width_one_is_the_plane_design_cell_for_cell() {
    for code in ["1", "7", "11", "13", "14", "9"] {
        for level in 1..=6 {
            let sheet = memory_sheet(2, 1, code, level).unwrap();
            let design = two_grid(code, 2, level, 0, 2).unwrap();
            assert_eq!(sheet.width, design.width, "code {code} level {level}");
            assert_eq!(sheet.height, design.height, "code {code} level {level}");
            assert_eq!(sheet.types, design.types, "code {code} level {level}");
        }
    }
}

#[test]
fn the_golden_rule_counts_the_fibonacci_numbers() {
    let read = read(1, 2, "7", 8);
    assert_eq!(
        read["counts"].to_string(),
        "[2,3,5,8,13,21,34,55]".to_string()
    );
    assert_eq!(
        format!("{:.6}", read["perron"].as_f64().unwrap()),
        "1.618034"
    );
    assert_eq!(
        format!("{:.6}", read["exponent"].as_f64().unwrap()),
        "0.694242"
    );
    assert_eq!(
        format!("{:.6}", read["kappa"].as_f64().unwrap()),
        "0.098239"
    );
    assert_eq!(read["window_count"].as_u64().unwrap(), 3);
}

#[test]
fn the_supergolden_rule_counts_the_narayana_cows() {
    let read = read(1, 3, "23", 8);
    assert_eq!(
        read["counts"].to_string(),
        "[2,4,4,6,9,13,19,28]".to_string()
    );
    assert_eq!(
        format!("{:.6}", read["perron"].as_f64().unwrap()),
        "1.465571"
    );
}

#[test]
fn the_width_three_presets_name_the_plastic_and_tribonacci_roots() {
    let plastic = read(1, 3, "54", 8);
    assert_eq!(
        plastic["counts"].to_string(),
        "[2,4,4,5,7,9,12,16]".to_string()
    );
    assert_eq!(
        format!("{:.9}", plastic["perron"].as_f64().unwrap()),
        "1.324717957"
    );
    assert_eq!(
        format!("{:.6}", plastic["kappa"].as_f64().unwrap()),
        "0.260981"
    );
    let tribonacci = read(1, 3, "127", 8);
    assert_eq!(
        tribonacci["counts"].to_string(),
        "[2,4,7,13,24,44,81,149]".to_string()
    );
    assert_eq!(
        format!("{:.9}", tribonacci["perron"].as_f64().unwrap()),
        "1.839286755"
    );
    assert_eq!(
        format!("{:.6}", tribonacci["kappa"].as_f64().unwrap()),
        "0.056639"
    );
}

#[test]
fn the_full_rule_carries_no_memory_cost() {
    let read = read(2, 2, "65535", 4);
    assert_eq!(read["counts"].to_string(), "[4,16,64,256]".to_string());
    assert_eq!(read["kappa"].as_f64().unwrap(), 0.0);
    assert_eq!(read["states"].as_u64().unwrap(), 4);
    assert_eq!(read["windows"].as_u64().unwrap(), 16);
}

#[test]
fn the_cantor_sheet_is_one_row_a_level() {
    let sheet = memory_sheet(1, 2, "7", 6).unwrap();
    assert_eq!((sheet.width, sheet.height), (64, 6));
    let filled: Vec<u32> = (0..6)
        .map(|row| {
            sheet.types[row * 64..(row + 1) * 64]
                .iter()
                .map(|&b| b as u32)
                .sum()
        })
        .collect();
    assert_eq!(filled, vec![64, 48, 40, 32, 26, 21]);
}

#[test]
fn a_rule_over_the_budget_is_refused() {
    assert!(memory_sheet(2, 1, "7", 9).is_err());
    assert!(memory_read(1, 7, "0", 4).is_err());
    assert!(memory_read(1, 2, "16", 4).is_err());
}
