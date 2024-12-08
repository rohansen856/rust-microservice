use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::{ClientConfig, Message};

pub async fn start() {
    let consumer: StreamConsumer = create();
    consume(consumer).await
}

fn create() -> StreamConsumer {
    // Bind the new ClientConfig to a variable to extend its lifetime
    let mut base_config = ClientConfig::new();
    let config = base_config
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .set("group.id", "test-group")
        .set("socket.timeout.ms", "4000");

    let consumer: StreamConsumer = config.create().expect("Fail to create consumer");

    consumer
}

async fn consume(consumer: StreamConsumer) {
    consumer
        .subscribe(&["test-topic"])
        .expect("Can't subscribe");

    loop {
        match consumer.recv().await {
            Err(e) => println!("{:?}", e),
            Ok(message) => {
                match message.payload_view::<str>() {
                    None => println!("None Message"),
                    Some(Ok(msg)) => println!("Message consumed {:?}", msg),
                    Some(Err(e)) => println!("Error Parsing {:?}", e),
                }
                consumer
                    .commit_message(&message, CommitMode::Async)
                    .unwrap();
            }
        }
    }
}
