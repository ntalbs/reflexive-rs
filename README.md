# reflexive

Implements a HTTP echo service using [Actix](https://actix.rs/). Extracts HTTP method, request path, headers, query parameters, and request body and put them all in the response as a JSON.

## Enable logging
Reflexive uses [env_logger](https://docs.rs/env_logger/latest/env_logger/) that can be configured through environment variables. Assuming the binary is `reflexive`:

```
$ RUST_LOG=info ./reflexive
[2022-05-12T19:13:51Z INFO  actix_server::builder] Starting 6 workers
[2022-05-12T19:13:51Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
[2022-05-12T19:14:02Z INFO  reflexive] Request: GET: /echo/hello/world?a=10&b=20&c=31&c=32&c=33
```
