//! Sort a slice of integers with quicksort.
//!
//! Solution highlights:
//! - A slice pattern (`[pivot, rest @ ..]`) splits head from tail and
//!   handles the empty base case in the same `let ... else`.
//! - `Iterator::partition` splits the rest into "<= pivot" and "> pivot"
//!   in one pass, so the recursion reads like the textbook definition.
//!
//! This is the clearest quicksort, not the fastest: it allocates and
//! never sorts in place. Clarity is the point of the exercise. When you
//! actually need to sort, reach for `slice::sort` / `sort_unstable`.

pub fn quicksort(input: &[i32]) -> Vec<i32> {
    let [pivot, rest @ ..] = input else {
        return Vec::new();
    };
    let (less, greater): (Vec<i32>, Vec<i32>) = rest.iter().partition(|&&x| x <= *pivot);

    let mut sorted = quicksort(&less);
    sorted.push(*pivot);
    sorted.extend(quicksort(&greater));
    sorted
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
