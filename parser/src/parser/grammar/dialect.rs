use crate::parser::ast::syntax_kind::SyntaxKind;

///This trait deals with the grammar of the SQL dialect we want to parse.
///We proly need to add more methods in the future.
pub trait Dialect<K: SyntaxKind>: Sized {
    //fn are_compatible(child: &N, parent: &N) -> Result<(), AstErr>;
}
