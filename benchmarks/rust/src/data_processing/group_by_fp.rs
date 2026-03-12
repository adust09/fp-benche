use std::collections::HashMap;

use crate::data_processing::group_by::CategoryTotal;

/// Group-by expressed as composition of singleton maps plus reduction.
/// The aggregation step becomes "combine two partial maps" rather than mutating
/// one long-lived accumulator in a loop.
pub fn group_by_category_mem_fp(records: &[(String, f64)]) -> Vec<CategoryTotal> {
    records
        .iter()
        .map(|(cat, amt)| HashMap::from([(cat.clone(), (*amt, 1u64))]))
        .reduce(merge_category_totals)
        .unwrap_or_default()
        .into_iter()
        .map(|(cat, (total, count))| CategoryTotal {
            category: cat,
            total,
            count,
        })
        .collect()
}

fn merge_category_totals(
    mut left: HashMap<String, (f64, u64)>,
    right: HashMap<String, (f64, u64)>,
) -> HashMap<String, (f64, u64)> {
    right.into_iter().for_each(|(category, (total, count))| {
        let entry = left.entry(category).or_insert((0.0, 0));
        entry.0 += total;
        entry.1 += count;
    });
    left
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
