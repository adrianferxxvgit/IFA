use ifa_core::{NodeId, Span};

use crate::SyntaxKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxNode {
    id: NodeId,
    kind: SyntaxKind,
    span: Span,
    children: Vec<NodeId>,
}

impl SyntaxNode {
    pub fn new(id: NodeId, kind: SyntaxKind, span: Span) -> Self {
        Self {
            id,
            kind,
            span,
            children: Vec::new(),
        }
    }

    pub const fn id(&self) -> NodeId {
        self.id
    }

    pub const fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub const fn span(&self) -> Span {
        self.span
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    pub fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::SyntaxNode;
    use crate::SyntaxKind;
    use ifa_core::{NodeId, Span};

    #[test]
    fn creates_node() {
        let span = Span::new(0, 10).expect("valid span");
        let node = SyntaxNode::new(NodeId::new(1), SyntaxKind::Indicator, span);

        assert_eq!(node.id(), NodeId::new(1));
        assert_eq!(node.kind(), SyntaxKind::Indicator);
        assert_eq!(node.span(), span);
        assert!(node.children().is_empty());
    }

    #[test]
    fn adds_children() {
        let span = Span::new(0, 10).expect("valid span");
        let mut node = SyntaxNode::new(NodeId::new(1), SyntaxKind::Root, span);

        node.add_child(NodeId::new(2));
        node.add_child(NodeId::new(3));

        assert_eq!(node.children(), &[NodeId::new(2), NodeId::new(3)]);
    }
}
