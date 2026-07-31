use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Option<Self> {
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn len(self) -> usize {
        self.end - self.start
    }

    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    pub const fn range(self) -> Range<usize> {
        self.start..self.end
    }

    pub const fn contains(self, offset: usize) -> bool {
        self.start <= offset && offset < self.end
    }

    pub const fn contains_span(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

#[cfg(test)]
mod tests {
    use super::Span;

    #[test]
    fn creates_valid_span() {
        let span = Span::new(3, 8).expect("3..8 should be valid");

        assert_eq!(span.start(), 3);
        assert_eq!(span.end(), 8);
        assert_eq!(span.len(), 5);
        assert!(!span.is_empty());
    }

    #[test]
    fn creates_empty_span() {
        let span = Span::new(5, 5).expect("5..5 should be valid");

        assert!(span.is_empty());
        assert_eq!(span.len(), 0);
    }

    #[test]
    fn rejects_reversed_span() {
        assert!(Span::new(8, 3).is_none());
    }

    #[test]
    fn converts_to_range() {
        let span = Span::new(2, 6).expect("2..6 should be valid");

        assert_eq!(span.range(), 2..6);
    }

    #[test]
    fn contains_offsets_using_half_open_interval() {
        let span = Span::new(2, 6).expect("2..6 should be valid");

        assert!(!span.contains(1));
        assert!(span.contains(2));
        assert!(span.contains(5));
        assert!(!span.contains(6));
    }

    #[test]
    fn contains_nested_span() {
        let outer = Span::new(2, 10).expect("2..10 should be valid");
        let inner = Span::new(4, 7).expect("4..7 should be valid");

        assert!(outer.contains_span(inner));
        assert!(!inner.contains_span(outer));
    }
}
