//! Return the words from `words` that are not in `dict` (case-insensitive).
//!
//! Solution highlights:
//! - The caller builds the `HashSet` once (lowercased) and reuses it -- O(n) lookups.
//! - `&[&str]` and `Vec<&'a str>` avoid the per-word `clone`.
//! - `to_ascii_lowercase` is cheaper than `to_lowercase` and right for ASCII input.

use std::collections::HashSet;

pub fn spell_check<'a>(words: &[&'a str], dict: &HashSet<String>) -> Vec<&'a str> {
    words
        .iter()
        .copied()
        .filter(|w| !dict.contains(&w.to_ascii_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dict(ws: &[&str]) -> HashSet<String> {
        ws.iter().map(|w| w.to_ascii_lowercase()).collect()
    }

    #[test]
    fn detects_misspellings() {
        let d = dict(&["the", "quick", "brown", "fox"]);
        let input = ["The", "qucik", "brown"];
        assert_eq!(spell_check(&input, &d), vec!["qucik"]);
    }

    #[test]
    fn empty_input() {
        let d = dict(&["a", "b"]);
        let input: [&str; 0] = [];
        assert!(spell_check(&input, &d).is_empty());
    }
}
