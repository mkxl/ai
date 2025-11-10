mod chat;
mod cli_args;
mod llm_input;
mod llm_type;
mod secret;

use crate::cli_args::CliArgs;
use anyhow::Error as AnyhowError;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), AnyhowError> {
    CliArgs::parse().run().await
}
