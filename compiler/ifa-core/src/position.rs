#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    offset: usize,
    line: usize,
    column: usize,
}

impl Position {
    pub const fn new(offset: usize, line: usize, column: usize) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }

    pub const fn offset(self) -> usize {
        self.offset
    }

    pub const fn line(self) -> usize {
        self.line
    }

    pub const fn column(self) -> usize {
        self.column
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn stores_position_components() {
        let position = Position::new(12, 3, 5);

        assert_eq!(position.offset(), 12);
        assert_eq!(position.line(), 3);
        assert_eq!(position.column(), 5);
    }

    #[test]
    fn positions_are_value_types() {
        let a = Position::new(12, 3, 5);
        let b = Position::new(12, 3, 5);
        let c = Position::new(13, 3, 5);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
