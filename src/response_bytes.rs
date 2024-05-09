use http::Response;

use crate::request_sender::ResponseBody;

pub fn to_bytes(response: Response<Option<ResponseBody>>) -> Vec<u8> {
    // https://datatracker.ietf.org/doc/html/rfc2616
    // https://datatracker.ietf.org/doc/html/rfc2616#page-39
    //
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF CRLF
    // body

    let status_line = format!("HTTP/1.1 {}r\n", response.status(),);

    let headers = response
        .headers()
        .into_iter()
        .map(|(header_name, header_value)| {
            format!("{}:{}", header_name, header_value.to_str().unwrap())
        })
        .collect::<Vec<_>>()
        .join("\r\n");

    let response_content = format!("{}{}\r\n", status_line, headers);
    let Some(body) = response.body() else {
        return response_content.bytes().collect();
    };

    let ResponseBody::Bytes(body_bytes) = body;
    let mut response_bytes: Vec<u8> = response_content.bytes().collect();
    response_bytes.push(b'\r');
    response_bytes.push(b'\n');
    response_bytes.extend(body_bytes);

    response_bytes
}
