// ANCHOR: listing
pub enum List {
    Item(i32, Box<List>),
    End,
}

pub fn sum(list: List) -> i32 {
    let mut sum = 0;
    let mut curr = list;
    while let List::Item(value, next) = curr {
        sum += value;
        curr = *next;
    }
    sum
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

    #[test]
    fn large_list_is_consumed_iteratively() {
        assert_eq!(sum(list(&vec![0; 50_000])), 0);
    }

    #[test]
    fn adds_in_input_order() {
        assert_eq!(sum(list(&[-1, i32::MAX, 1])), i32::MAX);
    }

    // These tests assume default debug overflow checks, not custom compiler flags.
    #[cfg(debug_assertions)]
    #[test]
    fn positive_intermediate_overflow() {
        assert!(std::panic::catch_unwind(|| sum(list(&[i32::MAX, 1, -1]))).is_err());
    }

    #[cfg(debug_assertions)]
    #[test]
    fn negative_intermediate_overflow() {
        assert!(std::panic::catch_unwind(|| sum(list(&[i32::MIN, -1, 1]))).is_err());
    }
}
// ANCHOR_END: tests
