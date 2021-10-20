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
    pub ask: f64
}

impl BidAskMessage {
    pub fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(payload)
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) -> Result<(), EncodeError> {
        prost::Message::encode(self, dest)
    }

}
