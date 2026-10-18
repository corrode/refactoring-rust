use std::collections::HashSet;

/// Return the words from `words` that are not in `dict`.
///
/// Compare Unicode lowercase strings, as in the starter, not full case folding.
/// Build the lookup set once per call and keep the original spelling and order.
#[allow(clippy::ptr_arg)] // Preserve the starter's public signature.
pub fn spell_check(words: &Vec<String>, dict: &Vec<String>) -> Vec<String> {
    let dict: HashSet<String> = dict.iter().map(|word| word.to_lowercase()).collect();
    words
        .iter()
        .filter(|word| !dict.contains(&word.to_lowercase()))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words(ws: &[&str]) -> Vec<String> {
        ws.iter().map(|w| (*w).to_string()).collect()
    }

    #[test]
    fn detects_misspellings() {
        let dict = words(&["the", "quick", "brown", "fox"]);
        let input = words(&["The", "qucik", "brown"]);
        assert_eq!(spell_check(&input, &dict), words(&["qucik"]));
    }

    #[test]
    fn empty_input() {
        let dict = words(&["a", "b"]);
        assert!(spell_check(&vec![], &dict).is_empty());
    }

    #[test]
    fn preserves_spelling_order_and_duplicates() {
        let dict = words(&["KNOWN", "known"]);
        let input = words(&["Typo", "known", "Other", "Typo"]);
        assert_eq!(
            spell_check(&input, &dict),
            words(&["Typo", "Other", "Typo"])
        );
        assert_eq!(spell_check(&input, &vec![]), input);
    }

    #[test]
    fn uses_unicode_lowercase_not_ascii_or_full_case_folding() {
        let dict = words(&["ÉCOLE", "İ", "straße", "ΟΣ"]);
        let input = words(&["école", "i\u{307}", "STRASSE", "ος"]);
        assert_eq!(spell_check(&input, &dict), words(&["STRASSE"]));
    }
}
