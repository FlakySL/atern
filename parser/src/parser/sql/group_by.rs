use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};

pub const GROUP_BY_GRAMMAR: Grammar = Children(&[GROUP, BY], GROUP_BY, &[List(&[IDENTIFIER])]);
