//! Start on floor zero. `(` takes you up one floor; `)` takes you down.
//! Return the final floor, or an error for the first invalid instruction.
//!
//! `try_fold` passes the current floor to the closure and stops on `Err`.
//! Use it when the next step depends on the floor you're on.

pub fn floor(instructions: &str) -> Result<i64, String> {
    instructions
        .chars()
        .try_fold(0, |floor, instruction| match instruction {
            '(' => Ok(floor + 1),
            ')' => Ok(floor - 1),
            c => Err(format!("Invalid instruction: {c}")),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_stays_on_floor_zero() {
        assert_eq!(floor(""), Ok(0));
    }

    #[test]
    fn balanced_instructions() {
        for instructions in ["(())", "()()", ")("] {
            assert_eq!(floor(instructions), Ok(0));
        }
    }

    #[test]
    fn positive_floors() {
        for instructions in ["(((", "(()(()(", "))((((("] {
            assert_eq!(floor(instructions), Ok(3));
        }
    }

    #[test]
    fn negative_floors() {
        for instructions in ["())", "))("] {
            assert_eq!(floor(instructions), Ok(-1));
        }
        for instructions in [")))", ")())())"] {
            assert_eq!(floor(instructions), Ok(-3));
        }
    }

    #[test]
    fn single_instruction() {
        assert_eq!(floor("("), Ok(1));
        assert_eq!(floor(")"), Ok(-1));
    }

    #[test]
    fn reports_first_invalid_instruction() {
        for instructions in ["x()", "(x)", "()x", "(x)y"] {
            assert_eq!(
                floor(instructions),
                Err("Invalid instruction: x".to_string())
            );
        }
    }

    #[test]
    fn rejects_whitespace_and_unicode() {
        for c in [' ', '\n', 'é', '🦀'] {
            assert_eq!(
                floor(&format!("(){c}(")),
                Err(format!("Invalid instruction: {c}"))
            );
        }
    }
}
