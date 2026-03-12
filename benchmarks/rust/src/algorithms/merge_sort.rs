/// Idiomatic Rust merge sort using Vec and iterators.
/// Pure functional style: creates new Vecs at each level.
pub fn merge_sort(xs: &[i64]) -> Vec<i64> {
    if xs.len() <= 1 {
        return xs.to_vec();
    }
    let mid = xs.len() / 2;
    let left = merge_sort(&xs[..mid]);
    let right = merge_sort(&xs[mid..]);
    merge(&left, &right)
}

fn merge(left: &[i64], right: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_empty_slice() {
        let empty: &[i64] = &[];
        assert_eq!(merge_sort(empty), Vec::<i64>::new());
    }

    #[test]
    fn should_sort_single_element() {
        assert_eq!(merge_sort(&[42]), vec![42]);
    }

    #[test]
    fn should_sort_already_sorted() {
        assert_eq!(merge_sort(&[1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_sort_reversed() {
        assert_eq!(merge_sort(&[5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn should_sort_with_duplicates() {
        assert_eq!(merge_sort(&[3, 1, 4, 1, 5, 9, 2, 6]), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }
}
