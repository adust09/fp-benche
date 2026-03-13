use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Count word frequencies from a file using BufReader + HashMap.
pub fn count_words(path: &Path) -> HashMap<String, u64> {
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    let mut counts: HashMap<String, u64> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("failed to read line");
        for word in line.split_whitespace() {
            let word = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if !word.is_empty() {
                *counts.entry(word).or_insert(0) += 1;
            }
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn should_count_words_correctly() {
        let dir = std::env::temp_dir().join("fp_bench_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test_wc.txt");
        {
            let mut f = File::create(&path).unwrap();
            writeln!(f, "hello world hello").unwrap();
            writeln!(f, "world foo").unwrap();
        }
        let counts = count_words(&path);
        assert_eq!(counts["hello"], 2);
        assert_eq!(counts["world"], 2);
        assert_eq!(counts["foo"], 1);
        std::fs::remove_file(&path).ok();
    }
}
