use std::collections::HashMap;
use tokio::time::Instant;

pub struct RateLimiter {
  request: HashMap<String, (usize, Instant)>,
}

impl RateLimiter {
  pub fn new() -> Self {
    RateLimiter {
      request: HashMap::new(),
    }
  }

  pub fn check_rate(&mut self, ip: &str, limit: usize) -> bool {
    let now = Instant::now();
    let entry = self.request.entry(ip.to_string()).or_insert((0, now));

    if now.duration_since(entry.1).as_secs() >= 1 {
      *entry = (1, now);
      true
    } else if entry.0 <limit {
      entry.0 += 1;
      true
    } else {
      false
    }
  }
}