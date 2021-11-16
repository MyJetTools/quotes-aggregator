use chrono::NaiveDateTime;
use prost::{DecodeError, EncodeError};

use crate::{BclDateTime};

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidAskMessageV2 {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, tag = "2")]
    pub datetime: Option<BclDateTime>,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64
}

impl BidAskMessageV2 {
    pub fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(payload)
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
        prost::Message::encode(self, dest)
    }

    pub fn serialize_to_vec(&self) -> Vec<u8>{
        let mut serialized_message = Vec::<u8>::new();
        self.serialize(serialized_message.as_mut()).unwrap();
        return serialized_message;
    }

    pub fn make_unfiltered(&self, lp: String) -> UnfilteredBidAskMessageV2{

        let unfiltered = UnfilteredBidAskMessageV2{
            id: self.id.clone(),
            datetime: self.datetime,
            bid: self.bid,
            ask: self.ask,
            source: lp,
        };

        return unfiltered;
    }


    pub fn parse_message_v2(mess: &String) -> BidAskMessageV2 {
        let message = mess.split(" ").collect::<Vec<&str>>();
    
        BidAskMessageV2 {
            id: message[0].into(),
            datetime: Some(timestamp_to_bcl(message[3].into())),
            bid: message[1].parse::<f64>().unwrap(),
            ask: message[2].parse::<f64>().unwrap(),
        }
    }
}


#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfilteredBidAskMessageV2 {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(message, tag = "2")]
    pub datetime: Option<BclDateTime>,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
    #[prost(string, tag = "5")]
    pub source: String
}

impl UnfilteredBidAskMessageV2 {
    pub fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(payload)
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
        prost::Message::encode(self, dest)
    }

    pub fn serialize_to_vec(&self) -> Vec<u8>{
        let mut serialized_message = Vec::<u8>::new();
        self.serialize(serialized_message.as_mut()).unwrap();
        return serialized_message;
    }

}
    
fn timestamp_to_bcl(str: String) -> BclDateTime {
    let date = NaiveDateTime::parse_from_str(&str, "%Y%m%d%H%M%S%3f").unwrap();
    return BclDateTime::from_miliseconds(date.timestamp_millis());
}
