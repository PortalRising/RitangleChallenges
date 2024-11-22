pub const PRIMES: [usize; 168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

// pub struct Primes {
//     cache: Vec<usize>,
// }

// impl Primes {
//     /// Create a new cache of primes that will automatically generate primes when needed
//     pub fn new() -> Self {
//         // Create a cache of primes with just the number 2 in it, to ignore 1 as its not a prime
//         Self { cache: vec![2] }
//     }

//     /// Check if a number is a prime without looking it up in the cache
//     fn is_prime(potential_prime: usize) -> bool {
//         // Check if number is divisible by all numbers between 0..(sqrt(test_number) + 1)
//         let potential_prime_sqrt = (potential_prime as f64).sqrt().ceil() as usize;
//         for test_number in 2..=potential_prime_sqrt {
//             if potential_prime % test_number == 0 {
//                 // This is not a prime
//                 return false;
//             }
//         }

//         // This must be a prime
//         return true;
//     }

//     /// Lookup in the cache whether the provided number is a prime,
//     /// if the cache has not calculated it then this function will
//     /// calculate them and store them in the cache
//     pub fn lookup_is_prime(&mut self, test_number: usize) -> bool {
//         // Start at the first prime
//         let mut prime_index: usize = 0;

//         // Go through all primes until one is either equal to or greater than the test_number
//         loop {
//             // Get prime at index
//             let prime = self.prime(prime_index);

//             // Is prime equal to the test number
//             if prime == test_number {
//                 // It is a prime
//                 return true;
//             }

//             // Is the prime greater than the the test number
//             if prime > test_number {
//                 // This cannot be a prime as we've checked all previous primes and they were not equal
//                 return false;
//             }

//             prime_index += 1;
//         }
//     }

//     /// Calculate prime up to and at index
//     pub fn calculate_prime(&mut self, index: usize) -> Option<usize> {
//         // Start from the last known prime, which should exist as we start with 2 in the cache
//         let known_start = self.cache.last().copied().unwrap();
//         let mut potential_prime = known_start + 1;

//         loop {
//             // Check if we have calculated enough primes
//             if self.cache.len() > index {
//                 break;
//             }

//             // Check if this number is a prime
//             if Self::is_prime(potential_prime) {
//                 // Add it to the cache
//                 println!("Calculated and cached the prime: {}", potential_prime);
//                 self.cache.push(potential_prime);
//             }

//             if potential_prime == usize::MAX {
//                 // We cannot go any higher
//                 return None;
//             }

//             potential_prime += 1;
//         }

//         // Return the prime at the requested index as we know it exists now
//         return Some(self.cache[index]);
//     }

//     /// Get the prime at the index
//     pub fn prime(&mut self, index: usize) -> usize {
//         // Calculate the prime if it has not been calculated already
//         self.calculate_prime(index);

//         return self.cache[index];
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::Primes;

//     #[test]
//     fn validate_primes() {
//         let mut primes = Primes::new();
//         let mut valid_primes = Vec::new();

//         for i in 0..98 {
//             println!("Test Number: {}", i);
//             if primes.lookup_is_prime(i) {
//                 println!("{} is prime", i);
//                 valid_primes.push(i);
//             }
//         }

//         assert_eq!(valid_primes, primes.cache);
//         assert_eq!(
//             primes.cache,
//             [
//                 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
//                 83, 89, 97
//             ]
//         );
//     }
// }
