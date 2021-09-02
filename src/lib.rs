mod kinds;

use std::fmt;
use std::iter;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct SyntaxKind(u16);

pub type Token = Arc<TokenData>;
#[derive(Debug)]
pub struct TokenData {
    kind: SyntaxKind,
    text: String,
}

impl fmt::Display for TokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.text(), f)
    }
}

impl TokenData {
    pub fn new(kind: SyntaxKind, text: String) -> TokenData {
        TokenData { kind, text }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
    pub fn text_len(&self) -> usize {
        self.text.len()
    }
}

pub type Node = Arc<NodeData>;
#[derive(Debug)]
pub struct NodeData {
    kind: SyntaxKind,
    children: Vec<NodeOrToken<Node, Token>>,
    len: usize,
}

impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in self.children() {
            fmt::Display::fmt(child, f)?;
        }
        Ok(())
    }
}

impl NodeData {
    pub fn new(kind: SyntaxKind, children: Vec<NodeOrToken<Node, Token>>) -> NodeData {
        let len = children.iter().map(|it| it.text_len()).sum();
        NodeData { kind, children, len }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub fn text_len(&self) -> usize {
        self.len
    }
    pub fn children(&self) -> &[NodeOrToken<Node, Token>] {
        self.children.as_slice()
    }
    pub fn replace_child(&self, idx: usize, new_child: NodeOrToken<Node, Token>) -> NodeData {
        assert!(idx < self.children().len());

        let left_children = self.children.iter().take(idx).cloned();
        let right_children = self.children.iter().skip(idx + 1).cloned();
        let new_children =
            left_children.chain(iter::once(new_child)).chain(right_children).collect();
        NodeData::new(self.kind, new_children)
    }
}

#[derive(Debug, Clone)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

impl From<Token> for NodeOrToken<Node, Token> {
    fn from(token: Token) -> NodeOrToken<Node, Token> {
        NodeOrToken::Token(token)
    }
}

impl From<Node> for NodeOrToken<Node, Token> {
    fn from(node: Node) -> NodeOrToken<Node, Token> {
        NodeOrToken::Node(node)
    }
}

impl fmt::Display for NodeOrToken<Node, Token> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeOrToken::Node(node) => fmt::Display::fmt(node, f),
            NodeOrToken::Token(token) => fmt::Display::fmt(token, f),
        }
    }
}

impl NodeOrToken<Node, Token> {
    pub fn text_len(&self) -> usize {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }
}

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
