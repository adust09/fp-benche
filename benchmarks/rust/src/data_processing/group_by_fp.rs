use std::collections::HashMap;

use crate::data_processing::group_by::CategoryTotal;

/// Fold-based group-by: iter().fold() with owned accumulator.
/// Same logic as the loop version in group_by.rs, expressed as a fold chain.
pub fn group_by_category_mem_fp(records: &[(String, f64)]) -> Vec<CategoryTotal> {
    records
        .iter()
        .fold(
            HashMap::<&str, (f64, u64)>::new(),
            |mut acc, (cat, amt)| {
                let e = acc.entry(cat.as_str()).or_insert((0.0, 0));
                e.0 += amt;
                e.1 += 1;
                acc
            },
        )
        .into_iter()
        .map(|(cat, (total, count))| CategoryTotal {
            category: cat.to_string(),
            total,
            count,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_processing::group_by;

    fn to_tuples(results: &[CategoryTotal]) -> Vec<(String, f64, u64)> {
        let mut tuples: Vec<_> = results
            .iter()
            .map(|ct| (ct.category.clone(), ct.total, ct.count))
            .collect();
        tuples.sort_by(|a, b| a.0.cmp(&b.0));
        tuples
    }

    #[test]
    fn should_group_and_sum_correctly() {
        let records = vec![
            ("food".to_string(), 10.0),
            ("drink".to_string(), 5.0),
            ("food".to_string(), 20.0),
            ("drink".to_string(), 3.0),
        ];
        let result = group_by_category_mem_fp(&records);
        let tuples = to_tuples(&result);

        assert_eq!(tuples.len(), 2);
        assert_eq!(tuples[0].0, "drink");
        assert!((tuples[0].1 - 8.0).abs() < f64::EPSILON);
        assert_eq!(tuples[0].2, 2);
        assert_eq!(tuples[1].0, "food");
        assert!((tuples[1].1 - 30.0).abs() < f64::EPSILON);
        assert_eq!(tuples[1].2, 2);
    }

    #[test]
    fn should_match_loop_based_group_by() {
        let records = vec![
            ("a".to_string(), 1.0),
            ("b".to_string(), 2.0),
            ("a".to_string(), 3.0),
            ("c".to_string(), 4.0),
            ("b".to_string(), 5.0),
        ];
        let loop_result = to_tuples(&group_by::group_by_category_mem(&records));
        let fold_result = to_tuples(&group_by_category_mem_fp(&records));
        assert_eq!(loop_result, fold_result);
    }
}
