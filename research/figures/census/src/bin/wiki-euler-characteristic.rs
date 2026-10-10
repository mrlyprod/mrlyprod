use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;
use mrlyrs::math::two::census::euler;
use mrlyrs::math::two::designs;

const NAME: &str = "wiki-euler-characteristic";
const SIDE: usize = 9;

fn holes() -> Result<Vec<[usize; 4]>> {
    let carpet = designs::carpet(3, 2)?;
    assert_eq!(carpet.width(), SIDE);
    let mut seen = [[false; SIDE]; SIDE];
    let mut boxes = Vec::new();
    for row in 0..SIDE {
        for col in 0..SIDE {
            if seen[row][col] || carpet.types().get(&[row, col])? != 0 {
                continue;
            }
            let mut stack = vec![(row, col)];
            let mut cells = Vec::new();
            seen[row][col] = true;
            while let Some((r, c)) = stack.pop() {
                cells.push((r, c));
                let step = |r: usize,
                            c: usize,
                            stack: &mut Vec<(usize, usize)>,
                            seen: &mut [[bool; SIDE]; SIDE]| {
                    if !seen[r][c] && carpet.types().get(&[r, c]).is_ok_and(|v| v == 0) {
                        seen[r][c] = true;
                        stack.push((r, c));
                    }
                };
                if r > 0 {
                    step(r - 1, c, &mut stack, &mut seen);
                }
                if r + 1 < SIDE {
                    step(r + 1, c, &mut stack, &mut seen);
                }
                if c > 0 {
                    step(r, c - 1, &mut stack, &mut seen);
                }
                if c + 1 < SIDE {
                    step(r, c + 1, &mut stack, &mut seen);
                }
            }
            let rows: Vec<usize> = cells.iter().map(|(r, _)| *r).collect();
            let cols: Vec<usize> = cells.iter().map(|(_, c)| *c).collect();
            let (r0, r1) = (*rows.iter().min().unwrap(), *rows.iter().max().unwrap());
            let (c0, c1) = (*cols.iter().min().unwrap(), *cols.iter().max().unwrap());
            boxes.push([r0, c0, r1, c1]);
        }
    }
    Ok(boxes)
}

fn main() -> Result<()> {
    let boxes = holes()?;
    let found = boxes.len();
    assert_eq!(found, 9);
    assert_eq!(euler(&designs::carpet(3, 2)?)?, 1 - found as i64);
    assert_eq!(euler(&designs::carpet(3, 3)?)?, -72);
    save(NAME, &json!({"holes": boxes}))
}
