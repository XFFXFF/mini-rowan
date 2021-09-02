use std::{iter, sync::Arc};

mod kinds;

#[derive(Clone, Copy)]
pub struct SyntaxKind(u16);

#[derive(Clone)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

pub type Token = Arc<TokenData>;
pub struct TokenData {
    kind: SyntaxKind,
    text: String,
}

pub type Node = Arc<NodeData>;
pub struct NodeData {
    kind: SyntaxKind,
    children: Vec<NodeOrToken<Node, Token>>,
    len: usize,
}

impl TokenData {
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

impl NodeOrToken<Node, Token> {
    pub fn text_len(&self) -> usize {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }
}


