#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultStatus {
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultMessage {
    status: ResultStatus,
    message: String,
}

impl ResultMessage {
    pub fn new<S>(status: ResultStatus, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            status,
            message: message.into(),
        }
    }

    pub const fn status(&self) -> ResultStatus {
        self.status
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilerResult<T> {
    value: Option<T>,
    messages: Vec<ResultMessage>,
}

impl<T> CompilerResult<T> {
    pub fn success(value: T) -> Self {
        Self {
            value: Some(value),
            messages: Vec::new(),
        }
    }

    pub fn with_messages(value: T, messages: Vec<ResultMessage>) -> Self {
        Self {
            value: Some(value),
            messages,
        }
    }

    pub fn failure(messages: Vec<ResultMessage>) -> Self {
        Self {
            value: None,
            messages,
        }
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn into_value(self) -> Option<T> {
        self.value
    }

    pub fn messages(&self) -> &[ResultMessage] {
        &self.messages
    }

    pub fn is_success(&self) -> bool {
        self.value.is_some()
            && !self
                .messages
                .iter()
                .any(|message| message.status == ResultStatus::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.messages
            .iter()
            .any(|message| message.status == ResultStatus::Warning)
    }

    pub fn has_errors(&self) -> bool {
        self.messages
            .iter()
            .any(|message| message.status == ResultStatus::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::{CompilerResult, ResultMessage, ResultStatus};

    #[test]
    fn success_contains_value() {
        let result = CompilerResult::success(42);

        assert_eq!(result.value(), Some(&42));
        assert!(result.is_success());
        assert!(!result.has_warnings());
        assert!(!result.has_errors());
    }

    #[test]
    fn warnings_do_not_make_result_unsuccessful() {
        let result = CompilerResult::with_messages(
            42,
            vec![ResultMessage::new(ResultStatus::Warning, "example warning")],
        );

        assert_eq!(result.value(), Some(&42));
        assert!(result.is_success());
        assert!(result.has_warnings());
        assert!(!result.has_errors());
    }

    #[test]
    fn errors_make_result_unsuccessful() {
        let result = CompilerResult::with_messages(
            42,
            vec![ResultMessage::new(ResultStatus::Error, "example error")],
        );

        assert_eq!(result.value(), Some(&42));
        assert!(!result.is_success());
        assert!(result.has_errors());
    }

    #[test]
    fn failure_has_no_value() {
        let result: CompilerResult<u32> =
            CompilerResult::failure(vec![ResultMessage::new(ResultStatus::Error, "failed")]);

        assert_eq!(result.value(), None);
        assert!(result.has_errors());
        assert!(!result.is_success());
    }

    #[test]
    fn messages_are_preserved() {
        let result = CompilerResult::with_messages(
            "value",
            vec![
                ResultMessage::new(ResultStatus::Warning, "warning"),
                ResultMessage::new(ResultStatus::Error, "error"),
            ],
        );

        assert_eq!(result.messages().len(), 2);
        assert_eq!(result.messages()[0].message(), "warning");
        assert_eq!(result.messages()[1].message(), "error");
    }
}
