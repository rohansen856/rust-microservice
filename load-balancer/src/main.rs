use dotenv::dotenv;
use regex::Regex;

use load_balancer::LoadBalancer;

#[actix_web::main]
async fn main(){
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

    // Use a regex to filter environment variables with "SERVER_URL_" prefix
    let server_regex = Regex::new(r"^SERVER_URL_\d+$").unwrap();
    let servers: Vec<String> = std::env::vars()
        .filter_map(|(key, value)| {
            if server_regex.is_match(&key) {
                Some(value)
            } else {
                None
            }
        })
        .collect();

    if servers.is_empty() {
        panic!("No servers configured. Please set SERVER_URL_1, SERVER_URL_2,... etc. in the .env file.");
    }
    println!("backend server urls: {:?}", servers);

    let load_balancer = LoadBalancer::new(port, servers);
    println!("Load Balancer running on {}", load_balancer.uri());
    load_balancer.run().await
}
