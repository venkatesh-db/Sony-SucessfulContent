
use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use prometheus::{Encoder, TextEncoder, Registry, IntCounter, IntGauge, Histogram, opts, histogram_opts};
use once_cell::sync::Lazy;
use sysinfo::System;

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

static REQUEST_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    let counter = IntCounter::with_opts(opts!("http_requests_total", "Total HTTP requests")).unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

static ERROR_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    let counter = IntCounter::with_opts(opts!("http_errors_total", "Total HTTP errors")).unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

static REQUEST_LATENCY: Lazy<Histogram> = Lazy::new(|| {
    let histogram = Histogram::with_opts(histogram_opts!(
        "http_request_duration_seconds",
        "Request latency in seconds"
    )).unwrap();
    REGISTRY.register(Box::new(histogram.clone())).unwrap();
    histogram
});

static DB_QUERY_DURATION: Lazy<Histogram> = Lazy::new(|| {
    let histogram = Histogram::with_opts(histogram_opts!(
        "db_query_duration_seconds",
        "Database query duration in seconds"
    )).unwrap();
    REGISTRY.register(Box::new(histogram.clone())).unwrap();
    histogram
});

static CPU_USAGE_GAUGE: Lazy<IntGauge> = Lazy::new(|| {
    let gauge = IntGauge::with_opts(opts!(
        "cpu_usage_percent",
        "CPU usage percentage"
    )).unwrap();
    REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

static MEMORY_USAGE_GAUGE: Lazy<IntGauge> = Lazy::new(|| {
    let gauge = IntGauge::with_opts(opts!(
        "memory_usage_mb",
        "Memory usage in MB"
    )).unwrap();
    REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

async fn serve_metrics(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    REQUEST_COUNTER.inc();
    let timer = REQUEST_LATENCY.start_timer();

    let db_timer = DB_QUERY_DURATION.start_timer();
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    db_timer.observe_duration();

    let mut sys = System::new_all();
    sys.refresh_cpu();
    sys.refresh_memory();
    let cpu_usage = sys.global_cpu_info().cpu_usage() as i64;
    let mem_usage_mb = (sys.used_memory() / 1024) as i64;

    CPU_USAGE_GAUGE.set(cpu_usage);
    MEMORY_USAGE_GAUGE.set(mem_usage_mb);

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
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    println!("Serving Prometheus metrics on http://{}/metrics", addr);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
            match req.uri().path() {
                "/metrics" => serve_metrics(req).await,
                _ => {
                    ERROR_COUNTER.inc();
                    Ok(Response::builder()
                        .status(404)
                        .body(Body::from("Not Found"))
                        .unwrap())
                }
            }
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
