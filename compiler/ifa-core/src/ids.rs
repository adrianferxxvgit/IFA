use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u32);

impl NodeId {
    pub const ROOT: Self = Self(0);

    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn index(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("NodeId").field(&self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::NodeId;

    #[test]
    fn creates_and_reads_id() {
        let id = NodeId::new(42);

        assert_eq!(id.index(), 42);
    }

    #[test]
    fn root_id_is_zero() {
        assert_eq!(NodeId::ROOT.index(), 0);
    }

    #[test]
    fn ids_are_value_types() {
        let a = NodeId::new(7);
        let b = NodeId::new(7);
        let c = NodeId::new(8);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn debug_representation_is_stable() {
        assert_eq!(format!("{:?}", NodeId::new(42)), "NodeId(42)");
    }
}
