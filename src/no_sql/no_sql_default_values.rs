use my_no_sql_tcp_reader::MyNoSqlEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoSqlDefaultValue {
    #[serde(rename = "RowKey")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String
}


impl MyNoSqlEntity for NoSqlDefaultValue {
    fn get_partition_key(&self) -> &str {
        "df"
    }

    fn get_row_key(&self) -> &str {
        &self.key
    }

    fn get_time_stamp(&self) -> i64 {
        self.time_stamp.parse::<i64>().unwrap()
    }
}
