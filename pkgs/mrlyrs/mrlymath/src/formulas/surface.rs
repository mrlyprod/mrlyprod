use crate::bang::factory;
use crate::bang::universe::Code;
use crate::census::exposed;
use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Frac {
    n: i128,
    d: i128,
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

impl Frac {
    fn new(n: i128, d: i128) -> Frac {
        let g = gcd(n, d).max(1);
        let sign = if d < 0 { -1 } else { 1 };
        Frac {
            n: sign * n / g,
            d: sign * d / g,
        }
    }
    fn int(v: i128) -> Frac {
        Frac { n: v, d: 1 }
    }
    fn add(self, other: Frac) -> Frac {
        Frac::new(self.n * other.d + other.n * self.d, self.d * other.d)
    }
    fn sub(self, other: Frac) -> Frac {
        Frac::new(self.n * other.d - other.n * self.d, self.d * other.d)
    }
    fn mul(self, other: Frac) -> Frac {
        Frac::new(self.n * other.n, self.d * other.d)
    }
    fn div(self, other: Frac) -> Frac {
        Frac::new(self.n * other.d, self.d * other.n)
    }
}

fn vh_state(grid: &Tensor) -> (i128, i128) {
    let occ: i128 = grid.bytes().iter().filter(|&&v| v != 0).count() as i128;
    let vf = exposed(grid) as i128;
    (vf, 6 * occ - vf)
}

fn solve_2x2(a: Frac, b: Frac, c: Frac, d: Frac, p: Frac, q: Frac) -> (Frac, Frac) {
    let det = a.mul(d).sub(b.mul(c));
    let x = p.mul(d).sub(b.mul(q)).div(det);
    let y = a.mul(q).sub(p.mul(c)).div(det);
    (x, y)
}

pub fn surface_of_tile(tile: &Tensor, level: u32) -> u128 {
    let (v1, h1) = vh_state(tile);
    if level == 1 {
        return v1 as u128;
    }
    let occ = tile.bytes().iter().filter(|&&v| v != 0).count() as u128;
    if h1 == 0 {
        return v1 as u128 * occ.pow(level - 1);
    }
    let g2 = tile.kron(tile);
    let g3 = g2.kron(tile);
    let (v2, h2) = vh_state(&g2);
    let (v3, h3) = vh_state(&g3);
    let (m00, m01) = solve_2x2(
        Frac::int(v1),
        Frac::int(h1),
        Frac::int(v2),
        Frac::int(h2),
        Frac::int(v2),
        Frac::int(v3),
    );
    let (m10, m11) = solve_2x2(
        Frac::int(v1),
        Frac::int(h1),
        Frac::int(v2),
        Frac::int(h2),
        Frac::int(h2),
        Frac::int(h3),
    );
    let mut r = [[Frac::int(1), Frac::int(0)], [Frac::int(0), Frac::int(1)]];
    for _ in 0..(level - 1) {
        r = [
            [
                r[0][0].mul(m00).add(r[0][1].mul(m10)),
                r[0][0].mul(m01).add(r[0][1].mul(m11)),
            ],
            [
                r[1][0].mul(m00).add(r[1][1].mul(m10)),
                r[1][0].mul(m01).add(r[1][1].mul(m11)),
            ],
        ];
    }
    let out = r[0][0].mul(Frac::int(v1)).add(r[0][1].mul(Frac::int(h1)));
    (out.n / out.d) as u128
}

pub fn surface(code: Code, number: usize, level: u32, base: usize) -> Result<u128> {
    let tile = factory::create(code, number, 3, base, 1)?;
    Ok(surface_of_tile(&tile, level))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::atoms;
    #[test]
    fn prediction_matches_census() {
        for code in [23u128, 129, 17] {
            for level in 1..4u32 {
                let direct = factory::create(code, 3, 3, 2, level as usize).unwrap();
                assert_eq!(
                    surface(code, 3, level, 2).unwrap(),
                    exposed(&direct),
                    "code={code} l={level}"
                );
            }
        }
    }
    #[test]
    fn solid_surfaces() {
        assert_eq!(surface_of_tile(&atoms::ones_3d(2), 3), 384);
        assert_eq!(surface_of_tile(&atoms::ones_3d(1), 5), 6);
    }
}
