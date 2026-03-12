use rustica::pvec::PersistentVector;

/// Merge sort using PersistentVector's structural sharing via split_at.
/// Measures RRB-tree split/push_back overhead vs Vec-based approaches.
pub fn merge_sort_rustica(xs: &[i64]) -> Vec<i64> {
    let pv = PersistentVector::from_slice(xs);
    sort_pvec(pv).to_vec()
}

fn sort_pvec(pv: PersistentVector<i64>) -> PersistentVector<i64> {
    if pv.len() <= 1 {
        return pv;
    }
    let mid = pv.len() / 2;
    let (left, right) = pv.split_at(mid);
    let sorted_left = sort_pvec(left);
    let sorted_right = sort_pvec(right);
    merge_pvec(&sorted_left, &sorted_right)
}

fn merge_pvec(
    left: &PersistentVector<i64>,
    right: &PersistentVector<i64>,
) -> PersistentVector<i64> {
    let mut result = PersistentVector::new();
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();

    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (Some(&&l), Some(&&r)) if l <= r => {
                result = result.push_back(l);
                left_iter.next();
            }
            (Some(_), Some(&&r)) => {
                result = result.push_back(r);
                right_iter.next();
            }
            (Some(&&l), None) => {
                result = result.push_back(l);
                left_iter.next();
            }
            (None, Some(&&r)) => {
                result = result.push_back(r);
                right_iter.next();
            }
            (None, None) => break,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::merge_sort;

    #[test]
    fn should_sort_empty_slice() {
        let empty: &[i64] = &[];
        assert_eq!(merge_sort_rustica(empty), Vec::<i64>::new());
    }

    #[test]
    fn should_sort_single_element() {
        assert_eq!(merge_sort_rustica(&[42]), vec![42]);
    }

    #[test]
    fn should_sort_reversed() {
        assert_eq!(merge_sort_rustica(&[5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_sort_with_duplicates() {
        assert_eq!(
            merge_sort_rustica(&[3, 1, 4, 1, 5, 9, 2, 6]),
            vec![1, 1, 2, 3, 4, 5, 6, 9]
        );
    }

    #[test]
    fn should_match_index_based_merge_sort() {
        let data: Vec<i64> = (0..1000).rev().collect();
        assert_eq!(
            merge_sort_rustica(&data),
            merge_sort::merge_sort(&data)
        );
    }
}
