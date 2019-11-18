use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, peek, value},
    multi::{many0, separated_list},
    sequence::{delimited, preceded, separated_pair, tuple},
    Err,
};

use crate::error::{PineError, PineErrorKind, PineResult};
use crate::name::{varname_ws, VarName};
use crate::stat_expr::exp;
use crate::stat_expr_types::{Exp, FunctionCall};
use crate::utils::eat_sep;

#[derive(Debug, PartialEq)]
struct FuncCallArg<'a> {
    name: Option<VarName<'a>>,
    arg: Exp<'a>,
}

fn func_call_arg(input: &str) -> PineResult<FuncCallArg> {
    if let Ok((input, result)) = map(tuple((varname_ws, eat_sep(tag("=")), exp)), |s| {
        FuncCallArg {
            name: Some(s.0),
            arg: s.2,
        }
    })(input)
    {
        Ok((input, result))
    } else {
        let result = map(exp, |s| FuncCallArg { name: None, arg: s })(input)?;
        Ok(result)
    }
}

fn func_call_args(input: &str) -> PineResult<(Vec<Exp>, Vec<(VarName, Exp)>)> {
    let (input, arg1) = opt(func_call_arg)(input)?;
    if arg1.is_none() {
        return Ok((input, (vec![], vec![])));
    }
    let arg1 = arg1.unwrap();
    let mut is_dict_args = arg1.name.is_some();
    let mut pos_args: Vec<Exp> = vec![];
    let mut dict_args: Vec<(VarName, Exp)> = vec![];
    if is_dict_args {
        dict_args = vec![(arg1.name.unwrap(), arg1.arg)]
    } else {
        pos_args = vec![arg1.arg];
    };

    let mut cur_input = input;

    while let Ok((next_input, arg)) = preceded(eat_sep(tag(",")), func_call_arg)(cur_input) {
        match arg.name {
            Some(name) => {
                is_dict_args = true;
                dict_args.push((name, arg.arg));
            }
            _ => {
                if is_dict_args {
                    return Err(Err::Error(PineError::from_pine_kind(
                        input,
                        PineErrorKind::InvalidFuncCallArgs(
                            "Position argument must appear before the dict argument",
                        ),
                    )));
                }
                pos_args.push(arg.arg);
            }
        }
        cur_input = next_input;
    }
    Ok((cur_input, (pos_args, dict_args)))
}

pub fn func_call(input: &str) -> PineResult<FunctionCall> {
    let (input, (method, (pos_args, dict_args))) = eat_sep(tuple((
        varname_ws,
        delimited(eat_sep(tag("(")), func_call_args, eat_sep(tag(")"))),
    )))(input)?;
    Ok((
        input,
        FunctionCall {
            method,
            pos_args,
            dict_args,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::Numeral;
    #[test]
    fn func_call_test() {
        assert_eq!(
            func_call_arg("a = true"),
            Ok((
                "",
                FuncCallArg {
                    name: Some(VarName("a")),
                    arg: Exp::Bool(true)
                }
            ))
        );
        assert_eq!(
            func_call("funa(arg1, arg2, a = true)"),
            Ok((
                "",
                FunctionCall {
                    method: VarName("funa"),
                    pos_args: vec![Exp::VarName(VarName("arg1")), Exp::VarName(VarName("arg2"))],
                    dict_args: vec![(VarName("a"), Exp::Bool(true))]
                }
            ))
        );
    }
}
