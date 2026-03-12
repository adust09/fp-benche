/// Naive recursive fibonacci — no memoization.
/// Uses wrapping arithmetic to avoid overflow for large n.
pub fn fib_naive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib_naive(n - 1).wrapping_add(fib_naive(n - 2))
}

/// Idiomatic Rust: iterator-based fold.
/// Computes fib(n) mod 2^64 using wrapping arithmetic.
pub fn fib_iter(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    (2..=n)
        .fold((0u64, 1u64), |(a, b), _| (b, a.wrapping_add(b)))
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_small_fibonacci_correctly() {
        assert_eq!(fib_naive(0), 0);
        assert_eq!(fib_naive(1), 1);
        assert_eq!(fib_naive(10), 55);
        assert_eq!(fib_naive(20), 6765);
    }

    #[test]
    fn should_match_naive_and_iter_for_small_n() {
        for n in 0..30 {
            assert_eq!(fib_naive(n), fib_iter(n), "mismatch at n={n}");
        }
    }

    #[test]
    fn should_handle_large_n_with_wrapping() {
        // Just ensure it doesn't panic
        let _ = fib_iter(1_000);
        let _ = fib_iter(100_000);
    }
}
