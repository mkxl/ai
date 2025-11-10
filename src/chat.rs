use crate::{cli_args::ChatArgs, llm_input::LlmInput, secret::Secret};
use anyhow::Error as AnyhowError;
use camino::Utf8Path;
use mkutils::Utils;
use tokio::task::JoinSet;
use walkdir::WalkDir;

#[allow(clippy::struct_field_names)]
pub struct Chat {
    secret: Secret,
    chat_args: ChatArgs,
    llm_inputs: Vec<LlmInput>,
}

impl Chat {
    pub async fn new(mut chat_args: ChatArgs) -> Result<Self, AnyhowError> {
        let secret = chat_args.secret_filepath.open()?.to_value_from_json_reader()?;
        let prompt = Self::prompt(&mut chat_args);
        let llm_inputs = Self::llm_inputs(&chat_args, prompt).await?;
        let chat = Self {
            secret,
            chat_args,
            llm_inputs,
        };

        chat.ok()
    }

    fn prompt(chat_args: &mut ChatArgs) -> String {
        chat_args.prompt.mem_take().into_iter().collect()
    }

    async fn llm_inputs(chat_args: &ChatArgs, prompt: String) -> Result<Vec<LlmInput>, AnyhowError> {
        let mut join_set = JoinSet::new();
        let mut llm_inputs = Vec::new();
        let llm_input = LlmInput::user_text(prompt);

        for input_path in &chat_args.input_paths {
            for dir_entry_res in WalkDir::new::<&Utf8Path>(&input_path.expand_user()) {
                let dir_entry = dir_entry_res?;

                if dir_entry.file_type().is_file() {
                    join_set.spawn(dir_entry.into_path().read_to_string_async());
                }

                tokio::task::yield_now().await;
            }
        }

        while let Some(read_value_res) = join_set.join_next().await {
            let read_value = read_value_res?;

            match read_value.result {
                Ok(text) => LlmInput::user_file(&read_value.filepath, &text).push_to(&mut llm_inputs),
                Err(io_err) => {
                    tracing::warn!(%io_err, filepath = %read_value.filepath.display(), "unable to read file");
                }
            }
        }

        llm_inputs.push(llm_input);

        llm_inputs.ok()
    }

    pub async fn run(&self) -> Result<(), AnyhowError> {
        tracing::info!(num_inputs = self.llm_inputs.len()).ok().ready().await
    }
}
