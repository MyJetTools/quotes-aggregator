use chrono::{NaiveDateTime};
use my_service_bus_tcp_client::MyServiceBusClient;
use quotes_mixer::{BclDateTime, BidAskMessage, BidAskTcpServer, Settings};
use std::{sync::Arc, time::Duration};
use stopwatch::Stopwatch;
use tokio::{fs, sync::mpsc::UnboundedReceiver};

#[tokio::main]
async fn main() {
    let settings = parse_settings().await;

    let bid_ask_servers = settings.lps.iter().map(|lp| {
        return BidAskTcpServer::new(lp.name.clone(), lp.hostport.clone());
    })
    .collect::<Vec<BidAskTcpServer>>();

    let mut pure_client = MyServiceBusClient::new(
        settings.sb_url.as_str(),
        "rust_price_mixer",
        "1.0.0",
        Duration::new(3, 0),
        Duration::new(3, 0),
    );

    pure_client.start().await;

    let client = Arc::new(pure_client);

    for mut itm in bid_ask_servers{
        let receiver = itm.subscribe();

        let cl_client = client.clone();

        tokio::task::spawn(async move { handle_event(receiver, cl_client).await });
        tokio::task::spawn(async move { itm.connect().await });
    }

    println!("Sb start end");

    tokio::time::sleep(Duration::new(9999999, 0)).await;
}

async fn handle_event(mut rx: UnboundedReceiver<String>, sb_client: Arc<MyServiceBusClient>) {
    loop {
        let sw_process = Stopwatch::start_new();

        let mut messages = Vec::<Vec<u8>>::new();

        loop {
            let line = rx.recv().await;

            if line.is_some() && messages.len() <= 100 {
                let sb_contract = parse_message(&line.unwrap());
                let mut serialized_message = Vec::<u8>::new();
                sb_contract.serialize(serialized_message.as_mut()).unwrap();
                messages.push(serialized_message);
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