// Reduced teaching adaptation of nom PR #1626, not a literal upstream excerpt.
// https://github.com/rust-bakery/nom/pull/1626
// Refactoring commit: bf8cddbdbdf768a2128b7817c4670da059351f9f
// Parent: 4c6e45eded6e76a227f8069ec3f77f8a14b988c8
// Result omits nom's remaining-input and error machinery.
// Decimal, parse_all, and the tests are specific to this exercise.
// Upstream copyright (c) 2014-2019 Geoffroy Couprie.
// Upstream MIT notice: ../THIRD_PARTY_NOTICES.md (nom).

// ANCHOR: listing
use std::marker::PhantomData;
use std::num::ParseIntError;

pub trait Parser<I, O, E> {
    fn parse(&mut self, input: I) -> Result<O, E>;

    fn map<F, U>(self, f: F) -> Map<Self, F, O>
    where
        Self: Sized,
        F: FnMut(O) -> U,
    {
        Map {
            parser: self,
            f,
            _output: PhantomData,
        }
    }
}

pub struct Map<P, F, O> {
    parser: P,
    f: F,
    _output: PhantomData<O>,
}

impl<I, O, U, E, P, F> Parser<I, U, E> for Map<P, F, O>
where
    P: Parser<I, O, E>,
    F: FnMut(O) -> U,
{
    fn parse(&mut self, input: I) -> Result<U, E> {
        self.parser.parse(input).map(&mut self.f)
    }
}

pub struct Decimal;

impl Parser<&str, u32, ParseIntError> for Decimal {
    fn parse(&mut self, input: &str) -> Result<u32, ParseIntError> {
        input.parse()
    }
}

pub fn parse_all<I, O, E, P>(
    mut parser: P,
    inputs: impl IntoIterator<Item = I>,
) -> Result<Vec<O>, E>
where
    P: Parser<I, O, E>,
{
    inputs
        .into_iter()
        .map(|input| parser.parse(input))
        .collect()
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    struct Counting<'a>(&'a mut usize);

    impl Parser<&str, u32, ParseIntError> for Counting<'_> {
        fn parse(&mut self, input: &str) -> Result<u32, ParseIntError> {
            *self.0 += 1;
            Decimal.parse(input)
        }
    }

    #[test]
    fn decimal_uses_standard_integer_parsing() {
        for input in ["0", "42", "+7", "4294967295", "-1", " 7", "7 "] {
            assert_eq!(Decimal.parse(input), input.parse::<u32>());
        }
    }

    #[test]
    fn maps_successes_and_changes_output_type() {
        let mut incremented: Map<Decimal, _, u32> = Decimal.map(|number| number + 1);
        assert_eq!(incremented.parse("41"), Ok(42));

        let mut rendered: Map<Decimal, _, u32> = Decimal.map(|number| number.to_string());
        assert_eq!(rendered.parse("42"), Ok(String::from("42")));
    }

    #[test]
    fn preserves_numeric_errors_without_calling_mapper() {
        let mut calls = 0;
        {
            let mut parser = Decimal.map(|number| {
                calls += 1;
                number
            });
            for input in ["", "invalid", "4294967296"] {
                assert_eq!(parser.parse(input), Err(input.parse::<u32>().unwrap_err()));
            }
        }
        assert_eq!(calls, 0);
    }

    #[test]
    fn retains_mapper_state_across_calls() {
        let mut total = 0;
        let mut parser = Decimal.map(|number| {
            total += number;
            total
        });
        assert_eq!(parser.parse("2"), Ok(2));
        assert!(parser.parse("invalid").is_err());
        assert_eq!(parser.parse("3"), Ok(5));
    }

    #[test]
    fn composes_mappings() {
        let mut parser = Decimal
            .map(|number| number * 2)
            .map(|number| number.to_string());
        assert_eq!(parser.parse("21"), Ok(String::from("42")));
        assert_eq!(parser.parse(""), Err("".parse::<u32>().unwrap_err()));
    }

    #[test]
    fn parse_all_preserves_order() {
        let mut calls = 0;
        let parser = Counting(&mut calls).map(|number| number.to_string());
        assert_eq!(
            parse_all(parser, ["3", "1", "2"]),
            Ok(vec![
                String::from("3"),
                String::from("1"),
                String::from("2")
            ])
        );
        assert_eq!(calls, 3);
    }

    #[test]
    fn parse_all_empty_does_not_parse() {
        let mut calls = 0;
        assert_eq!(parse_all(Counting(&mut calls), []), Ok(vec![]));
        assert_eq!(calls, 0);
    }

    #[test]
    fn parse_all_stops_at_first_error() {
        let mut calls = 0;
        let mut mapped = 0;
        let mut inputs = ["2", "invalid", "3", ""].into_iter();
        let parser = Counting(&mut calls).map(|number| {
            mapped += 1;
            number.to_string()
        });
        assert_eq!(
            parse_all(parser, inputs.by_ref()),
            Err("invalid".parse::<u32>().unwrap_err())
        );
        assert_eq!(calls, 2);
        assert_eq!(mapped, 1);
        assert_eq!(inputs.next(), Some("3"));
    }

    #[test]
    fn mapping_and_collecting_do_not_require_clone() {
        #[derive(Debug, PartialEq)]
        struct Token(u32);
        #[derive(Debug, PartialEq)]
        struct Failure;
        struct Forward;

        impl Parser<Result<Token, Failure>, Token, Failure> for Forward {
            fn parse(&mut self, input: Result<Token, Failure>) -> Result<Token, Failure> {
                input
            }
        }

        let mut calls = 0;
        let mut parser = Forward.map(|token| {
            calls += 1;
            token
        });
        assert_eq!(parser.parse(Err(Failure)), Err(Failure));
        assert_eq!(
            parse_all(parser, [Ok(Token(2)), Ok(Token(3))]),
            Ok(vec![Token(2), Token(3)])
        );
        assert_eq!(calls, 2);
    }

    #[test]
    fn parses_through_a_trait_object() {
        let mut decimal = Decimal;
        let parser: &mut dyn Parser<&str, u32, ParseIntError> = &mut decimal;
        assert_eq!(parser.parse("42"), Ok(42));
    }

    #[test]
    fn one_parser_and_input_can_have_different_outputs_and_errors() {
        use std::str::ParseBoolError;

        struct Flexible;

        impl Parser<&str, u32, ParseIntError> for Flexible {
            fn parse(&mut self, input: &str) -> Result<u32, ParseIntError> {
                input.parse()
            }
        }

        impl Parser<&str, bool, ParseBoolError> for Flexible {
            fn parse(&mut self, input: &str) -> Result<bool, ParseBoolError> {
                input.parse()
            }
        }

        let mut parser = Flexible;
        let number: Result<u32, ParseIntError> = parser.parse("42");
        let boolean: Result<bool, ParseBoolError> = parser.parse("true");
        assert_eq!(number, Ok(42));
        assert_eq!(boolean, Ok(true));
    }
}
// ANCHOR_END: tests
