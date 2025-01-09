use serde::Deserialize;
use std::fs;

use crate::consts::INPUT_PATH;

#[derive(Deserialize, Debug, Clone)]
pub struct Input {
    pub name: String,
    pub facts: Vec<String>,
    pub files: Vec<String>,
}

impl Input {
    pub fn new() -> Result<Self, anyhow::Error> {
        let input_content = fs::read_to_string(INPUT_PATH)?;
        let input: Input = serde_json::from_str(&input_content)?;
        Ok(input)
    }
}
