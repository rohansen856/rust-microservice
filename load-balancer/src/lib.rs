use std::{fmt::Display, sync::atomic::{AtomicUsize, Ordering}};
use actix_web::{
    http::header::ContentType,
    web::{self},
    App, HttpRequest, HttpResponse, HttpServer, ResponseError,
};
use reqwest::Client;

pub struct LoadBalancer {
    port: u16,
    servers: Vec<String>,
}

struct AppState {
    servers: Vec<String>,
    current_index: AtomicUsize,
}

impl LoadBalancer {
    pub fn new(port: u16, servers: Vec<String>) -> Self {
        LoadBalancer { port, servers }
    }

    pub fn uri(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    pub async fn run(&self) {
        let data = web::Data::new(AppState {
            servers: self.servers.clone(),
            current_index: AtomicUsize::new(0),
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
        .unwrap();
    }

    async fn handler(
        req: HttpRequest,
        data: web::Data<AppState>,
        bytes: web::Bytes,
    ) -> Result<HttpResponse, LoadBalanceError> {
        // Get the next server in round-robin order
        let current_index = data.current_index.fetch_add(1, Ordering::SeqCst) % data.servers.len();
        let server = &data.servers[current_index];
        let uri = format!("{}{}", server, req.uri());

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
        response_builder.append_header(("load-balancer-status", "ok"));
        let body = response.bytes().await?;
        Ok(response_builder.body(body))
    }
}

#[derive(Debug)]
struct LoadBalanceError {
    inner: reqwest::Error,
}

impl Display for LoadBalanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Forwarding error: {}", self.inner)
    }
}

impl From<reqwest::Error> for LoadBalanceError {
    fn from(value: reqwest::Error) -> Self {
        LoadBalanceError { inner: value }
    }
}

impl ResponseError for LoadBalanceError {
    fn status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
