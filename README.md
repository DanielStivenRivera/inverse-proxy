# Reverse Proxy in Rust

This project is an implementation of a reverse proxy in Rust. A reverse proxy is a server that sits between clients and one or more backend servers, forwarding client requests to the appropriate servers and returning the responses to the client.

## Features

- **Load balancing**: Distributes requests among multiple backend servers.
- **Flexible configuration**: Allows configuration of multiple backend servers through a configuration file.
- **High performance**: Leverages Rust's efficiency and security to handle a high volume of requests.
- **Easy to use**: Simple to configure and run.

## Requirements

- Rust 1.56 or higher (for 2021 edition support)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/youruser/reverse-proxy-rust.git
   cd reverse-proxy-rust
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Configuration

The reverse proxy is configured using a `config.yml` file. Here is an example configuration:

```yml
microservices:
  - path_prefix: "/users"
    instances:
      - "http://localhost:3200"
      - "http://localhost:3201"
  - path_prefix: "/orders"
    instances:
      - "http://localhost:3500"
      - "http://localhost:3501"
rate_limit: 100
```

## Contribution

Contributions are welcome! If you would like to contribute to the project, please follow these steps:

1. Fork the repository.

2. Create a branch for your feature:  
   ```bash
   git checkout -b feature/new-feature
   ```

3. Commit your changes:  
   ```bash
   git commit -am "Add new feature"
   ```

4. Push the branch:  
   ```bash
   git push origin feature/new-feature
   ```

5. Open a Pull Request.
