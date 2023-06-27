pub mod req;
pub mod res;

#[cfg(test)]
mod tests;

use reqwest::{Client, RequestBuilder};

pub struct Api {
    path: String,
    client: Client,
    key: String,
    account_id: String,
    login: Option<res::Login>,
}

impl Api {
    pub async fn send<R: req::Request>(&self, request: &R) -> Result<R::Response, Error> {
        let builder = self.build_request(request);

        let response: res::Result<R::Response> =
            builder.json(&request).send().await?.json().await?;

        response.to_std_result().map_err(Error::from)
    }

    pub(crate) async fn debug_send<R: req::Request>(&self, request: &R) -> String {
        let builder = self.build_request(request);

        builder
            .json(&request)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    pub(crate) fn build_request<R: req::Request>(&self, request: &R) -> RequestBuilder {
        let mut url = self.path.clone();
        url += "/";
        url += R::PATH;
        if let Some(path) = request.path() {
            url += "/";
            url += path;
        }

        println!("request: {:?}, method: {}, url: {}", request, R::METHOD, url);

        let mut builder = self
            .client
            .request(R::METHOD, url)
            .header("X-IG-API-KEY", &self.key)
            .header("IG-ACCOUNT-ID", &self.account_id)
            .json(request);


        if let Some(login) = &self.login {
            builder = builder.bearer_auth(&login.oauth_token.access_token);
        } else {
            builder = builder.header("VERSION", "3");
        }
        
        builder
    }

    pub async fn login() -> Result<Api, Error> {
        dotenv::dotenv().ok();

        let client = reqwest::Client::new();

        let mut api = Api {
            path: String::from("https://demo-api.ig.com/gateway/deal"),
            client,
            key: dotenv::var("IG_API_KEY").unwrap(),
            account_id: dotenv::var("IG_ACCOUNT_ID").unwrap(),
            login: None,
        };

        let login = req::Login {
            identifier: &dotenv::var("IG_USERNAME").unwrap(),
            password: &dotenv::var("IG_PASSWORD").unwrap(),
        };

        api.login = Some(api.send(&login).await?);
        
        /* 
        let map = std::collections::HashMap::<String, String>::new();
        
        let res = api.client.get("https://demo-api.ig.com/gateway/deal/marketnavigation")
            //.header("VERSION", "3")
            .header("X-IG-API-KEY", &api.key)
            .header("IG-ACCOUNT-ID", "Z59JTV")
            .header("Authorization", format!("{} {}", &api.login.clone().unwrap().oauth_token.token_type, &api.login.clone().unwrap().oauth_token.access_token))
            .json(&map)
            .send()
            .await
            .unwrap();

        println!("{:?}", res.text().await);
        */

        Ok(api)
    }
}

#[derive(Debug)]
pub enum Error {
    Response(res::Error),
    Request(reqwest::Error),
}

impl From<res::Error> for Error {
    fn from(error: res::Error) -> Self {
        Error::Response(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(error)
    }
}
