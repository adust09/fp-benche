use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// A single sales record from CSV.
#[derive(Debug, Clone)]
pub struct Sale {
    pub category: String,
    pub amount: f64,
}

/// Aggregated result per category.
#[derive(Debug, Clone)]
pub struct CategoryTotal {
    pub category: String,
    pub total: f64,
    pub count: u64,
}

/// Read CSV and group-by category, summing amounts.
/// Uses HashMap with iterator chain (idiomatic Rust).
pub fn group_by_category(path: &Path) -> Vec<CategoryTotal> {
    let file = File::open(path).expect("failed to open file");
    let mut reader = csv::Reader::from_reader(file);
    let mut groups: HashMap<String, (f64, u64)> = HashMap::new();

    for result in reader.records() {
        let record = result.expect("failed to read CSV record");
        let category = record[0].to_string();
        let amount: f64 = record[1].parse().expect("failed to parse amount");
        let entry = groups.entry(category).or_insert((0.0, 0));
        entry.0 += amount;
        entry.1 += 1;
    }

    groups
        .into_iter()
        .map(|(category, (total, count))| CategoryTotal {
            category,
            total,
            count,
        })
        .collect()
}

/// In-memory version for benchmarking without I/O.
pub fn group_by_category_mem(records: &[(String, f64)]) -> Vec<CategoryTotal> {
    let mut groups: HashMap<&str, (f64, u64)> = HashMap::new();

    for (category, amount) in records {
        let entry = groups.entry(category.as_str()).or_insert((0.0, 0));
        entry.0 += amount;
        entry.1 += 1;
    }

    groups
        .into_iter()
        .map(|(category, (total, count))| CategoryTotal {
            category: category.to_string(),
            total,
            count,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_group_and_sum_correctly() {
        let records = vec![
            ("food".to_string(), 10.0),
            ("drink".to_string(), 5.0),
            ("food".to_string(), 20.0),
            ("drink".to_string(), 3.0),
        ];
        let mut result = group_by_category_mem(&records);
        result.sort_by(|a, b| a.category.cmp(&b.category));

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].category, "drink");
        assert!((result[0].total - 8.0).abs() < f64::EPSILON);
        assert_eq!(result[0].count, 2);
        assert_eq!(result[1].category, "food");
        assert!((result[1].total - 30.0).abs() < f64::EPSILON);
        assert_eq!(result[1].count, 2);
    }
}
