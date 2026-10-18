use std::collections::HashMap;

pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

/// Compute summary statistics for the words in a piece of text.
///
/// Solution highlights:
/// - Map values are occurrence counts, so summing them gives the total.
/// - Every map key is unique, so the map length gives the unique-word count.
/// - Word lengths are weighted by their occurrence counts.
/// - Empty input has an explicit average of zero.
pub fn text_stats(text: &str) -> (usize, usize, f64) {
    let counts = count_words(text);
    let total_words: usize = counts.values().sum();
    let unique_words = counts.len();
    let total_characters: usize = counts
        .iter()
        .map(|(word, count)| word.chars().count() * count)
        .sum();
    let average_word_length = if total_words == 0 {
        0.0
    } else {
        total_characters as f64 / total_words as f64
    };

    (total_words, unique_words, average_word_length)
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

    #[test]
    fn repeated_words_affect_totals_and_average() {
        let (total, unique, average) = text_stats("a a rust");

        assert_eq!((total, unique), (3, 2));
        assert!((average - 2.0).abs() < 0.001);
    }

    #[test]
    fn empty_input_has_zero_average() {
        let (total, unique, average) = text_stats("");

        assert_eq!((total, unique), (0, 0));
        assert_eq!(average, 0.0);
    }
}
