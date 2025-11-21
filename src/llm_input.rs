use crate::anthropic::AnthropicMessage;
use derive_more::Constructor;
use mkutils::Utils;
use std::path::Path;
use strum::{Display, IntoStaticStr};

#[derive(Display, IntoStaticStr)]
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

    pub const fn as_content_str(&self) -> &str {
        match &self.content {
            Content::Text(text) => text.as_str(),
        }
    }

    pub fn take_content_string(&mut self) -> String {
        match &mut self.content {
            Content::Text(text) => text.mem_take(),
        }
    }

    pub fn anthropic_message(&self) -> AnthropicMessage<'_> {
        AnthropicMessage::new((&self.role).into(), self.as_content_str())
    }
}
