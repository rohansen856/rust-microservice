use dotenv::dotenv;
use std::env;
use rate_limiter::RateLimiter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get configuration values from environment variables
    let port: u16 = env::var("PORT")
        .expect("No PORT provided in env var")
        .parse()
        .expect("PORT must be a valid u16");
    
    let forward_url = env::var("SERVER_URL").expect("No SERVER_URL provided in env var");
    let redis_url = env::var("REDIS_URL").expect("No REDIS_URL provided in env var");
    
    // Define the request limit
    let request_limit: usize = 10; // Example limit, can also be loaded from env vars

    // Create and run the RateLimiter
    let rate_limiter = RateLimiter::new(port, forward_url, redis_url, request_limit);
    println!("Rate Limiter running on {}", rate_limiter.uri());
    rate_limiter.run().await
}
