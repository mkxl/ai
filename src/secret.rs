use serde::Deserialize;

#[derive(Deserialize)]
pub struct Secret {
    pub anthropic_api_key: String,
}
