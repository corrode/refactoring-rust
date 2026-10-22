// ANCHOR: listing
//! Replace stringly typed feedback with explicit categories.
//!
//! You may change the public API and adapt callers and tests. Preserve the
//! messages, order, and duplicates for valid categories. Consider what happens
//! when a caller supplies an unknown category.

pub struct ValidationReport {
    pub feedback: Vec<String>,
}

impl ValidationReport {
    pub fn suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();
        for feedback in &self.feedback {
            match feedback.as_str() {
                "too short" => suggestions.push("Use at least eight characters.".to_owned()),
                "missing uppercase" => suggestions.push("Include an uppercase letter.".to_owned()),
                "missing digit" => suggestions.push("Include a digit.".to_owned()),
                _ => {}
            }
        }
        suggestions
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_report_has_no_suggestions() {
        let report = ValidationReport { feedback: vec![] };
        assert!(report.suggestions().is_empty());
    }

    #[test]
    fn each_category_has_its_exact_message() {
        for (feedback, expected) in [
            ("too short", "Use at least eight characters."),
            ("missing uppercase", "Include an uppercase letter."),
            ("missing digit", "Include a digit."),
        ] {
            let report = ValidationReport {
                feedback: vec![feedback.to_owned()],
            };
            assert_eq!(report.suggestions(), vec![expected]);
        }
    }

    #[test]
    fn preserves_order_and_duplicates() {
        let report = ValidationReport {
            feedback: vec![
                "missing digit".into(),
                "too short".into(),
                "missing digit".into(),
                "missing uppercase".into(),
            ],
        };
        assert_eq!(
            report.suggestions(),
            vec![
                "Include a digit.",
                "Use at least eight characters.",
                "Include a digit.",
                "Include an uppercase letter.",
            ]
        );
    }

    #[test]
    fn repeated_calls_leave_feedback_unchanged() {
        let report = ValidationReport {
            feedback: vec!["missing uppercase".into(), "too short".into()],
        };
        let original = report.feedback.clone();
        let expected = vec![
            "Include an uppercase letter.",
            "Use at least eight characters.",
        ];
        assert_eq!(report.suggestions(), expected);
        assert_eq!(report.suggestions(), expected);
        assert_eq!(report.feedback, original);
    }
}
// ANCHOR_END: tests

// Provenance: original instructor-written fixture inspired by the stringly typed
// password-feedback pattern in beginner programming courses; no learner code copied.
