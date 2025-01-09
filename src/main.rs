mod character;
mod completion;
mod config;
mod consts;
mod gen;
mod input;
use completion::CompletionModelEnum;
use config::CompletionProvider;
use dotenv::dotenv;
use fern::colors::{Color, ColoredLevelConfig};
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // init logging
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    info!("Starting FABELIS.AI Character Gen...");

    // load config.json
    info!("[SETUP] Loading from config.json...");
    let config = config::Config::new().expect("Failed to load config.json");
    info!("[SETUP] Loaded config.json: {:#?}", config);

    // load input.json
    info!("[SETUP] Loading from input.json...");
    let input = input::Input::new().expect("Failed to load input.json");
    info!("[SETUP] Loaded input.json: {:#?}", input);

    // load .env
    dotenv().ok().expect("Failed to load .env");
    info!("[SETUP] Loaded .env");

    // load completion model
    let completion_model: CompletionModelEnum = match config.completion_provider {
        CompletionProvider::Anthropic => {
            let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set");
            let model =
                env::var("ANTHROPIC_COMPLETION_MODEL").expect("ANTHROPIC_COMPLETION_MODEL not set");
            let client = rig::providers::anthropic::ClientBuilder::new(&api_key).build();
            let model = CompletionModelEnum::Anthropic(client.completion_model(&model));
            info!("[SETUP] Loaded Anthropic Completion Model");
            model
        }
        CompletionProvider::Cohere => {
            let api_key = env::var("COHERE_API_KEY").expect("COHERE_API_KEY not set");
            let model =
                env::var("COHERE_COMPLETION_MODEL").expect("COHERE_COMPLETION_MODEL not set");
            let client = rig::providers::cohere::Client::new(&api_key);
            let model = CompletionModelEnum::Cohere(client.completion_model(&model));
            info!("[SETUP] Loaded Cohere Completion Model");
            model
        }
        CompletionProvider::Gemini => {
            let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
            let model =
                env::var("GEMINI_COMPLETION_MODEL").expect("GEMINI_COMPLETION_MODEL not set");
            let client = rig::providers::gemini::Client::new(&api_key);
            let model = CompletionModelEnum::Gemini(client.completion_model(&model));
            info!("[SETUP] Loaded Gemini Completion Model");
            model
        }
        CompletionProvider::OpenAI => {
            let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
            let model =
                env::var("OPENAI_COMPLETION_MODEL").expect("OPENAI_COMPLETION_MODEL not set");
            let client = rig::providers::openai::Client::new(&api_key);
            let model = CompletionModelEnum::OpenAI(client.completion_model(&model));
            info!("[SETUP] Loaded OpenAI Completion Model");
            model
        }
        CompletionProvider::Perplexity => {
            let api_key = env::var("PERPLEXITY_API_KEY").expect("PERPLEXITY_API_KEY not set");
            let model = env::var("PERPLEXITY_COMPLETION_MODEL")
                .expect("PERPLEXITY_COMPLETION_MODEL not set");
            let client = rig::providers::perplexity::Client::new(&api_key);
            let model = CompletionModelEnum::Perplexity(client.completion_model(&model));
            info!("[SETUP] Loaded Perplexity Completion Model");
            model
        }
        CompletionProvider::XAI => {
            let api_key = env::var("XAI_API_KEY").expect("XAI_API_KEY not set");
            let model = env::var("XAI_COMPLETION_MODEL").expect("XAI_COMPLETION_MODEL not set");
            let client = rig::providers::xai::Client::new(&api_key);
            let model = CompletionModelEnum::XAI(client.completion_model(&model));
            info!("[SETUP] Loaded XAI Completion Model");
            model
        }
    };

    let mut gen = gen::Generator::new(config, input, completion_model);
    gen.start().await;

    Ok(())
}
