use producer::produce;

mod consumer;
mod producer;

#[tokio::main]
async fn main() {
    let producer = producer::create();
    produce(producer, String::from("Hello World, Testing!!!")).await;

    consumer::start().await;
}
