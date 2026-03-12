use std::collections::HashMap;

use rustica::pvec::PersistentVector;

/// Word count using PersistentVector pipeline: flat_map → map → filter → fold.
/// Each stage eagerly materializes a PVec (unlike lazy iterator chains).
/// The final fold accumulates into a mutable HashMap (no persistent Map available).
pub fn count_words_mem_rustica(lines: &[String]) -> HashMap<String, u64> {
    let pv = PersistentVector::from_slice(lines);

    let words = pv.flat_map(|line| {
        let ws: Vec<String> = line
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .collect();
        PersistentVector::from_slice(&ws)
    });

    let filtered = words.filter(|w| !w.is_empty());

    filtered.fold(HashMap::new(), |mut acc, word| {
        *acc.entry(word.clone()).or_insert(0) += 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_processing::word_count_fp;

    fn sample_lines() -> Vec<String> {
        vec![
            "hello world hello".to_string(),
            "world foo".to_string(),
            "bar, baz! bar".to_string(),
        ]
    }

    #[test]
    fn should_count_words_correctly() {
        let counts = count_words_mem_rustica(&sample_lines());
        assert_eq!(counts["hello"], 2);
        assert_eq!(counts["world"], 2);
        assert_eq!(counts["foo"], 1);
    }

    #[test]
    fn should_match_imperative_and_fp() {
        let lines = sample_lines();
        assert_eq!(
            count_words_mem_rustica(&lines),
            word_count_fp::count_words_mem_imperative(&lines)
        );
        assert_eq!(
            count_words_mem_rustica(&lines),
            word_count_fp::count_words_mem_fp(&lines)
        );
    }

    #[test]
    fn should_handle_empty_input() {
        let empty: Vec<String> = vec![];
        assert_eq!(count_words_mem_rustica(&empty), HashMap::new());
    }

    #[test]
    fn should_handle_punctuation_only() {
        let lines = vec!["... !!! ???".to_string()];
        assert_eq!(count_words_mem_rustica(&lines), HashMap::new());
    }
}
