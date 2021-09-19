use std::sync::Arc;

use crate::kinds;
use crate::{GreenElement, GreenNode, GreenNodeData, GreenToken, GreenTokenData, SyntaxKind};
use crate::{RedElement, RedNode, RedNodeData};

trait AstNode {
    fn cast(node: RedNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &RedNode;

    fn child_of_type<C: AstNode>(&self) -> Option<C> {
        self.syntax().children().filter_map(RedElement::into_node).find_map(C::cast)
    }
}

struct Struct(RedNode);
impl AstNode for Struct {
    fn cast(node: RedNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == kinds::STRUCT {
            Some(Struct(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &RedNode {
        &self.0
    }
}

impl Struct {
    fn name(&self) -> Option<Name> {
        self.child_of_type()
    }

    fn fields(&self) -> impl Iterator<Item = Field> + '_ {
        self.syntax().children().filter_map(RedElement::into_node).filter_map(Field::cast)
    }
}

struct Field(RedNode);
impl AstNode for Field {
    fn cast(node: RedNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == kinds::FIELD {
            Some(Field(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &RedNode {
        &self.0
    }
}

impl Field {
    fn name(&self) -> Option<Name> {
        self.child_of_type()
    }
}

struct Name(RedNode);
impl AstNode for Name {
    fn cast(node: RedNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == kinds::NAME {
            Some(Name(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &RedNode {
        &self.0
    }
}

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
    let strukt = Struct::cast(RedNodeData::new(strukt)).unwrap();
    println!("{}", strukt.syntax());
    println!("{}", strukt.name().unwrap().syntax());
    let fields = strukt.fields().collect::<Vec<Field>>();
    let foo_field = fields.first().unwrap();
    println!("{}", foo_field.name().unwrap().syntax());
}
