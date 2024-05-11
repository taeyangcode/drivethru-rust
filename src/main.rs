#![allow(dead_code)]

mod audio;
mod env;

use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{AssistantObject, CreateAssistantRequestArgs},
    Client,
};
use env::{EnvBuilder, OpenAIOptions};
use strum::AsRefStr;

const LLM_CLIENT_PROGRAM_CONTEXT: &str =
    "You are an AI assistant designed to generate an order history
    in a json format with product name, size, extras, and price for each item.
    You must take into account the following constraints:
    1. Each drink must have one of three sizes, tall, grande, or venti.
    2. Each drink can have extra toppings for a cost.
    3. A customer cannot order more than 10 items.
    4. You cannot provide any additional commentary or suggestions beyond what is requested.
    5. You cannot speak for the user.
    6. You can only produce the json order history and nothing else.";

const ASSISTANT_ID: &str = "asst_4cCB3pkqgBEwggBywlwh7bvW";
const ASSISTANT_NAME: &str = "Starbucks Drive-thru Attendant";
const ASSISTANT_MODEL: &str = "gpt-3.5-turbo-1106";
const ASSISTANT_CONTEXT: &str = "You are an AI chatbot designed to assist customers
    in placing orders at a Starbucks drive through.
    A customer has just pulled in to the drive through.
    You must take into account the following constraints:
    1. Each drink must have one of three sizes, tall, grande, or venti.
    2. Each drink can have extra toppings for a cost.
    3. A customer cannot order more than 10 items.
    4. You must provide the customer with their order total when they are finished ordering.
    Do not provide any additional commentary or suggestions beyond what is requested.
    Do not speak for the user.";

#[derive(AsRefStr)]
enum EnvLevel {
    #[strum(serialize = "debug.env")]
    Debug,
    #[strum(serialize = "dev.env")]
    Dev,
    #[strum(serialize = "production.env")]
    Production,
}

const ENV_LEVEL: EnvLevel = EnvLevel::Debug;

struct Logger;

impl Logger {
    pub fn debug(message: &str) {
        match ENV_LEVEL {
            EnvLevel::Debug => println!("{message}"),
            EnvLevel::Dev => {}
            EnvLevel::Production => {}
        };
    }
}

struct AssistantFactory;

impl AssistantFactory {
    pub async fn create(
        client: Client<OpenAIConfig>,
        name: &'static str,
        instructions: &'static str,
        model: audio::SpeechModel,
    ) -> Result<AssistantObject, anyhow::Error> {
        let assistant_request = CreateAssistantRequestArgs::default()
            .name(name)
            .instructions(instructions)
            .model(model.as_ref())
            .build()?;

        Ok(client.assistants().create(assistant_request).await?)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    const ENV_PATH: &str = "example.env";

    let env = EnvBuilder::new().path(&ENV_PATH).build();

    let OpenAIOptions {
        api_key,
        organization_id,
        project_id,
    } = env.openai_options();

    let openai_config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_org_id(organization_id)
        .with_project_id(project_id);

    let openai_client = Client::with_config(openai_config);

    let assistant = openai_client.assistants().retrieve(ASSISTANT_ID).await?;

    Ok(())
}
