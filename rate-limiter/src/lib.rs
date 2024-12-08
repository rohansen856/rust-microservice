use std::{fmt::Display, sync::Arc};
use actix_web::{
    http::header::ContentType,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use redis::{AsyncCommands, Client as RedisClient};
use reqwest::Client;
use serde_json::json;
use tokio::sync::Mutex;

pub struct RateLimiter {
    port: u16,
    forward_url: String,
    redis_url: String,
    request_limit: usize,
}

struct AppState {
    forward_url: String,
    redis_client: Arc<Mutex<RedisClient>>,
    request_limit: usize,
}

impl RateLimiter {
    pub fn new(port: u16, forward_url: String, redis_url: String, request_limit: usize) -> Self {
        RateLimiter { port, forward_url, redis_url, request_limit }
    }

    pub fn uri(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let redis_client = RedisClient::open(self.redis_url.clone()).expect("Invalid Redis URL");
        let data = Data::new(AppState {
            forward_url: self.forward_url.clone(),
            redis_client: Arc::new(Mutex::new(redis_client)),
            request_limit: self.request_limit,
        });

        HttpServer::new(move || {
            App::new()
                .default_service(web::to(Self::handler))
                .app_data(data.clone())
        })
        .bind(("127.0.0.1", self.port))
        .unwrap()
        .run()
        .await
    }

    async fn handler(
        req: HttpRequest,
        data: Data<AppState>,
        bytes: web::Bytes,
    ) -> Result<HttpResponse, RateLimitError> {
        let client_ip = req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_else(|| "unknown".to_string());
        let redis_client = data.redis_client.clone();
        let mut con = redis_client.lock().await.get_multiplexed_async_connection().await.unwrap();

        // Increment request count and check against limit
        let _: () = con.incr(client_ip.clone(), 1).await.unwrap();
        let _: () = con.expire(client_ip.clone(), 10).await.unwrap();
        let request_count: i32 = con.get(client_ip.clone()).await.unwrap_or(0);

        if request_count > data.request_limit as i32 {
            return Ok(HttpResponse::TooManyRequests().json(json!({
                "error": "Rate limit exceeded. Please try again later."
            })));
        }

        let uri: String = format!("{}{}", data.forward_url, req.uri());

        let client = Client::new();
        let request_builder = client
            .request(req.method().clone(), uri)
            .headers(req.headers().into())
            .body(bytes);

        let response = request_builder.send().await?;

        let mut response_builder = HttpResponse::build(response.status());
        for h in response.headers().iter() {
            response_builder.append_header(h);
        }
        response_builder.append_header(("rate-limiter-status", "ok"));
        let body = response.bytes().await?;
        Ok(response_builder.body(body))
    }
}

#[derive(Debug)]
struct RateLimitError {
    inner: reqwest::Error,
}

impl Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Forwarding error: {}", self.inner)
    }
}

impl From<reqwest::Error> for RateLimitError {
    fn from(value: reqwest::Error) -> Self {
        RateLimitError { inner: value }
    }
}

impl ResponseError for RateLimitError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
