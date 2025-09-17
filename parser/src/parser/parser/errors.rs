use crate::parser::ast::syntax_kind::SyntaxKind;
use thiserror::Error;

/// Possible errors at the time of generating the ast
#[derive(Error, Debug)]
pub enum ParserError<K: SyntaxKind> {
    // TODO: this error should be in LexerError and not in ParserError
    #[error("Invalid Token {0}")]
    InvalidToken(String),

    /// This error is triggered when a trailing comma is left at the time of
    /// enumeration, e.g. 1, 2,
    #[error("Trailing Comma is not allowed")]
    TrailingComma,

    /// This error is triggered when it expects a specific node and receives a
    /// node of another type.
    #[error("Expected {0} found {1}")]
    ExpectedType(K, K),

    /// This error is triggered when the node does not match with the expected
    /// by the context
    #[error("Unexpected Node {0}")]
    UnexpectedNode(K),

    /// This error is triggered when the definition of the context is incomplete
    /// e.g.: SELECT; (without passing any body)
    #[error("Expected Body for {0}")]
    ExpectedBodyFor(K),

    /// this error appears when the content is finished but the current grammar
    /// rule needs more content to complete.
    #[error("Unexpected EOF")]
    UnexpectedEof,
}
