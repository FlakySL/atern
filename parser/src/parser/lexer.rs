use logos::Logos;

/// Possible tokens when parsing the Sql code
#[derive(Logos,Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
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

///to_syntax method won't b used in the parser because some tokens
///depend of the context for having a certain syntax kind. The method
///its commented for a posible boilerplate reutilization for methods
///that use this enum
impl Token {
    // #[inline]
    // /// Function used to take a token and its content (if any) and convert it
    // /// into a SyntaxKind (enum used by the Parser).
    // pub(crate) fn to_syntax(&self) -> (SyntaxKind, String) {
    //     match &self {
    //         Token::All => (SyntaxKind::ALL, String::from("*")),
    //         Token::Select => (SyntaxKind::SELECT, String::from("SELECT")),
    //         Token::Identifier(i) => (SyntaxKind::IDENTIFIER, String::from(i)),
    //         Token::From => (SyntaxKind::FROM, String::from("FROM")),
    //         Token::Text(t) => (SyntaxKind::TEXT, String::from(t)),
    //         Token::Number(n) => (SyntaxKind::NUMBER, String::from(n)),
    //         Token::Comma => (SyntaxKind::COMMA, String::from(",")),
    //         Token::Semicolon => (SyntaxKind::SEMICOLON, String::from(";")),
    //         Token::Create => (SyntaxKind::CREATE, String::from("CREATE")),
    //         Token::Table => (SyntaxKind::TABLE, String::from("TABLE")),
    //         Token::ParenthesesStart => (SyntaxKind::PARENTHESES_START, String::from("(")),
    //         Token::ParenthesesEnd => (SyntaxKind::PARENTHESES_END, String::from(")")),
    //         Token::Equal => (SyntaxKind::EQUAL, String::from("=")),
    //         Token::Where => (SyntaxKind::WHERE, String::from("WHERE")),
    //         Token::LT => (SyntaxKind::LT, String::from("<")),
    //         Token::GT => (SyntaxKind::GT, String::from(">")),
    //         Token::LEQT => (SyntaxKind::LT, String::from("<=")),
    //         Token::GEQT => (SyntaxKind::GT, String::from(">=")),
    //         Token::And => (SyntaxKind::AND, String::from("AND")),
    //         Token::Or => (SyntaxKind::OR, String::from("OR")),
    //         Token::Not => (SyntaxKind::NOT, String::from("NOT")),
    //         Token::Group => (SyntaxKind::GROUP, String::from("GROUP")),
    //         Token::By => (SyntaxKind::BY, String::from("BY")),
    //         Token::Desc => (SyntaxKind::DESC, String::from("DESC")),
    //         Token::ASC => (SyntaxKind::ASC, String::from("ASC")),
    //         Token::Order => (SyntaxKind::ORDER, String::from("ORDER")),
    //         Token::Distinct => (SyntaxKind::DISTINCT, String::from("DISTINCT")),
    //     }
    // }
}
