use ifa_core::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticCode {
    Generic,
    InvalidSource,
    UnexpectedToken,
    UnknownIdentifier,
    TypeMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    code: DiagnosticCode,
    severity: Severity,
    message: String,
    span: Option<Span>,
    notes: Vec<String>,
}

impl Diagnostic {
    pub fn new<S>(code: DiagnosticCode, severity: Severity, message: S, span: Option<Span>) -> Self
    where
        S: Into<String>,
    {
        Self {
            code,
            severity,
            message: message.into(),
            span,
            notes: Vec::new(),
        }
    }

    pub fn with_note<S>(mut self, note: S) -> Self
    where
        S: Into<String>,
    {
        self.notes.push(note.into());
        self
    }

    pub const fn code(&self) -> DiagnosticCode {
        self.code
    }

    pub const fn severity(&self) -> Severity {
        self.severity
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn span(&self) -> Option<Span> {
        self.span
    }

    pub fn notes(&self) -> &[String] {
        &self.notes
    }
}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, DiagnosticCode, Severity};
    use ifa_core::Span;

    #[test]
    fn creates_error_diagnostic() {
        let span = Span::new(2, 5).expect("valid span");

        let diagnostic = Diagnostic::new(
            DiagnosticCode::UnexpectedToken,
            Severity::Error,
            "unexpected token",
            Some(span),
        );

        assert_eq!(diagnostic.code(), DiagnosticCode::UnexpectedToken);
        assert_eq!(diagnostic.severity(), Severity::Error);
        assert_eq!(diagnostic.message(), "unexpected token");
        assert_eq!(diagnostic.span(), Some(span));
    }

    #[test]
    fn creates_diagnostic_without_span() {
        let diagnostic = Diagnostic::new(
            DiagnosticCode::Generic,
            Severity::Info,
            "compiler information",
            None,
        );

        assert_eq!(diagnostic.span(), None);
    }

    #[test]
    fn notes_are_preserved() {
        let diagnostic = Diagnostic::new(
            DiagnosticCode::TypeMismatch,
            Severity::Error,
            "type mismatch",
            None,
        )
        .with_note("expected Integer")
        .with_note("found Percentage");

        assert_eq!(
            diagnostic.notes(),
            &[
                "expected Integer".to_string(),
                "found Percentage".to_string()
            ]
        );
    }
}
