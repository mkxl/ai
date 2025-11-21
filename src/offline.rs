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
    #[allow(unstable_name_collisions)]
    fn stream_texts(&mut self, system_prompt: String, mut llm_inputs: Vec<LlmInput>) -> LlmStream<'_> {
        crate::llm::llm_stream! {
            let text_iter = llm_inputs.iter_mut().map(LlmInput::take_content_string);
            let text_iter = system_prompt.once().chain(text_iter).intersperse("\n".to_owned());

            for text in text_iter {
                yield text;
            }
        }
    }
}
