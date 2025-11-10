use strum::{Display, EnumString};

#[derive(Clone, Display, EnumString)]
pub enum LlmType {
    #[strum(to_string = "claude-haiku-4-5")]
    AnthropicClaudeHaiku45,
}
