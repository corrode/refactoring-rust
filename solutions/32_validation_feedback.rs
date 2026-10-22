// ANCHOR: listing
//! Model feedback as categories with exhaustive message selection.
//!
//! Callers now supply `Feedback` variants rather than strings. The returned
//! vector may allocate, but its messages are static strings with no per-message
//! string allocations.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Feedback {
    TooShort,
    MissingUppercase,
    MissingDigit,
}

impl Feedback {
    pub fn suggestion(&self) -> &'static str {
        match self {
            Self::TooShort => "Use at least eight characters.",
            Self::MissingUppercase => "Include an uppercase letter.",
            Self::MissingDigit => "Include a digit.",
        }
    }
}

pub struct ValidationReport {
    pub feedback: Vec<Feedback>,
}

impl ValidationReport {
    pub fn suggestions(&self) -> Vec<&'static str> {
        self.feedback.iter().map(Feedback::suggestion).collect()
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
            (Feedback::TooShort, "Use at least eight characters."),
            (Feedback::MissingUppercase, "Include an uppercase letter."),
            (Feedback::MissingDigit, "Include a digit."),
        ] {
            let report = ValidationReport {
                feedback: vec![feedback],
            };
            assert_eq!(report.suggestions(), vec![expected]);
        }
    }

    #[test]
    fn preserves_order_and_duplicates() {
        let report = ValidationReport {
            feedback: vec![
                Feedback::MissingDigit,
                Feedback::TooShort,
                Feedback::MissingDigit,
                Feedback::MissingUppercase,
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
            feedback: vec![Feedback::MissingUppercase, Feedback::TooShort],
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

    #[test]
    fn message_outlives_report() {
        let text: &'static str = {
            let report = ValidationReport {
                feedback: vec![Feedback::MissingDigit],
            };
            report.suggestions()[0]
        };
        assert_eq!(text, "Include a digit.");
    }
}
// ANCHOR_END: tests

// Provenance: original instructor-written fixture inspired by the stringly typed
// password-feedback pattern in beginner programming courses; no learner code copied.
