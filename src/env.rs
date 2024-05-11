use strum::AsRefStr;

#[derive(AsRefStr)]
enum EnvKey {
    #[strum(serialize = "OPENAI_API_KEY")]
    OpenAIAPIKey,
    #[strum(serialize = "OPENAI_ORG_ID")]
    OpenAIOrganizationID,
    #[strum(serialize = "OPENAI_PROJ_ID")]
    OpenAIProjectID,
}

#[derive(Default)]
pub struct EnvBuilder {
    env_path: Option<&'static str>,
}

impl EnvBuilder {
    pub fn new() -> Self {
        EnvBuilder::default()
    }

    pub fn path(mut self, env_path: &'static str) -> Self {
        self.env_path = Some(env_path);
        self
    }

    pub fn build(self) -> Env {
        dotenv::from_filename(self.env_path.unwrap()).unwrap();
        Env
    }
}

#[derive(Debug)]
pub struct OpenAIOptions {
    pub api_key: String,
    pub organization_id: String,
    pub project_id: String,
}

pub struct Env;

impl Env {
    pub fn openai_options(&self) -> OpenAIOptions {
        OpenAIOptions {
            api_key: dotenv::var(EnvKey::OpenAIAPIKey.as_ref()).unwrap(),
            organization_id: dotenv::var(EnvKey::OpenAIOrganizationID.as_ref()).unwrap(),
            project_id: dotenv::var(EnvKey::OpenAIProjectID.as_ref()).unwrap(),
        }
    }
}
