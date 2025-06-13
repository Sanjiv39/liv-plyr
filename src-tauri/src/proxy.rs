use crate::utils::{self, headers_to_json, warp_response_builder};
use futures_util::{StreamExt, TryStreamExt};
use http::HeaderMap;
use regex::Regex;
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::vec;
use std::{net::SocketAddr, sync::Arc};
use tauri::ipc::IpcResponse;
use tauri::{AppHandle, Manager, Runtime, State};
use tokio::sync::Mutex;
use urlencoding;
use warp::filters::path::param;
use warp::{body::bytes, http::HeaderValue, http::Response, Filter};

#[derive(Deserialize)]
pub struct ProxyRequest {
    url: String,
    method: String,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
    stream: Option<bool>,
}

#[derive(Serialize)]
pub struct AxiosLikeResponse {
    data: String,
    status: u16,
    statusText: String,
    headers: HashMap<String, String>,
}

#[tauri::command]
pub async fn proxy_request(req: ProxyRequest) -> Result<AxiosLikeResponse, String> {
    let client = Client::new();

    let method = req
        .method
        .to_uppercase()
        .parse::<Method>()
        .map_err(|e| e.to_string())?;

    let mut builder = client.request(method, &req.url);

    if let Some(hdrs) = &req.headers {
        for (key, value) in hdrs {
            builder = builder.header(key, value);
        }
    }

    if let Some(body) = &req.body {
        builder = builder.body(body.clone());
    }

    let res = builder.send().await.map_err(|e| e.to_string())?;

    let status = res.status();
    let status_text = status.canonical_reason().unwrap_or("Unknown").to_string();

    let mut headers_map = HashMap::new();
    for (key, value) in res.headers().iter() {
        headers_map.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
    }

    // let mut requester = HashMap::new();
    // requester.insert("headers", );

    let data = if req.stream.unwrap_or(false) {
        let bytes = res.bytes().await.map_err(|e| e.to_string())?;
        String::from_utf8_lossy(&bytes).to_string()
    } else {
        res.text().await.map_err(|e| e.to_string())?
    };

    Ok(AxiosLikeResponse {
        data,
        status: status.as_u16(),
        statusText: status_text,
        headers: headers_map,
    })
}

async fn handle_proxy(
    param: String,
    tail: warp::path::Tail,
    method: Method,
    params: std::collections::HashMap<String, String>,
    headers: warp::http::HeaderMap,
    body: warp::hyper::body::Bytes,
) -> Result<impl warp::Reply, warp::Rejection> {
    let skip_patterns = vec![Regex::new(r"(host|origin|referer|sec[-])").unwrap()];
    print!(
        "Method : {}\nParams : {:?}\nHeaders : {:?}\nBody : {:?} \n",
        method, params, headers, body
    );

    let decoded_params = urlencoding::decode(&param).map_err(|_| warp::reject())?;
    let params_parsed: serde_json::Value =
        serde_json::from_str(&decoded_params).map_err(|_| warp::reject())?;

    let url = params_parsed.get("url").unwrap().as_str().unwrap().trim();
    if url.len() == 0
        || Regex::new(r"[\n ]+").unwrap().is_match(url)
        || Regex::new(r"^http(s|)://[^. ]+[.][^. ]+")
            .unwrap()
            .is_match(url)
            == false
    {
        return warp_response_builder(
            400,
            utils::ProxyBody::Json(serde_json::json!(
                {"success": false, "error": "Invalid param [url]. Must be a valid http url.", "type": "invalid-url"
            })),
            serde_json::json!({"X-Err-Type": "invalid-url"}),
        );
    }

    // let url = match params.get("url") {
    //     Some(u) => u.clone(),
    //     None => {
    //         return Ok(Response::builder()
    //             .status(400)
    //             .body("Missing 'url' param".into())
    //             .unwrap())
    //     }
    // };
    let is_stream = params.get("stream").map(|v| v == "true").unwrap_or(false);

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .unwrap();
    let mut req = client.get(url);

    let mut all_headers: HashMap<String, String> = HashMap::new();
    let skip_headers = ["host", "content-length", "origin", "referer"];
    // Use pre existing headers
    // for (key, val) in headers.iter() {
    //     if let Ok(mut val_str) = val.to_str() {
    //         val_str = val_str.trim();
    //         match skip_patterns
    //             .iter()
    //             .find(|pattern| pattern.is_match(val_str.trim()))
    //         {
    //             Some(_) => continue,
    //             None => {
    //                 all_headers.insert(key.to_string().trim().to_lowercase(), val_str.to_string());
    //             } // req = req.header(key, val_str);
    //         }
    //     }
    // }
    if let Some(value) = headers.get("cookie") {
        if let Ok(mut value_str) = value.to_str() {
            value_str = value_str.trim();
            if value_str.len() != 0 {
                print!("\nCookie found : {}\n", value_str);
                all_headers.insert("cookie".trim().to_string(), value_str.to_string());
            }
        }
    };

    // Append custom headers
    let headers_json = params.get("headers");
    if let Some(headers_str) = headers_json {
        if let Ok(json_val) = serde_json::from_str::<Value>(headers_str) {
            if let Some(obj) = json_val.as_object() {
                for (key, val) in obj.iter() {
                    if let Some(val_str) = val.as_str() {
                        all_headers
                            .insert(key.trim().to_string().to_lowercase(), val_str.to_string());
                        // req = req.header(key.to_lowercase(), val_str);
                    }
                }
            }
        }
    }

    for (key, val) in all_headers.iter() {
        req = req.header(key, val);
    }

    print!(
        "\nRequest Config----------\n\n Query : {:?} \nHeaders : {}",
        params,
        serde_json::to_string_pretty(&all_headers).unwrap(),
    );

    let res = req.send().await.map_err(|_| warp::reject())?;
    let status = res.status();
    let res_headers = res.headers().clone();

    let mut builder = warp::http::Response::builder().status(status);
    for (k, v) in res_headers.clone() {
        if let (Some(k), Ok(v)) = (k, v.to_str()) {
            builder = builder.header(k, v);
        }
    }
    let res_headers_json = serde_json::json!(headers_to_json(&res_headers.clone()));

    if is_stream {
        let stream = res
            .bytes_stream()
            .map(|item| item.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));
        // let body_stream = warp::hyper::Body::wrap_stream(stream);

        // let response = builder.body(body_stream).unwrap();
        let response = warp_response_builder(
            status.as_u16(),
            utils::ProxyBody::Stream(Box::pin(stream)),
            res_headers_json.clone(),
        )
        .unwrap();
        return Ok::<_, warp::Rejection>(response);
    } else {
        let body = res.bytes().await.map_err(|_| warp::reject())?;
        // let body_data = warp::hyper::Body::from(body);
        let response = warp_response_builder(
            status.as_u16(),
            utils::ProxyBody::Bytes(body),
            res_headers_json.clone(),
        )
        .unwrap();
        Ok::<_, warp::Rejection>(response)
    }
}

pub async fn start_media_proxy() {
    let route = warp::path::param()
        .and(warp::path::tail())
        .and(warp::method())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then(handle_proxy);

    let addr: SocketAddr = "127.0.0.1:5009".parse().unwrap();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(["GET", "POST", "PUT", "DELETE", "HEAD", "PATCH"])
        .allow_headers(vec!["*"])
        .allow_credentials(true);
    warp::serve(route.with(cors))
        .run(([127, 0, 0, 1], 5009))
        .await;
}
