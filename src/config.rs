use serde::Deserialize;
use std::fs;

use crate::consts::CONFIG_PATH;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub completion_provider: CompletionProvider,
    pub output_file_name: String,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        let config_content = fs::read_to_string(CONFIG_PATH)?;
        let config: Config = serde_json::from_str(&config_content)?;
        Ok(config)
    }
}

// PROVIDERS
#[derive(Deserialize, Debug, Clone)]
pub enum CompletionProvider {
    #[serde(rename = "anthropic")]
    Anthropic,
    #[serde(rename = "cohere")]
    Cohere,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "perplexity")]
    Perplexity,
    #[serde(rename = "xai")]
    XAI,
}
