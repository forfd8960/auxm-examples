#[derive(Debug)]
pub enum TodoError {
    NotFound(String),
}

impl std::error::Error for TodoError {}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::NotFound(msg) => write!(f, "{}", msg),
        }
    }
}
