use super::{Grammar, GrammarType};
use crate::parser::ast::SyntaxKind::{self, *};
use crate::parser::ast::{AstError, Parser};

pub fn process_grammar(
    parser: &mut Parser,
    father: SyntaxKind,
    grammar: &[Grammar],
) -> Result<(), AstError> {
    parser.builder.start_node_at(parser.builder.checkpoint(), father.into());
    parser.next();

    for rule in grammar {
        process_rule(&rule, father, parser)?;
    }

    parser.builder.finish_node();

    Ok(())
}

fn process_rule(rule: &Grammar, father: SyntaxKind, parser: &mut Parser) -> Result<(), AstError> {
    match rule {
        Grammar::List(t) => {
            process_list(t, father, parser)?;
        },
        Grammar::Loop(child, stop) => {
            process_loop(child, *stop, father, parser)?;
        },
        Grammar::Combo(optional, children) => {
            process_combo(*optional, children, father, parser)?;
        },
        Grammar::Children(start, body) => {
            process_children(start, body, father, parser)?;
        },
    }

    Ok(())
}

fn process_children(
    start: &GrammarType,
    body: &[Grammar],
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), AstError> {
    if start != &parser.peek().unwrap_or(EMPTY) {
        return Err(AstError::UnexpectedNode(parser.peek().unwrap_or(EMPTY)));
    }

    let s = parser.peek().unwrap();
    parser.builder.start_node_at(parser.builder.checkpoint(), s.into());
    parser.next();

    for rule in body {
        process_rule(&rule, father, parser)?;
    }

    parser.builder.finish_node();

    Ok(())
}

fn process_combo(
    optional: bool,
    children: &[Grammar],
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), AstError> {
    let mut good = true;

    for child in children {
        match process_rule(&child, father, parser) {
            Ok(_) => {
                good = true;
                break;
            },
            Err(_) => {
                good = false;
                break;
            },
        };
    }

    if !good && !optional {
        return Err(AstError::ExpectedBodyFor(father));
    }

    if !good {
        parser.next();
    }

    Ok(())
}

fn process_loop(
    child: &Box<Grammar>,
    stop: SyntaxKind,
    father: SyntaxKind,
    parser: &mut Parser,
) -> Result<(), AstError> {
    while parser.peek() != Some(stop) && parser.peek() != None {
        process_rule(&child, father, parser)?;
    }

    Ok(())
}

fn process_list(t: &[SyntaxKind], father: SyntaxKind, parser: &mut Parser) -> Result<(), AstError> {
    if parser.peek() == None || parser.peek() == Some(SEMICOLON) {
        return Err(AstError::ExpectedBodyFor(father));
    }

    while let Some(token) = parser.peek() {
        if token == COMMA {
            parser.next();

            if t.contains(&parser.peek().unwrap_or(EMPTY)) {
                return Err(AstError::TrailingComma);
            }

            continue;
        }

        if !t.contains(&token) {
            break;
        }

        parser.bump();
    }

    Ok(())
}
