# MicroGW

MicroGW is a simple API gateway written in Rust, using the Hyper and Reqwest
libraries. Based on [API Gateway](https://github.com/adaoraul/api-gateway) from
[Adão Raul](http://adaoraul.github.io).

This gateway can be used to forward requests to different backend services based
on the request host and path, and it also provides optional authentication.

We plan to make it production ready. Until then, it's meant to be used as a
local environment development API Gateway.

## Features

- Request forwarding based on host and path matching
- Support for regex in host and path matching
- Optional authentication for specific services
- Detailed logging using the tracing crate

## Requirements

- Rust 1.53 or later
- Cargo

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/nauar/microgw.git
cd microgw
```

2. Build and run the project:

```bash
cargo build --release
./target/release/microgw
```

The server will start on `0.0.0.0:8080`.

## Configuration

The configuration for the API gateway is located in the `config.yaml` file.
Here's an example configuration:

```yaml
authorization_api_url: "https://auth.example.com/api/v1/authorization"
services:
  - host: "^localhost"
    path: "^/users"
    target_service: "http://user-service.default.svc.cluster.local"
    target_port: "8080"
    authentication_required: false
  - host: "^localhost"
    path: "^/users"
    target_service: "http://order-service.default.svc.cluster.local"
    target_port: "8080"
    authentication_required: false
```

- `authorization_api_url`: The URL for the authentication API.
- `services`: An array of service configurations, including:
  - `host`: The regex pattern to match for the request host.
  - `path`: The regex pattern to match for the request path.
  - `target_service`: The URL of the backend service to forward requests to.
  - `target_port`: The port of the backend service.
  - `authentication_required` (optional): Whether authentication is required for
  the service. Defaults to `true` if omitted.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file
for details.
