// Select
//    All - *
//    From
//      Identifier - Table
use super::lexer::Token;
use logos::Lexer;
use rowan::{GreenNodeBuilder, NodeOrToken};
use std::iter::Peekable;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstError {
    #[error("Invalid Token {0}")]
    InvalidToken(String),

    #[error("Trailing Comma is not allowed")]
    TrailingComma,

    #[error("Expected {0} found {1}")]
    ExpectedType(SyntaxKind, SyntaxKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    WHITESPACE = 0,
    SELECT,
    FROM,
    IDENTIFIER,
    TEXT,
    ALL,
    COMMA,
    NUMBER,
    ROOT,
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

pub struct Parser {
    builder: GreenNodeBuilder<'static>,
    iter: Peekable<std::vec::IntoIter<(SyntaxKind, String)>>,
}

impl Parser {
    fn peek(&mut self) -> Option<SyntaxKind> {
        while self
            .iter
            .peek()
            .map(|&(t, _)| t == WHITESPACE)
            .unwrap_or(false)
        {
            self.bump();
        }
        self.iter.peek().map(|&(t, _)| t)
    }
    fn next(&mut self) {
        self.iter.next();
    }
    fn bump(&mut self) {
        if let Some((token, string)) = self.iter.next() {
            self.builder.token(token.into(), string.as_str());
        }
    }
    fn handle_val(&mut self) -> Result<(), AstError> {
        match self.peek().unwrap() {
            SELECT => {
                self.builder
                    .start_node_at(self.builder.checkpoint(), SELECT.into());
                self.next();
                while self.peek() != Some(FROM.into()) {
                    self.bump();
                }

                self.builder
                    .start_node_at(self.builder.checkpoint(), FROM.into());
                self.next();

                while let Some(token) = self.peek() {
                    if token == COMMA {
                        self.next();
                        if self.peek() != Some(IDENTIFIER) {
                            return Err(AstError::TrailingComma);
                        }
                        continue;
                    }

                    if token != IDENTIFIER {
                        return Err(AstError::ExpectedType(IDENTIFIER, token));
                    }

                    self.bump();
                }

                self.builder.finish_node();

                self.builder.finish_node();
            }
            _ => {}
        }

        Ok(())
    }
    pub fn parse(mut self) -> Result<SyntaxNode, AstError> {
        self.builder.start_node(ROOT.into());

        while let Some(_) = self.peek() {
            self.handle_val()?;
        }

        self.builder.finish_node();

        Ok(SyntaxNode::new_root(self.builder.finish()))
    }
    pub fn from_tokens(lex: &mut Lexer<'_, Token>) -> Result<Parser, AstError> {
        let mut nodes = Vec::new();

        while let Some(token) = lex.next() {
            match token {
                Ok(t) => {
                    nodes.push(t.to_syntax());
                }
                Err(_) => return Err(AstError::InvalidToken(lex.slice().to_string())),
            }
        }

        Ok(Parser {
            builder: GreenNodeBuilder::new(),
            iter: nodes.into_iter().peekable(),
        })
    }
}
