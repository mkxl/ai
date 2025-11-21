use crate::llm_input::LlmInput;
use anyhow::Error as AnyhowError;
use futures::{Stream, StreamExt};
use mkutils::Utils;
use std::pin::Pin;

pub type LlmStream<'a> = Pin<Box<dyn 'a + Stream<Item = Result<String, AnyhowError>>>>;

pub trait Llm {
    fn stream_texts(&mut self, system_prompt: String, llm_inputs: Vec<LlmInput>) -> LlmStream<'_>;

    async fn print_response(&mut self, system_prompt: String, llm_inputs: Vec<LlmInput>) -> Result<(), AnyhowError> {
        let mut stdout = tokio::io::stdout();
        let mut text_res_stream = self.stream_texts(system_prompt, llm_inputs);

        while let Some(text_res) = text_res_stream.next().await {
            stdout.write_all_and_flush_async(text_res?).await?;
        }

        ().ok()
    }
}
