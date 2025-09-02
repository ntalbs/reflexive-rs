# reflexive-rs

Implements a HTTP echo service using [Actix Web](https://actix.rs/). Extracts HTTP method, request path, headers, query parameters, and request body and put them all in a JSON response.

## Usage
```
Usage: reflexive [OPTIONS]

Options:
  -p, --port <PORT>        [default: 3000]
  -w, --workers <WORKERS>  [default: 6]
  -h, --help               Print help
  -V, --version            Print version
```

Reflexive uses `3000` as a default port and starts 6 workers. Those can be overridden with `-p` or `--port` option for port, `-w` or `--workers` option for the number of workers to start.

Once `reflexive-rs` is up, it produces echo responses like the following:

```
$ curl -X POST "http://localhost:3000/path/to/req?q1=hello&q2=world" \
  -H 'h1:a' \
  -H 'h2:b' \
  -H "Content-Type: application/x-www-form-urlencoded"  \
  -d 'name=ntalbs&email=ntalbsen@gmail.com' | jq
...
{
  "method": "POST",
  "path": "/path/to/req",
  "queries": {
    "q1": "hello",
    "q2": "world"
  },
  "headers": {
    "accept": "*/*",
    "content-length": "36",
    "content-type": "application/x-www-form-urlencoded",
    "h1": "a",
    "h2": "b",
    "host": "localhost:3000",
    "user-agent": "curl/8.7.1"
  },
  "body": "name=ntalbs&email=ntalbsen@gmail.com"
}
```

## Override Logging level
Reflexive uses [env_logger](https://docs.rs/env_logger/latest/env_logger/) that can be configured through environment variables. The default logging level is `info`. Assuming the binary is `reflexive`, you can override the default logging level like the following:

```
$ RUST_LOG=trace ./reflexive
[2022-05-24T08:47:41Z INFO  reflexive] Starting server on port 3000
[2022-05-24T08:47:41Z INFO  actix_server::builder] Starting 6 workers
[2022-05-24T08:47:41Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
[2022-05-24T08:47:41Z TRACE actix_server::worker] starting server worker 0
[2022-05-24T08:47:41Z TRACE mio::poll] registering event source with poller: token=Token(0), interests=READABLE | WRITABLE
[2022-05-24T08:47:41Z TRACE actix_server::worker] starting server worker 1
[2022-05-24T08:47:41Z TRACE actix_server::worker] Service "actix-web-service-0.0.0.0:3000" is available
[2022-05-24T08:47:41Z TRACE mio::poll] registering event source with poller: token=Token(0), interests=READABLE | WRITABLE
[2022-05-24T08:47:41Z TRACE actix_server::worker] Service "actix-web-service-0.0.0.0:3000" is available
[2022-05-24T08:47:41Z TRACE actix_server::worker] starting server worker 2
...
```
