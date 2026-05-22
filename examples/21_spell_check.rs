//! Return the words from `words` that are not in `dict`.
//! The check is case-insensitive with respect to ASCII.

pub fn spell_check(words: &Vec<String>, dict: &Vec<String>) -> Vec<String> {
    let mut misspelled = Vec::new();
    for word in words {
        let mut found = false;
        for d in dict {
            if d.to_lowercase() == word.to_lowercase() {
                found = true;
                break;
            }
        }
        if !found {
            misspelled.push(word.clone());
        }
    }
    misspelled
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
        assert_eq!(spell_check(&input, &dict), vec!["qucik".to_string()]);
    }

    #[test]
    fn empty_input() {
        let dict = words(&["a", "b"]);
        assert!(spell_check(&vec![], &dict).is_empty());
    }
}
