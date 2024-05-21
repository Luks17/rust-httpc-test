use std::fmt::Display;

pub enum AuthHeaderType {
    Basic,
    Bearer,
}

impl Display for AuthHeaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            AuthHeaderType::Basic => "Basic",
            AuthHeaderType::Bearer => "Bearer",
        };

        write!(f, "{}", formatted)
    }
}

pub(crate) struct AuthHeader {
    pub kind: AuthHeaderType,
    pub content: String,
}

impl Display for AuthHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind, self.content)
    }
}
