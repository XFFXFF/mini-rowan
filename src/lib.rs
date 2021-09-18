/*
Requirements:

* full-fidelity (whitespace & comments are part of the tree)
* resilient & semi-structured (can represent invalid code)
* cheaply updatable (for refactors & increamental reparsing)
* conveniently updatable

* immutable value-type
* easy to navigate (go from node to children, parent, siblings)
 */

pub mod kinds;
mod green;
mod red;
mod ast;

pub use crate::{
    green::{GreenElement, GreenNode, GreenNodeData, GreenToken, GreenTokenData},
    red::{RedNode, RedNodeData, RedToken, RedTokenData},
};

#[derive(Debug, Clone, Copy)]
pub struct SyntaxKind(u16);

#[derive(Debug, Clone)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

impl<N, T> NodeOrToken<N, T> {
    pub fn into_node(self) -> Option<N> {
        match self {
            NodeOrToken::Node(node) => Some(node),
            NodeOrToken::Token(_) => None,
        }
    }
    pub fn into_token(self) -> Option<T> {
        match self {
            NodeOrToken::Node(_) => None,
            NodeOrToken::Token(token) => Some(token),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{kinds, GreenNodeData, GreenTokenData, RedNodeData};

    #[test]
    fn smoke() {
        let ws = Arc::new(GreenTokenData::new(kinds::WHITESPACE, " ".to_string()));
        let one = Arc::new(GreenTokenData::new(kinds::INT, "1".to_string()));
        let star = Arc::new(GreenTokenData::new(kinds::STAR, "*".to_string()));
        let two = Arc::new(GreenTokenData::new(kinds::INT, "2".to_string()));

        // 1 * 2
        let multiplication = Arc::new(GreenNodeData::new(
            kinds::BIN_EXPR,
            vec![one.into(), ws.clone().into(), star.into(), ws.clone().into(), two.into()],
        ));

        let plus = Arc::new(GreenTokenData::new(kinds::PLUS, "+".to_string()));
        // 1 * 2 + 1 * 2
        let addition = Arc::new(GreenNodeData::new(
            kinds::BIN_EXPR,
            vec![
                multiplication.clone().into(),
                ws.clone().into(),
                plus.into(),
                ws.into(),
                multiplication.into(),
            ],
        ));
        println!("{}", addition);

        // let mul2 = addition.children().nth(4).unwrap().into_node().unwrap();
        // let one = mul2.children().nth(0).unwrap().into_token().unwrap();
        // println!("{}", one);

        let addition = RedNodeData::new(addition);
        let mul2 = addition.children().nth(4).unwrap().into_node().unwrap();
        let one2 = mul2.children().nth(0).unwrap().into_token().unwrap();

        println!("\n{} at {}", one2, one2.text_offset());
        let p = one2.parent().unwrap();
        println!("{}", p);

        let three = Arc::new(GreenTokenData::new(kinds::INT, "3".to_string()));
        let addtion_root = mul2.replace_child(0, three.into());
        println!("{}", addtion_root);
    }
}
