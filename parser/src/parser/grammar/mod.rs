mod utils;
mod process;

pub use process::process_grammar;
use utils::template::TemplateConfig;

use super::ast::SyntaxKind;

pub enum Grammar {
    List(&'static [SyntaxKind]), // separated by comma
    Children(GrammarType, &'static [Grammar]),
    Combo(bool, &'static [Grammar]),
    Expect(SyntaxKind, bool),
    Template(&'static [GrammarType], TemplateConfig),
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
