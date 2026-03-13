use rayon::prelude::*;

/// Parallel map: compute sqrt of each element using rayon.
/// Returns the sum to force evaluation of all elements.
pub fn parallel_map_sum(data: &[f64]) -> f64 {
    data.par_iter().map(|x| x.sqrt()).sum()
}

/// Sequential version for comparison.
pub fn sequential_map_sum(data: &[f64]) -> f64 {
    data.iter().map(|x| x.sqrt()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_produce_same_result_parallel_and_sequential() {
        let data: Vec<f64> = (1..=10000).map(|i| i as f64).collect();
        let par = parallel_map_sum(&data);
        let seq = sequential_map_sum(&data);
        assert!((par - seq).abs() < 1e-6);
    }
}
