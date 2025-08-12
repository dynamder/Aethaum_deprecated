use crate::ast::basic::AetherSpan;
use crate::ast::basic::expr::Expr;
use crate::ast::basic::function::ClosureExpr;
use crate::ast::basic::unit_value::{TimeUnit, TimeUnitValue, UnitValue};

#[derive(Debug, Clone, PartialEq)]
pub struct QueryBlock<'de> {
    pub constraint: ComponentConstraint<'de>,
    pub alias: AetherSpan<'de>,
    pub condition: Option<Expr<'de>>,
    pub span: AetherSpan<'de>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ComponentConstraint<'de> {
    Primitive(Vec<ComponentRef<'de>>),
    List(Box<ComponentConstraint<'de>>),
    Tuple(Vec<ComponentConstraint<'de>>)
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComponentRef<'de> {
    pub name: AetherSpan<'de>,
    pub path: Option<Vec<AetherSpan<'de>>>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UpdateBlock<'de> {
    pub interval : TimeUnitValue,
    pub closure: ClosureExpr<'de>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WatchBlock<'de> {
    pub triggers: Vec<EventTrigger<'de>>,
    pub catch_all: Option<ClosureExpr<'de>>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EventTrigger<'de> {
    pub event: AetherSpan<'de>,
    pub closure: ClosureExpr<'de>,
    pub span: AetherSpan<'de>
}
