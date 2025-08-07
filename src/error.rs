use thiserror::Error;
#[derive(Error, Debug)]
pub enum AethaumError {
    #[error("lexical error")]
    LexicalError(LexicalError),
    #[error("syntax error")]
    SyntaxError(SyntaxError),
    #[error("runtime error")]
    RuntimeError(RuntimeError),
}
#[derive(Error, Debug)]
pub enum LexicalError {
    
}
#[derive(Error, Debug)]
pub enum SyntaxError {
    
}
#[derive(Error, Debug)]
pub enum RuntimeError {
    
}