/// Merge sort with a declarative merge step based on slice decomposition.
/// The recursive structure is identical to the index-based version in merge_sort.rs.
/// Only the merge helper differs: immutable slice views vs while-loop indices.
pub fn merge_sort_fp(xs: &[i64]) -> Vec<i64> {
    if xs.len() <= 1 {
        return xs.to_vec();
    }
    let mid = xs.len() / 2;
    let left = merge_sort_fp(&xs[..mid]);
    let right = merge_sort_fp(&xs[mid..]);
    merge_fp(&left, &right)
}

/// Merge two sorted slices by pattern-matching on their heads.
fn merge_fp(left: &[i64], right: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_rest = left;
    let mut right_rest = right;
    result.extend(std::iter::from_fn(|| match (left_rest.split_first(), right_rest.split_first()) {
        (Some((&l, l_tail)), Some((&r, _))) if l <= r => {
            left_rest = l_tail;
            Some(l)
        }
        (Some(_), Some((&r, r_tail))) => {
            right_rest = r_tail;
            Some(r)
        }
        (Some((&l, l_tail)), None) => {
            left_rest = l_tail;
            Some(l)
        }
        (None, Some((&r, r_tail))) => {
            right_rest = r_tail;
            Some(r)
        }
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
