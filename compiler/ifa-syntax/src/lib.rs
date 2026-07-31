mod syntax_node;
mod syntax_tree;

use ifa_core::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyntaxKind {
    Root,
    Module,
    Import,
    Indicator,
    Property,
    Input,
    Output,
    Parameter,
    Dependency,
    Contract,
    Equation,
    Identifier,
    Literal,
    FunctionCall,
    MemberAccess,
    UnaryExpression,
    BinaryExpression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyntaxToken {
    kind: SyntaxKind,
    span: Span,
}

impl SyntaxToken {
    pub const fn new(kind: SyntaxKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub const fn kind(self) -> SyntaxKind {
        self.kind
    }

    pub const fn span(self) -> Span {
        self.span
    }
}

pub use syntax_node::SyntaxNode;
pub use syntax_tree::SyntaxTree;

#[cfg(test)]
mod tests {
    use super::{SyntaxKind, SyntaxToken};
    use ifa_core::Span;

    #[test]
    fn syntax_kinds_are_copyable_values() {
        let kind = SyntaxKind::Indicator;
        let copied = kind;

        assert_eq!(kind, copied);
    }

    #[test]
    fn token_preserves_kind_and_span() {
        let span = Span::new(4, 12).expect("valid span");
        let token = SyntaxToken::new(SyntaxKind::Identifier, span);

        assert_eq!(token.kind(), SyntaxKind::Identifier);
        assert_eq!(token.span(), span);
    }

    #[test]
    fn syntax_kinds_are_distinct() {
        assert_ne!(SyntaxKind::Input, SyntaxKind::Output);
        assert_ne!(SyntaxKind::Identifier, SyntaxKind::Literal);
    }
}
