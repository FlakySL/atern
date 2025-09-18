use logos::Logos;

/// Possible tokens when parsing the Sql code
#[derive(Logos,Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum PgToken {
    #[regex("(?i)SELECT")]
    Select,

    #[regex("(?i)CREATE")]
    Create,

    #[regex("(?i)DELETE")]
    Delete,
    
    #[regex("(?i)UPDATE")]
    Update,
    
    #[regex("(?i)ALTER")]
    Alter,

    #[regex("(?i)TABLE")]
    Table,

    #[regex("(?i)FROM")]
    From,

    #[regex("(?i)WHERE")]
    Where,

    #[regex("(?i)AND")]
    And,

    #[regex("(?i)OR")]
    Or,

    #[regex("(?i)NOT")]
    Not,
    
    #[regex("(?i)TRUE")]
    True,
    
    #[regex("(?i)FALSE")]
    False,
    
    #[regex("(?i)NO")]
    No,

    #[regex("(?i)GROUP")]
    Group,

    #[regex("(?i)DISTINCT")]
    Distinct,
    
    #[regex("(?i)RESTRICT")]
    Restrict,
    
    #[regex("(?i)FULL")]
    Full,
    
    #[regex("(?i)SIMPLR")]
    Simple,
    
    #[regex("(?i)PARTIAL")]
    Partial,    
    
    #[regex("(?i)CASCADE")]
    Cascade,

    #[regex("(?i)DEFAULT")]
    Default,

    #[regex("(?i)ASC")]
    ASC,

    #[regex("(?i)DESC")]
    Desc,

    #[regex("(?i)ORDER")]
    Order,

    #[regex("(?i)BY")]
    By,
    
    #[regex("(?i)ON")]
    On,  
    
    #[regex("(?i)SET")]
    Set,  
    
    #[regex("(?i)CHECK")]
    Check,
    
    #[regex("(?i)ACTION")]
    Action,
    
    #[regex("(?i)NULL")]
    Null,
    
    #[regex("(?i)NULLS")]
    Nulls,
    
    #[regex("(?i)KEY")]
    Key,
    
    #[regex("(?i)PRIMARY")]
    Primary,
    
    #[regex("(?i)UNIQUE")]
    Unique,
    
    #[regex("(?i)REFERENCES")]
    References,

    #[regex("(?i)MATCH")]
    Match,

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
    Star,
    
    #[token("/")]
    Slash,
    
    #[token("%")]
    Mod,
    
    #[token("^")]
    Exp,
    
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,

    #[token(",")]
    Comma,
    
    #[token(".")]
    Dot,

    #[token("=")]
    Equal,
    
    #[regex(r"(!=)|(<>)")]
    UnEqual,

    #[token(">=")]
    GEQT,

    #[token("<=")]
    LEQT,

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
    
    Error
}