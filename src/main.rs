use chrono::{NaiveDateTime};
use my_no_sql_tcp_reader::MyNoSqlTcpConnection;
use my_service_bus_tcp_client::MyServiceBusClient;
use prometheus::core::{AtomicF64, GenericCounter};
use quotes_mixer::{BclDateTime, BidAskMessage, BidAskTcpServer, Metrics, Settings, http_start};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use stopwatch::Stopwatch;
use tokio::{fs, sync::mpsc::UnboundedReceiver};

const APP_NAME: &str = "rust_price_mixer";

#[tokio::main]
async fn main() {
    let settings = parse_settings().await;
    let metrics = Arc::new(Metrics::new());

    let bid_ask_servers = settings.lps.iter().map(|lp| {
        return BidAskTcpServer::new(lp.name.clone(), lp.hostport.clone());
    })
    .collect::<Vec<BidAskTcpServer>>();

    let mut sb_client = MyServiceBusClient::new(
        settings.sb_url.as_str(),
        APP_NAME,
        "1.0.0",
        Duration::new(3, 0),
        Duration::new(3, 0),
    );


    let ns = MyNoSqlTcpConnection::new(settings.no_sql_url,APP_NAME.clone().into());

    // let instruments 

    sb_client.start().await;

    let client = Arc::new(sb_client);

    for mut itm in bid_ask_servers{
        let receiver = itm.subscribe();

        let cl_client = client.clone();
        let met = metrics.clone().to_owned();

        tokio::task::spawn(async move { handle_event(receiver, cl_client, met).await });
        tokio::task::spawn(async move { itm.connect().await });
    }

    tokio::spawn(http_start(SocketAddr::from(([0, 0, 0, 0], 8081)), metrics.clone()));


    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn handle_event(mut rx: UnboundedReceiver<(String, String)>, sb_client: Arc<MyServiceBusClient>, metrics: Arc<Metrics>) {
    loop {
        let sw_process = Stopwatch::start_new();

        let mut messages = Vec::<Vec<u8>>::new();
        let mut instrument_metrics: HashMap<String, GenericCounter<AtomicF64>> = HashMap::new();

        loop {
            let line = rx.try_recv();

            if !line.is_ok() && messages.len() <= 100 {
                let (lp_name, mess) = &line.unwrap();
                let sb_contract = parse_message(mess);

                let key = format!("{}-{}", lp_name, sb_contract.id);

                let mut serialized_message = Vec::<u8>::new();
                sb_contract.serialize(serialized_message.as_mut()).unwrap();
                messages.push(serialized_message);

                match instrument_metrics.get(&key) {
                    Some(metric) => metric.inc(),
                    None => {
                        let metric_to_insert_into_list = metrics.average_income_to_socket.with_label_values(&[&sb_contract.id, lp_name]);
                        metric_to_insert_into_list.inc();
                        instrument_metrics.insert(key, metric_to_insert_into_list.clone());
                    },
                }

                continue;
            }

            break;
        }

        let messages_count = messages.len();

        if messages_count == 0 {
            continue;
        }

        let single_quote = Stopwatch::start_new();

        match sb_client.publish_chunk("bidask", messages).await {
            Ok(_) => println!("Publish {} messages", messages_count),
            Err(_) => println!("Error"),
        }

        println!("Publish took {}ms", single_quote.elapsed_ms());
        println!("Total took {}ms", sw_process.elapsed_ms());
    }
}

fn parse_message(mess: &String) -> BidAskMessage {
    let message = mess.split(" ").collect::<Vec<&str>>();

    BidAskMessage {
        id: message[0].into(),
        datetime: Some(parse_date(message[3].into())),
        bid: message[1].parse::<f64>().unwrap(),
        ask: message[2].parse::<f64>().unwrap(),
    }
}

fn parse_date(str: String) -> BclDateTime {
    let date = NaiveDateTime::parse_from_str(&str, "%Y%m%d%H%M%S%3f").unwrap();
    return BclDateTime::from_miliseconds(date.timestamp_millis());
}

async fn parse_settings() -> Settings{
    let content = fs::read_to_string("./settings.json").await.unwrap();
    let parsed_json : Settings = serde_json::from_str(&content).unwrap();
    return parsed_json;
}