use crate::parser::ast::syntax_kind::SyntaxKind;
/// Nodes that the ast can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum PgKind {
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
    
    ON,

    NOT_EQUAL,
    EQUAL,
    GT,
    LT,
    LEQT,
    GEQT,

    AND,
    OR,
    NOT,
    
    ADD,
    SUB,
    MUL,
    DIV,
    MODULO,
    EXPONENTIAL,
    
    NEG,
    POS,
    
    NULL,
    NOT_NULL,
    NULLS,
    UNIQUE,
    PRIMARY_KEY,
    FOREGEIN_KEY,

    TRUE,
    FALSE,
    STRING,
    BOOLEAN_OP,

    GROUP,
    ORDER,
    BY,
    DESC,
    ASC,
    DISTINCT,

    EMPTY,
    ROOT,
    
    MATCH_SIMPLE,
    MATCH_PARTIAL,
    MATCH_FULL,
    
    ON_DELETE,
    ON_UPDATE,
    
    CASCADE,
    SET_NULL,
    SET_DEFAULT,
    NO_ACTION,
    RESTRICT,
    
    TABLE_COL,
    EXPRESSION,
    CHECK,
}

impl SyntaxKind for PgKind{
}

use PgKind::*;

impl PgKind {
    pub fn is_dql(&self) -> bool {
        //(2..=3).contains(&(*self as u16))
        *self == PgKind::SELECT
    }
    pub fn is_ddl(&self) -> bool {
        //(4..=4).contains(&(*self as u16))
        match *self {
            CREATE | ALTER | DROP | TRUNCATE => true,
            _ => false,
        }
    }
}

impl Default for PgKind{
    fn default() -> PgKind{
        ROOT
    }
}

impl std::fmt::Display for PgKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}