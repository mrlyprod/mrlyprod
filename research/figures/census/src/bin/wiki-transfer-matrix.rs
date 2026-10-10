use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;

const NAME: &str = "wiki-transfer-matrix";
const STATES: usize = 3;
const LENGTH: usize = 8;
const RULE: [[u64; STATES]; STATES] = [[1, 1, 0], [0, 0, 1], [1, 0, 0]];

// POWERS

fn times(a: &[[u64; STATES]; STATES], b: &[[u64; STATES]; STATES]) -> [[u64; STATES]; STATES] {
    let mut out = [[0u64; STATES]; STATES];
    for row in 0..STATES {
        for mid in 0..STATES {
            for col in 0..STATES {
                out[row][col] += a[row][mid] * b[mid][col];
            }
        }
    }
    out
}

fn total(a: &[[u64; STATES]; STATES]) -> u64 {
    a.iter().map(|row| row.iter().sum::<u64>()).sum()
}

// WRITE

fn main() -> Result<()> {
    let mut power = RULE;
    let mut walks = Vec::with_capacity(LENGTH);
    for _ in 0..LENGTH {
        walks.push(total(&power));
        power = times(&power, &RULE);
    }
    assert_eq!(walks, vec![4, 6, 9, 13, 19, 28, 41, 60]);
    let ratio = walks[LENGTH - 1] as f64 / walks[LENGTH - 2] as f64;
    assert!((ratio - 1.465571).abs() < 0.01);
    save(
        NAME,
        &json!({"rule": RULE.map(|row| row.to_vec()).to_vec(), "walks": walks}),
    )
}
