use crate::parser::{ast::{Parser, ParserError}, grammar::GrammarType};

pub fn process_type(gtype: &GrammarType, parser: &mut Parser) -> Result<(), ParserError> {
    if parser.peek().is_none() {
        return Err(ParserError::UnexpectedEof);
    }

    if gtype != &parser.peek().unwrap() {
        return Err(ParserError::UnexpectedNode(parser.peek().unwrap()));
    }

    Ok(())
}
