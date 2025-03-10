pub fn sanitize_input(input: &str) -> String {
  let mut sanitized = String::new();
  for c in input.chars() {
    match c {
      '\'' | '"' | ';' | '\\' | '-' | '#' | '<' | '>' | '&' => continue,
      _ => sanitized.push(c)
    }

  }
  sanitized
}