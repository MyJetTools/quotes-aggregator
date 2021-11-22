use std::{collections::BTreeMap, sync::Arc};

use my_no_sql_tcp_reader::MyNoSqlDataReader;


use super::no_sql_ticker::NoSqlTickerModel;

pub struct MyNoSqlTickerReader{
    reader: Arc<MyNoSqlDataReader<NoSqlTickerModel>>
}

impl MyNoSqlTickerReader {
    pub fn new(reader: Arc<MyNoSqlDataReader<NoSqlTickerModel>>) -> MyNoSqlTickerReader{
        return MyNoSqlTickerReader{
            reader: reader 
        }
    }

    pub async fn get_by_lp(&self, lp: &str) -> Option<BTreeMap<String, Arc<NoSqlTickerModel>>>{
        let instrument = self.reader
            .get_by_partition_key(lp)
        .await;

        return instrument;
    }

    pub async fn get_by_lp_and_ticker(&self, lp: &str, ticker: &str) -> Option<Arc<NoSqlTickerModel>>{
        let instrument = self.reader
            .get_entity(lp, ticker)
        .await;

        return instrument;
    }

    pub async fn get_all(&self) -> Option<BTreeMap<String, Arc<NoSqlTickerModel>>>{
        let instrument = self.reader.get_by_partition_key("tckr").await;

        return instrument;
    }
}