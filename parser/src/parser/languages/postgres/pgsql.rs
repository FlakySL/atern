use crate::parser::grammar::dialect::Dialect;
use super::pg_kind::PgKind;

pub struct PostgreSQL{
}

impl Dialect<PgKind> for PostgreSQL{
}
