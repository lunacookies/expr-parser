use crate::lexer::SyntaxKind;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::fmt;
use std::ops::Range;

pub(crate) struct SyntaxError {
    pub(crate) kind: SyntaxErrorKind,
    pub(crate) range: Range<usize>,
}

impl SyntaxError {
    pub(crate) fn as_diagnostic<FileId>(&self, file_id: FileId) -> Diagnostic<FileId> {
        Diagnostic::error().with_labels(vec![
            Label::primary(file_id, self.range.clone()).with_message(self.kind.to_string())
        ])
    }
}

pub(crate) enum SyntaxErrorKind {
    FoundExpected {
        found: SyntaxKind,
        expected: &'static [SyntaxKind],
    },
    Expected {
        expected: &'static [SyntaxKind],
    },
}

impl fmt::Display for SyntaxErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expected_kinds = match self {
            Self::FoundExpected { found, expected } => {
                write!(f, "found {}, ", found)?;
                expected
            }
            Self::Expected { expected } => expected,
        };

        let num_expected_kinds = expected_kinds.len();

        f.write_str("expected ")?;

        let is_first = |idx| idx == 0;
        let is_last = |idx| idx == num_expected_kinds - 1;

        for (idx, expected_kind) in expected_kinds.iter().enumerate() {
            match (is_first(idx), is_last(idx)) {
                (true, _) => write!(f, "{}", expected_kind)?,
                (false, false) => write!(f, ", {}", expected_kind)?,
                (false, true) => write!(f, " or {}", expected_kind)?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn expected_with_one_kind_has_no_separators() {
        assert_eq!(
            SyntaxErrorKind::Expected {
                expected: &[SyntaxKind::Number],
            }
            .to_string(),
            "expected a number literal",
        );
    }

    #[test]
    fn expected_with_two_kinds_separates_with_or() {
        assert_eq!(
            SyntaxErrorKind::Expected {
                expected: &[SyntaxKind::Plus, SyntaxKind::Minus],
            }
            .to_string(),
            "expected a plus sign or a minus sign",
        );
    }

    #[test]
    fn expected_with_four_kinds_separates_with_comma_then_or() {
        assert_eq!(
            SyntaxErrorKind::Expected {
                expected: &[
                    SyntaxKind::Plus,
                    SyntaxKind::Star,
                    SyntaxKind::Slash,
                    SyntaxKind::Minus,
                ],
            }
            .to_string(),
            "expected a plus sign, an asterisk, a slash or a minus sign",
        );
    }

    #[test]
    fn found_expected_with_one_kind_has_no_separators() {
        assert_eq!(
            SyntaxErrorKind::FoundExpected {
                found: SyntaxKind::Plus,
                expected: &[SyntaxKind::Number],
            }
            .to_string(),
            "found a plus sign, expected a number literal",
        );
    }

    #[test]
    fn found_expected_with_two_kinds_separates_with_or() {
        assert_eq!(
            SyntaxErrorKind::FoundExpected {
                found: SyntaxKind::Minus,
                expected: &[SyntaxKind::Number, SyntaxKind::Star],
            }
            .to_string(),
            "found a minus sign, expected a number literal or an asterisk",
        );
    }

    #[test]
    fn found_expected_with_three_kinds_separates_with_comma_then_or() {
        assert_eq!(
            SyntaxErrorKind::FoundExpected {
                found: SyntaxKind::Slash,
                expected: &[SyntaxKind::Plus, SyntaxKind::Minus, SyntaxKind::Star],
            }
            .to_string(),
            "found a slash, expected a plus sign, a minus sign or an asterisk",
        );
    }
}
