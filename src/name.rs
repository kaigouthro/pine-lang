use crate::error::{PineError, PineErrorKind, PineResult};
use crate::utils::skip_ws;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha0, alphanumeric0},
    combinator::recognize,
    sequence::pair,
    Err,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct VarName<'a>(pub &'a str);

fn reserved(input: &str) -> PineResult {
    alt((
        tag("and"),
        tag("or"),
        tag("not"),
        tag("break"),
        tag("continue"),
        tag("else"),
        tag("if"),
        tag("true"),
        tag("false"),
        tag("for"),
        tag("while"),
        tag("return"),
        tag("na"),
    ))(input)
}

fn alpha_or_underscore(input: &str) -> PineResult {
    match input.chars().next().map(|t: char| {
        let b = t.is_alphabetic() || t == '_';
        (t, b)
    }) {
        Some((t, true)) => Ok((&input[1..], &input[0..1])),
        _ => Err(Err::Error(PineError::from_pine_kind(
            &input[1..],
            PineErrorKind::InvalidIdentifier("The identifier must start with alphabetic or _"),
        ))),
    }
    // alt((alpha0, tag("_")))(input)
}

fn is_alphanum_or_underscore(input: char) -> bool {
    input.is_alphanumeric() || input == '_'
}

fn varname(input: &str) -> PineResult<VarName> {
    let (input, _) = skip_ws(input)?;
    let (input, name) = recognize(pair(
        alpha_or_underscore,
        take_while(is_alphanum_or_underscore),
    ))(input)?;
    if let Ok((rest, _)) = reserved(name) {
        if rest.len() == 0 {
            return Err(Err::Error(PineError::from_pine_kind(
                input,
                PineErrorKind::ReservedVarName,
            )));
        }
    }
    Ok((input, VarName(name)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_test() {
        assert_eq!(reserved("na"), Ok(("", "na")));
        assert_eq!(alpha_or_underscore("_hello"), Ok(("hello", "_")));
        assert_eq!(alpha_or_underscore("hello"), Ok(("ello", "h")));
        assert!(alpha_or_underscore("2hello").is_err());

        assert_eq!(varname("hello_world"), Ok(("", VarName("hello_world"))));
        assert_eq!(varname("hed_12s"), Ok(("", VarName("hed_12s"))));
    }
}
