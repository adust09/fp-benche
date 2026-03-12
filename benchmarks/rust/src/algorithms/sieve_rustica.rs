use rustica::pvec::PersistentVector;

/// Prime counting via PersistentVector::filter with trial division.
/// Same algorithm as sieve_fp (trial division), expressed with PVec instead of iterators.
pub fn sieve_count_rustica(n: usize) -> usize {
    if n < 2 {
        return 0;
    }
    let candidates: Vec<usize> = (2..=n).collect();
    let pv = PersistentVector::from_slice(&candidates);
    pv.filter(|&x| is_prime(x)).len()
}

/// Returns all primes up to n as a Vec using PVec filter.
pub fn sieve_list_rustica(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let candidates: Vec<usize> = (2..=n).collect();
    let pv = PersistentVector::from_slice(&candidates);
    pv.filter(|&x| is_prime(x)).to_vec()
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
        assert_eq!(sieve_count_rustica(10), 4);
    }

    #[test]
    fn should_count_primes_up_to_100() {
        assert_eq!(sieve_count_rustica(100), 25);
    }

    #[test]
    fn should_list_primes_up_to_20() {
        assert_eq!(
            sieve_list_rustica(20),
            vec![2, 3, 5, 7, 11, 13, 17, 19]
        );
    }

    #[test]
    fn should_handle_edge_cases() {
        assert_eq!(sieve_count_rustica(0), 0);
        assert_eq!(sieve_count_rustica(1), 0);
        assert_eq!(sieve_count_rustica(2), 1);
    }

    #[test]
    fn should_match_imperative_sieve() {
        for n in [10, 100, 1000, 10_000] {
            assert_eq!(
                sieve_count_rustica(n),
                sieve::sieve_count(n),
                "count mismatch at n={n}"
            );
            assert_eq!(
                sieve_list_rustica(n),
                sieve::sieve_list(n),
                "list mismatch at n={n}"
            );
        }
    }
}
