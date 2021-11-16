use crate::{BidAskMessage, UnfilteredBidAskMessage};

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

    pub fn get_unfilered_bidask(&self) -> UnfilteredBidAskMessage {
        return UnfilteredBidAskMessage {
            id: self.bidask.id.clone(),
            datetime: self.bidask.datetime,
            bid: self.bidask.bid,
            ask: self.bidask.ask,
            source: self.lp.clone(),
        };
    }

    pub fn get_bidask(&self) -> BidAskMessage {
        return BidAskMessage {
            id: self.bidask.id.clone(),
            datetime: self.bidask.datetime,
            bid: self.bidask.bid,
            ask: self.bidask.ask
        };
    }
}
