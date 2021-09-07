use std::fmt;
use std::iter;
use std::sync::Arc;

use crate::{NodeOrToken, SyntaxKind};

pub type GreenToken = Arc<GreenTokenData>;
#[derive(Debug)]
pub struct GreenTokenData {
    kind: SyntaxKind,
    text: String,
}

impl fmt::Display for GreenTokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.text(), f)
    }
}

impl GreenTokenData {
    pub fn new(kind: SyntaxKind, text: String) -> GreenTokenData {
        GreenTokenData { kind, text }
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

pub type GreenNode = Arc<GreenNodeData>;
#[derive(Debug)]
pub struct GreenNodeData {
    kind: SyntaxKind,
    children: Vec<NodeOrToken<GreenNode, GreenToken>>,
    len: usize,
}

impl fmt::Display for GreenNodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in self.children() {
            fmt::Display::fmt(&child, f)?;
        }
        Ok(())
    }
}

impl GreenNodeData {
    pub fn new(
        kind: SyntaxKind,
        children: Vec<NodeOrToken<GreenNode, GreenToken>>,
    ) -> GreenNodeData {
        let len = children.iter().map(|it| it.text_len()).sum();
        GreenNodeData { kind, children, len }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
    pub fn text_len(&self) -> usize {
        self.len
    }
    pub fn children(&self) -> impl Iterator<Item = GreenElement> + '_ {
        self.children.iter().cloned()
    }
    pub fn replace_child(
        &self,
        idx: usize,
        new_child: NodeOrToken<GreenNode, GreenToken>,
    ) -> GreenNodeData {
        assert!(idx < self.children.len());

        let left_children = self.children().take(idx);
        let right_children = self.children().skip(idx + 1);
        let new_children =
            left_children.chain(iter::once(new_child)).chain(right_children).collect();
        GreenNodeData::new(self.kind, new_children)
    }
}

impl From<GreenToken> for NodeOrToken<GreenNode, GreenToken> {
    fn from(token: GreenToken) -> NodeOrToken<GreenNode, GreenToken> {
        NodeOrToken::Token(token)
    }
}

impl From<GreenNode> for NodeOrToken<GreenNode, GreenToken> {
    fn from(node: GreenNode) -> NodeOrToken<GreenNode, GreenToken> {
        NodeOrToken::Node(node)
    }
}

impl fmt::Display for NodeOrToken<GreenNode, GreenToken> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeOrToken::Node(node) => fmt::Display::fmt(node, f),
            NodeOrToken::Token(token) => fmt::Display::fmt(token, f),
        }
    }
}

impl NodeOrToken<GreenNode, GreenToken> {
    pub fn text_len(&self) -> usize {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }
}

pub type GreenElement = NodeOrToken<GreenNode, GreenToken>;
