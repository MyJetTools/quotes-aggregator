use prometheus::{CounterVec, Encoder, GaugeVec, Opts, Registry, TextEncoder};

pub struct Metrics {
    registry: Registry,
    pub average_income_to_socket: CounterVec,
    pub average_income_to_socket_process_time: GaugeVec,
}

impl Metrics {

    pub fn new() -> Metrics{
        let registry = Registry::new();

        let average_income = CounterVec::new(Opts::new("average_income_to_socket", "average income into tcp socket"), &["instrument", "lp"]).unwrap();
        let average_income_process_time = GaugeVec::new(Opts::new("average_income_to_socket_process_time", "average income into socket process time (sum)"), &["instrument", "lp"]).unwrap();

    
        registry.register(Box::new(average_income.clone())).unwrap();
        registry.register(Box::new(average_income_process_time.clone())).unwrap();

        Metrics{
            registry: registry,
            average_income_to_socket: average_income,
            average_income_to_socket_process_time: average_income_process_time  
        }
    }

    pub fn get_data(&self) -> String {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        return String::from_utf8(buffer).unwrap(); 
    }
}
