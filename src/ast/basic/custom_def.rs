use crate::ast::basic::AetherSpan;
use crate::ast::basic::ecs::{QueryBlock, UpdateBlock, WatchBlock};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldDef<'de> {
    pub name: AetherSpan<'de>,
    pub type_spec: AetherSpan<'de>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructDef<'de> {
    pub name: AetherSpan<'de>,
    pub fields: Vec<FieldDef<'de>>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComponentDef<'de> {
    pub name: AetherSpan<'de>,
    pub fields: Vec<FieldDef<'de>>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, PartialEq)]
pub struct SystemDef<'de> {
    pub name: AetherSpan<'de>,
    pub query: QueryBlock<'de>,
    pub update: Option<UpdateBlock<'de>>,
    pub watches: Vec<WatchBlock<'de>>,
    pub span: AetherSpan<'de>
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EventDef<'de> {
    pub name: AetherSpan<'de>,
    pub fields: Vec<FieldDef<'de>>,
    pub span: AetherSpan<'de>
}