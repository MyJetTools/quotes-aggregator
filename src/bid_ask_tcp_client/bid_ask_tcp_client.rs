use tokio::io::{self, AsyncReadExt, ReadHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct  QuotesReader{
    reader: ReadHalf<TcpStream>,
    messages: Vec<String>,
    last_serialize_vector: Vec<u8>
}

impl QuotesReader {
    fn new(reader: ReadHalf<TcpStream>) -> QuotesReader{
        QuotesReader{
            reader: reader,
            messages: Vec::new(),
            last_serialize_vector: Vec::new()
        }
    }

    async fn read_next(&mut self) -> Option<Vec<String>>{
        let mut buf: Vec<u8> = vec![0; 1024];
        let readed_bytes = self.reader.read(&mut buf).await.unwrap();

        let mut serialize_buff: Vec<u8> = self.last_serialize_vector.clone();
        for byte in &buf[..readed_bytes]{
            serialize_buff.push(byte.clone());

            let buff_len = serialize_buff.len();

            if buff_len < 2 {
                continue;
            }

            if serialize_buff[buff_len -1] == 10 && serialize_buff[buff_len -2] == 13 {
                let serialized_message = std::str::from_utf8(&serialize_buff[..buff_len-2]).unwrap();
                self.messages.push(serialized_message.clone().into());
                serialize_buff.clear();
            }
        }

        self.last_serialize_vector = serialize_buff;


        if self.messages.len() > 0 {
            let value_to_return = self.messages.clone();
            self.messages.clear();
            return Some(value_to_return);
        }

        return None;
    }


}

pub struct BidAskTcpServer {
    lp: String,
    hostport: String,
    sender: Option<UnboundedSender<(String, String)>>,
}

impl BidAskTcpServer {
    pub fn new(hostport: String, lp: String) -> BidAskTcpServer {
        BidAskTcpServer {
            hostport: hostport,
            sender: None,
            lp: lp
        }
    }

    pub fn subscribe(&mut self) -> UnboundedReceiver<(String, String)>{
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<(String, String)>();
        self.sender = Some(tx);
        return rx;
    }

    pub async fn connect(&mut self) -> QuotesReader {

        println!("Tcp connect");

        if self.sender.is_none() {
            panic!("Not found subscriber for socket.");
        }

        loop {
            let socket = TcpStream::connect(self.hostport.as_str()).await.unwrap();
            let (rd, _) = io::split(socket);
            let mut reader = QuotesReader::new(rd);

            loop {
                match reader.read_next().await {
                    Some(messages) => {
                        for mess in messages {
                            self.sender.as_ref().unwrap().send((self.lp.clone() ,mess)).unwrap();
                        }
                    }
                    None => {}
                }
            }


            
        }
    }
}
