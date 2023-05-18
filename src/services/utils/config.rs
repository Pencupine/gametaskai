use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub openai: OpenAIConfig,
    pub electron: ElectronConfig,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIConfig {
    pub account: AccountConfig,
}

#[derive(Debug, Deserialize)]
pub struct AccountConfig {
    #[serde(rename = "api-key")]
    pub api_key: ApiKeyConfig,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiKeyConfig {
    pub ApiKey: String,
}

#[derive(Debug, Deserialize)]
pub struct ElectronConfig {
    pub path: String,
    pub is_dev: bool
}