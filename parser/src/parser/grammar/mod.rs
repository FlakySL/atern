mod process;
pub use process::process_grammar;

use super::ast::{AstError, Parser, SyntaxKind};

#[derive(Clone)]
pub enum Grammar {
    List(&'static [SyntaxKind]), // separated by comma
    Children(GrammarType, &'static [Grammar]),
    Combo(bool, &'static [Grammar]),
    Loop(Box<Grammar>, SyntaxKind),
}

#[derive(Debug, Clone)]
pub enum GrammarType {
    Dql,
    Type(SyntaxKind),
}

impl PartialEq<SyntaxKind> for GrammarType {
    fn eq(&self, other: &SyntaxKind) -> bool {
        match self {
            Self::Dql => other.is_ddl(),
            Self::Type(t) => t == other,
        }
    }
}
