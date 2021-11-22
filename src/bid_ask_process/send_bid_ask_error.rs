pub enum ProcessBidAskError {
    Ok,
    TickerOrInstrumentNotFound,
    PublishError(String)
}