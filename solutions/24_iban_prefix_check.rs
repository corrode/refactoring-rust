//! Validate that a string looks like an IBAN.
//!
//! Solution highlights:
//! - `match` with `|` arms maps country -> length declaratively.
//! - `as_bytes()` indexing is O(1) and avoids allocating a `Vec<char>`.
//! - One `bytes().all(...)` replaces the per-index loop; the four explicit
//!   checks pin down the prefix shape.

pub fn is_valid_iban(s: &str) -> bool {
    let expected = match s.get(0..2) {
        Some("DE") | Some("GB") => 22,
        Some("FR") => 27,
        Some("CH") => 21,
        Some("NL") => 18,
        Some("AT") => 20,
        _ => return false,
    };
    if s.len() != expected {
        return false;
    }

    let b = s.as_bytes();
    b[0].is_ascii_alphabetic()
        && b[1].is_ascii_alphabetic()
        && b[2].is_ascii_digit()
        && b[3].is_ascii_digit()
        && s.bytes().all(|c| c.is_ascii_alphanumeric())
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
