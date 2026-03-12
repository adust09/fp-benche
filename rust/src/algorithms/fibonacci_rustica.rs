use rustica::pvec::PersistentVector;

/// Fibonacci via PersistentVector::fold over an RRB-tree.
/// Measures the overhead of RRB-tree traversal compared to a simple range fold.
pub fn fib_rustica(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let indices: Vec<u64> = (2..=n).collect();
    let pv = PersistentVector::from_slice(&indices);
    pv.fold((0u64, 1u64), |acc, _| (acc.1, acc.0.wrapping_add(acc.1)))
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::fibonacci;

    #[test]
    fn should_compute_small_fibonacci_correctly() {
        assert_eq!(fib_rustica(0), 0);
        assert_eq!(fib_rustica(1), 1);
        assert_eq!(fib_rustica(10), 55);
        assert_eq!(fib_rustica(20), 6765);
    }

    #[test]
    fn should_match_iter_version() {
        for n in 0..30 {
            assert_eq!(
                fib_rustica(n),
                fibonacci::fib_iter(n),
                "mismatch at n={n}"
            );
        }
    }

    #[test]
    fn should_handle_large_n_with_wrapping() {
        let _ = fib_rustica(1_000);
        let _ = fib_rustica(100_000);
    }
}
