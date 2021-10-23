use crate::{BidAskMessage, BidAskMessageV2};


#[derive(Clone, Debug)]
pub struct LpBidAsk {
    pub lp: String,
    pub bidask: BidAskMessageV2,
}

impl LpBidAsk {
    pub fn new(lp: String, mess: BidAskMessageV2) -> LpBidAsk {
        LpBidAsk {
            lp: lp,
            bidask: mess,
        }
    }
}