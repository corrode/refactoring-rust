//! Generate the Fibonacci sequence as a lazy iterator.
//!
//! Solution highlights:
//! - One `struct` holds the state; `next` advances it in O(1).
//! - Implementing `Iterator` unlocks `take`, `map`, `zip`, `collect`, ...
//! - For overflow safety, swap `+` for `checked_add` and return `None`.

use std::iter::FusedIterator;

pub struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            current: 0,
            next: 1,
        }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

impl FusedIterator for Fibonacci {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_ten() {
        let fib: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn next_advances_state() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
    }
}
