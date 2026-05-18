//! Clean up a log line: trim, collapse internal whitespace, strip
//! a leading `[INFO]` / `[WARN]` / `[ERROR]` tag.
//!
//! Solution highlights:
//! - `trim()` replaces both the start-skipping and end-popping loops.
//! - `strip_prefix` returns `Option<&str>` — exactly the "matched and here's the rest" shape.
//! - `split_whitespace()` + `join(" ")` collapses any run of whitespace into single spaces.

pub fn trim_log_line(line: &str) -> String {
    let trimmed = line.trim();
    let without_tag = ["[INFO]", "[WARN]", "[ERROR]"]
        .iter()
        .find_map(|tag| trimmed.strip_prefix(tag))
        .unwrap_or(trimmed);
    without_tag.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_info_tag() {
        assert_eq!(trim_log_line("[INFO] hello world"), "hello world");
    }

    #[test]
    fn collapses_internal_whitespace() {
        assert_eq!(trim_log_line("a   b\t\tc"), "a b c");
    }

    #[test]
    fn trims_outer_whitespace() {
        assert_eq!(trim_log_line("   hi   "), "hi");
    }

    #[test]
    fn keeps_line_without_tag() {
        assert_eq!(trim_log_line("plain message"), "plain message");
    }
}
