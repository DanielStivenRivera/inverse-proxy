mod config;
mod metrics;
mod rate_limiter;
mod handler;
mod server;
mod sanitizer;

use std::sync::Arc;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use crate::config::ProxyConfig;
use crate::server::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

  let file = File::open("config.yml")?;
  let reader = BufReader::new(file);
  let config: ProxyConfig = serde_yaml::from_reader(reader)?;
  let config = Arc::new(config);
  run_server(config).await
}