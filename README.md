# reflexive

Implements a HTTP echo service using [Actix](https://actix.rs/). Extracts HTTP method, request path, headers, query parameters, and request body and put them all in the response as a JSON.

## Usage
```
USAGE:
    reflexive [OPTIONS]

OPTIONS:
    -h, --help                 Print help information
    -p, --port <PORT>          [default: 8080]
    -V, --version              Print version information
    -w, --workers <WORKERS>    [default: 6]
```

Reflexive uses `8080` as a default port and starts 6 workers. Those can be overridden with `-p` or `--port` option for port, `-w` or `--workers` option for the number of workers to start.

## Enable logging
Reflexive uses [env_logger](https://docs.rs/env_logger/latest/env_logger/) that can be configured through environment variables. Assuming the binary is `reflexive`:

```
$ RUST_LOG=info ./reflexive
[2022-05-16T09:19:02Z INFO  reflexive] Starting server on port 8080
[2022-05-12T19:13:51Z INFO  actix_server::builder] Starting 6 workers
[2022-05-12T19:13:51Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
[2022-05-12T19:14:02Z INFO  reflexive] Request: GET: /echo/hello/world?a=10&b=20&c=31&c=32&c=33
```
