use rowan::NodeOrToken;
use sql_parser::parser::ast::{ParserError, SyntaxElement, SyntaxKind};
use sql_parser::SqlBuilder;

fn print(indent: usize, element: SyntaxElement) {
    let kind: SyntaxKind = element.kind();
    print!("{:indent$}", "", indent = indent);
    match element {
        NodeOrToken::Node(node) => {
            println!("- {:?}", kind);
            for child in node.children_with_tokens() {
                print(indent + 2, child);
            }
        },

        NodeOrToken::Token(token) => println!("- {:?} {:?}", token.text(), kind),
    }
}

fn main() -> Result<(), ParserError> {
    let ast =
        SqlBuilder::from("SELECT * FROM Customers WHERE Country='Mexico';".to_string()).build()?;
    print(0, ast.into());

    Ok(())
}
