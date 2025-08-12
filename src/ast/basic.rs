mod expr;
mod aether_type;
mod custom_def;
mod ecs;
mod unit_value;
mod function;

use std::borrow::Cow;
use pest::Span;
#[derive(Debug,Clone, Eq, PartialEq)]
pub struct AetherSpan<'de> {
    input: Cow<'de, str>,
    start: usize,
    end: usize,
}
impl<'de> From<pest::Span<'de>> for AetherSpan<'de> {
    fn from(span: Span<'de>) -> Self {
        Self {
            input: Cow::Borrowed(span.as_str()),
            start: span.start(),
            end: span.end(),
        }
    }
}

