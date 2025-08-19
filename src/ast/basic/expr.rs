use ordered_float::OrderedFloat;
use crate::ast::basic::AetherSpan;
use crate::ast::basic::function::ClosureExpr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Less,
    Greater,
    LessEqual,
    GreatEqual,
    Equal,
    NotEqual,
    And,
    Or,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Bool(bool),
    Null
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr<'de> {
    Identifier(AetherSpan<'de>),
    Literal {
        interpreted: Literal,
        span: AetherSpan<'de>
    },
    Binary {
        left: Box<Expr<'de>>,
        op: BinaryOp,
        right: Box<Expr<'de>>,
        span: AetherSpan<'de>
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr<'de>>,
        span: AetherSpan<'de>
    },

    //调用和控制流
    FunctionCall {
        name: AetherSpan<'de>,
        args: Vec<Expr<'de>>,
        span: AetherSpan<'de>
    },
    If {
        cond: Box<Expr<'de>>,
        then_branch: Box<Expr<'de>>,
        else_branch: Option<Box<Expr<'de>>>,
        span: AetherSpan<'de>
    },
    Match {
        value: Box<Expr<'de>>,
        //arms: Vec<(Pattern<'de>, Expr<'de>)>, TODO after Pattern
        span: AetherSpan<'de>
    },
    Block(BlockExpr<'de>),
    Closure(ClosureExpr<'de>),
}

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct BlockExpr<'de> {
    span : AetherSpan<'de>, //TODO after Stmt
}
