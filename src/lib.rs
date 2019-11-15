// #[macro_use]
extern crate nom;

#[macro_use]
extern crate lazy_static;

// mod comment_expr;
mod color;
mod comment;
mod error;
mod name;
mod num;
mod op;
mod stat_expr;
mod stat_expr_types;
mod string;
mod trans;
mod utils;
// mod op_expr;
// mod identifier_expr;

// The integer literal is like \d+_\d+
// named!(pub decimal<usize>,
//     map_res!(
//         map_res!(
//             digit1,
//             str::from_utf8),
//         |s| usize::from_str_radix(s, 10)
//     )
//  );

// named!(
//     int_literal<i32>,
//     map!(
//         do_parse!(sign: opt!(alt!(tag!("+") | tag!("-"))) >> expt: decimal >> (sign, expt)),
//         |(sign, expt): (Option<&[u8]>, usize)| {
//             match sign {
//                 Some(b"+") | None => expt as i32,
//                 Some(b"-") => -(expt as i32),
//                 _ => unreachable!(),
//             }
//         }
//     )
// );

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
