use logos::Logos;

use super::ast::SyntaxKind;

/// Possible tokens when parsing the Sql code
#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("(?i)SELECT")]
    Select,

    #[regex("(?i)CREATE")]
    Create,

    #[regex("(?i)TABLE")]
    Table,

    #[regex("(?i)FROM")]
    From,

    #[regex("(?i)WHERE")]
    Where,

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

    #[token("=")]
    Equal,

    #[token(">")]
    GT,

    #[token("<")]
    LT,

    #[token(";")]
    Semicolon,

    #[token("(")]
    ParenthesesStart,

    #[token(")")]
    ParenthesesEnd,
}

impl Token {
    #[inline]
    /// Function used to take a token and its content (if any) and convert it
    /// into a SyntaxKind (enum used by the Parser).
    pub(crate) fn to_syntax(&self) -> (SyntaxKind, String) {
        match &self {
            Token::All => (SyntaxKind::ALL, String::from("*")),
            Token::Select => (SyntaxKind::SELECT, String::from("SELECT")),
            Token::Identifier(i) => (SyntaxKind::IDENTIFIER, String::from(i)),
            Token::From => (SyntaxKind::FROM, String::from("FROM")),
            Token::Text(t) => (SyntaxKind::TEXT, String::from(t)),
            Token::Number(n) => (SyntaxKind::NUMBER, String::from(n)),
            Token::Comma => (SyntaxKind::COMMA, String::from(",")),
            Token::Semicolon => (SyntaxKind::SEMICOLON, String::from(";")),
            Token::Create => (SyntaxKind::CREATE, String::from("CREATE")),
            Token::Table => (SyntaxKind::TABLE, String::from("TABLE")),
            Token::ParenthesesStart => (SyntaxKind::PARENTHESES_START, String::from("(")),
            Token::ParenthesesEnd => (SyntaxKind::PARENTHESES_END, String::from(")")),
            Token::Equal => (SyntaxKind::EQUAL, String::from("=")),
            Token::Where => (SyntaxKind::WHERE, String::from("WHERE")),
            Token::LT => (SyntaxKind::LT, String::from("<")),
            Token::GT => (SyntaxKind::GT, String::from(">")),
        }
    }
}
