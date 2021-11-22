use std::sync::Arc;

use crate::{AppContext, LpBidAsk, send_bid_ask};

use super::send_bid_ask_error::ProcessBidAskError;

pub async fn process_bid_ask(app: &AppContext, bidask :LpBidAsk)  -> Result<(), ProcessBidAskError> {
   let result =  send_bid_ask(app, bidask).await;

   return result;
}