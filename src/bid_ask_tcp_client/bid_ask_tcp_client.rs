use std::collections::HashSet;

use tokio::io::{self};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{BidAskMessage, BidAskMessageV2, LpBidAsk};

use super::QuotesReader;

pub struct BidAskTcpServer {
    lp: String,
    hostport: String,
    sender: Option<UnboundedSender<LpBidAsk>>,
    instruments_to_handle: HashSet<String>,
    contract_version: u8
}


impl BidAskTcpServer {
    pub fn new(
        hostport: String,
        lp: String,
        instruments_to_handle: HashSet<String>,
        sb_contract_version: u8
    ) -> BidAskTcpServer {
        BidAskTcpServer {
            hostport: hostport,
            sender: None,
            lp: lp,
            instruments_to_handle: instruments_to_handle,
            contract_version: sb_contract_version

        }
    }

    pub fn subscribe(&mut self) -> UnboundedReceiver<LpBidAsk> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<LpBidAsk>();
        self.sender = Some(tx);
        return rx;
    }

    pub async fn connect(&mut self) -> QuotesReader {
        println!("Tcp connect");

        if self.sender.is_none() {
            panic!("Not found subscriber for socket.");
        }

        loop {
            let socket = TcpStream::connect(self.hostport.as_str())
                .await
                .expect(&format!(
                    "Cant connect to lp socket into: {}",
                    &self.hostport
                ));
            let (rd, _) = io::split(socket);
            let mut reader = QuotesReader::new(rd);

            loop {
                match reader.read_next().await {
                    Some(messages) => {
                        for mess in messages {
                            let sb_contract = BidAskMessage::parse_message_v1(&mess);

                            if !self.instruments_to_handle.contains(&sb_contract.id) {
                                continue;
                            }

                            let message = LpBidAsk::new(self.lp.clone(), sb_contract);
                            self.sender.as_ref().unwrap().send(message).unwrap();
                        }
                    }
                    None => {}
                }
            }
        }
    }
}