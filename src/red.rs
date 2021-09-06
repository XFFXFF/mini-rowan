use std::rc::Rc;
use std::fmt;

use crate::{GreenNode, GreenToken, NodeOrToken, SyntaxKind};

// Goals:
//  * .parent()
//  * .text_offset()

pub type RedNode = Rc<RedNodeData>;
pub struct RedNodeData {
    parent: Option<RedNode>,
    text_offset: usize,
    green: GreenNode,
}

impl fmt::Display for RedNodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.green(), f)
    }
}

impl RedNodeData {
    pub fn new(root: GreenNode) -> RedNode {
        Rc::new(RedNodeData { parent: None, text_offset: 0, green: root })
    }
    fn green(&self) -> &GreenNode {
        &self.green
    }
    pub fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }
    pub fn text_len(&self) -> usize {
        self.green().text_len()
    }
    pub fn text_offset(&self) -> usize {
        self.text_offset
    }
    pub fn parent(&self) -> Option<&RedNode> {
        self.parent.as_ref()
    }
    pub fn children<'a>(self: &'a RedNode) -> impl Iterator<Item = RedElement> + 'a {
        let mut offset_in_parent = 0;
        self.green().children().map(move |green_child| {
            let text_offset = offset_in_parent + self.text_offset();
            offset_in_parent += green_child.text_len();
            match green_child {
                NodeOrToken::Node(node) => Rc::new(RedNodeData {
                    parent: Some(Rc::clone(self)),
                    text_offset,
                    green: node,
                })
                .into(),
                NodeOrToken::Token(token) => Rc::new(RedTokenData {
                    parent: Some(Rc::clone(self)),
                    text_offset,
                    green: token,
                })
                .into(),
            }
        })
    }
}

pub type RedToken = Rc<RedTokenData>;
pub struct RedTokenData {
    parent: Option<RedNode>,
    text_offset: usize,
    green: GreenToken,
}

impl fmt::Display for RedTokenData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.green(), f)
    }
}

impl RedTokenData {
    pub fn new(parent: Option<RedNode>, text_offset: usize, green: GreenToken) -> RedToken {
        Rc::new(RedTokenData { parent, text_offset, green })
    }
    fn green(&self) -> &GreenToken {
        &self.green
    }
    pub fn kind(&self) -> SyntaxKind {
        self.green().kind()
    }
    pub fn text_len(&self) -> usize {
        self.green().text_len()
    }
    pub fn text_offset(&self) -> usize {
        self.text_offset
    }
    pub fn parent(&self) -> Option<&RedNode> {
        self.parent.as_ref()
    }
}

type RedElement = NodeOrToken<RedNode, RedToken>;

impl From<RedNode> for RedElement {
    fn from(node: RedNode) -> RedElement {
        NodeOrToken::Node(node)
    }
}

impl From<RedToken> for RedElement {
    fn from(token: RedToken) -> RedElement {
        NodeOrToken::Token(token)
    }
}
