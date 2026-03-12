use rustica::pvec::PersistentVector;

/// Sequential map + sum using PersistentVector::map and fold.
/// Two-pass evaluation (map materializes a new PVec, then fold traverses it)
/// vs iterator's single-pass lazy evaluation.
/// No parallel version: rayon cannot integrate with PVec.
pub fn sequential_map_sum_rustica(data: &[f64]) -> f64 {
    let pv = PersistentVector::from_slice(data);
    let mapped = pv.map(|&x| x.sqrt());
    mapped.fold(0.0f64, |acc, &x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concurrency::parallel_map;

    #[test]
    fn should_produce_same_result_as_sequential() {
        let data: Vec<f64> = (1..=10000).map(|i| i as f64).collect();
        let rustica = sequential_map_sum_rustica(&data);
        let seq = parallel_map::sequential_map_sum(&data);
        assert!((rustica - seq).abs() < 1e-6);
    }

    #[test]
    fn should_handle_empty_input() {
        let empty: Vec<f64> = vec![];
        assert_eq!(sequential_map_sum_rustica(&empty), 0.0);
    }

    #[test]
    fn should_handle_single_element() {
        assert!((sequential_map_sum_rustica(&[4.0]) - 2.0).abs() < f64::EPSILON);
    }
}
