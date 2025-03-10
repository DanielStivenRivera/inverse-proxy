use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{ Client};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::client::HttpConnector;
use tokio::sync::Mutex;

use crate::config::ProxyConfig;
use crate::handler::handle_request;
use crate::metrics::Metrics;
use crate::rate_limiter::RateLimiter;

pub async fn run_server(config: Arc<ProxyConfig>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let rate_limiter = Arc::new(Mutex::new(RateLimiter::new()));
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let metrics = Arc::new(Mutex::new(Metrics::new()));

  // Http/2 client with TLS support
  let mut http = HttpConnector::new();
  http.enforce_http(false);
  let https = HttpsConnector::new_with_connector(http);
  let client = Arc::new(Client::builder().http1_title_case_headers(true).build(https));

  // Make service
  let make_svc = make_service_fn(move |conn: &hyper::server::conn::AddrStream| {
    let client = client.clone();
    let config = config.clone();
    let rate_limiter = rate_limiter.clone();
    let client_ip = conn.remote_addr().ip().to_string();
    let metrics = metrics.clone();

    async move {
      Ok::<_, hyper::Error>(service_fn(move |req| {
        handle_request(req, client.clone(), config.clone(), rate_limiter.clone(), metrics.clone(), client_ip.clone())
      }))
    }
  });

  // Init server
  let server = Server::bind(&addr)
    .http2_only(false)
    .serve(make_svc);

  println!("Proxy corriendo en http://{}", addr);

  if let Err(e) = server.await {
    eprintln!("Error en el servidor: {}", e);
  }
  Ok(())
}