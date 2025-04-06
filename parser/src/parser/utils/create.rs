use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{AstError, Parser};

fn expect_token_condition<F>(parser: &mut Parser, condition: F) -> Result<(), AstError>
where
    F: Fn(SyntaxKind) -> bool,
{
    match parser.peek() {
        Some(t) if condition(t) => {
            parser.bump();
            Ok(())
        }
        Some(t) => Err(AstError::UnexpectedNode(t)),
        None => Err(AstError::ExpectedBodyFor(CREATE)),
    }
}

fn expect_token_eq(parser: &mut Parser, expected: SyntaxKind) -> Result<(), AstError> {
    match parser.peek() {
        Some(t) if t == expected => {
            parser.bump();
            Ok(())
        }
        Some(t) => Err(AstError::ExpectedType(expected, t)),
        None => Err(AstError::ExpectedBodyFor(CREATE)),
    }
}

pub fn process_create(parser: &mut Parser) -> Result<(), AstError> {
    parser.builder.start_node_at(parser.builder.checkpoint(), CREATE.into());
    
    parser.next();

    expect_token_condition(parser, |t| t.is_ddl())?;
    
    expect_token_eq(parser, IDENTIFIER)?;
    
    expect_token_eq(parser, PARENTHESES_START)?;

    parser.builder.finish_node();
    Ok(())
}


