pub mod req;
pub mod res;

use std::{collections::HashMap};

pub struct Api {
}

impl Api {
    pub async fn send<R: req::Request>(request: R) -> R::Response {
        todo!()
    }

    pub async fn login_env() {
        dotenv::dotenv().ok();
        Api::login(&dotenv::var("IG_TRADING_USERNAME").unwrap(), &dotenv::var("IG_TRADING_PASSWORD").unwrap(), &dotenv::var("IG_TRADING_KEY").unwrap()).await
    }

    pub async fn login(username: &str, password: &str, key: &str) {
        let client = reqwest::Client::new();
        
        let mut map = HashMap::new();
        map.insert("identifier", username);
        map.insert("password", password);
        
        let res = client.post("https://demo-api.ig.com/gateway/deal/session")
            .header("VERSION", "3")
            .header("X-IG-API-KEY", key)
            .header("IG-ACCOUNT-ID", "Z59JTV")
            .json(&map)
            .send()
            .await
            .unwrap();

        let login = res.json::<res::Login>().await.unwrap();

        println!("{:?}", login);

        let res = client.get("https://demo-api.ig.com/gateway/deal/marketnavigation")
            .header("VERSION", "1")
            .header("X-IG-API-KEY", key)
            .header("IG-ACCOUNT-ID", "Z59JTV")
            .header("Authorization", format!("{} {}", login.oauth_token.token_type, login.oauth_token.access_token))
            .json(&map)
            .send()
            .await
            .unwrap();

        println!("{:?}", res.text().await);
    }
}

#[tokio::test]
async fn test() {
    Api::login_env().await
}


