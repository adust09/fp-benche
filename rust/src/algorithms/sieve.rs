/// Sieve of Eratosthenes using mutable Vec<u8>.
/// Returns the count of primes up to n.
pub fn sieve_count(n: usize) -> usize {
    if n < 2 {
        return 0;
    }
    let mut is_prime = vec![1u8; n + 1];
    is_prime[0] = 0;
    is_prime[1] = 0;

    let mut p = 2;
    while p * p <= n {
        if is_prime[p] == 1 {
            let mut multiple = p * p;
            while multiple <= n {
                is_prime[multiple] = 0;
                multiple += p;
            }
        }
        p += 1;
    }

    is_prime.iter().map(|&x| x as usize).sum()
}

/// Returns all primes up to n as a Vec.
pub fn sieve_list(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }
    let mut is_prime = vec![1u8; n + 1];
    is_prime[0] = 0;
    is_prime[1] = 0;

    let mut p = 2;
    while p * p <= n {
        if is_prime[p] == 1 {
            let mut multiple = p * p;
            while multiple <= n {
                is_prime[multiple] = 0;
                multiple += p;
            }
        }
        p += 1;
    }

    (2..=n).filter(|&i| is_prime[i] == 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_count_primes_up_to_10() {
        assert_eq!(sieve_count(10), 4); // 2, 3, 5, 7
    }

    #[test]
    fn should_count_primes_up_to_100() {
        assert_eq!(sieve_count(100), 25);
    }

    #[test]
    fn should_list_primes_up_to_20() {
        assert_eq!(sieve_list(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn should_handle_edge_cases() {
        assert_eq!(sieve_count(0), 0);
        assert_eq!(sieve_count(1), 0);
        assert_eq!(sieve_count(2), 1);
    }
}
