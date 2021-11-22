use std::sync::Arc;

use crate::{AppContext, BidAskMessage, LpBidAsk, NoSqlInstrumentModel, ProcessBidAskError};

use super::process_bid_ask;

pub async fn process_ticker(app: &AppContext, bidask: LpBidAsk) -> Result<(), ProcessBidAskError> {
    let ticker_config = app
        .tickers_reader
        .as_ref()
        .unwrap()
        .get_by_lp_and_ticker(&bidask.lp, &bidask.bidask.id)
        .await;

    let instruments = app.instruments_reader.as_ref().unwrap().get_all().await;

    if ticker_config.is_none() || instruments.is_none() {
        return Err(ProcessBidAskError::TickerOrInstrumentNotFound);
    }

    let ticker_config = ticker_config.unwrap();
    let instruments = instruments.unwrap();

    let instruments = instruments.values().filter(
        |inst| inst.ticker == ticker_config.ticker_group
    ).collect::<Vec<&Arc<NoSqlInstrumentModel>>>();


    for inst in instruments{
        let ticker_to_bidask = LpBidAsk {
            lp: bidask.lp.clone(),
            bidask: BidAskMessage {
                id: inst.id.clone(),
                datetime: bidask.bidask.datetime,
                bid: bidask.bidask.bid,
                ask: bidask.bidask.ask
            },
        };

        let process_result = process_bid_ask(app, ticker_to_bidask).await;

        if process_result.is_err() {
            return Err(process_result.err().unwrap());
        }
        
    }

    return Ok(());

}
