// Select
//    All - *
//    From
//      Identifier - Table
use std::iter::Peekable;

use logos::Lexer;
use rowan::{GreenNodeBuilder, NodeOrToken};
use thiserror::Error;

use super::grammar::process_grammar;
use super::lexer::Token;
use super::sql::select::SELECT_GRAMMAR;

/// Possible errors at the time of generating the ast
#[derive(Error, Debug)]
pub enum ParserError {
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
    ExpectedType(SyntaxKind, SyntaxKind),

    /// This error is triggered when the node does not match with the expected
    /// by the context
    #[error("Unexpected Node {0}")]
    UnexpectedNode(SyntaxKind),

    /// This error is triggered when the definition of the context is incomplete
    /// e.g.: SELECT; (without passing any body)
    #[error("Expected Body for {0}")]
    ExpectedBodyFor(SyntaxKind),

    /// this error appears when the content is finished but the current grammar
    /// rule needs more content to complete.
    #[error("Unexpected EOF")]
    UnexpectedEof,
}

/// Nodes that the ast can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    WHITESPACE = 0,

    SELECT,
    FROM,
    WHERE,
    ORDER_BY,
    GROUP_BY,

    CREATE,

    TABLE,

    IDENTIFIER,
    TEXT,
    ALL,
    COMMA,
    NUMBER,
    SEMICOLON,
    PARENTHESES_START,
    PARENTHESES_END,
    VALUES,
    DEFINITION,
    EQUAL,
    GT,
    LT,
    AND,
    OR,
    GROUP,
    ORDER,
    BY,
    DESC,
    ASC,
    DISTINCT,

    COMPARE,
    GREATER,
    LESS,
    EMPTY,
    ROOT,
}

impl SyntaxKind {
    pub fn is_dql(&self) -> bool {
        (2..=3).contains(&(*self as u16))
    }
    pub fn is_ddl(&self) -> bool {
        (4..=4).contains(&(*self as u16))
    }
}

impl std::fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
pub type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

/// This struct symbolizes the parser, receives a Peekable that is used to
/// iterate over the nodes and has a GreenNodeBuilder that symbolizes the ast
pub struct Parser {
    pub builder: GreenNodeBuilder<'static>,
    pub iter: Peekable<std::vec::IntoIter<(SyntaxKind, String)>>,
}

impl Parser {
    pub fn peek(&mut self) -> Option<SyntaxKind> {
        while self.iter.peek().map(|&(t, _)| t == WHITESPACE).unwrap_or(false) {
            self.bump();
        }
        self.iter.peek().map(|&(t, _)| t)
    }
    pub fn peek_with_content(&mut self) -> Option<(SyntaxKind, &String)> {
        while self.iter.peek().map(|&(t, _)| t == WHITESPACE).unwrap_or(false) {
            self.bump();
        }
        self.iter.peek().map(|&(t, ref c)| (t, c))
    }
    pub fn next(&mut self) {
        self.iter.next();
    }
    pub fn bump(&mut self) {
        if let Some((token, string)) = self.iter.next() {
            self.builder.token(token.into(), string.as_str());
        }
    }
    fn handle_val(&mut self) -> Result<(), ParserError> {
        match self.peek().unwrap() {
            SELECT => {
                process_grammar(self, SELECT, SELECT_GRAMMAR)?;
            },
            CREATE => {
                process_grammar(self, CREATE, &[])?;
            },
            SEMICOLON => {
                self.next();
            },
            n => return Err(ParserError::UnexpectedNode(n)),
        }

        Ok(())
    }
    /// Parses the entire contents of the iter and returns an ast
    pub fn parse(mut self) -> Result<SyntaxNode, ParserError> {
        self.builder.start_node(ROOT.into());

        while let Some(_) = self.peek() {
            self.handle_val()?;
        }

        self.builder.finish_node();

        Ok(SyntaxNode::new_root(self.builder.finish()))
    }
    /// Receives a lexer and iterates its tokens (making sure they are not an
    /// Err) and returns a Parser.
    pub fn from_tokens(lex: &mut Lexer<'_, Token>) -> Result<Parser, ParserError> {
        let mut nodes = Vec::new();

        while let Some(token) = lex.next() {
            match token {
                Ok(t) => {
                    nodes.push(t.to_syntax());
                },
                Err(_) => return Err(ParserError::InvalidToken(lex.slice().to_string())),
            }
        }

        Ok(Parser {
            builder: GreenNodeBuilder::new(),
            iter: nodes.into_iter().peekable(),
        })
    }
}
