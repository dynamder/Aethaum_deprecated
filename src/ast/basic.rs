use pest::Span;
#[derive(Clone)]
pub struct OwnedSpan {
    input: String,
    start: usize,
    end: usize,
}
impl<'i> From<pest::Span<'i>> for OwnedSpan {
    fn from(span: Span) -> Self {
        Self {
            input: span.as_str().to_string(),
            start: span.start(),
            end: span.end(),
        }
    }
}