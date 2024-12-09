use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use std::time::Duration;
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Self {
        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", brokers);
    
        let producer: FutureProducer = config.create().expect("Failure in creating producer");

        KafkaProducer { producer }
    }

    pub async fn produce(&self, topic: &str, msg: &str) {
        let record = FutureRecord::to(topic)
            .payload(msg)
            .key("Test-Key");
    
        let status_delivery =  self.producer
            .send(record, Timeout::After(Duration::from_secs(2)))
            .await;
    
        match status_delivery {
            Ok(report) => println!("Message sent {:?}", report),
            Err(e) => println!("Error producing.. {:?}", e),
        }
    }
}


