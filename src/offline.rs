use crate::{
    llm::{Llm, LlmStream},
    llm_input::LlmInput,
};
use derive_more::Constructor;
use mkutils::Utils;

#[derive(Constructor, Default)]
pub struct Offline;

impl Llm for Offline {
    fn stream_texts(&mut self, system_prompt: String, llm_inputs: Vec<LlmInput>) -> LlmStream<'_> {
        crate::llm::llm_stream! {
            yield system_prompt;
            yield "\n".to_owned();

            for mut llm_input in llm_inputs {
                yield llm_input.take_content_string();
                yield "\n".to_owned();
            }
        }
    }
}
