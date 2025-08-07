
use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use prometheus::{Encoder, TextEncoder, Registry, IntCounter, opts};
use once_cell::sync::Lazy;

// Global Prometheus registry and counter
static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);
static REQUEST_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    let counter_opts = opts!("my_requests_total", "Total HTTP requests received");
    let counter = IntCounter::with_opts(counter_opts).unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

async fn serve_metrics(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    REQUEST_COUNTER.inc();

    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let response = Response::builder()
        .header("Content-Type", encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    println!("Serving Prometheus metrics at http://{}/metrics", addr);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
            match req.uri().path() {
                "/metrics" => serve_metrics(req).await,
                _ => Ok(Response::builder()
                    .status(404)
                    .body(Body::from("Not Found"))
                    .unwrap()),
            }
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}

