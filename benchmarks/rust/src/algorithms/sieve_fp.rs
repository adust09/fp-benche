/// FP-style prime counting via declarative trial division.
/// This favors expression of the prime predicate over in-place mutation, so it
/// intentionally uses a different algorithm from the imperative sieve.
pub fn sieve_count_fp(n: usize) -> usize {
    primes_up_to(n).count()
}

/// Returns all primes up to n as a Vec using the same predicate pipeline.
pub fn sieve_list_fp(n: usize) -> Vec<usize> {
    primes_up_to(n).collect()
}

fn primes_up_to(n: usize) -> impl Iterator<Item = usize> {
    (2..=n).filter(|&candidate| is_prime(candidate))
}

fn is_prime(candidate: usize) -> bool {
    let limit = (candidate as f64).sqrt() as usize;
    (2..=limit).all(|divisor| candidate % divisor != 0)
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
