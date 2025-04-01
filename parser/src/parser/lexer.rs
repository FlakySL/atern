use logos::Logos;

use super::ast::SyntaxKind;

#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("(?i)SELECT")]
    Select,

    #[regex("(?i)FROM")]
    From,

    #[regex(r#"(?:"[^"]*"|'[^']*')"#, |lex| {
        let content = lex.slice();
        content[1..content.len()-1].to_string()
    })]
    Text(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r"[0-9]+", |lex| lex.slice().to_string())]
    Number(String),

    #[token("*")]
    All,

    #[token(",")]
    Comma,
}

impl Token {
    #[inline]
    pub(crate) fn to_syntax(&self) -> (SyntaxKind, String) {
        match &self {
            Token::All => (SyntaxKind::ALL, String::from("*")),
            Token::Select => (SyntaxKind::SELECT, String::from("SELECT")),
            Token::Identifier(i) => (SyntaxKind::IDENTIFIER, String::from(i)),
            Token::From => (SyntaxKind::FROM, String::from("FROM")),
            Token::Text(t) => (SyntaxKind::TEXT, String::from(t)),
            Token::Number(n) => (SyntaxKind::NUMBER, String::from(n)),
            Token::Comma => (SyntaxKind::COMMA, String::from(",")),
        }
    }
}
