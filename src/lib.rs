mod kinds;
mod green;

pub use green::{Node, NodeData, Token, TokenData};

#[derive(Debug, Clone, Copy)]
pub struct SyntaxKind(u16);

#[derive(Debug, Clone)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{kinds, NodeData, TokenData};

    #[test]
    fn smoke() {
        let ws = Arc::new(TokenData::new(kinds::WHITESPACE, " ".to_string()));
        let one = Arc::new(TokenData::new(kinds::INT, "1".to_string()));
        let star = Arc::new(TokenData::new(kinds::STAR, "*".to_string()));
        let two = Arc::new(TokenData::new(kinds::INT, "2".to_string()));

        // 1 * 2
        let multiplication = Arc::new(NodeData::new(
            kinds::BIN_EXPR,
            vec![one.into(), ws.clone().into(), star.into(), ws.clone().into(), two.into()],
        ));

        let plus = Arc::new(TokenData::new(kinds::PLUS, "+".to_string()));
        // 1 * 2 + 1 * 2
        let addition = Arc::new(NodeData::new(
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
    }
}
