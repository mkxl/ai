use strum::{Display, EnumString};

#[derive(Clone, Copy)]
pub enum LlmProvider {
    Anthropic,
}

#[derive(Clone, Display, EnumString)]
pub enum LlmType {
    #[strum(to_string = "claude-haiku-4-5")]
    AnthropicClaudeHaiku45,
}

impl LlmType {
    pub const fn provider(&self) -> LlmProvider {
        match self {
            Self::AnthropicClaudeHaiku45 => LlmProvider::Anthropic,
        }
    }
}
