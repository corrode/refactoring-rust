// ANCHOR: listing
/// Return the even numbers below `max`.
pub fn evens(max: u32) -> Vec<u32> {
    (0..max).step_by(2).collect()
}

// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_signature() {
        let generate: fn(u32) -> Vec<u32> = evens;
        assert_eq!(generate(2), vec![0]);
    }

    #[test]
    fn first_few() {
        assert_eq!(evens(10), vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn empty() {
        assert_eq!(evens(0), Vec::<u32>::new());
    }

    #[test]
    fn odd_bound() {
        assert_eq!(evens(7), vec![0, 2, 4, 6]);
    }

    #[test]
    fn smallest_nonempty_bound() {
        assert_eq!(evens(1), vec![0]);
    }
}
// ANCHOR_END: tests
