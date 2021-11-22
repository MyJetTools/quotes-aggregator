use my_no_sql_tcp_reader::MyNoSqlEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickerSourceSettings{
    #[serde(rename = "ExchangeTicker")]
    pub exchange_ticker: String,
    #[serde(rename = "LiquidityProvider")]
    pub liquidity_provider: String,
    #[serde(rename = "Weight")]
    pub weight: i64,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoSqlTickerModel {
    #[serde(rename = "PartitionKey")]
    pub Lp: String,
    #[serde(rename = "RowKey")]
    pub LpBidAsk: String,
    #[serde(rename = "TickerGroup")]
    pub ticker_group: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String
}

impl MyNoSqlEntity for NoSqlTickerModel {
    fn get_partition_key(&self) -> &str {
        &self.Lp
    }

    fn get_row_key(&self) -> &str {
        &self.LpBidAsk
    }

    fn get_time_stamp(&self) -> i64 {
        self.time_stamp.parse::<i64>().unwrap()
    }

}
