use std::{iter, sync::LazyLock};

use itertools::Itertools;
use num_integer::Integer;
use petgraph::{graphmap::Neighbors, prelude::UnGraphMap, Undirected};

use crate::lookup_tables::primes::PrimeTable;

/// A table of all Co-Primes and their relations
pub struct CoprimesTable;

impl CoprimesTable {
    pub fn coprimes() -> &'static UnGraphMap<usize, ()> {
        static COPRIMES: LazyLock<UnGraphMap<usize, ()>> = LazyLock::new(|| {
            // Generate all coprimes up to 1000
            let mut all_coprimes = UnGraphMap::with_capacity(1024, 1024);

            // Go through every number from 4 to 1000
            for num in 1..1000 {
                // Add this number as a node, it should not exist already
                all_coprimes.add_node(num);

                // Check all other numbers
                for other_num in 1..num {
                    // Check if the LCM is one, therefore they are coprimes
                    if num.gcd(&other_num) != 1 {
                        continue;
                    }

                    // Add this number relation, it should exist because we've added all the composite numbers below us
                    all_coprimes.add_edge(num, other_num, ());
                }
            }

            all_coprimes
        });

        &COPRIMES
    }

    pub fn composite_coprimes() -> &'static UnGraphMap<usize, ()> {
        static COMPOSITE_COPRIMES: LazyLock<UnGraphMap<usize, ()>> = LazyLock::new(|| {
            // Filter out the primes from the coprime table
            let mut composite_coprimes = CoprimesTable::coprimes().clone();

            // Remove one, it literally binds with everything and has no use
            composite_coprimes.remove_node(1);

            for prime in PrimeTable::primes().iter().copied() {
                composite_coprimes.remove_node(prime);
            }

            composite_coprimes
        });

        &COMPOSITE_COPRIMES
    }

    /// Get all composite coprimes of a number, returns None if the number is 1 or a prime itself
    pub fn composite_coprimes_of<'a>(number: usize) -> Option<Neighbors<'a, usize, Undirected>> {
        if number == 1 || PrimeTable::is_prime(number) {
            return None;
        }

        // Get composite coprime table
        Some(Self::composite_coprimes().neighbors(number))
    }

    /// Checks if all numbers are composite coprimes with each other
    pub fn are_composite_coprimes(numbers: &[usize]) -> bool {
        // Get coprimes
        let coprimes = Self::composite_coprimes();

        // Go through all combinations of the numbers
        for (a, b) in numbers.iter().copied().tuple_combinations() {
            // Check if there is an edge between a and b
            if !coprimes.contains_edge(a, b) {
                // These are not coprime, so return false
                return false;
            }
        }

        true
    }

    /// Get all coprimes of a number
    pub fn coprimes_of<'a>(
        number: usize,
    ) -> iter::Chain<Neighbors<'a, usize>, std::array::IntoIter<usize, 1>> {
        // Get coprime table
        Self::coprimes().neighbors(number).chain([1])
    }

    /// Checks if all numbers are coprimes with each other
    pub fn are_coprimes(numbers: &[usize]) -> bool {
        // Get coprimes
        let coprimes = Self::coprimes();

        // Go through all combinations of the numbers
        for (a, b) in numbers.iter().copied().tuple_combinations() {
            // Check if there is an edge between a and b
            if !coprimes.contains_edge(a, b) {
                // These are not coprime, so return false
                return false;
            }
        }

        true
    }
}
