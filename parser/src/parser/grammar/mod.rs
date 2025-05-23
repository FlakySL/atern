mod process;
mod utils;

pub use process::process_grammar;
pub use utils::template::TemplateConfig;

use super::ast::SyntaxKind;

/// Possible grammar rules to be parsed
pub enum Grammar {
    /// Takes a list of nodes that can be in a list, does not support trailing
    /// commas, e.g. List(&[IDENTIFIER, TEXT]), symbolizes a list that can
    /// contain identifiers or text
    List(&'static [SyntaxKind]), // separated by comma
    /// It expects a GrammarType and if it is satisfied it wraps everything in
    /// that type, besides executing more grammar rules.
    Children(&'static [SyntaxKind], SyntaxKind, &'static [Grammar]),
    /// It takes several rules, if a rule gives error it passes with the next
    /// one, if it does not give error it applies it and finishes its execution
    /// if the first parameter is true at least one rule has to be fulfilled, if
    /// it is false all of them can fail.
    Combo(bool, &'static [Grammar]),
    /// It takes a node and verifies that the current node of the parser matches
    /// the node passed in the rule definition, the next parameter is a boolean
    /// that indicates if you want to consume that parameter (add it to the ast)
    /// or not.
    Expect(SyntaxKind, bool),
    /// It takes a template and will compare that template with the current
    /// definition.
    Template(&'static [GrammarType], TemplateConfig),
    /// Execute a rule several times until a specific node at which it will stop
    /// executing that rule (e.g. a semicolon).
    Loop(&'static Grammar, SyntaxKind),

    Optional(&'static Grammar),

    GType(GrammarType),
}

#[derive(Debug, Clone)]
/// It represents a more flexible way of classifying nodes.
pub enum GrammarType {
    /// checks if the node belongs to the Data Query Language group
    Dql,
    /// Checks if a node is equal to the node that was passed in the definition.
    Type(SyntaxKind),
    /// Checks if the node is in the defined node list.
    Multi(&'static [SyntaxKind]),
}

impl PartialEq<SyntaxKind> for GrammarType {
    fn eq(&self, other: &SyntaxKind) -> bool {
        match self {
            Self::Dql => other.is_ddl(),
            Self::Type(t) => t == other,
            Self::Multi(l) => l.contains(other),
        }
    }
}
