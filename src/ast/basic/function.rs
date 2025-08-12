use crate::ast::basic::aether_type::Type;
use crate::ast::basic::AetherSpan;
use crate::ast::basic::expr::BlockExpr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FuncParam<'de> {
    pub name: AetherSpan<'de>,
    pub type_spec: Type<'de>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClosureParam<'de> {
    pub name: AetherSpan<'de>,
    pub type_spec: Option<Type<'de>>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClosureExpr<'de> {
    pub params: Vec<ClosureParam<'de>>,
    pub body: BlockExpr<'de>,
    pub span: AetherSpan<'de>
}