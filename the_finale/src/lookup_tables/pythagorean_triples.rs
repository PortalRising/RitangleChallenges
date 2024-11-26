use std::sync::LazyLock;

use itertools::Itertools;

use crate::lookup_tables::coprimes::CoprimesTable;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PythagoreanTriple(pub usize, pub usize, pub usize);

pub struct PythagoreanTriplesTable;

impl PythagoreanTriplesTable {
    pub fn triples() -> &'static Vec<PythagoreanTriple> {
        static TRIPLES: LazyLock<Vec<PythagoreanTriple>> = LazyLock::new(|| {
            let mut triples = hashbrown::HashSet::new();

            // Use Euclids formula to generate all pythagorean triples, starting from the first composite number
            for m in 2..100usize {
                // Get all coprimes of m
                let coprimes = CoprimesTable::coprimes_of(m);

                // When m is even we want n to be odd and vice versa
                let remainder_of_n = (m + 1) % 2;

                let filtered_coprimes = coprimes
                    .filter(|&coprime| coprime < m)
                    .filter(|coprime| coprime % 2 == remainder_of_n);

                for n in filtered_coprimes {
                    // Create values of a, b, c
                    let a = m.pow(2) - n.pow(2);
                    let b = 2 * m * n;
                    let c = m.pow(2) + n.pow(2);

                    // Make sure a,b,c are not greater than 999
                    if [a, b, c].into_iter().any(|length| length > 999) {
                        // Too big
                        continue;
                    }

                    triples.insert(PythagoreanTriple(a, b, c));
                }
            }

            // Collect into a vec and order them so we can binary search
            let mut triples: Vec<PythagoreanTriple> = triples.into_iter().collect();
            triples.sort();

            triples
        });

        &TRIPLES
    }

    pub fn is_triple(numbers: [usize; 3]) -> bool {
        // Go through every permutation
        for number_permutation in numbers.into_iter().permutations(3) {
            // We can use binary search as this set is sorted
            if Self::triples()
                .binary_search(&PythagoreanTriple(
                    number_permutation[0],
                    number_permutation[1],
                    number_permutation[2],
                ))
                .is_ok()
            {
                return true;
            }
        }

        false
    }
}
