use derive_more::Constructor;
use std::path::Path;

pub enum Role {
    User,
}

pub enum Content {
    Text(String),
}

#[derive(Constructor)]
pub struct LlmInput {
    role: Role,
    content: Content,
}

impl LlmInput {
    pub const fn user_text(text: String) -> Self {
        let content = Content::Text(text);

        Self::new(Role::User, content)
    }

    pub fn user_file(filepath: &Path, text: &str) -> Self {
        let text = std::format!("### {filepath}\n===\n{text}", filepath = filepath.display());

        Self::user_text(text)
    }
}
