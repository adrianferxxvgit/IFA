use ifa_core::SourceText;

pub struct Lexer<'a> {
    source: &'a SourceText,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a SourceText) -> Self {
        Self { source }
    }

    pub fn source(&self) -> &'a SourceText {
        self.source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_lexer() {
        let source = SourceText::new("");

        let lexer = Lexer::new(&source);

        assert_eq!(lexer.source().text(), "");
    }
}
