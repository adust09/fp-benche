use std::collections::HashMap;

use rustica::pvec::PersistentVector;

use crate::data_processing::group_by::CategoryTotal;

/// Group-by using PersistentVector::fold for accumulation and PVec::map for result conversion.
/// The fold accumulates into a mutable HashMap (no persistent Map in rustica).
pub fn group_by_category_mem_rustica(records: &[(String, f64)]) -> Vec<CategoryTotal> {
    let pv = PersistentVector::from_slice(records);

    let groups = pv.fold(
        HashMap::<String, (f64, u64)>::new(),
        |mut acc, (category, amount)| {
            let entry = acc.entry(category.clone()).or_insert((0.0, 0));
            entry.0 += amount;
            entry.1 += 1;
            acc
        },
    );

    let entries: Vec<(String, (f64, u64))> = groups.into_iter().collect();
    let pv_entries = PersistentVector::from_slice(&entries);
    pv_entries
        .map(|(cat, (total, count))| CategoryTotal {
            category: cat.clone(),
            total: *total,
            count: *count,
        })
        .to_vec()
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
        let result = group_by_category_mem_rustica(&records);
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
        let rustica_result = to_tuples(&group_by_category_mem_rustica(&records));
        assert_eq!(loop_result, rustica_result);
    }

    #[test]
    fn should_handle_empty_input() {
        let empty: Vec<(String, f64)> = vec![];
        let result = group_by_category_mem_rustica(&empty);
        assert!(result.is_empty());
    }
}
