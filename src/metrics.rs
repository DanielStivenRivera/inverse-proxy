use std::collections::HashMap;
use std::time::Duration;

pub struct Metrics {
  requests: HashMap<String, usize>,
  response_times: HashMap<String, Vec<Duration>>,
  instance_status: HashMap<String, bool>,
}

impl Metrics {
  pub fn new() -> Self {
    Metrics {
      requests: HashMap::new(),
      response_times: HashMap::new(),
      instance_status: HashMap::new(),
    }
  }

  pub fn record_request(&mut self, path: &str) {
    *self.requests.entry(path.to_string()).or_insert(0) += 1;
  }

  pub fn record_response_time(&mut self, path: &str, duration: Duration) {
    self.response_times
      .entry(path.to_string())
      .or_insert_with(Vec::new)
      .push(duration);
  }

  pub fn generate_report(&self) -> String {
    let mut report = String::new();
    report.push_str("Microservices Metrics:\n");

    for (path, count) in &self.requests {

      let empty_vec = Vec::new();
      let times = self.response_times.get(path).unwrap_or(&empty_vec);
      let avg_time = if !times.is_empty() {
        times.iter().sum::<Duration>().as_millis() / times.len() as u128
      } else {
        0
      };
      report.push_str(&format!("- {}: {} requests, avg response time: {}ms\n", path, count, avg_time));
    }

    report.push_str("\nInstance status:\n");
    for (instance, status) in &self.instance_status {
      report.push_str(&format!(
        "- {}: {}\n",
        instance, if *status {"UP"} else {"DOWN"},
      ));
    }

    report
  }

  pub fn update_instance_status(&mut self, instance: &str, status: bool) {
    self.instance_status.insert(instance.to_string(), status);
  }

}