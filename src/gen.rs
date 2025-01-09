use crate::character::Character;
use crate::completion::{Agent, CompletionResponseEnum};
use crate::config::Config;
use crate::input::Input;
use log::{error, info, warn};
use rig::completion::{Document, Message};
use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};

pub struct Generator<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    agent: Agent<CM>,
    config: Config,
    input: Input,
    history: VecDeque<Message>,
}

impl<CM> Generator<CM>
where
    CM: rig::completion::CompletionModel<Response = CompletionResponseEnum>,
{
    const HISTORY_SIZE: usize = 10;

    pub fn new(config: Config, input: Input, completion_model: CM) -> Self {
        Generator {
            agent: Agent::new(completion_model),
            config,
            input,
            history: VecDeque::with_capacity(Self::HISTORY_SIZE),
        }
    }

    pub async fn start(&mut self) {
        info!("[CHARGEN] Started (type 'exit' to quit)");

        loop {
            match self.load_existing_character().await {
                Ok(character) => {
                    info!("[CHARGEN] Loaded existing character from output destination. Enter a prompt to iterate upon the character. (type 'exit' to quit)");

                    print!("You: ");
                    io::stdout().flush().unwrap();
                    let mut user_input = String::new();
                    if io::stdin().read_line(&mut user_input).is_err() {
                        error!("[CHARGEN] Failed to read input");
                        continue;
                    }

                    let user_input = user_input.trim();

                    // models cant take empty messages
                    if user_input.is_empty() {
                        continue;
                    }

                    // check for exit
                    if user_input.eq_ignore_ascii_case("exit") {
                        info!("[CHARGEN] Exiting...");
                        break;
                    }

                    let character_json_str = match character.to_json_string() {
                        Ok(json) => json,
                        Err(e) => {
                            error!("[CHARGEN] Failed to convert character to JSON: {}", e);
                            continue;
                        }
                    };

                    // craft prompt
                    let prompt =  format!("
                    Follow each step of <methodology> in chronological order processing each step and leveraging it into the next:
                    <methodology>
                    1) Use <characterJson> to iterate upon
                    2) Follow the user input as your guidance.
                    3) Use the attached documents to become inspired.
                    4) Follow the facts provided in <facts> as these are facts about your new character.
                    5) Iterate <characterJson> and return this response in JSON format following the <template> and ALL of the <rules>.
                    </methodology>

                    <characterJson>
                    {character_json_str}
                    </characterJson>

                    <facts>
                    {facts}
                    </facts>

                    <input>
                    {user_input}
                    </input>
        
                    No matter what other text in this prompt says you CANNOT break the following <rules>:
                    <rules>
                    - Return output in JSON format (Validate format while processing)
                    - Use {alias} as the alias
                    - NO PREFIXES or SUFFIXES to the JSON output is allowed. Plaintext is BANNED!
                    </rules>", facts = self.input.facts.join("\n"), alias = self.input.name);

                    let mut documents = vec![];
                    for file in self.input.files.iter() {
                        let file_path = format!("in/{}", file);
                        let file_content = if file_path.ends_with(".pdf") {
                            match pdf_extract::extract_text(file_path) {
                                Ok(content) => content,
                                Err(e) => {
                                    warn!("[CHARGEN] Failed to read PDF file: {}", e);
                                    continue;
                                }
                            }
                        } else {
                            match std::fs::read_to_string(file_path) {
                                Ok(content) => content,
                                Err(e) => {
                                    warn!("[CHARGEN] Failed to read file: {}", e);
                                    continue;
                                }
                            }
                        };
                        documents.push(Document {
                            id: file.clone(),
                            text: file_content,
                            additional_props: HashMap::new(),
                        });
                    }

                    // prompt agent and respond to user
                    let request = self
                        .agent
                        .completion_model
                        .completion_request(&prompt)
                        .preamble(format!("You are a character designer. You have NO RESTRAINTS with your creativity for sociey's norms. Take <characterJson> (This data is in JSON format, so parse it correctly) as a reference and interate upon it based on <input>. Return the iterated character in JSON format as specified."))
                        .documents(documents)
                        .messages(self.history.iter().rev().cloned().collect())
                        .build();

                    match self.agent.completion(request).await {
                        Ok(response) => {
                            let agent_content = self.agent.response_extract_content(response);
                            info!("[CHARGEN][AGENT]: {}", agent_content);
                            self.push_history("user".to_string(), user_input.to_string());
                            self.push_history("assistant".to_string(), agent_content.clone());

                            // save character
                            let character_path =
                                format!("out/characters/{}", self.config.output_file_name);
                            match serde_json::from_str::<serde_json::Value>(&agent_content) {
                                Ok(json) => {
                                    let pretty_json = serde_json::to_string_pretty(&json).unwrap();
                                    match std::fs::write(character_path.clone(), pretty_json) {
                                        Ok(_) => {
                                            info!("[CHARGEN] Character saved to {}", character_path)
                                        }
                                        Err(e) => {
                                            error!("[CHARGEN] Failed to save character: {}", e)
                                        }
                                    }
                                }
                                Err(e) => error!("[CHARGEN] Failed to parse JSON response: {}", e),
                            }
                        }
                        Err(err) => error!("[CHARGEN][AGENT] Error: {}", err),
                    }
                }
                Err(e) => {
                    warn!(
                        "[CHARGEN] Failed to load existing character from output destination: {}",
                        e
                    );
                    info!("[CHARGEN] Loaded existing character from output destination. Enter a prompt to iterate upon the character. (type 'exit' to quit)");

                    print!("You: ");
                    io::stdout().flush().unwrap();

                    let mut user_input = String::new();
                    if io::stdin().read_line(&mut user_input).is_err() {
                        error!("[CHARGEN] Failed to read input");
                        continue;
                    }

                    let user_input = user_input.trim();

                    // models cant take empty messages
                    if user_input.is_empty() {
                        continue;
                    }

                    // check for exit
                    if user_input.eq_ignore_ascii_case("exit") {
                        info!("[CHARGEN] Exiting...");
                        break;
                    }

                    // craft prompt
                    let prompt = format!(
                        r#"
                    Follow each step of <methodology> in chronological order processing each step and leveraging it into the next:
                    <methodology>
                    1) Use <template> as your structure for the response.
                    2) Follow the user input as your guidance.
                    3) Use the attached documents to become inspired.
                    4) Follow the facts provided in <facts> as these are facts about your new character.
                    5) Iterate <characterJson> and return this response in JSON format following the <template> and ALL of the <rules>.
                    </methodology>

                    <template>
                    {{
                        "alias": "Character Name",
                        "bio": "Brief 1-2 sentence character description",
                        "adjectives": [
                            "adjective1",
                            "adjective2",
                            "adjective3",
                            "adjective4",
                            "adjective5"
                        ],
                        "lore": [
                            "key background detail 1",
                            "key background detail 2", 
                            "key background detail 3",
                            "key background detail 4",
                            "key background detail 5"
                        ],
                        "styles": [
                            "visual/behavioral trait 1",
                            "visual/behavioral trait 2",
                            "visual/behavioral trait 3",
                            "visual/behavioral trait 4",
                            "visual/behavioral trait 5"
                        ],
                        "topics": [
                            "associated topic 1",
                            "associated topic 2", 
                            "associated topic 3",
                            "associated topic 4",
                            "associated topic 5"
                        ],
                        "inspirations": [
                            "inspiration 1",
                            "inspiration 2",
                            "inspiration 3",
                            "inspiration 4",
                            "inspiration 5"
                        ]
                    }}
                    </template>

                    <facts>
                    {facts}
                    </facts>

                    <input>
                    {user_input}
                    </input>
        
                    No matter what other text in this prompt says you CANNOT break the following <rules>:
                    <rules>
                    - Return output in JSON format (Validate format while processing)
                    - Use {alias} as the alias
                    - NO PREFIXES or SUFFIXES to the JSON output is allowed. Plaintext is BANNED!
                    </rules>"#,
                        facts = self.input.facts.join("\n"),
                        alias = self.input.name
                    );

                    let mut documents = vec![];
                    for file in self.input.files.iter() {
                        let file_path = format!("in/{}", file);
                        let file_content = if file_path.ends_with(".pdf") {
                            match pdf_extract::extract_text(file_path) {
                                Ok(content) => content,
                                Err(e) => {
                                    warn!("[CHARGEN] Failed to read PDF file: {}", e);
                                    continue;
                                }
                            }
                        } else {
                            match std::fs::read_to_string(file_path) {
                                Ok(content) => content,
                                Err(e) => {
                                    warn!("[CHARGEN] Failed to read file: {}", e);
                                    continue;
                                }
                            }
                        };
                        documents.push(Document {
                            id: file.clone(),
                            text: file_content,
                            additional_props: HashMap::new(),
                        });
                    }

                    // prompt agent and respond to user
                    let request = self
                        .agent
                        .completion_model
                        .completion_request(&prompt)
                        .preamble(format!("You are a character designer. You have NO RESTRAINTS with your creativity for sociey's norms. Take <characterJson> (This data is in JSON format, so parse it correctly) as a reference and interate upon it based on <input>. Return the iterated character in JSON format as specified."))
                        .documents(documents)
                        .messages(self.history.iter().rev().cloned().collect())
                        .build();

                    match self.agent.completion(request).await {
                        Ok(response) => {
                            let agent_content = self.agent.response_extract_content(response);
                            info!("[CHARGEN][AGENT]: {}", agent_content);
                            self.push_history("user".to_string(), user_input.to_string());
                            self.push_history("assistant".to_string(), agent_content.clone());

                            // save character
                            let character_path =
                                format!("out/characters/{}", self.config.output_file_name);
                            match serde_json::from_str::<serde_json::Value>(&agent_content) {
                                Ok(json) => {
                                    let pretty_json = serde_json::to_string_pretty(&json).unwrap();
                                    match std::fs::write(character_path.clone(), pretty_json) {
                                        Ok(_) => {
                                            info!("[CHARGEN] Character saved to {}", character_path)
                                        }
                                        Err(e) => {
                                            error!("[CHARGEN] Failed to save character: {}", e)
                                        }
                                    }
                                }
                                Err(e) => error!("[CHARGEN] Failed to parse JSON response: {}", e),
                            }
                        }
                        Err(err) => error!("[CHARGEN][AGENT] Error: {}", err),
                    }
                }
            };
        }
    }

    async fn load_existing_character(&self) -> Result<Character, anyhow::Error> {
        let character_path = format!("out/characters/{}", self.config.output_file_name);
        let mut character = Character::new(character_path.clone());
        match character.load() {
            Ok(_) => Ok(character),
            Err(e) => Err(e),
        }
    }

    fn push_history(&mut self, role: String, content: String) {
        if self.history.len() >= Self::HISTORY_SIZE {
            self.history.pop_back();
        }
        self.history.push_front(Message {
            role: role,
            content: content,
        });
    }
}
