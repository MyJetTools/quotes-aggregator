use my_no_sql_tcp_reader::{MyNoSqlDataReader, MyNoSqlTcpConnection};
use my_service_bus_tcp_client::MyServiceBusClient;
use prometheus::core::{AtomicF64, GenericCounter};
use quotes_mixer::{BidAskTcpServer, LpBidAsk, Metrics, NoSqlInstrumentModel, Settings, http_start};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use stopwatch::Stopwatch;
use tokio::{fs, sync::mpsc::UnboundedReceiver};

const APP_NAME: &str = "rust_price_mixer";

#[tokio::main]
async fn main() {
    let settings = parse_settings().await;
    let metrics = Arc::new(Metrics::new());

    let mut nosql_client = MyNoSqlTcpConnection::new(settings.no_sql_url,APP_NAME.clone().into());
    let instruments_reader = nosql_client.get_reader::<NoSqlInstrumentModel>("instruments".into()).await;

    let bid_ask_servers = settings.lps.iter().map(|lp| {
        return BidAskTcpServer::new(lp.hostport.clone(), lp.name.clone(), lp.instruments.clone());
    })
    .collect::<Vec<BidAskTcpServer>>();

    let mut sb_client = MyServiceBusClient::new(
        settings.sb_url.as_str(),
        APP_NAME,
        "1.0.0",
        Duration::new(3, 0),
        Duration::new(3, 0),
    );

    sb_client.start().await;
    nosql_client.start();

    let client = Arc::new(sb_client);

    for mut itm in bid_ask_servers{
        let receiver = itm.subscribe();

        let cl_client = client.clone();
        let met = metrics.clone().to_owned();
        let reader = instruments_reader.clone();

        tokio::task::spawn(async move { handle_event(receiver, cl_client, met, reader).await });
        tokio::task::spawn(async move { itm.connect().await });
    }

    tokio::spawn(http_start(SocketAddr::from(([0, 0, 0, 0], 8081)), metrics.clone()));


    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn handle_event(mut rx: UnboundedReceiver<LpBidAsk>, sb_client: Arc<MyServiceBusClient>, metrics: Arc<Metrics>, instruments_reader: Arc<MyNoSqlDataReader<NoSqlInstrumentModel>>) {
    loop {
        let sw_process = Stopwatch::start_new();

        let mut messages = Vec::<Vec<u8>>::new();
        let mut instrument_metrics: HashMap<String, GenericCounter<AtomicF64>> = HashMap::new();

        loop {
            let bidask = rx.recv().await.unwrap();
            if  messages.len() <= 1 {
                let instrument = instruments_reader.get_entity("i".into(), &bidask.bidask.id).await;

                if instrument.is_none() {
                    continue;
                }

                let key = format!("{}-{}", bidask.lp, bidask.bidask.id);
                let mut serialized_message = Vec::<u8>::new();
                bidask.bidask.serialize(serialized_message.as_mut()).unwrap();

                let mut mess_with_splitter = vec![0];
                mess_with_splitter.extend(serialized_message);
                messages.push(mess_with_splitter);

                match instrument_metrics.get(&key) {
                    Some(metric) => metric.inc(),
                    None => {
                        let metric_to_insert_into_list = 
                            metrics.average_income_to_socket.with_label_values(&[&bidask.bidask.id, &bidask.lp]);
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

async fn parse_settings() -> Settings{
    let content = fs::read_to_string("./settings.json").await.unwrap();
    let parsed_json : Settings = serde_json::from_str(&content).unwrap();
    return parsed_json;
}