use std::sync::Arc;

use my_service_bus_tcp_client::MyServiceBusClient;
use prost::Message;

pub struct SbPublusher {
    pub sb_client: Arc<MyServiceBusClient>,
    pub topic_name: String,
}

impl SbPublusher {
    pub fn new(sb_client: Arc<MyServiceBusClient>, topic_name: String) -> SbPublusher{
        SbPublusher{
            sb_client: sb_client,
            topic_name: topic_name
        }
    }

    pub async fn publish<T: Message>(&self, message: &mut T, version: u8) -> Result<(), String> {
        let mut serialized_message = vec![version];
        let mut buffer = Vec::<u8>::new();

        prost::Message::encode(message, &mut buffer);
        serialized_message.extend(buffer);

        return match self
            .sb_client
            .publish(&self.topic_name, serialized_message)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}