use std::collections::HashSet;

use chrono::NaiveDateTime;
use tokio::io::{self, AsyncReadExt, ReadHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{BclDateTime, BidAskMessage};

pub struct QuotesReader {
    reader: ReadHalf<TcpStream>,
    messages: Vec<String>,
    last_serialize_vector: Vec<u8>,
}

impl QuotesReader {
    fn new(reader: ReadHalf<TcpStream>) -> QuotesReader {
        QuotesReader {
            reader: reader,
            messages: Vec::new(),
            last_serialize_vector: Vec::new(),
        }
    }

    async fn read_next(&mut self) -> Option<Vec<String>> {
        let mut buf: Vec<u8> = vec![0; 1024];
        let readed_bytes = self.reader.read(&mut buf).await.unwrap();

        let mut serialize_buff: Vec<u8> = self.last_serialize_vector.clone();
        for byte in &buf[..readed_bytes] {
            serialize_buff.push(byte.clone());

            let buff_len = serialize_buff.len();

            if buff_len < 2 {
                continue;
            }

            if serialize_buff[buff_len - 1] == 10 && serialize_buff[buff_len - 2] == 13 {
                let serialized_message =
                    std::str::from_utf8(&serialize_buff[..buff_len - 2]).unwrap();
                self.messages.push(serialized_message.clone().into());
                serialize_buff.clear();
            }
        }

        self.last_serialize_vector = serialize_buff;

        if self.messages.len() > 0 {
            let value_to_return = self.messages.clone();
            self.messages.clear();
            return Some(value_to_return);
        }

        return None;
    }
}

pub struct BidAskTcpServer {
    lp: String,
    hostport: String,
    sender: Option<UnboundedSender<LpBidAsk>>,
    instruments_to_handle: HashSet<String>
}

#[derive(Clone, Debug)]
pub struct LpBidAsk {
    pub lp: String,
    pub bidask: BidAskMessage,
}

impl LpBidAsk {
    pub fn new(lp: String, mess: BidAskMessage) -> LpBidAsk {
        LpBidAsk {
            lp: lp,
            bidask: mess,
        }
    }
}

impl BidAskTcpServer {
    pub fn new(hostport: String, lp: String, instruments_to_handle: HashSet<String>) -> BidAskTcpServer {
        BidAskTcpServer {
            hostport: hostport,
            sender: None,
            lp: lp,
            instruments_to_handle: instruments_to_handle
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
                            let sb_contract = parse_message(&mess);

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
