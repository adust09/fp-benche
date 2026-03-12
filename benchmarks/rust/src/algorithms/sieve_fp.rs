/// Eratosthenes sieve using fold + for_each instead of while loops.
/// Same algorithm (O(n log log n)) and data representation (Vec<u8>) as the
/// index-based version in sieve.rs.
///
/// Note: a truly immutable (no-mutation) sieve would require trial division
/// at O(n√n) cost, which is an algorithm change, not a style change.
pub fn sieve_count_fp(n: usize) -> usize {
    if n < 2 {
        return 0;
    }
    let limit = (n as f64).sqrt() as usize;
    let sieve = (2..=limit).fold(vec![1u8; n + 1], |mut sieve, p| {
        if sieve[p] == 1 {
            (p * p..=n).step_by(p).for_each(|m| sieve[m] = 0);
        }
        sieve
    });
    sieve.iter().skip(2).map(|&x| x as usize).sum()
}

/// Returns all primes up to n as a Vec, using fold + filter + collect.
pub fn sieve_list_fp(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let limit = (n as f64).sqrt() as usize;
    let sieve = (2..=limit).fold(vec![1u8; n + 1], |mut sieve, p| {
        if sieve[p] == 1 {
            (p * p..=n).step_by(p).for_each(|m| sieve[m] = 0);
        }
        sieve
    });
    (2..=n).filter(|&i| sieve[i] == 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::sieve;

    #[test]
    fn should_count_primes_up_to_10() {
        assert_eq!(sieve_count_fp(10), 4);
    }

    #[test]
    fn should_count_primes_up_to_100() {
        assert_eq!(sieve_count_fp(100), 25);
    }

    #[test]
    fn should_list_primes_up_to_20() {
        assert_eq!(sieve_list_fp(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn should_handle_edge_cases() {
        assert_eq!(sieve_count_fp(0), 0);
        assert_eq!(sieve_count_fp(1), 0);
        assert_eq!(sieve_count_fp(2), 1);
    }

    #[test]
    fn should_match_index_based_sieve() {
        for n in [10, 100, 1000, 10_000] {
            assert_eq!(
                sieve_count_fp(n),
                sieve::sieve_count(n),
                "mismatch at n={n}"
            );
            assert_eq!(
                sieve_list_fp(n),
                sieve::sieve_list(n),
                "list mismatch at n={n}"
            );
        }
    }
}
