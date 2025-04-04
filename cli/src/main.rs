use sql_parser::{SqlBuilder, parser::ast::{AstError, SyntaxElement, SyntaxKind}};
use rowan::NodeOrToken;

fn print(indent: usize, element: SyntaxElement) {
    let kind: SyntaxKind = element.kind();
    print!("{:indent$}", "", indent = indent);
    match element {
        NodeOrToken::Node(node) => {
            println!("- {:?}", kind);
            for child in node.children_with_tokens() {
                print(indent + 2, child);
            }
        }

        NodeOrToken::Token(token) => println!("- {:?} {:?}", token.text(), kind),
    }
}

fn main() -> Result<(), AstError> {
    let ast = SqlBuilder::from("SELECT * FROM TABLE; SELECT * FROM USERS;".to_string())
        .build()?;
    print(0, ast.into());

    Ok(())
}
