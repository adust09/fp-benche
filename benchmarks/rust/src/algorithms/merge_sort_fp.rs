/// Merge sort with iterator-based merge step.
/// The recursive structure is identical to the index-based version in merge_sort.rs.
/// Only the merge helper differs: Peekable + iter::from_fn vs while-loop with indices.
pub fn merge_sort_fp(xs: &[i64]) -> Vec<i64> {
    if xs.len() <= 1 {
        return xs.to_vec();
    }
    let mid = xs.len() / 2;
    let left = merge_sort_fp(&xs[..mid]);
    let right = merge_sort_fp(&xs[mid..]);
    merge_fp(&left, &right)
}

/// Iterator-based merge using Peekable + from_fn pattern matching.
/// Contrast with merge_sort::merge which uses while-loop + index tracking.
fn merge_fp(left: &[i64], right: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut li = left.iter().peekable();
    let mut ri = right.iter().peekable();
    result.extend(std::iter::from_fn(|| match (li.peek(), ri.peek()) {
        (Some(&&l), Some(&&r)) if l <= r => li.next().copied(),
        (Some(_), Some(_)) => ri.next().copied(),
        (Some(_), None) => li.next().copied(),
        (None, Some(_)) => ri.next().copied(),
        (None, None) => None,
    }));
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::merge_sort;

    #[test]
    fn should_sort_empty_slice() {
        let empty: &[i64] = &[];
        assert_eq!(merge_sort_fp(empty), Vec::<i64>::new());
    }

    #[test]
    fn should_sort_single_element() {
        assert_eq!(merge_sort_fp(&[42]), vec![42]);
    }

    #[test]
    fn should_sort_reversed() {
        assert_eq!(merge_sort_fp(&[5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_sort_with_duplicates() {
        assert_eq!(
            merge_sort_fp(&[3, 1, 4, 1, 5, 9, 2, 6]),
            vec![1, 1, 2, 3, 4, 5, 6, 9]
        );
    }

    #[test]
    fn should_match_index_based_merge_sort() {
        let data: Vec<i64> = (0..1000).rev().collect();
        assert_eq!(merge_sort_fp(&data), merge_sort::merge_sort(&data));
    }
}
