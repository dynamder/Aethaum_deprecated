use crate::ast::basic::AetherSpan;
use crate::ast::basic::expr::Literal;

// 需要完善的模式匹配：
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern<'de> {
    Wildcard(AetherSpan<'de>),              // _
    Identifier(AetherSpan<'de>),            // ident
    Literal(Literal),                       // 1, "hello", true
    Tuple(Vec<Pattern<'de>>),               // (pat1, pat2, ...)
    Struct {                                // Struct { field1, field2: pat }
        name: AetherSpan<'de>,
        fields: Vec<StructPatternField<'de>>,
        span: AetherSpan<'de>
    },
    Enum {                                  // Enum::Variant(pat)
        enum_name: AetherSpan<'de>,
        variant: AetherSpan<'de>,
        pattern: Option<Box<Pattern<'de>>>,
        span: AetherSpan<'de>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructPatternField<'de> {
    pub name: AetherSpan<'de>,
    pub pattern: Option<Pattern<'de>>,  // None 表示 field 简写形式
    pub span: AetherSpan<'de>
}
