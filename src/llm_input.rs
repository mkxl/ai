use crate::anthropic::AnthropicMessage;
use derive_more::Constructor;
use std::path::Path;
use strum::Display;

#[derive(Display)]
pub enum Role {
    #[strum(to_string = "user")]
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

    pub const fn content_string(&self) -> &str {
        match &self.content {
            Content::Text(text) => text.as_str(),
        }
    }

    pub fn anthropic_message(&self) -> AnthropicMessage {
        AnthropicMessage::new(self.role.to_string(), self.content_string().to_owned())
    }
}
