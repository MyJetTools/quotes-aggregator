use chrono::NaiveDateTime;
use prost::{DecodeError, EncodeError};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidAskMessage {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub datetime: i64,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
}

impl BidAskMessage {
    pub fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(payload)
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
        prost::Message::encode(self, dest)
    }

    pub fn parse_message_v1(mess: &String) -> BidAskMessage {
        let message = mess.split(" ").collect::<Vec<&str>>();

        return BidAskMessage {
            id: message[0].into(),
            datetime: parse_date(message[3].into()),
            bid: message[1].parse::<f64>().unwrap(),
            ask: message[2].parse::<f64>().unwrap(),
        };
    }
}
fn parse_date(str: String) -> i64 {
    let date = NaiveDateTime::parse_from_str(&str, "%Y%m%d%H%M%S%3f").unwrap();
    return date.timestamp_millis();
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfilteredBidAskMessage {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(int64, tag = "2")]
    pub datetime: i64,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
    #[prost(string, tag = "5")]
    pub source: String,
}

impl UnfilteredBidAskMessage {
    pub fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(payload)
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
        prost::Message::encode(self, dest)
    }
}
