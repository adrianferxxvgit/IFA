use crate::NodeId;

#[derive(Debug)]
pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Arena<T> {
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn insert(&mut self, value: T) -> NodeId {
        let index = self.items.len();

        let index = u32::try_from(index).expect("Arena cannot contain more than u32::MAX elements");

        self.items.push(value);

        NodeId::new(index)
    }

    pub fn get(&self, id: NodeId) -> Option<&T> {
        self.items.get(id.index() as usize)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut T> {
        self.items.get_mut(id.index() as usize)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains(&self, id: NodeId) -> bool {
        (id.index() as usize) < self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Arena;
    use crate::NodeId;

    #[test]
    fn new_arena_is_empty() {
        let arena = Arena::<String>::new();

        assert!(arena.is_empty());
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn insert_returns_sequential_ids() {
        let mut arena = Arena::new();

        let first = arena.insert("first");
        let second = arena.insert("second");

        assert_eq!(first, NodeId::new(0));
        assert_eq!(second, NodeId::new(1));
        assert_eq!(arena.len(), 2);
    }

    #[test]
    fn retrieves_values_by_id() {
        let mut arena = Arena::new();

        let id = arena.insert("hello");

        assert_eq!(arena.get(id), Some(&"hello"));
    }

    #[test]
    fn unknown_id_returns_none() {
        let arena = Arena::<String>::new();

        assert_eq!(arena.get(NodeId::new(42)), None);
    }

    #[test]
    fn mutable_access_updates_value() {
        let mut arena = Arena::new();

        let id = arena.insert(10);

        *arena.get_mut(id).expect("inserted value must exist") = 20;

        assert_eq!(arena.get(id), Some(&20));
    }

    #[test]
    fn contains_reports_valid_ids() {
        let mut arena = Arena::new();

        let id = arena.insert("value");

        assert!(arena.contains(id));
        assert!(!arena.contains(NodeId::new(42)));
    }
}
