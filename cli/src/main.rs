use sql_parser::{parse, pg_ast::PgAst, pg_parser::PgParser};

fn main() {
    let parsed = parse::<_, PgParser, _, _, PgAst>("CREATE TABLE JAIMANITAS(
            ID INT UNIQUE NULLS DISTINCT PRIMARY KEY,
            NAME STRING NOT NULL, 
            CAR INT References parking_lot (Car));");
    
    
    match parsed {
        Ok(val) => println!("{}", val),
        Err(errs) => println!("{:?}", errs)
    }
   
}
