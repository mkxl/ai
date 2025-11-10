use enum_assoc::Assoc;
use strum::{Display, EnumString};

#[derive(Clone, Copy)]
pub enum LlmProvider {
    Anthropic,
}

#[derive(Assoc, Clone, Display, EnumString)]
#[func(pub const fn provider(&self) -> LlmProvider)]
pub enum LlmType {
    #[assoc(provider = LlmProvider::Anthropic)]
    #[strum(to_string = "claude-haiku-4-5")]
    AnthropicClaudeHaiku45,
}
