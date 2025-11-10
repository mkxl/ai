use crate::{chat::Chat, llm_type::LlmType};
use anyhow::Error as AnyhowError;
use camino::Utf8PathBuf;
use clap::{Args, Parser};
use mkutils::{Tracing, Utils};

#[derive(Args)]
pub struct ChatArgs {
    #[arg(long = "secret", default_value_t = Self::default_secret_filepath())]
    pub secret_filepath: Utf8PathBuf,

    #[arg(long = "input")]
    pub input_paths: Vec<Utf8PathBuf>,

    #[arg(long = "model", default_value_t = Self::DEFAULT_LLM_TYPE)]
    pub llm_type: LlmType,

    #[arg(long, default_value = Self::DEFAULT_SYSTEM_PROMPT)]
    pub system_prompt: String,

    pub prompt: Vec<String>,
}

impl ChatArgs {
    const DEFAULT_LLM_TYPE: LlmType = LlmType::AnthropicClaudeHaiku45;
    const DEFAULT_SYSTEM_PROMPT: &'static str = indoc::indoc! {"
        Answer user queries concisely but correctly.

        At the end of each response include in parentheses a better written version of the prompt to help the user get
        better at prompting.
    "};

    fn default_secret_filepath() -> Utf8PathBuf {
        "~/.config/ai/secret.json".expand_user().into_owned()
    }
}

#[derive(Parser)]
pub struct CliArgs {
    #[arg(long = "tokio-console")]
    tokio_console_enabled: bool,

    #[command(flatten)]
    chat_args: ChatArgs,
}

impl CliArgs {
    const TRACING_JSON_ENABLED: bool = true;

    fn init_tracing(&self) {
        Tracing::default()
            .with_json_enabled(Self::TRACING_JSON_ENABLED)
            .with_tokio_console_enabled(self.tokio_console_enabled)
            .init();
    }

    pub async fn run(self) -> Result<(), AnyhowError> {
        self.init_tracing();

        Chat::new(self.chat_args).await?.run().await
    }
}
