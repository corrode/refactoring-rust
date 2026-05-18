//! Count the number of distinct characters in a string.

pub fn count_distinct(s: &str) -> usize {
    let mut seen: Vec<char> = Vec::new();
    for c in s.chars() {
        if !seen.contains(&c) {
            seen.push(c);
        }
    }
    seen.len()
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
