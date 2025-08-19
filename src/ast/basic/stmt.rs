use crate::ast::basic::aether_type::Type;
use crate::ast::basic::AetherSpan;
use crate::ast::basic::expr::Expr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StmtKind<'de> {
    Let(LetStmt<'de>),
    Expression(Expr<'de>),
    Return(Option<Expr<'de>>),
    Summon(SummonStmt<'de>),
    Import(ImportStmt<'de>)
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LetStmt<'de> {
    pub ident: AetherSpan<'de>,
    pub type_spec: Type<'de>,
    pub init_value: Option<Expr<'de>>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SummonStmt<'de> {
    pub ident: AetherSpan<'de>,
    pub base_init: Option<Expr<'de>>,//TODO: find a proper representation
    pub extra_comp: Option<Expr<'de>>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ImportStmt<'de> {
    pub path: Vec<AetherSpan<'de>>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Stmt<'de> {
    pub kind: StmtKind<'de>,
    pub span: AetherSpan<'de>
}