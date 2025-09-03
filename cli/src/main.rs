use sql_parser::{parse, pg_ast::PgAst, pg_parser::PgParser};
// fn print(indent: usize, element: SyntaxElement) {
//     let kind: SyntaxKind = element.kind();
//     print!("{:indent$}", "", indent = indent);
//     match element {
//         NodeOrToken::Node(node) => {
//             println!("- {:?}", kind);
//             for child in node.children_with_tokens() {
//                 print(indent + 2, child);
//             }
//         },
//
//         NodeOrToken::Token(token) => println!("- {:?} {:?}", token.text(), kind),
//     }
// }
//
// fn main() -> Result<(), ParserError> {
//     let ast =
//         SqlBuilder::from("SELECT DISTINCT FROM Users WHERE Country='USA' ORDER BY age DESC".to_string()).build()?;
//     print(0, ast.into());
//
//     Ok(())
// }
//

fn main() {
    let parsed = parse::<_, PgParser, _, _, PgAst>("CREATE TABLE JAIMANITAS(ID INT UNIQUE NULLS DISTINCT, NAME STRING);");
    match parsed {
        Ok(val) => println!("{}", val),
        Err(errs) => println!("{:?}", errs)
    }
   
}
