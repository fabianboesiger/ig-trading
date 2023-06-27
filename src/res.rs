use serde::{Deserialize, de::DeserializeOwned};

pub trait Response: DeserializeOwned + std::fmt::Debug + Clone {}

#[derive(Clone, Debug, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub expires_in: String,
}

impl Response for OAuthToken {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub client_id: String,
    pub account_id: String,
    pub timezone_offset: u32,
    pub lightstreamer_endpoint: String,
    pub oauth_token: OAuthToken,
}

