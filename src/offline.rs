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
    fn stream_texts(&mut self, system_prompt: String, llm_inputs: Vec<LlmInput>) -> LlmStream<'_> {
        let text_iter = llm_inputs
            .into_iter()
            .map(|mut llm_input| llm_input.take_content_string());
        let text_iter = system_prompt.once().chain(text_iter).intersperse("\n".to_owned());

        crate::llm::llm_stream! {
            for text in text_iter {
                yield text;
            }
        }
    }
}
