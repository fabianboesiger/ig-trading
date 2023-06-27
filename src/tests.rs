use crate::{req, res, Api};

#[tokio::test]
async fn login() {
    Api::login().await.unwrap();
}

#[tokio::test]
async fn market_navigation() {
    let api = Api::login().await.unwrap();

    println!("{:?}", api.send(&req::MarketNavigation::default()).await.unwrap());
}
 
#[tokio::test]
async fn market_navigation_recursive() {
    let api = Api::login().await.unwrap();

    let mut id = None;

    loop {
        match api.send(&req::MarketNavigation {
            id: id.as_deref()
        }).await.unwrap() {
            res::NodesOrMarkets::Markets(markets) => {
                println!("{:?}", markets);
                break;
            },
            res::NodesOrMarkets::Nodes(nodes) => {
                id = Some(nodes.nodes[0].id.clone());
            }
        }
    }
}
