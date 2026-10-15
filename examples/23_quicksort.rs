//! Sort a slice of integers with quicksort.
//!
//! This version works, but it's a mouthful: it copies into a `Vec`,
//! then juggles indices through a hand-written Lomuto partition with a
//! mutating helper. The algorithm is in there somewhere, buried under
//! bookkeeping. Rewrite it so it reads the way you'd explain quicksort
//! on a whiteboard. (You have free rein to change the signature.)

pub fn quicksort(input: &[i32]) -> Vec<i32> {
    let mut values = input.to_vec();
    let len = values.len();
    if len > 1 {
        sort_range(&mut values, 0, len - 1);
    }
    values
}

fn sort_range(values: &mut [i32], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let pivot = values[hi];
    let mut i = lo;
    let mut j = lo;
    while j < hi {
        if values[j] <= pivot {
            values.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    values.swap(i, hi);
    if i > 0 {
        sort_range(values, lo, i - 1);
    }
    sort_range(values, i + 1, hi);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_a_typical_slice() {
        assert_eq!(
            quicksort(&[3, 1, 4, 1, 5, 9, 2, 6]),
            vec![1, 1, 2, 3, 4, 5, 6, 9]
        );
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(quicksort(&[]), Vec::<i32>::new());
        assert_eq!(quicksort(&[42]), vec![42]);
    }

    #[test]
    fn already_sorted_and_reversed() {
        assert_eq!(quicksort(&[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(quicksort(&[3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn duplicates_and_negatives() {
        assert_eq!(quicksort(&[5, -1, 5, 0, -1]), vec![-1, -1, 0, 5, 5]);
    }
}
