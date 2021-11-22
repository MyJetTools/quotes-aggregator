use my_no_sql_tcp_reader::MyNoSqlEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoSqlInstrumentModel {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "Ticker")]
    pub ticker: String
}

impl MyNoSqlEntity for NoSqlInstrumentModel {
    fn get_partition_key(&self) -> &str {
        "i"
    }

    fn get_row_key(&self) -> &str {
        &self.id
    }

    fn get_time_stamp(&self) -> i64 {
        self.time_stamp.parse::<i64>().unwrap()
    }
    
}
