use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};

pub const FROM_GRAMMAR: Grammar = Children(&[FROM], FROM, &[List(&[IDENTIFIER])]);
