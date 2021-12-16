use std::collections::HashMap;

use actix_web::*;
use actix_web::http::HeaderMap;
use serde_json::json;

fn body_as_str<'a>(body: &'a web::Bytes) -> &'a str {
    std::str::from_utf8(&body[..]).unwrap()
}

fn headers_as_map(headers: &HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (header, value) in headers.iter() {
        let k = header.as_str();
        let v = value
            .to_str()
            .unwrap_or("Non-ASCII header value")
            .into();
        match map.get_mut(k) {
            None => {
                map.insert(k.into(), v);
            }
            Some(old_val) => {
                *old_val = format!("{}, {}", old_val, v);
            }
        };
    }
    map
}

#[route(
    "/*",
    method = "GET",
    method = "POST",
    method = "PUT",
    method = "DELETE",
    method = "HEAD",
    method = "CONNECT",
    method = "OPTIONS",
    method = "TRACE",
    method = "PATCH"
)]
async fn echo(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let response = json!({
        "method": req.method().as_str(),
        "path": req.path(),
        "query": req.query_string(),
        "headers": headers_as_map(req.headers()),
        "body": body_as_str(&body),
    });

    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .body(response.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
