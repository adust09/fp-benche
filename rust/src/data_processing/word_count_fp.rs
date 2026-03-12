use std::collections::HashMap;

/// Loop-based in-memory word count (matches logic in word_count.rs but without I/O).
/// Used as the imperative baseline for style comparison.
pub fn count_words_mem_imperative(lines: &[String]) -> HashMap<String, u64> {
    let mut counts = HashMap::new();
    for line in lines {
        for word in line.split_whitespace() {
            let w = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if !w.is_empty() {
                *counts.entry(w).or_insert(0) += 1;
            }
        }
    }
    counts
}

/// Iterator chain word count — flat_map + fold, no intermediate allocation.
/// Same logic as the loop version, expressed as a single iterator pipeline.
pub fn count_words_mem_fp(lines: &[String]) -> HashMap<String, u64> {
    lines
        .iter()
        .flat_map(|line| {
            line.split_whitespace()
                .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
                .filter(|w| !w.is_empty())
        })
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_lines() -> Vec<String> {
        vec![
            "hello world hello".to_string(),
            "world foo".to_string(),
            "bar, baz! bar".to_string(),
        ]
    }

    #[test]
    fn should_count_words_imperative() {
        let counts = count_words_mem_imperative(&sample_lines());
        assert_eq!(counts["hello"], 2);
        assert_eq!(counts["world"], 2);
        assert_eq!(counts["foo"], 1);
    }

    #[test]
    fn should_count_words_fp() {
        let counts = count_words_mem_fp(&sample_lines());
        assert_eq!(counts["hello"], 2);
        assert_eq!(counts["world"], 2);
        assert_eq!(counts["foo"], 1);
    }

    #[test]
    fn should_match_imperative_and_fp() {
        let lines = sample_lines();
        assert_eq!(
            count_words_mem_imperative(&lines),
            count_words_mem_fp(&lines)
        );
    }

    #[test]
    fn should_handle_empty_input() {
        let empty: Vec<String> = vec![];
        assert_eq!(count_words_mem_imperative(&empty), HashMap::new());
        assert_eq!(count_words_mem_fp(&empty), HashMap::new());
    }

    #[test]
    fn should_handle_punctuation_only() {
        let lines = vec!["... !!! ???".to_string()];
        assert_eq!(count_words_mem_imperative(&lines), HashMap::new());
        assert_eq!(count_words_mem_fp(&lines), HashMap::new());
    }
}
