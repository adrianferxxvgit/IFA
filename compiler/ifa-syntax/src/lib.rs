use ifa_core::{SourceText, Span};
use ifa_syntax::{SyntaxKind, SyntaxToken};

pub struct Lexer<'a> {
    source: &'a SourceText,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a SourceText) -> Self {
        Self { source, offset: 0 }
    }

    pub fn lex(mut self) -> Vec<SyntaxToken> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            let start = self.offset;

            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.consume_whitespace();
                    tokens.push(SyntaxToken::new(
                        SyntaxKind::Whitespace,
                        Span::new(start, self.offset).unwrap(),
                    ));
                }

                '(' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::LeftParen));
                }

                ')' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::RightParen));
                }

                '{' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::LeftBrace));
                }

                '}' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::RightBrace));
                }

                '[' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::LeftBracket));
                }

                ']' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::RightBracket));
                }

                ',' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::Comma));
                }

                ':' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::Colon));
                }

                ';' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::Semicolon));
                }

                '.' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::Dot));
                }

                '+' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::Plus));
                }

                '-' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::Minus));
                }

                '*' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::Star));
                }

                '/' => {
                    self.bump();
                    tokens.push(self.token(start, SyntaxKind::Slash));
                }

                '=' => {
                    self.bump();

                    if self.peek() == Some('=') {
                        self.bump();
                        tokens.push(SyntaxToken::new(
                            SyntaxKind::EqualEqual,
                            Span::new(start, self.offset).unwrap(),
                        ));
                    } else {
                        tokens.push(self.token(start, SyntaxKind::Equal));
                    }
                }

                '!' => {
                    self.bump();

                    if self.peek() == Some('=') {
                        self.bump();
                        tokens.push(SyntaxToken::new(
                            SyntaxKind::BangEqual,
                            Span::new(start, self.offset).unwrap(),
                        ));
                    } else {
                        tokens.push(self.token(start, SyntaxKind::Bang));
                    }
                }

                c if c.is_ascii_digit() => {
                    self.consume_number();
                    tokens.push(SyntaxToken::new(
                        SyntaxKind::IntegerLiteral,
                        Span::new(start, self.offset).unwrap(),
                    ));
                }

                c if is_identifier_start(c) => {
                    self.consume_identifier();

                    let text = &self.source.text()[start..self.offset];

                    let kind = match text {
                        "indicator" => SyntaxKind::KeywordIndicator,
                        "if" => SyntaxKind::KeywordIf,
                        "else" => SyntaxKind::KeywordElse,
                        "while" => SyntaxKind::KeywordWhile,
                        "return" => SyntaxKind::KeywordReturn,
                        _ => SyntaxKind::Identifier,
                    };

                    tokens.push(SyntaxToken::new(
                        kind,
                        Span::new(start, self.offset).unwrap(),
                    ));
                }

                '"' => {
                    self.bump();

                    while let Some(c) = self.peek() {
                        self.bump();

                        if c == '"' {
                            break;
                        }
                    }

                    tokens.push(SyntaxToken::new(
                        SyntaxKind::StringLiteral,
                        Span::new(start, self.offset).unwrap(),
                    ));
                }

                _ => {
                    self.bump();

                    tokens.push(SyntaxToken::new(
                        SyntaxKind::Unknown,
                        Span::new(start, self.offset).unwrap(),
                    ));
                }
            }
        }

        tokens.push(SyntaxToken::new(
            SyntaxKind::EndOfFile,
            Span::new(self.offset, self.offset).unwrap(),
        ));

        tokens
    }

    fn token(&self, start: usize, kind: SyntaxKind) -> SyntaxToken {
        SyntaxToken::new(kind, Span::new(start, self.offset).unwrap())
    }

    fn peek(&self) -> Option<char> {
        self.source.text()[self.offset..].chars().next()
    }

    fn bump(&mut self) {
        if let Some(ch) = self.peek() {
            self.offset += ch.len_utf8();
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.bump();
        }
    }

    fn consume_identifier(&mut self) {
        while let Some(ch) = self.peek() {
            if !is_identifier_continue(ch) {
                break;
            }
            self.bump();
        }
    }

    fn consume_number(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_ascii_digit() {
                break;
            }
            self.bump();
        }
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_identifier_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_simple_program() {
        let source = SourceText::new("indicator test {}");

        let tokens = Lexer::new(&source).lex();

        assert_eq!(tokens[0].kind(), SyntaxKind::KeywordIndicator);
        assert_eq!(tokens[1].kind(), SyntaxKind::Whitespace);
        assert_eq!(tokens[2].kind(), SyntaxKind::Identifier);
        assert_eq!(tokens[3].kind(), SyntaxKind::Whitespace);
        assert_eq!(tokens[4].kind(), SyntaxKind::LeftBrace);
        assert_eq!(tokens[5].kind(), SyntaxKind::RightBrace);
        assert_eq!(tokens[6].kind(), SyntaxKind::EndOfFile);
    }
}
