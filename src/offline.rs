use crate::{
    llm::{Llm, LlmStream},
    llm_input::LlmInput,
};
use derive_more::Constructor;
use itertools::Itertools;
use mkutils::Utils;

#[derive(Constructor, Default)]
pub struct Offline;

impl Llm for Offline {
    fn stream_texts(&mut self, system_prompt: String, mut llm_inputs: Vec<LlmInput>) -> LlmStream<'_> {
        crate::llm::llm_stream! {
            let text_iter = llm_inputs.iter_mut().map(LlmInput::take_content_string);
            let newline_iter = "\n".to_owned().repeat();
            let text_iter = system_prompt.once().chain(text_iter).interleave_shortest(newline_iter);

            for text in text_iter {
                yield text;
            }
        }
    }
}
