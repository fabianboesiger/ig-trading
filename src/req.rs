use crate::res;
use http::Method;
use serde::Serialize;

pub trait Request: Serialize + std::fmt::Debug + Clone {
    const PATH: &'static str;
    const METHOD: Method;

    type Response: res::Response;

    fn path(&self) -> Option<&str> {
        None
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Login<'a> {
    pub identifier: &'a str,
    pub password: &'a str,
}

impl Request for Login<'_> {
    type Response = res::Login;

    const METHOD: Method = Method::POST;
    const PATH: &'static str = "session";
}

#[derive(Clone, Debug, Serialize, Default)]
pub(crate) struct MarketNavigation<'a> {
    #[serde(skip_serializing)] 
    pub id: Option<&'a str>,
}

impl<'a> Request for MarketNavigation<'a> {
    type Response = res::NodesOrMarkets;

    const METHOD: Method = Method::GET;
    const PATH: &'static str = "marketnavigation";

    fn path(&self) -> Option<&str> {
        self.id
    }
}
