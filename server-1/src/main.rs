use actix_web::{web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use dotenv::dotenv;

mod services;
use services::{create_todo, fetch_todos};

mod header;
use header::CustomHeader;

mod kafka;
use kafka::KafkaProducer;

mod prom;
use prom::PrometheusMetrics;

pub struct AppState {
    db: Pool<Postgres>,
    kafka_producer: Arc<KafkaProducer>,
    prometheus: Arc<PrometheusMetrics>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: u16 = std::env::var("PORT")
        .expect("PORT environment variable not set")
        .parse()
        .expect("PORT must be a valid integer");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let kafka_brokers = std::env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());
    let kafka_producer = Arc::new(KafkaProducer::new(&kafka_brokers));

    // Initialize Prometheus metrics
    let prometheus = Arc::new(PrometheusMetrics::new());

    println!("Server running on port: {}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: pool.clone(),
                kafka_producer: kafka_producer.clone(),
                prometheus: prometheus.clone(),
            }))
            .wrap(CustomHeader)
            .service(create_todo)
            .service(fetch_todos)
            .route("/metrics", actix_web::web::get().to(PrometheusMetrics::metrics_handler))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
