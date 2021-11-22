use std::{sync::Arc, time::Duration};

use my_no_sql_tcp_reader::MyNoSqlTcpConnection;
use my_service_bus_tcp_client::MyServiceBusClient;

use crate::{Metrics, NoSqlInstrumentModel, SbPublusher, Settings, no_sql::{MyNoSqlTickerReader, NoSqlInstrumentReader}};

const APP_NAME: &str = "rust_price_mixer";

pub struct AppContext {
    pub settings: Arc<Settings>,
    pub instruments_reader: Option<NoSqlInstrumentReader>,
    pub tickers_reader: Option<MyNoSqlTickerReader>,
    pub metrics: Arc<Metrics>,
    pub unfiltered_bid_ask_publisher: Option<SbPublusher>,
    pub bid_ask_publisher: Option<SbPublusher>,
}

impl AppContext {
    pub fn new(settings: Settings) -> AppContext {
        AppContext {
            settings: Arc::new(settings),
            instruments_reader: None,
            metrics: Arc::new(Metrics::new()),
            unfiltered_bid_ask_publisher: None,
            bid_ask_publisher: None,
            tickers_reader: None
        }
    }

    pub async fn init_nosql(mut self) -> (AppContext, MyNoSqlTcpConnection) {
        let nosql_client =
            MyNoSqlTcpConnection::new(self.settings.no_sql_url.clone(), APP_NAME.clone().into());

        let instruments_reader = nosql_client
            .get_reader::<NoSqlInstrumentModel>("instruments".into())
            .await;

        self.instruments_reader = Some(NoSqlInstrumentReader::new(instruments_reader));
        return (self, nosql_client);
    }

    pub async fn init_sb(mut self) -> (AppContext, Arc<MyServiceBusClient>) {

        let mut sb_client = MyServiceBusClient::new(
            self.settings.sb_url.as_str(),
            APP_NAME,
            "1.0.0",
            Duration::new(3, 0),
            Duration::new(3, 0),
        );
        sb_client.start().await;
        let sb_client = Arc::new(sb_client);

        self.bid_ask_publisher = Some(SbPublusher::new(sb_client.clone(), "bidask".into()));
        self.unfiltered_bid_ask_publisher = Some(SbPublusher::new(
            sb_client.clone(),
            "unfiltered-bidask".into(),
        ));

        return (self, sb_client);
    }
}
