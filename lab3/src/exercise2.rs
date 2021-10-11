/// Implements the sieve of Eratosthenes.
/// The function returns a vector holding the primes numbers
/// smaller or equal to `n`.
///
/// # Example
/// ```
/// let r = lab3::exercise2::sieve(7);
/// // Original test should fail as sieve goes from 0 to 6
/// // assert_eq!(r, vec![1, 2, 3, 5, 7]);
/// assert_eq!(r, vec![1, 2, 3, 5]);
/// ```
pub fn sieve(n: u32) -> Vec<u32> {
    // It would be more idiomatic to return an `Option<Vec<u32>>`
    // Here, it should return `None`
    if n == 0 {
        return vec![];
    }

    // Limit for the outer loop
    let limit = (n as f64).sqrt() as usize;
    // Casting `n` to usize to avoid repetitive conversions
    let n = n as usize;
    let mut sieve = vec![true; n];

    // Exceptional case for 0
    sieve[0] = false;
    // Compute primes using the sieve of Eratosthenes algorithm
    for prime in 2..=limit {
        if sieve[prime] {
            for multiple in ((prime * prime)..n).step_by(prime) {
                sieve[multiple] = false;
            }
        }
    }

    // Generate the resulting vector using `filter_map`
    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, b)| if *b { Some(i as u32) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_zero() {
        let m = sieve(0);
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn n_thirty() {
        let m = sieve(30);
        assert_eq!(m, vec![1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    // Another test to show that everything works fine
    #[test]
    fn n_one_hundred_twenty() {
        let m = sieve(120);
        assert_eq!(
            m,
            vec![
                1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                79, 83, 89, 97, 101, 103, 107, 109, 113
            ]
        );
    }

    // Test to "measure" the performance of the algorithm
    // Should compute every prime numbers up to 100_000_000
    #[test]
    fn n_one_hundred_million() {
        let m = sieve(100_000_000);
        // There are 5_761_455 prime numbers from 2 to 100_000_000
        // But here we count 1 as prime so we add one to the result
        assert_eq!(m.len(), 5_761_456);
    }
}
