use super::VarResult;
use crate::ast::syntax_type::{FunctionType, FunctionTypes, SimpleSyntaxType, SyntaxType};
use crate::helper::{pine_ref_to_f64, pine_ref_to_i64};
use crate::runtime::context::{downcast_ctx, Ctx};
use crate::types::{Callable, Float, Int, PineFrom, PineRef, RuntimeErr, Series, SeriesCall, NA};
use std::mem;
use std::mem::transmute;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
struct MathCallVal {
    func: *mut (),
}

impl MathCallVal {
    pub fn new(func: *mut ()) -> MathCallVal {
        MathCallVal { func }
    }
}

impl<'a> SeriesCall<'a> for MathCallVal {
    fn step(
        &mut self,
        _ctx: &mut dyn Ctx<'a>,
        mut param: Vec<Option<PineRef<'a>>>,
        func_type: FunctionType<'a>,
    ) -> Result<PineRef<'a>, RuntimeErr> {
        let xval = mem::replace(&mut param[0], None);

        let handler = unsafe { transmute::<_, fn(Option<PineRef<'a>>) -> Float>(self.func) };
        match ((func_type.signature.0)[0]).1 {
            SyntaxType::Simple(SimpleSyntaxType::Float) => {
                let res = handler(xval);
                Ok(PineRef::new_box(res))
            }
            SyntaxType::Series(SimpleSyntaxType::Float) => {
                let res = handler(xval);
                Ok(PineRef::new_rc(Series::from(res)))
            }
            _ => unreachable!(),
        }
    }

    fn copy(&self) -> Box<dyn SeriesCall<'a> + 'a> {
        Box::new(self.clone())
    }
}
pub const VAR_NAME: &'static str = "cos";

pub fn declare_math_var<'a>(
    varname: &'static str,
    func: fn(Option<PineRef<'a>>) -> Float,
) -> VarResult<'a> {
    let value = PineRef::new(Callable::new(
        None,
        Some(Box::new(MathCallVal::new(func as *mut ()))),
    ));

    // plot(series, title, color, linewidth, style, trackprice, transp, histbase, offset, join, editable, show_last) → plot

    let func_type = FunctionTypes(vec![
        FunctionType::new((vec![("x", SyntaxType::float())], SyntaxType::float())),
        FunctionType::new((
            vec![("x", SyntaxType::float_series())],
            SyntaxType::float_series(),
        )),
    ]);
    let syntax_type = SyntaxType::Function(Rc::new(func_type));
    VarResult::new(value, syntax_type, varname)
}

fn float_cos<'a>(xval: Option<PineRef<'a>>) -> Float {
    match pine_ref_to_f64(xval) {
        None => None,
        Some(v) => Some(v.cos()),
    }
}

pub fn declare_cos_var<'a>() -> VarResult<'a> {
    declare_math_var("cos", float_cos)
}

fn float_acos<'a>(xval: Option<PineRef<'a>>) -> Float {
    match pine_ref_to_f64(xval) {
        None => None,
        Some(v) => Some(v.acos()),
    }
}

pub fn declare_acos_var<'a>() -> VarResult<'a> {
    declare_math_var("acos", float_acos)
}

fn float_sin<'a>(xval: Option<PineRef<'a>>) -> Float {
    match pine_ref_to_f64(xval) {
        None => None,
        Some(v) => Some(v.sin()),
    }
}

pub fn declare_sin_var<'a>() -> VarResult<'a> {
    declare_math_var("sin", float_sin)
}

fn float_asin<'a>(xval: Option<PineRef<'a>>) -> Float {
    match pine_ref_to_f64(xval) {
        None => None,
        Some(v) => Some(v.asin()),
    }
}

pub fn declare_asin_var<'a>() -> VarResult<'a> {
    declare_math_var("asin", float_asin)
}

fn float_tan<'a>(xval: Option<PineRef<'a>>) -> Float {
    match pine_ref_to_f64(xval) {
        None => None,
        Some(v) => Some(v.tan()),
    }
}

pub fn declare_tan_var<'a>() -> VarResult<'a> {
    declare_math_var("tan", float_tan)
}

fn float_atan<'a>(xval: Option<PineRef<'a>>) -> Float {
    match pine_ref_to_f64(xval) {
        None => None,
        Some(v) => Some(v.atan()),
    }
}

pub fn declare_atan_var<'a>() -> VarResult<'a> {
    declare_math_var("atan", float_atan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::stat_expr_types::VarIndex;
    use crate::ast::syntax_type::SimpleSyntaxType;
    use crate::runtime::{AnySeries, NoneCallback, VarOperate};
    use crate::{LibInfo, PineParser, PineRunner};

    #[test]
    fn cos_test() {
        let lib_info = LibInfo::new(
            vec![
                declare_cos_var(),
                declare_acos_var(),
                declare_sin_var(),
                declare_asin_var(),
                declare_tan_var(),
                declare_atan_var(),
            ],
            vec![("close", SyntaxType::Series(SimpleSyntaxType::Float))],
        );
        let src = "m1 = cos(0)\nm2 = acos(1)\nm3 = sin(0)\nm4 = asin(0)\nm5 = tan(0)\nm6 = atan(0)";
        let blk = PineParser::new(src, &lib_info).parse_blk().unwrap();
        let mut runner = PineRunner::new(&lib_info, &blk, &NoneCallback());

        runner
            .run(
                &vec![("close", AnySeries::from_float_vec(vec![Some(-2f64)]))],
                None,
            )
            .unwrap();
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(7, 0)),
            Some(PineRef::new(Some(1f64)))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(8, 0)),
            Some(PineRef::new(Some(0f64)))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(9, 0)),
            Some(PineRef::new(Some(0f64)))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(10, 0)),
            Some(PineRef::new(Some(0f64)))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(11, 0)),
            Some(PineRef::new(Some(0f64)))
        );
        assert_eq!(
            runner.get_context().move_var(VarIndex::new(12, 0)),
            Some(PineRef::new(Some(0f64)))
        );
    }
}