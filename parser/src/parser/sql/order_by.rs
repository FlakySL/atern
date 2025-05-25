use crate::parser::ast::SyntaxKind::*;
use crate::parser::grammar::Grammar::{self, *};
use crate::parser::grammar::GrammarType::*;

pub const ORDER_BY_GRAMMAR: Grammar = Children(&[ORDER, BY], ORDER_BY, &[List(&[IDENTIFIER]),  Optional(&GType(Multi(&[ASC, DESC])))]);

