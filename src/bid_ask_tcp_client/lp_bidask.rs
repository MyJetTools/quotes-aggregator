use crate::BidAskMessage;


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