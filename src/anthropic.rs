use crate::{
    llm::{Llm, LlmStream},
    llm_input::LlmInput,
};
use derive_more::Constructor;
use futures::StreamExt;
use mkutils::Utils;
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

#[derive(Constructor, Serialize)]
pub struct AnthropicMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Constructor, Serialize)]
struct RequestBody<'a> {
    model: String,
    max_tokens: usize,
    system: String,
    messages: Vec<AnthropicMessage<'a>>,
    stream: bool,
}

#[derive(Deserialize)]
struct Delta {
    text: String,
}

#[derive(Deserialize)]
struct ContentBlockDelta {
    delta: Delta,
}

#[derive(Constructor)]
pub struct Anthropic {
    http_client: ReqwestClient,
    model: String,
    api_key: String,
}

impl Anthropic {
    const HEADER_NAME_ANTHROPIC_VERSION: &'static str = "anthropic-version";
    const HEADER_NAME_API_KEY: &'static str = "x-api-key";
    const HEADER_VALUE_ANTHROPIC_VERSION: &'static str = "2023-06-01";
    const MAX_TOKENS: usize = 2048;
    const URL: &'static str = "https://api.anthropic.com/v1/messages";
    const STREAM: bool = true;
}

impl Llm for Anthropic {
    fn stream_texts(&mut self, system_prompt: String, llm_inputs: Vec<LlmInput>) -> LlmStream<'_> {
        crate::llm::llm_stream! {
            let messages = llm_inputs.iter().map(LlmInput::anthropic_message).collect();
            let request_body = RequestBody::new(self.model.mem_take(), Self::MAX_TOKENS, system_prompt, messages, Self::STREAM);
            let mut line_res_stream = self
                .http_client
                .post(Self::URL)
                .header(Self::HEADER_NAME_API_KEY, &self.api_key)
                .header(Self::HEADER_NAME_ANTHROPIC_VERSION, Self::HEADER_VALUE_ANTHROPIC_VERSION)
                .json(&request_body)
                .send()
                .await?
                .check_status()
                .await?
                .bytes_stream()
                .map(Utils::io_result)
                .into_stream_reader()
                .into_line_frames();

            while let Some(line_res) = line_res_stream.next().await {
                let line = line_res?;
                let Some(json_str) = line.strip_prefix("data: ") else { continue };
                let Ok(content_block_delta) = json_str.to_value_from_json_byte_str::<ContentBlockDelta>() else { continue };

                yield content_block_delta.delta.text;
            }
        }
    }
}
