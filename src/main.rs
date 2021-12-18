use std::collections::BTreeMap;

use actix_web::http::HeaderMap;
use actix_web::*;
use serde::Serialize;
use serde::ser::{SerializeStruct, SerializeSeq};
use serde_urlencoded::from_str;

enum SingleOrMulti<'a> {
    Single(&'a str),
    Multi(Vec<&'a str>),
}

impl <'a> Serialize for SingleOrMulti<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SingleOrMulti::Single(v) => {
                serializer.serialize_str(v)
            },
            SingleOrMulti::Multi(vs) => {
                let mut seq = serializer.serialize_seq(Some(vs.len()))?;
                for e in vs {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
    }
}

fn headers_as_map(headers: &HeaderMap) -> BTreeMap<&str, SingleOrMulti> {
    let mut ret = BTreeMap::new();
    for key in headers.keys() {
        let vs: Vec<&str> = headers.get_all(key).into_iter()
            .map(|v| v.to_str().unwrap_or("<<Error: Contains Non-visible ASCII characters>>"))
            .collect();

        let k = key.as_str();
        let v = if vs.len() > 1 {
            SingleOrMulti::Multi(vs)
        } else {
            SingleOrMulti::Single(vs[0])
        };
        ret.insert(k, v);
    }
    ret
}

fn queries_as_map(query_string: &str) -> BTreeMap<&str, SingleOrMulti> {
    let mut ret = BTreeMap::new();
    let queries = from_str::<Vec<(&str, &str)>>(query_string).unwrap();

    for (k, v) in queries {
        match ret.get_mut(k) {
            None => {
                ret.insert(k, SingleOrMulti::Single(v));
            }
            Some(SingleOrMulti::Single(ov)) => {
                let vs = vec![*ov, v];
                ret.insert(k, SingleOrMulti::Multi(vs));
            }
            Some(SingleOrMulti::Multi(vs)) => {
                vs.push(v);
            }
        }
    }

    ret
}

struct EchoResponse<'a> {
    method: &'a str,
    path: &'a str,
    queries: BTreeMap<&'a str, SingleOrMulti<'a>>,
    headers: BTreeMap<&'a str, SingleOrMulti<'a>>,
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
