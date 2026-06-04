//! Return the even numbers below `max` as a lazy iterator.
//!
//! Solution highlights:
//! - `impl Iterator<Item = u32>` returns the iterator without allocating.
//! - `step_by(2)` says exactly what we mean: walk in steps of two.
//! - Callers stay in control: they can `take`, `map`, `sum`, or `collect`.

pub fn evens(max: u32) -> impl Iterator<Item = u32> {
    (0..max).step_by(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_few() {
        let evens: Vec<u32> = evens(10).collect();
        assert_eq!(evens, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn empty() {
        assert_eq!(evens(0).count(), 0);
    }

    #[test]
    fn odd_bound() {
        let evens: Vec<u32> = evens(7).collect();
        assert_eq!(evens, vec![0, 2, 4, 6]);
    }

    #[test]
    fn stays_lazy() {
        let first_three: Vec<u32> = evens(u32::MAX).take(3).collect();
        assert_eq!(first_three, vec![0, 2, 4]);
    }
}
