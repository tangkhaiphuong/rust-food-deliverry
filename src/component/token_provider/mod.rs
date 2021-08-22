use chrono::Utc;
use std::error::Error;

pub trait TokenProvider {
    fn validate(token: String) -> Result<TokenPayload, Box<dyn Error + Send + Sync>>;
    fn generate(data: TokenPayload, expiry: i32) -> Result<Token, Box<dyn Error + Send + Sync>>;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct Token {
    token: String,
    created: chrono::DateTime<Utc>,
    expiry: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct TokenPayload {
    user_id: u64,
    role: String,
}
