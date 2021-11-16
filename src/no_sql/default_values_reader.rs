use my_no_sql_tcp_reader::MyNoSqlDataReader;

use crate::NoSqlDefaultValue;

pub struct NoSqlDefaultValueReader{
    rader: MyNoSqlDataReader<NoSqlDefaultValue>
}

impl NoSqlDefaultValueReader {
    pub fn new(reader: MyNoSqlDataReader<NoSqlDefaultValue>) -> NoSqlDefaultValueReader{
        return NoSqlDefaultValueReader{
            rader: reader
        }
    }
}