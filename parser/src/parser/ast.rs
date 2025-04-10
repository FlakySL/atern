// Select
//    All - *
//    From
//      Identifier - Table
use std::iter::Peekable;

use logos::Lexer;
use rowan::{GreenNodeBuilder, NodeOrToken};
use thiserror::Error;

use super::grammar::process_grammar;
use super::grammar::Grammar::*;
use super::grammar::GrammarType::*;
use super::lexer::Token;

#[derive(Error, Debug)]
pub enum AstError {
    #[error("Invalid Token {0}")]
    InvalidToken(String),

    #[error("Trailing Comma is not allowed")]
    TrailingComma,

    #[error("Expected {0} found {1}")]
    ExpectedType(SyntaxKind, SyntaxKind),

    #[error("Unexpected Node {0}")]
    UnexpectedNode(SyntaxKind),

    #[error("Expected Body for {0}")]
    ExpectedBodyFor(SyntaxKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    WHITESPACE = 0,

    SELECT,
    FROM,

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

    EMPTY,
    ROOT,
}

impl SyntaxKind {
    pub fn is_dql(&self) -> bool {
        (2..=2).contains(&(*self as u16))
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

pub struct Parser {
    pub builder: GreenNodeBuilder<'static>,
    iter: Peekable<std::vec::IntoIter<(SyntaxKind, String)>>,
}

impl Parser {
    pub fn peek(&mut self) -> Option<SyntaxKind> {
        while self.iter.peek().map(|&(t, _)| t == WHITESPACE).unwrap_or(false) {
            self.bump();
        }
        self.iter.peek().map(|&(t, _)| t)
    }
    pub fn next(&mut self) {
        self.iter.next();
    }
    pub fn bump(&mut self) {
        if let Some((token, string)) = self.iter.next() {
            self.builder.token(token.into(), string.as_str());
        }
    }
    fn handle_val(&mut self) -> Result<(), AstError> {
        match self.peek().unwrap() {
            SELECT => {
                process_grammar(
                    self,
                    SELECT,
                    &[
                        List(&[IDENTIFIER, ALL]),
                        Loop(
                            Box::from(Combo(true, &[Children(Type(FROM), &[List(&[IDENTIFIER])])])),
                            SEMICOLON,
                        ),
                    ],
                )?;
            },
            n => return Err(AstError::UnexpectedNode(n)),
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
                },
                Err(_) => return Err(AstError::InvalidToken(lex.slice().to_string())),
            }
        }

        Ok(Parser {
            builder: GreenNodeBuilder::new(),
            iter: nodes.into_iter().peekable(),
        })
    }
}
