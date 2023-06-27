use serde::{de::DeserializeOwned, Deserialize};

pub trait Response: DeserializeOwned + std::fmt::Debug + Clone {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub error_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Result<R> {
    Ok(R),
    Err(Error),
}

impl<R> Result<R> {
    pub(crate) fn to_std_result(self) -> std::result::Result<R, Error> {
        match self {
            Result::Ok(r) => Ok(r),
            Result::Err(e) => Err(e),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub expires_in: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    pub client_id: String,
    pub account_id: String,
    pub timezone_offset: u32,
    pub lightstreamer_endpoint: String,
    pub oauth_token: OAuthToken,
}

impl Response for Login {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub epic: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nodes {
    pub nodes: Vec<Node>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Markets {
    pub markets: Vec<Market>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum NodesOrMarkets {
    Nodes(Nodes),
    Markets(Markets),
}

impl Response for NodesOrMarkets {}