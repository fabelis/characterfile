use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub alias: String,
    pub bio: String,
    pub adjectives: Vec<String>,
    pub lore: Vec<String>,
    pub styles: Vec<String>,
    pub topics: Vec<String>,
    pub inspirations: Vec<String>,
    #[serde(skip)]
    pub path: String,
}

impl Character {
    pub fn new(path: String) -> Self {
        Character {
            alias: "".to_string(),
            bio: "".to_string(),
            adjectives: vec![],
            lore: vec![],
            styles: vec![],
            topics: vec![],
            inspirations: vec![],
            path,
        }
    }

    pub fn load(&mut self) -> Result<(), anyhow::Error> {
        let content = fs::read_to_string(self.path.clone())?;
        *self = serde_json::from_str(&content)?;
        Ok(())
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
