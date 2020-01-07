pub mod context;
pub mod data_src;
pub mod error_format;
pub mod exp;
pub mod function;
pub mod op;
pub mod runtime_convert;
pub mod statement;

// use crate::ast::stat_expr_types::Block;
// use crate::types::PineRef;
// use context::{Context, ContextType, PineRuntimeError, Runner, VarOperate};
// use std::collections::HashMap;

// pub fn run<'a>(
//     blk: &'a Block<'a>,
//     vars: HashMap<&'a str, PineRef<'a>>,
// ) -> Result<(), PineRuntimeError> {
//     let mut context = Context::new(None, ContextType::Normal);
//     for (k, v) in vars.into_iter() {
//         context.create_var(k, v);
//     }
//     blk.run(&mut context)?;
//     Ok(())
// }
