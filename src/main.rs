use quotes_mixer::{http_start, start_bid_ask_client, AppContext, BidAskTcpClient, Settings};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{fs};

#[tokio::main]
async fn main() {
    let settings = parse_settings().await;
    let app = AppContext::new(settings);

    let (app, mut nosql_client) = app.init_nosql().await;
    let (app, _) = app.init_sb().await;

    let context = Arc::new(app);
    tokio::spawn(http_start(SocketAddr::from(([0, 0, 0, 0], 8081)), context.clone()));
    kickoff_tcp_clients(context.clone());

    nosql_client.start();

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn parse_settings() -> Settings {
    let content = fs::read_to_string("./settings.json").await.unwrap();
    let parsed_json: Settings = serde_json::from_str(&content).unwrap();
    return parsed_json;
}

fn get_bid_ask_servers(settings: Arc<Settings>, app: Arc<AppContext>) -> Vec<BidAskTcpClient> {
    return settings
        .lps
        .iter()
        .map(|lp| {
            return BidAskTcpClient::new(
                app.clone(),
                lp.hostport.clone(),
                lp.name.clone(),
                lp.instruments.clone(),
            );
        })
        .collect::<Vec<BidAskTcpClient>>();
}

fn kickoff_tcp_clients(app: Arc<AppContext>) {
    let bid_ask_servers = get_bid_ask_servers(app.settings.clone(), app);

    for client in bid_ask_servers {
        tokio::task::spawn(async move { start_bid_ask_client(Arc::new(client)).await });
    }
}
