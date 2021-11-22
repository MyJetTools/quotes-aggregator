use std::{collections::BTreeMap, sync::Arc};

use my_no_sql_tcp_reader::MyNoSqlDataReader;

use crate::NoSqlInstrumentModel;

pub struct NoSqlInstrumentReader{
    reader: Arc<MyNoSqlDataReader<NoSqlInstrumentModel>>
}

impl NoSqlInstrumentReader {
    pub fn new(reader: Arc<MyNoSqlDataReader<NoSqlInstrumentModel>>) -> NoSqlInstrumentReader{
        return NoSqlInstrumentReader{
            reader: reader
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Option<Arc<NoSqlInstrumentModel>>{
        let instrument = self.reader
            .get_entity("i".into(), id)
        .await;

        return instrument;
    }

    pub async fn get_all(&self) -> Option<BTreeMap<String, Arc<NoSqlInstrumentModel>>>{
        let instrument = self.reader.get_by_partition_key("i".into()).await;

        return instrument;
    }
}