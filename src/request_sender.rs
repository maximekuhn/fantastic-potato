use std::net::SocketAddr;

use http::{header, Request, Response};
use thiserror::Error;
use tracing::trace;

use crate::request_parser::RequestBody;

#[derive(Debug)]
pub enum ResponseBody {
    Bytes(Vec<u8>),
}

#[derive(Debug, Error)]
pub enum SendRequestError {
    #[error("Something went wrong with reqwest: '{err}'")]
    ReqwestError {
        #[from]
        err: reqwest::Error,
    },
}

pub async fn send_request(
    req: Request<Option<RequestBody>>,
    backend_addr: SocketAddr,
) -> Result<Response<Option<ResponseBody>>, SendRequestError> {
    // XXX: profile this function, as creating a new reqwest::Client can be expensive
    // Maybe use a clients pool or something similar
    let client = reqwest::Client::new();

    // TODO: handle body
    // FIXME: remove trailing '/' from backend_addr, if any
    let url = format!("http://{}{}", backend_addr, req.uri());
    trace!(%url);
    let req_builder = client
        .request(req.method().clone(), url)
        .headers(req.headers().clone());

    let res = match req.body() {
        Some(body) => req_builder.body(body),
        None => req_builder,
    }
    .send()
    .await?;

    let mut response_builder = Response::builder().status(res.status());

    // add headers
    let header_map = response_builder.headers_mut().unwrap();
    for (header_name, header_value) in res.headers() {
        header_map.insert(header_name, header_value.clone());
    }

    // TODO: body
    let Some(content_length) = header_map.get(header::CONTENT_LENGTH) else {
        let response = response_builder.body(None).unwrap();
        return Ok(response);
    };
    let content_length: usize = content_length.to_str().unwrap().parse().unwrap();
    if content_length == 0 {
        let response = response_builder.body(None).unwrap();
        return Ok(response);
    }

    let body = res.text().await.unwrap();

    let response = response_builder
        .body(Some(ResponseBody::Bytes(body.as_bytes().to_vec())))
        .unwrap();

    Ok(response)
}
