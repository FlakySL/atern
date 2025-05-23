use crate::parser::ast::{Parser, ParserError};
use crate::parser::grammar::GrammarType;

pub fn process_gtype(gtype: &GrammarType, parser: &mut Parser) -> Result<(), ParserError> {
    if parser.peek().is_none() {
        return Err(ParserError::UnexpectedEof);
    }

    if gtype != &parser.peek().unwrap() {
        return Err(ParserError::UnexpectedNode(parser.peek().unwrap()));
    }

    parser.bump();

    Ok(())
}
