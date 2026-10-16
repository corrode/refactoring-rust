//! Generate Fibonacci numbers without repeated recursive calls.
//!
//! Keep the starter's functions and return types. Values through F(93) fit in
//! `u64`; larger values retain the starter's ordinary integer overflow behavior.

pub fn fib(n: u32) -> u64 {
    let (mut previous, mut current) = (1, 0);
    for _ in 0..n {
        (previous, current) = (current, previous + current);
    }
    current
}

pub fn first_n(n: usize) -> Vec<u64> {
    (0..n)
        .scan((1, 0), |(previous, current), i| {
            // Advance only when another value is requested, never past the last one.
            if i > 0 {
                (*previous, *current) = (*current, *previous + *current);
            }
            Some(*current)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_ten() {
        let fib: Vec<u64> = first_n(10);
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn single_values() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn short_prefixes() {
        assert!(first_n(0).is_empty());
        assert_eq!(first_n(1), vec![0]);
        assert_eq!(first_n(2), vec![0, 1]);
    }

    #[test]
    fn includes_the_last_representable_value_without_computing_the_next() {
        assert_eq!(fib(93), 12_200_160_415_121_876_738);
        let values = first_n(94);
        assert_eq!(values.len(), 94);
        for (n, value) in values.into_iter().enumerate() {
            assert_eq!(value, fib(n as u32));
        }
    }
}
