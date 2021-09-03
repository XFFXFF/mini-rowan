use std::rc::Rc;

use crate::{GreenNode, GreenToken, SyntaxKind};

// Goals:
//  * .parent()
//  * .text_offset()

pub type RedNode = Rc<RedNodeData>;
pub struct RedNodeData {
    parent: Option<RedNode>,
    text_offset: usize,
    green: GreenNode,
}

impl RedNodeData {
    pub fn new(root: GreenNode) -> RedNode {
        Rc::new(RedNodeData { parent: None, text_offset: 0, green: root })
    }
    pub fn kind(&self) -> SyntaxKind {
        self.green.kind()
    }
    pub fn text_len(&self) -> usize {
        self.green.text_len()
    }
    pub fn text_offset(&self) -> usize {
        self.text_offset
    }
    pub fn parent(&self) -> Option<&RedNode> {
        self.parent.as_ref()
    }
}

pub type RedToken = Rc<RedTokenData>;
pub struct RedTokenData {
    parent: Option<RedToken>,
    text_offset: usize,
    green: GreenToken,
}

impl RedTokenData {
    pub fn new(parent: Option<RedToken>, text_offset: usize, green: GreenToken) -> RedToken {
        Rc::new(RedTokenData { parent, text_offset, green })
    }
    pub fn kind(&self) -> SyntaxKind {
        self.green.kind()
    }
    pub fn text_len(&self) -> usize {
        self.green.text_len()
    }
    pub fn text_offset(&self) -> usize {
        self.text_offset
    }
    pub fn parent(&self) -> Option<&RedToken> {
        self.parent.as_ref()
    }
}
