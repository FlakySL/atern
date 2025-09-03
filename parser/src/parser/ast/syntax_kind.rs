/// Nodes that the ast can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    WHITESPACE = 0,

    SELECT,
    FROM,
    WHERE,
    ORDER_BY,
    GROUP_BY,

    CREATE,
    ALTER,
    DROP,
    TRUNCATE,

    TABLE,
    COLUMN,
    COLUMN_CONSTRAINTS,
    
    NAME,
    TYPE,
    TABLE_BODY,

    IDENTIFIER,
    TEXT,
    ALL,
    COMMA,
    NUMBER,
    SEMICOLON,
    PARENTHESES_START,
    PARENTHESES_END,
    VALUES,
    DEFINITION,

    EQUAL,
    GT,
    LT,
    LEQT,
    GEQT,

    AND,
    OR,
    NOT,
    
    NULL,
    NOT_NULL,
    NULLS,
    UNIQUE,
    PRIMARY_KEY,

    TRUE,
    FALSE,
    BOOLEAN_OP,

    GROUP,
    ORDER,
    BY,
    DESC,
    ASC,
    DISTINCT,

    EMPTY,
    ROOT,
}

use SyntaxKind::*;

impl SyntaxKind {
    pub fn is_dql(&self) -> bool {
        //(2..=3).contains(&(*self as u16))
        *self == SyntaxKind::SELECT
    }
    pub fn is_ddl(&self) -> bool {
        //(4..=4).contains(&(*self as u16))
        match *self {
            CREATE | ALTER | DROP | TRUNCATE => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
