//! Compute summary statistics for the words in a piece of text.
//!
//! The tests pass, but they only use words that occur once. Add a repeated-word
//! test before deciding whether the implementation matches the contract.

use std::collections::HashMap;

pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

pub fn text_stats(text: &str) -> (usize, usize, f64) {
    let counts = count_words(text);
    let total_words = counts.values().count();
    let unique_words = counts.iter().filter(|&(_, &count)| count == 1).count();
    let total_length: usize = counts.keys().map(|word| word.chars().count()).sum();

    (
        total_words,
        unique_words,
        total_length as f64 / total_words as f64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_words_occur_once() {
        let (total, unique, average) = text_stats("Rust makes systems programming approachable");

        assert_eq!((total, unique), (5, 5));
        assert!((average - 7.8).abs() < 0.001);
    }
}
