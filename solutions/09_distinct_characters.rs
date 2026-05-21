//! Count the number of distinct characters in a string.
//!
//! Solution highlights:
//! - A `HashSet` deduplicates automatically and gives O(1) membership.
//! - `collect` builds it directly from the `chars()` iterator.
//! - Complexity drops from O(n²) to O(n), and the code is shorter.

use std::collections::HashSet;

pub fn count_distinct(s: &str) -> usize {
    s.chars().collect::<HashSet<_>>().len()
}

pub fn count_distinct_without_allocations(s: &str) -> usize {
    s.char_indices().filter(|(pos, c)| s.chars().skip(*pos + 1).find(|n| n == c).is_none()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typical() {
        assert_eq!(count_distinct("lllasdflasf"), 5);
    }

    #[test]
    fn empty() {
        assert_eq!(count_distinct(""), 0);
    }

    #[test]
    fn all_unique() {
        assert_eq!(count_distinct("abcd"), 4);
    }

    #[test]
    fn all_same() {
        assert_eq!(count_distinct("aaaa"), 1);
    }

    #[test]
    fn case_sensitive() {
        assert_eq!(count_distinct("Aa"), 2);
    }
}
