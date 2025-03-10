use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Microservice {
  pub path_prefix: String,
  pub instances: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyConfig {
  pub microservices: Vec<Microservice>,
  pub rate_limit: usize,
}