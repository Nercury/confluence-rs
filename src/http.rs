//! HTTP helpers.

use std::io::Read;
use std::result;
use hyper::Client;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
pub use hyper::status::StatusCode;
pub use hyper::Error as HttpError;

header! { (SoapAction, "SOAPAction") => [String] }

/// Simplified HTTP response representation.
#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: String,
}

/// Perform a GET request to specified URL.
pub fn get(url: &str) -> Result<Response> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());

    let mut body = String::new();
    try!(response.read_to_string(&mut body));

    Ok(Response {
        status: response.status,
        body: body,
    })
}

/// Perform a SOAP action to specified URL.
pub fn soap_action(url: &str, action: &str, xml: &str) -> Result<Response> {
    let client = Client::new();
    let mut response = try!(
        client.post(url)
            .header(ContentType(Mime(TopLevel::Text, SubLevel::Xml,
                     vec![(Attr::Charset, Value::Utf8)])))
            .header(SoapAction(action.into()))
            .body(xml)
            .send()
    );

    let mut body = String::new();
    try!(response.read_to_string(&mut body));

    Ok(Response {
        status: response.status,
        body: body,
    })
}

pub type Result<T> = result::Result<T, HttpError>;
