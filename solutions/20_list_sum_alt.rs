// ANCHOR: listing
pub enum List {
    Item(i32, Box<List>),
    End,
}

// Not behavior-preserving: addition happens from the tail back to the head,
// changing which intermediate sums overflow when overflow checks are enabled.
// Recursion can exhaust the stack on long lists. This is not tail-recursive,
// and Rust does not guarantee tail-call optimization anyway.
pub fn sum(list: List) -> i32 {
    match list {
        List::End => 0,
        List::Item(value, next) => value + sum(*next),
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    fn list(values: &[i32]) -> List {
        values
            .iter()
            .rev()
            .fold(List::End, |next, &value| List::Item(value, Box::new(next)))
    }

    #[test]
    fn empty() {
        assert_eq!(sum(list(&[])), 0);
    }

    #[test]
    fn single() {
        assert_eq!(sum(list(&[7])), 7);
    }

    #[test]
    fn positive() {
        assert_eq!(sum(list(&[1, 2, 3])), 6);
    }

    #[test]
    fn negative() {
        assert_eq!(sum(list(&[-1, -2, -3])), -6);
    }

    #[test]
    fn zeros() {
        assert_eq!(sum(list(&[0, 0, 0])), 0);
    }

    #[test]
    fn mixed() {
        assert_eq!(sum(list(&[5, -3, 0, -4, 1])), -1);
    }

    #[test]
    fn duplicates() {
        assert_eq!(sum(list(&[4, 4, 4])), 12);
    }

    #[cfg(debug_assertions)]
    fn iterative_sum(list: List) -> i32 {
        let mut sum = 0;
        let mut curr = list;
        while let List::Item(value, next) = curr {
            sum += value;
            curr = *next;
        }
        sum
    }

    // These tests assume default debug overflow checks, not custom compiler flags.
    #[cfg(debug_assertions)]
    #[test]
    fn recursion_avoids_an_iterative_overflow() {
        let values = [i32::MAX, 1, -1];
        assert!(std::panic::catch_unwind(|| iterative_sum(list(&values))).is_err());
        assert_eq!(sum(list(&values)), i32::MAX);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn recursion_introduces_an_overflow() {
        let values = [-1, i32::MAX, 1];
        assert_eq!(iterative_sum(list(&values)), i32::MAX);
        assert!(std::panic::catch_unwind(|| sum(list(&values))).is_err());
    }
}
// ANCHOR_END: tests
