use census::save;
use mrlyrs::core::error::Result;
use mrlyrs::core::json;
use mrlyrs::num::prime::{prime_count, primes};

const NAME: &str = "wiki-beurling-generalized-primes";
const TOP: usize = 100_000;
const SAMPLES: usize = 1000;

fn least_factor(top: usize) -> Vec<usize> {
    let mut least: Vec<usize> = (0..=top).collect();
    let mut p = 2;
    while p * p <= top {
        if least[p] == p {
            for m in (p * p..=top).step_by(p) {
                if least[m] == m {
                    least[m] = p;
                }
            }
        }
        p += 1;
    }
    least
}

fn counts(top: usize, keep: impl Fn(usize) -> bool) -> Vec<usize> {
    let mut out = vec![0; top + 1];
    for n in 1..=top {
        out[n] = out[n - 1] + keep(n) as usize;
    }
    out
}

fn sampled(count: &[usize]) -> Vec<usize> {
    let top = count.len() - 1;
    (0..=SAMPLES).map(|k| count[top * k / SAMPLES]).collect()
}

fn main() -> Result<()> {
    let least = least_factor(TOP);
    let mut member = vec![false; TOP + 1];
    member[1] = true;
    for n in 2..=TOP {
        let p = least[n];
        member[n] = p % 4 == 1 && member[n / p];
    }
    let all_integers = counts(TOP, |_| true);
    let toy_integers = counts(TOP, |n| member[n]);
    let all_primes = counts(TOP, |n| n > 1 && least[n] == n);
    let toy_primes = counts(TOP, |n| n > 1 && least[n] == n && n % 4 == 1);

    assert_eq!(all_integers[TOP], TOP);
    assert_eq!(toy_integers[TOP], 9623);
    assert_eq!(all_primes[TOP], prime_count(TOP));
    assert_eq!(all_primes[TOP], 9592);
    assert_eq!(toy_primes[TOP], 4783);
    assert_eq!(primes(TOP).len(), 9592);

    save(
        NAME,
        &json!({"toy_integers": sampled(&toy_integers), "toy_primes": sampled(&toy_primes)}),
    )
}
