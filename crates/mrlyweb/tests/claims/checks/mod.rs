mod moire;
mod registry_integers;
mod spin;

use crate::registry::{Cost, Entry};

pub fn table() -> &'static [Entry] {
    &[
        (
            "flat-carpet-stack/the-moire-correlation-law-for-odd",
            Cost::Cheap,
            moire::the_moire_correlation_law_for_odd,
        ),
        (
            "flat-carpet-stack/the-stack-is-an-exact-prime",
            Cost::Cheap,
            moire::the_stack_is_an_exact_prime,
        ),
        (
            "spin/the-coprime-law-survives-the-spin",
            Cost::Cheap,
            spin::the_coprime_law_survives_the_spin,
        ),
        (
            "the-registry-s-integers/the-census-of-1-100000-over-the",
            Cost::Cheap,
            registry_integers::the_census_of_1_100000_over_the,
        ),
        (
            "the-registry-s-integers/the-miss-set-s-arithmetic-269-a",
            Cost::Dear,
            registry_integers::the_miss_set_s_arithmetic_269_a,
        ),
    ]
}
