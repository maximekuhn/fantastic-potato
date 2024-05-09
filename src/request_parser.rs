use http::Request;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestParseError {
    #[error("Request is not HTTP/1.1")]
    NotHttp1,
}

#[derive(Debug)]
pub enum RequestBody {}

pub fn parse(_bytes: &[u8]) -> Result<Request<Option<RequestBody>>, RequestParseError> {
    // TODO: parse
    Err(RequestParseError::NotHttp1)
}
