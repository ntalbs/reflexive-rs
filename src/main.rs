use std::collections::HashMap;

use actix_web::http::HeaderMap;
use actix_web::*;
use serde::Serialize;
use serde::ser::SerializeStruct;
use serde_urlencoded::from_str;

fn headers_as_map(headers: &HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (header, value) in headers.iter() {
        let k = header.as_str();
        let v = value.to_str().unwrap_or("Non-ASCII header value").into();
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
            }
            Some(old_val) => {
                *old_val = format!("{}, {}", old_val, v);
            }
        }
    }
    map
}

struct EchoResponse<'a> {
    method: &'a str,
    path: &'a str,
    queries: HashMap<String, String>,
    headers: HashMap<String, String>,
    body: String,
}

impl<'a> Serialize for EchoResponse<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Echo", 5)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("queries", &self.queries)?;
        state.serialize_field("headers", &self.headers)?;
        state.serialize_field("body", &self.body)?;
        state.end()
    }
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
    let response = EchoResponse {
        method: req.method().as_str(),
        path: req.path(),
        queries: queries_as_map(req.query_string()),
        headers: headers_as_map(req.headers()),
        body,
    };

    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&response).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
