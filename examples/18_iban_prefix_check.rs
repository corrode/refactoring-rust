//! Validate that a string looks like an IBAN.
//!
//! Rules:
//! - First two characters: ASCII letters (country code).
//! - Next two characters: ASCII digits (check digits).
//! - Remaining characters: ASCII alphanumerics.
//! - Total length must match the expected length for the country.

pub fn is_valid_iban(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 4 {
        return false;
    }

    let country = &s[0..2];

    if !chars[0].is_ascii_alphabetic() || !chars[1].is_ascii_alphabetic() {
        return false;
    }
    if !chars[2].is_ascii_digit() || !chars[3].is_ascii_digit() {
        return false;
    }
    for i in 4..chars.len() {
        if !chars[i].is_ascii_alphanumeric() {
            return false;
        }
    }

    let expected = if country == "DE" {
        22
    } else if country == "GB" {
        22
    } else if country == "FR" {
        27
    } else if country == "CH" {
        21
    } else if country == "NL" {
        18
    } else if country == "AT" {
        20
    } else {
        return false;
    };

    chars.len() == expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_de() {
        assert!(is_valid_iban("DE89370400440532013000"));
    }

    #[test]
    fn valid_nl() {
        assert!(is_valid_iban("NL91ABNA0417164300"));
    }

    #[test]
    fn wrong_length_for_country() {
        assert!(!is_valid_iban("DE8937040044053201"));
    }

    #[test]
    fn unknown_country() {
        assert!(!is_valid_iban("ZZ89370400440532013000"));
    }

    #[test]
    fn non_digits_in_check() {
        assert!(!is_valid_iban("DEAB370400440532013000"));
    }
}
