use crate::ast::basic::AetherSpan;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Primitive {
    Int,
    Float,
    Str,
    Bool,
    Entity
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type<'de> {
    Primitive(Primitive),
    List(Box<Type<'de>>),
    Tuple(Vec<Type<'de>>),
    Dict(Box<Type<'de>>, Box<Type<'de>>),
    Path(AetherSpan<'de>),
    Generic(AetherSpan<'de>, Vec<Type<'de>>),
    Function(Vec<Type<'de>>, Box<Type<'de>>),
    Unknown,
    Error
}