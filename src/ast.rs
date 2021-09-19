use std::sync::Arc;

use crate::kinds;
use crate::{GreenElement, GreenNode, GreenNodeData, GreenToken, GreenTokenData, SyntaxKind};

fn make_token(kind: SyntaxKind, text: &str) -> GreenToken {
    Arc::new(GreenTokenData::new(kind, text.to_string()))
}

fn make_node(kind: SyntaxKind, children: Vec<GreenElement>) -> GreenNode {
    Arc::new(GreenNodeData::new(kind, children))
}

fn make_whitespace(ws: &str) -> GreenToken {
    make_token(kinds::WHITESPACE, ws)
}

fn make_field(name: &str, ty: &str) -> GreenNode {
    Arc::new(GreenNodeData::new(
        kinds::FIELD,
        vec![
            make_whitespace("    ").into(),
            make_name(name).into(),
            make_token(kinds::COLON, ":").into(),
            make_whitespace(" ").into(),
            make_node(kinds::TYPE, vec![make_token(kinds::IDENT, ty).into()]).into(),
            make_token(kinds::COMA, ";").into(),
            make_whitespace("\n").into(),
        ],
    ))
}

fn make_name(name: &str) -> GreenNode {
    make_node(kinds::NAME, vec![make_token(kinds::IDENT, name).into()])
}

fn make_struct(name: &str, fields: Vec<GreenNode>) -> GreenNode {
    let mut children: Vec<GreenElement> = Vec::new();
    children.push(make_token(kinds::STRUCT_KW, "struct").into());
    children.push(make_whitespace(" ").into());
    children.push(make_name(name).into());
    children.push(make_whitespace(" ").into());
    children.push(make_token(kinds::L_CURLY, "{").into());
    children.push(make_whitespace("\n").into());
    children.extend(fields.into_iter().map(GreenElement::from));
    children.push(make_token(kinds::R_CURLY, "}").into());
    make_node(kinds::STRUCT, children)
}

#[test]
fn test_struct() {
    let strukt = make_struct("Foo", vec![make_field("foo", "String"), make_field("bar", "Int")]);
    println!("{:#?}", strukt);
}
