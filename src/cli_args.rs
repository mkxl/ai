use crate::{chat::Chat, llm_type::LlmType};
use anyhow::Error as AnyhowError;
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
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

        At the end of each response include in parentheses a better written version of the prompt to help the user \
        get better at prompting.
    "};

    fn default_secret_filepath() -> Utf8PathBuf {
        "~/.config/ai/secret.json".expand_user().into_owned()
    }
}

#[derive(Subcommand)]
enum CliCommand {
    Chat(ChatArgs),
    List,
}

#[derive(Parser)]
pub struct CliArgs {
    #[arg(long = "tokio-console")]
    tokio_console_enabled: bool,

    #[command(subcommand)]
    cli_command: Option<CliCommand>,

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

        match self.cli_command {
            Some(CliCommand::Chat(chat_args)) => Chat::new(chat_args).await?.run().await,
            Some(CliCommand::List) => std::todo!(),
            None => Chat::new(self.chat_args).await?.run().await,
        }
    }
}
