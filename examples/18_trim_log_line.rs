//! Clean up a log line by trimming whitespace, collapsing runs of
//! internal whitespace, and removing a leading `[INFO]`, `[WARN]`, or
//! `[ERROR]` tag.

pub fn trim_log_line(line: &str) -> String {
    let mut s = String::new();
    let mut started = false;
    for c in line.chars() {
        if !started && c.is_whitespace() {
            continue;
        }
        started = true;
        s.push(c);
    }
    while s.ends_with(' ') || s.ends_with('\t') || s.ends_with('\n') {
        s.pop();
    }

    for tag in &["[INFO]", "[WARN]", "[ERROR]"] {
        if s.starts_with(tag) {
            s = s[tag.len()..].to_string();
            if s.starts_with(' ') {
                s = s[1..].to_string();
            }
            break;
        }
    }

    let collapsed = s.replace("\t", " ").replace("\n", " ");
    let mut out = String::new();
    let mut prev_space = false;
    for c in collapsed.chars() {
        if c == ' ' {
            if !prev_space {
                out.push(' ');
            }
            prev_space = true;
        } else {
            out.push(c);
            prev_space = false;
        }
    }
    out
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
