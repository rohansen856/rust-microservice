use prometheus::{Encoder, HistogramVec, IntCounterVec, Registry, TextEncoder};
use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web::Data};

use crate::AppState;

pub struct PrometheusMetrics {
    pub registry: Arc<Registry>,
    pub http_requests_total: IntCounterVec,
    pub http_request_duration_seconds: HistogramVec,
}

impl PrometheusMetrics {
    pub fn new() -> Self {
        // Create a Prometheus registry
        let registry = Registry::new_custom(Some("actix_web".to_string()), None).unwrap();

        // Create HTTP request metrics
        let http_requests_total = IntCounterVec::new(
            prometheus::opts!("http_requests_total", "Total number of HTTP requests"),
            &["method", "endpoint"],
        )
        .unwrap();
        let http_request_duration_seconds = HistogramVec::new(
            prometheus::opts!("http_request_duration_seconds", "HTTP request latencies in seconds").into(),
            &["method", "endpoint"],
        )
        .unwrap();

        // Register metrics
        registry.register(Box::new(http_requests_total.clone())).unwrap();
        registry
            .register(Box::new(http_request_duration_seconds.clone()))
            .unwrap();

        Self {
            registry: Arc::new(registry),
            http_requests_total,
            http_request_duration_seconds,
        }
    }

    pub async fn metrics_handler(state: Data<AppState>) -> impl Responder {
        let prometheus = &state.prometheus;
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();
        let metric_families = prometheus.registry.gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(buffer)
    }
}
