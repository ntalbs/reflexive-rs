use std::collections::HashMap;

use actix_web::*;
use actix_web::http::HeaderMap;
use serde_json::json;
use serde_urlencoded::from_str;

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

fn queries_as_map(query_string: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let queries = from_str::<Vec<(String, String)>>(query_string).unwrap();

    for (k, v) in queries.iter() {
        match map.get_mut(k) {
            None => {
                map.insert(k.into(), v.into());
            },
            Some(old_val) => {
                *old_val = format!("{}, {}", old_val, v);
            }
        }
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
async fn echo(req: HttpRequest, body: String) -> impl Responder {
    let response = json!({
        "method": req.method().as_str(),
        "path": req.path(),
        "query": queries_as_map(req.query_string()),
        "headers": headers_as_map(req.headers()),
        "body": body,
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
