use std::{io::BufRead, str::FromStr};

use http::{HeaderMap, HeaderName, HeaderValue, Method, Request, Uri, Version};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestParseError {
    #[error("Request is not HTTP/1.1")]
    NotHttp1,

    #[error("IO error: '{err}'")]
    IoError {
        #[from]
        err: std::io::Error,
    },

    #[error("No request line found")]
    NoRequestLine,

    #[error("Invalid request line: '{err_msg}'")]
    InvalidRequestLine { err_msg: String },

    #[error("Invalid header(s): '{err_msg}'")]
    InvalidHeaders { err_msg: String },

    #[error("Expected a body, but nothing was found")]
    NoBodyFound,
}

#[derive(Debug)]
pub enum RequestBody {
    Bytes(Vec<u8>),
}

pub fn parse(raw_bytes: &[u8]) -> Result<Request<Option<RequestBody>>, RequestParseError> {
    // https://datatracker.ietf.org/doc/html/rfc2616
    // https://datatracker.ietf.org/doc/html/rfc2616#page-35
    //
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF CRLF
    // message-body

    // check if all lines are valid UTF-8 strings
    let all_lines: Vec<_> = raw_bytes.lines().collect();
    let mut lines = Vec::with_capacity(all_lines.len());
    for line in all_lines {
        lines.push(line?);
    }
    let lines = lines;

    // parse request line
    let request_line = lines.first().ok_or(RequestParseError::NoRequestLine)?;
    let (method, uri, version) =
        parse_request_line(request_line).map_err(|err| RequestParseError::InvalidRequestLine {
            err_msg: err.to_string(),
        })?;
    if version != Version::HTTP_11 {
        return Err(RequestParseError::NotHttp1);
    }

    // parse headers
    let headers_line: Vec<_> = lines
        .iter()
        .skip(1) // skip first line (request line)
        .take_while(|line| !line.is_empty())
        .collect();
    let headers = parse_headers(headers_line).map_err(|err| RequestParseError::InvalidHeaders {
        err_msg: err.to_string(),
    })?;

    // create a request builder with the request line and the headers
    let mut request_builder = Request::get(uri).method(method).version(version);
    let header_map = request_builder.headers_mut().unwrap();
    for (header_name, header_value) in headers {
        header_map.insert(header_name.expect("header name is here"), header_value);
    }

    // Check if there is a body by getting the 'Content-Length' header
    let Some(content_length) = header_map.get("Content-Length") else {
        // no content length => no body
        let request = request_builder.body(None).unwrap();
        return Ok(request);
    };

    let content_length: usize = match content_length.to_str() {
        Ok(content_length_str) => {
            content_length_str
                .parse()
                .map_err(|err| RequestParseError::InvalidHeaders {
                    err_msg: format!("Could not parse Content-Length into a number: '{}", err),
                })?
        }
        Err(_) => {
            return Err(RequestParseError::InvalidHeaders {
                err_msg: "Content-Length is invalid".to_string(),
            })
        }
    };

    if content_length == 0 {
        // no body
        let request = request_builder.body(None).unwrap();
        return Ok(request);
    }

    // content length > 0, we need to check Content-Type to know what is it
    // TODO: check content type
    let Some(body) = lines.last() else {
        return Err(RequestParseError::NoBodyFound);
    };
    let body = RequestBody::Bytes(body.as_bytes().to_vec());
    let request = request_builder.body(Some(body)).unwrap();
    Ok(request)
}

fn parse_request_line(req_line: &str) -> Result<(Method, Uri, Version), String> {
    let fragments: Vec<_> = req_line.split_whitespace().collect();
    if fragments.len() != 3 {
        return Err("expected Method Request-URI HTTP-Version".to_string());
    }

    let method = fragments.first().expect("method is here");
    let request_uri = fragments.get(1).expect("request uri is here");
    let http_version = fragments.last().expect("http version is here");

    let method = Method::from_str(method).map_err(|err| err.to_string())?;
    let request_uri = Uri::from_str(request_uri).map_err(|err| err.to_string())?;

    if *http_version != "HTTP/1.1" {
        return Err("incorrect version, expected HTTP/1.1".to_string());
    }

    Ok((method, request_uri, Version::HTTP_11))
}

fn parse_headers(headers_line: Vec<&String>) -> Result<HeaderMap, String> {
    let mut header_map = HeaderMap::new();
    for header in headers_line {
        let fragments: Vec<_> = header.splitn(2, ':').collect();
        if fragments.len() != 2 {
            return Err("header must be 'HeaderName: HeaderValue'".to_string());
        }

        let name = fragments.first().expect("header name is here");
        let value = fragments.last().expect("header value is here").trim();
        let header_name = HeaderName::from_str(name).map_err(|err| err.to_string())?;
        let header_value = HeaderValue::from_str(value).map_err(|err| err.to_string())?;

        header_map.append(header_name, header_value);
    }

    Ok(header_map)
}
