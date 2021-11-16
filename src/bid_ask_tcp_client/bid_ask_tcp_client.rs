use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{self};
use tokio::net::TcpStream;
use tokio::time::sleep;

use crate::{send_bid_ask, AppContext, BidAskMessage, LpBidAsk};

use super::dead_socket_detector::{DeadSocketDetector};
use super::QuotesReader;

pub struct BidAskTcpClient {
    app: Arc<AppContext>,
    instruments_to_handle: HashSet<String>,
    lp: String,
    dead_socket_detector: Arc<DeadSocketDetector>,
    hostport: String,
}

impl BidAskTcpClient {
    pub fn new(
        app: Arc<AppContext>,
        hostport: String,
        lp: String,
        instruments_to_handle: HashSet<String>,
    ) -> BidAskTcpClient {
        BidAskTcpClient {
            app: app,
            instruments_to_handle: instruments_to_handle,
            lp,
            dead_socket_detector: Arc::new(DeadSocketDetector::new(9)),
            hostport,
        }
    }
}

pub async fn start_bid_ask_client(client: Arc<BidAskTcpClient>) -> QuotesReader {
    loop {
        let client_clone = client.clone();
        sleep(Duration::from_secs(3)).await;
        client.dead_socket_detector.reset();
        println!(
            "Tcp connect. Hostport: {}. Lp: {}",
            client_clone.hostport, client_clone.lp
        );

        let socket = TcpStream::connect(client_clone.hostport.as_str())
            .await
            .expect(&format!(
                "Cant connect to lp socket into: {}",
                &client_clone.hostport
            ));
        let (rd, _) = io::split(socket);
        let mut reader = QuotesReader::new(rd);


        let reed_loop_client = client_clone.clone();
        tokio::task::spawn(async move {
            read_loop(
                reed_loop_client.app.clone(),
                reed_loop_client.instruments_to_handle.clone(),
                reed_loop_client.lp.clone(),
                &mut reader,
                reed_loop_client.dead_socket_detector.clone()
            )
            .await;
        });

        start_dead_socket_detector(client_clone.clone()).await;
    }
}

pub async fn start_dead_socket_detector(client: Arc<BidAskTcpClient>) {
    loop {

        let socket_status = client.dead_socket_detector.is_timeout();
        sleep(Duration::from_secs(3)).await;

        match socket_status {
            super::dead_socket_detector::SocketTimeoutStatus::Ok => break,
            super::dead_socket_detector::SocketTimeoutStatus::Timeout(timeout) => {
                println!(
                    "Long time no message from lp: {}. Timeout: {}, Disconect....",
                    client.lp, timeout
                );

                break;
            }
        }
    }
}

pub async fn read_loop(
    app: Arc<AppContext>,
    instruments_to_handle: HashSet<String>,
    lp: String,
    reader: &mut QuotesReader,
    dead_socket_detector: Arc<DeadSocketDetector>
) {
    loop {
        match reader.read_next().await {
            Some(messages) => {
                for mess in messages {
                    let sb_contract = BidAskMessage::parse_message_v1(&mess);

                    if !instruments_to_handle.contains(&sb_contract.id) {
                        continue;
                    }

                    let message = LpBidAsk::new(lp.clone(), sb_contract);

                    let publish_result = send_bid_ask(app.as_ref(), message).await;

                    match publish_result {
                        Ok(_) => { dead_socket_detector.track_event() }
                        Err(err) => {
                            println!("Cant publish bid ask. Break cycle. Lp: {}. Err: {:?}", lp, err.get_text());
                            break;
                        }
                    }
                }
            }
            None => {
                println!("Somehow no message. Write to statistic. Lp: {}", lp)
            }
        }
    }
}
