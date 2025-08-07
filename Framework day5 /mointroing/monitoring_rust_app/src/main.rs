
use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};  // ✅ Single import
use hyper::service::{make_service_fn, service_fn};
use prometheus::{Encoder, TextEncoder, Registry, IntCounter, Histogram, opts, histogram_opts};
use once_cell::sync::Lazy;
use sysinfo::{System, RefreshKind};
use tracing::{info, error};
use tracing_subscriber;

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

static REQUEST_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    let counter = IntCounter::with_opts(opts!("http_requests_total", "Total HTTP requests")).unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

static REQUEST_LATENCY: Lazy<Histogram> = Lazy::new(|| {
    let histogram = Histogram::with_opts(histogram_opts!("http_request_duration_seconds", "Request latency in seconds")).unwrap();
    REGISTRY.register(Box::new(histogram.clone())).unwrap();
    histogram
});

async fn serve_metrics(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    REQUEST_COUNTER.inc();
    let timer = REQUEST_LATENCY.start_timer();

    let mut sys = System::new_with_specifics(RefreshKind::everything());
    sys.refresh_cpu();
    sys.refresh_memory();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_usage = sys.used_memory() / 1024 / 1024;

    info!("CPU Usage: {:.2}%", cpu_usage);
    info!("Memory Usage: {} MB", memory_usage);

    timer.observe_duration();

    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(Response::builder()
        .header("Content-Type", encoder.format_type())
        .body(Body::from(buffer))
        .unwrap())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    info!("Serving Prometheus metrics on http://{}/metrics", addr);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req| async move {
            match req.uri().path() {
                "/metrics" => serve_metrics(req).await,
                _ => Ok(Response::builder()
                    .status(404)
                    .body(Body::from("Not Found"))
                    .unwrap()),
            }
        }))
    });

    if let Err(e) = Server::bind(&addr).serve(make_svc).await {
        error!("Server error: {}", e);
    }
}
