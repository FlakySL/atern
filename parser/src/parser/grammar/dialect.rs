//use crate::parser::ast::errors::AstErr;

///This trait deals with the grammar of the SQL dialect we want to parse.
///We proly need to add more methods in the future.
pub trait Dialect<N>: Sized {
    //fn are_compatible(child: &N, parent: &N) -> Result<(), AstErr>;
}
