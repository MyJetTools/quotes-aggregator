use crate::{AppContext, BidAskMessage, LpBidAsk, ProcessBidAskError, UnfilteredBidAskMessage};


impl ProcessBidAskError {
    pub fn get_text(&self) -> String{
        match self {
            ProcessBidAskError::Ok => "Ok".into(),
            ProcessBidAskError::TickerOrInstrumentNotFound => "ticker not found".into(),
            ProcessBidAskError::PublishError(ms) => format!("Timeout: {} ms", ms),
        }
    }
}

pub async fn send_bid_ask(app: &AppContext, bidask: LpBidAsk) -> Result<(), ProcessBidAskError> {
    loop {

        app.metrics
            .average_income_to_socket
            .with_label_values(&[&bidask.bidask.id, &bidask.lp])
            .inc();

        let unfiltered_publish_result = app
            .unfiltered_bid_ask_publisher
            .as_ref()
            .unwrap()
            .publish::<UnfilteredBidAskMessage>(&mut bidask.get_unfilered_bidask(), 1)
            .await;


        if unfiltered_publish_result.is_err() {
            return Err(ProcessBidAskError::PublishError("Cant publish to unfiltered bid ask".into()));
        }

        let instrument = app
            .instruments_reader
            .as_ref()
            .unwrap()
            .get_by_id(&bidask.bidask.id)
            .await;

        if instrument.is_none() {
            continue;
        }

        let publish_result = app
            .bid_ask_publisher
            .as_ref()
            .unwrap()
            .publish::<BidAskMessage>(&mut bidask.get_bidask(), 1)
            .await;

            return match publish_result {
                Ok(_) => Ok(()),
                Err(err) => Err(ProcessBidAskError::PublishError(err)),
            }

    }
}