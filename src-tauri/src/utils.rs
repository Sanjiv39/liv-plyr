use futures_util::Stream;
use http::HeaderMap;
use reqwest::StatusCode;
use serde_json::Value;
use std::pin::Pin;
use warp::http::Response;
use warp::hyper::body::Bytes;

pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;

pub enum ProxyBody {
    Json(serde_json::Value),
    Text(String),
    Bytes(warp::hyper::body::Bytes),
    Stream(ByteStream),
}

pub fn handle_body(body: ProxyBody) -> warp::hyper::Body {
    match body {
        ProxyBody::Json(v) => warp::hyper::Body::from(serde_json::to_string(&v).unwrap()),
        ProxyBody::Text(s) => warp::hyper::Body::from(s),
        ProxyBody::Bytes(b) => warp::hyper::Body::from(b),
        ProxyBody::Stream(s) => warp::hyper::Body::wrap_stream(s),
    }
}

pub fn headers_to_json(headers: &HeaderMap) -> Value {
    let mut json_map = serde_json::Map::new();

    for (key, value) in headers.iter() {
        if let Ok(val_str) = value.to_str() {
            json_map.insert(key.to_string(), Value::String(val_str.to_string()));
        }
    }

    Value::Object(json_map)
}

pub fn warp_response_builder(
    status: u16,
    body: ProxyBody,
    headers: serde_json::Value,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut builder = Response::builder().status(StatusCode::from_u16(status).unwrap());

    println!(
        "Builder Param [headers] : {:?}",
        headers.as_object().unwrap()
    );

    for (key, val) in headers.as_object().unwrap() {
        if val.is_string() {
            builder = builder.header(key, val.as_str().unwrap());
        } else {
            if val.to_string().trim().len() != 0 {
                builder = builder.header(key, val.to_string());
            }
        }
    }

    let build_body = handle_body(body);

    let response = builder.body(build_body).unwrap();

    return Ok(response);
}
