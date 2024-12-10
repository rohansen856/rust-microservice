use std::sync::Arc;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::{create_todo, fetch_todos};

mod header;
use header::CustomHeader;

mod kafka;
use kafka::KafkaProducer;

pub struct AppState {
    db: Pool<Postgres>,
    kafka_producer: Arc<KafkaProducer>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Attempt to read and parse an environment variable as an integer
    let port: u16 = match std::env::var("PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Error: PORT must be a valid integer.");
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprintln!("Error: PORT environment variable not set.");
            std::process::exit(1);
        }
    };

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let kafka_brokers = std::env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());
    let kafka_producer = Arc::new(KafkaProducer::new(&kafka_brokers));

    println!("Server 2 running on port: {}", port.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone(), kafka_producer: kafka_producer.clone(), }))
            .wrap(CustomHeader)
            .service(create_todo)
            .service(fetch_todos)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
