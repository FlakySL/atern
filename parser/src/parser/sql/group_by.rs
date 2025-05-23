use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};
use crate::parser::grammar::GrammarType::*;

pub const GROUP_BY_GRAMMAR: Grammar =
    Children(&[GROUP, BY], GROUP_BY, &[List(&[IDENTIFIER]), Optional(&GType(Multi(&[ASC, DESC])))]);
