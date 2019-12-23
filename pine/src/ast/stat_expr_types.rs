use super::color::ColorNode;
use super::input::{Input, Position, StrRange};
use super::name::VarName;
use super::num::Numeral;
use super::op::{BinaryOp, BinaryOpNode, UnaryOp, UnaryOpNode};
use super::string::StringNode;
use super::syntax_type::{FunctionType, SyntaxType};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall<'a> {
    pub method: Exp<'a>,
    pub pos_args: Vec<Exp<'a>>,
    pub dict_args: Vec<(VarName<'a>, Exp<'a>)>,
    pub ctxid: i32,
    pub range: StrRange,
    pub func_type: Option<FunctionType<'a>>,
}

impl<'a> FunctionCall<'a> {
    #[inline]
    pub fn new(
        method: Exp<'a>,
        pos_args: Vec<Exp<'a>>,
        dict_args: Vec<(VarName<'a>, Exp<'a>)>,
        ctxid: i32,
        range: StrRange,
    ) -> Self {
        FunctionCall {
            method,
            pos_args,
            dict_args,
            ctxid,
            range,
            func_type: None,
        }
    }

    pub fn new_no_ctxid(
        method: Exp<'a>,
        pos_args: Vec<Exp<'a>>,
        dict_args: Vec<(VarName<'a>, Exp<'a>)>,
        range: StrRange,
    ) -> Self {
        FunctionCall {
            method,
            pos_args,
            dict_args,
            ctxid: 0,
            range,
            func_type: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RefCall<'a> {
    pub name: Exp<'a>,
    pub arg: Exp<'a>,
    pub range: StrRange,
}

impl<'a> RefCall<'a> {
    #[inline]
    pub fn new(name: Exp<'a>, arg: Exp<'a>, range: StrRange) -> RefCall<'a> {
        RefCall { name, arg, range }
    }

    pub fn new_no_input(name: Exp<'a>, arg: Exp<'a>) -> RefCall<'a> {
        RefCall {
            name,
            arg,
            range: StrRange::new_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Condition<'a> {
    pub cond: Exp<'a>,
    pub exp1: Exp<'a>,
    pub exp2: Exp<'a>,
    pub range: StrRange,
    pub result_type: SyntaxType<'a>,
}

impl<'a> Condition<'a> {
    #[inline]
    pub fn new(cond: Exp<'a>, exp1: Exp<'a>, exp2: Exp<'a>, range: StrRange) -> Condition<'a> {
        Condition {
            cond,
            exp1,
            exp2,
            range,
            result_type: SyntaxType::Any,
        }
    }

    #[inline]
    pub fn new_no_input(cond: Exp<'a>, exp1: Exp<'a>, exp2: Exp<'a>) -> Condition<'a> {
        Condition {
            cond,
            exp1,
            exp2,
            range: StrRange::new_empty(),
            result_type: SyntaxType::Any,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NaNode {
    pub range: StrRange,
}

impl NaNode {
    pub fn new(range: StrRange) -> NaNode {
        NaNode { range }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoolNode {
    pub value: bool,
    pub range: StrRange,
}

impl BoolNode {
    pub fn new(value: bool, range: StrRange) -> BoolNode {
        BoolNode { value, range }
    }

    pub fn new_no_range(value: bool) -> BoolNode {
        BoolNode {
            value,
            range: StrRange::new_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExp<'a> {
    pub op: UnaryOp,
    pub exp: Exp<'a>,
    pub range: StrRange,
}

impl<'a> UnaryExp<'a> {
    pub fn new(op: UnaryOp, exp: Exp<'a>, range: StrRange) -> UnaryExp<'a> {
        UnaryExp { op, exp, range }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExp<'a> {
    pub op: BinaryOp,
    pub exp1: Exp<'a>,
    pub exp2: Exp<'a>,
    pub range: StrRange,
    pub ref_type: SyntaxType<'a>,
    pub result_type: SyntaxType<'a>,
}

impl<'a> BinaryExp<'a> {
    pub fn new(op: BinaryOp, exp1: Exp<'a>, exp2: Exp<'a>, range: StrRange) -> BinaryExp<'a> {
        BinaryExp {
            op,
            exp1,
            exp2,
            range,
            ref_type: SyntaxType::Any,
            result_type: SyntaxType::Any,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TupleNode<'a> {
    pub exps: Vec<Exp<'a>>,
    pub range: StrRange,
}

impl<'a> TupleNode<'a> {
    pub fn new(exps: Vec<Exp<'a>>, range: StrRange) -> TupleNode<'a> {
        TupleNode { exps, range }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LVTupleNode<'a> {
    pub names: Vec<VarName<'a>>,
    pub range: StrRange,
}

impl<'a> LVTupleNode<'a> {
    pub fn new(names: Vec<VarName<'a>>, range: StrRange) -> LVTupleNode<'a> {
        LVTupleNode { names, range }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp<'a> {
    Na(NaNode),
    Bool(BoolNode),
    Num(Numeral),
    Str(StringNode),
    Color(ColorNode<'a>),
    VarName(VarName<'a>),
    // RetTuple(Box<Vec<VarName<'a>>>),
    Tuple(Box<TupleNode<'a>>),
    TypeCast(Box<TypeCast<'a>>),
    FuncCall(Box<FunctionCall<'a>>),
    RefCall(Box<RefCall<'a>>),
    PrefixExp(Box<PrefixExp<'a>>),
    Condition(Box<Condition<'a>>),
    Ite(Box<IfThenElse<'a>>),
    ForRange(Box<ForRange<'a>>),
    UnaryExp(Box<UnaryExp<'a>>),
    BinaryExp(Box<BinaryExp<'a>>),
}

impl<'a> Exp<'a> {
    pub fn range(&self) -> StrRange {
        match self {
            Exp::Na(na) => na.range,
            Exp::Bool(node) => node.range,
            Exp::Num(node) => node.range(),
            Exp::Str(node) => node.range,
            Exp::Color(node) => node.range,
            Exp::VarName(node) => node.range,
            Exp::Tuple(node) => node.range,
            Exp::TypeCast(node) => node.range,
            Exp::FuncCall(node) => node.range,
            Exp::RefCall(node) => node.range,
            Exp::PrefixExp(node) => node.range,
            Exp::Condition(node) => node.range,
            Exp::Ite(node) => node.range,
            Exp::ForRange(node) => node.range,
            Exp::UnaryExp(node) => node.range,
            Exp::BinaryExp(node) => node.range,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpOrExp2<'a> {
    Op(UnOrBinOp),
    Exp2(Exp2<'a>),
}

impl<'a> OpOrExp2<'a> {
    pub fn range(&self) -> StrRange {
        match self {
            OpOrExp2::Op(op) => op.range(),
            OpOrExp2::Exp2(exp) => exp.range(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnOrBinOp {
    UnaryOp(UnaryOpNode),
    BinaryOp(BinaryOpNode),
}

impl UnOrBinOp {
    pub fn range(&self) -> StrRange {
        match self {
            UnOrBinOp::UnaryOp(node) => node.range,
            UnOrBinOp::BinaryOp(node) => node.range,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlatExp<'a> {
    pub exps: Vec<OpOrExp2<'a>>,
    pub range: StrRange,
}

pub struct UnOpExp2<'a> {
    pub ops: Vec<UnaryOpNode>,
    pub exp: Exp2<'a>,
    pub range: StrRange,
}

impl<'a> UnOpExp2<'a> {
    pub fn new(ops: Vec<UnaryOpNode>, exp: Exp2<'a>, range: StrRange) -> UnOpExp2<'a> {
        UnOpExp2 { ops, exp, range }
    }
}

impl<'a> FlatExp<'a> {
    pub fn new(exps: Vec<OpOrExp2<'a>>, range: StrRange) -> FlatExp<'a> {
        FlatExp { exps, range }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp2<'a> {
    Na(NaNode),
    Bool(BoolNode),
    Num(Numeral),
    Str(StringNode),
    Color(ColorNode<'a>),
    VarName(VarName<'a>),
    // RetTuple(Box<Vec<VarName<'a>>>),
    Tuple(Box<TupleNode<'a>>),
    TypeCast(Box<TypeCast<'a>>),
    FuncCall(Box<FunctionCall<'a>>),
    RefCall(Box<RefCall<'a>>),
    PrefixExp(Box<PrefixExp<'a>>),
    Exp(Exp<'a>),
}

impl<'a> Exp2<'a> {
    pub fn range(&self) -> StrRange {
        match self {
            Exp2::Na(na) => na.range,
            Exp2::Bool(node) => node.range,
            Exp2::Num(node) => node.range(),
            Exp2::Str(node) => node.range,
            Exp2::Color(node) => node.range,
            Exp2::VarName(node) => node.range,
            Exp2::Tuple(node) => node.range,
            Exp2::TypeCast(node) => node.range,
            Exp2::FuncCall(node) => node.range,
            Exp2::RefCall(node) => node.range,
            Exp2::PrefixExp(node) => node.range,
            Exp2::Exp(node) => node.range(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeCast<'a> {
    pub data_type: DataType,
    pub exp: Exp<'a>,
    pub range: StrRange,
}

impl<'a> TypeCast<'a> {
    pub fn new(data_type: DataType, exp: Exp<'a>, range: StrRange) -> TypeCast<'a> {
        TypeCast {
            data_type,
            exp,
            range,
        }
    }

    pub fn new_no_input(data_type: DataType, exp: Exp<'a>) -> TypeCast<'a> {
        TypeCast {
            data_type,
            exp,
            range: StrRange::new_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixExp<'a> {
    pub var_chain: Vec<VarName<'a>>,
    pub range: StrRange,
}

impl<'a> PrefixExp<'a> {
    pub fn new(var_chain: Vec<VarName<'a>>, range: StrRange) -> PrefixExp<'a> {
        PrefixExp { var_chain, range }
    }

    pub fn new_no_input(var_chain: Vec<VarName<'a>>) -> PrefixExp<'a> {
        PrefixExp {
            var_chain,
            range: StrRange::new_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Float,
    Int,
    Bool,
    Color,
    String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment<'a> {
    pub names: Vec<VarName<'a>>,
    pub val: Exp<'a>,
    pub var_type: Option<DataType>,
    pub var: bool,
    pub range: StrRange,
}

impl<'a> Assignment<'a> {
    pub fn new(
        names: Vec<VarName<'a>>,
        val: Exp<'a>,
        var: bool,
        var_type: Option<DataType>,
        range: StrRange,
    ) -> Assignment<'a> {
        Assignment {
            names,
            val,
            var,
            var_type,
            range,
        }
    }

    pub fn new_no_input(
        names: Vec<VarName<'a>>,
        val: Exp<'a>,
        var: bool,
        var_type: Option<DataType>,
    ) -> Assignment<'a> {
        Assignment {
            names,
            val,
            var,
            var_type,
            range: StrRange::new_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarAssignment<'a> {
    pub name: VarName<'a>,
    pub val: Exp<'a>,
    pub range: StrRange,
}

impl<'a> VarAssignment<'a> {
    pub fn new(name: VarName<'a>, val: Exp<'a>, range: StrRange) -> VarAssignment<'a> {
        VarAssignment { name, val, range }
    }

    pub fn new_no_input(name: VarName<'a>, val: Exp<'a>) -> VarAssignment<'a> {
        VarAssignment {
            name,
            val,
            range: StrRange::new_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    pub stmts: Vec<Statement<'a>>,
    pub ret_stmt: Option<Exp<'a>>,
    pub range: StrRange,
}

impl<'a> Block<'a> {
    pub fn new(stmts: Vec<Statement<'a>>, ret_stmt: Option<Exp<'a>>, range: StrRange) -> Block<'a> {
        Block {
            stmts,
            ret_stmt,
            range,
        }
    }

    pub fn new_no_input(stmts: Vec<Statement<'a>>, ret_stmt: Option<Exp<'a>>) -> Block<'a> {
        Block {
            stmts,
            ret_stmt,
            range: StrRange::new_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfThenElse<'a> {
    pub cond: Exp<'a>,
    pub then_blk: Block<'a>,
    pub then_ctxid: i32,
    // pub elseifs: Vec<(Exp<'a>, Block<'a>)>,
    pub else_blk: Option<Block<'a>>,
    pub else_ctxid: i32,
    pub range: StrRange,
    pub result_type: SyntaxType<'a>,
}

impl<'a> IfThenElse<'a> {
    pub fn new(
        cond: Exp<'a>,
        then_blk: Block<'a>,
        else_blk: Option<Block<'a>>,
        then_ctxid: i32,
        else_ctxid: i32,
        range: StrRange,
    ) -> Self {
        IfThenElse {
            cond,
            then_blk,
            then_ctxid,
            else_blk,
            else_ctxid,
            range,
            result_type: SyntaxType::Any,
        }
    }

    pub fn new_no_ctxid(
        cond: Exp<'a>,
        then_blk: Block<'a>,
        else_blk: Option<Block<'a>>,
        range: StrRange,
    ) -> Self {
        IfThenElse {
            cond,
            then_blk,
            else_blk,
            then_ctxid: 0,
            else_ctxid: 1,
            range,
            result_type: SyntaxType::Any,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForRange<'a> {
    pub var: VarName<'a>,
    pub start: Exp<'a>,
    pub end: Exp<'a>,
    pub step: Option<Exp<'a>>,
    pub do_blk: Block<'a>,
    pub ctxid: i32,
    pub range: StrRange,
    pub result_type: SyntaxType<'a>,
}

impl<'a> ForRange<'a> {
    pub fn new(
        var: VarName<'a>,
        start: Exp<'a>,
        end: Exp<'a>,
        step: Option<Exp<'a>>,
        do_blk: Block<'a>,
        ctxid: i32,
        range: StrRange,
    ) -> Self {
        ForRange {
            var,
            start,
            end,
            step,
            do_blk,
            ctxid,
            range,
            result_type: SyntaxType::Any,
        }
    }

    pub fn new_no_ctxid(
        var: VarName<'a>,
        start: Exp<'a>,
        end: Exp<'a>,
        step: Option<Exp<'a>>,
        do_blk: Block<'a>,
        range: StrRange,
    ) -> Self {
        ForRange {
            var,
            start,
            end,
            step,
            do_blk,
            ctxid: 0,
            range,
            result_type: SyntaxType::Any,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDef<'a> {
    pub name: VarName<'a>,
    pub params: Vec<VarName<'a>>,
    pub body: Block<'a>,
    pub range: StrRange,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'a> {
    Break(StrRange),
    Continue(StrRange),
    None(StrRange),
    Assignment(Box<Assignment<'a>>),
    VarAssignment(Box<VarAssignment<'a>>),
    Ite(Box<IfThenElse<'a>>),
    ForRange(Box<ForRange<'a>>),
    FuncCall(Box<FunctionCall<'a>>),
    FuncDef(Box<FunctionDef<'a>>),
}

impl<'a> Statement<'a> {
    pub fn range(&self) -> StrRange {
        match self {
            &Statement::Break(range) => range,
            &Statement::Continue(range) => range,
            &Statement::None(range) => range,
            Statement::Assignment(assign) => assign.range,
            Statement::VarAssignment(assign) => assign.range,
            Statement::Ite(ite) => ite.range,
            Statement::ForRange(for_range) => for_range.range,
            Statement::FuncCall(func_call) => func_call.range,
            Statement::FuncDef(func_def) => func_def.range,
        }
    }
}
