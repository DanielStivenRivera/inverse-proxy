use hyper::{Body, Request, Response, StatusCode, Uri};
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use std::sync::Arc;
use hyper::header::{CONTENT_SECURITY_POLICY, HeaderValue, X_XSS_PROTECTION, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS};
use tokio::sync::Mutex;
use rand::seq::SliceRandom;
use tokio::time::Instant;

use crate::config::ProxyConfig;
use crate::metrics::Metrics;
use crate::rate_limiter::RateLimiter;
use crate::sanitizer::sanitize_input;

pub async fn handle_request(
  req: Request<Body>,
  client: Arc<Client<HttpsConnector<HttpConnector>>>,
  config: Arc<ProxyConfig>,
  rate_limiter: Arc<Mutex<RateLimiter>>,
  metrics: Arc<Mutex<Metrics>>,
  client_ip: String,
) -> Result<Response<Body>, hyper::Error> {

  println!("Solicitud recibida: {:?}", req.uri());
  if req.uri().path() == "/metrics" {
    let metrics = metrics.lock().await;
    return Ok(Response::new(Body::from(metrics.generate_report())));
  }

  //DDos protection
  {
    let mut limiter = rate_limiter.lock().await;
    if !limiter.check_rate(&client_ip, config.rate_limit) {
      return Ok(Response::builder().status(429).body(Body::from("Rate limit exceeded")).unwrap());
    }
  }

  //Select service or microservice per path
  let path = req.uri().path().to_string();
  let target_service = match config.microservices.iter().find(|ms| path.starts_with(&ms.path_prefix)) {
    Some(service) => service,
    None => {
      return Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("No microservice found for this path")).unwrap());
    }
  };

  //load balancer
  let instance = target_service.instances.choose(&mut rand::thread_rng()).unwrap();

  let sanitized_path = sanitize_input(req.uri().path());
  let sanitized_query = req.uri().query()
    .map(|q| sanitize_input(q))
    .unwrap_or_default();

  let new_uri = format!("{}{}?{}", instance, sanitized_path, sanitized_query);

  let new_uri = new_uri.parse::<Uri>().unwrap();
  //construct http connection
  let new_req = Request::builder()
    .method(req.method())
    .uri(new_uri)
    .body(req.into_body()).unwrap();
  {
    let mut metrics = metrics.lock().await;
    metrics.record_request(&*path);
  }

  let start = Instant::now();
  let response = client.request(new_req).await?;
  let duration = start.elapsed();

  {
    let mut metrics = metrics.lock().await;
    metrics.record_response_time(&path, duration);
    let is_ok = response.status() == StatusCode::OK;
    metrics.update_instance_status(&path, is_ok);
  }
  println!("Sanitized request took {:?}", duration);
  let (parts, body) = response.into_parts();
  let status = parts.status;
  Ok(create_secure_response(body, status))

}

pub fn create_secure_response(body: Body, status: StatusCode) -> Response<Body> {
  let mut response = Response::new(body);
  response.headers_mut().insert(
    CONTENT_SECURITY_POLICY,
    HeaderValue::from_static("default-src 'self'; script-src 'self'; style-src 'self'; img_src 'self'; font-src 'self';")
  );
  response.headers_mut().insert(
    X_XSS_PROTECTION,
    HeaderValue::from_static("1; mode=block")
  );
  response.headers_mut().insert(
    X_CONTENT_TYPE_OPTIONS,
    HeaderValue::from("nosniff")
  );
  response.headers_mut().insert(
    X_FRAME_OPTIONS,
    HeaderValue::from_static("DENY")
  );

  *response.status_mut() = status;
  response
}