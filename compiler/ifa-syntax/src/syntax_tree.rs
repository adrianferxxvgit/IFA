use ifa_core::{Arena, NodeId, Span};

use crate::{SyntaxKind, SyntaxNode};

#[derive(Debug, Default)]
pub struct SyntaxTree {
    nodes: Arena<SyntaxNode>,
    root: Option<NodeId>,
}

impl SyntaxTree {
    pub const fn new() -> Self {
        Self {
            nodes: Arena::new(),
            root: None,
        }
    }

    pub fn create_node(&mut self, kind: SyntaxKind, span: Span) -> NodeId {
        let expected_id = NodeId::new(self.nodes.len() as u32);
        let node = SyntaxNode::new(expected_id, kind, span);
        let id = self.nodes.insert(node);

        debug_assert_eq!(id, expected_id);

        id
    }

    pub fn set_root(&mut self, root: NodeId) {
        debug_assert!(self.nodes.contains(root));
        self.root = Some(root);
    }

    pub const fn root(&self) -> Option<NodeId> {
        self.root
    }

    pub fn node(&self, id: NodeId) -> Option<&SyntaxNode> {
        self.nodes.get(id)
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut SyntaxNode> {
        self.nodes.get_mut(id)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::SyntaxTree;
    use crate::SyntaxKind;
    use ifa_core::Span;

    #[test]
    fn creates_empty_tree() {
        let tree = SyntaxTree::new();

        assert!(tree.is_empty());
        assert_eq!(tree.root(), None);
    }

    #[test]
    fn creates_and_reads_root_node() {
        let mut tree = SyntaxTree::new();
        let span = Span::new(0, 10).expect("valid span");

        let root = tree.create_node(SyntaxKind::Root, span);
        tree.set_root(root);

        assert_eq!(tree.root(), Some(root));
        assert_eq!(tree.len(), 1);

        let node = tree.node(root).expect("root must exist");

        assert_eq!(node.kind(), SyntaxKind::Root);
        assert_eq!(node.span(), span);
    }

    #[test]
    fn creates_parent_and_child() {
        let mut tree = SyntaxTree::new();
        let parent_span = Span::new(0, 10).expect("valid span");
        let child_span = Span::new(2, 5).expect("valid span");

        let parent = tree.create_node(SyntaxKind::Indicator, parent_span);
        let child = tree.create_node(SyntaxKind::Identifier, child_span);

        tree.node_mut(parent)
            .expect("parent must exist")
            .add_child(child);

        assert_eq!(
            tree.node(parent).expect("parent must exist").children(),
            &[child]
        );
    }
}
