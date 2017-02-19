
use hyper::client::{Client, Response};
use hyper::header::Headers;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde::Serialize;
use serde_json;
use std::env;
use std::io::Read;

/// X-Custom header macros for hyper
header! { (XSmtp2goApi, "X-Smtp2go-Api") => [String] }
header! { (XSmtp2goApiVersion, "X-Smtp2go-Api-Version") => [String] }
header! { (XSmtp2goApiKey, "X-Smtp2go-Api-Key") => [String] }

pub trait Smtp2goApiRequest: Serialize {}

#[derive(Debug, Deserialize)]
pub struct Smtp2goApiResponse {
    request_id: String,
    data: serde_json::Value,
}

#[derive(Debug)]
pub enum Smtp2goApiError {
    MissingAPIKey(String),
    MissingRequiredField(String),
    RequestError(String),
    EndpointError(String),
    InvalidJSON(String),
}

pub type Smtp2goApiResult = Result<Smtp2goApiResponse, Smtp2goApiError>;

impl Smtp2goApiResponse {
    pub fn new<T: Into<String>>(request_id: T, data: serde_json::Value) -> Smtp2goApiResponse {
        Smtp2goApiResponse {
            request_id: request_id.into(),
            data: data,
        }
    }
}

fn parse_response(mut response: Response) -> Smtp2goApiResult {

    // read all the response data into a string
    let mut text: String = String::new();
    response.read_to_string(&mut text).unwrap();

    // deserialize the response text into the response struct
    let response: Smtp2goApiResponse = match serde_json::from_str(&text) {
        Ok(val) => val,
        Err(_) => {
            return Err(Smtp2goApiError::InvalidJSON("Unable to parse response JSON".to_string()))
        }
    };

    Ok(response)
}

pub fn api_request<T: Into<String>, U: Smtp2goApiRequest>(endpoint: T, request: &mut U) -> Smtp2goApiResult {

    // grab the api_root and api_key environment variables
    let api_root: String = match env::var("SMTP2GO_API_ROOT") {
        Ok(api_root) => api_root,
        Err(_) => "https://api.smtp2go.com/v3".to_string(),
    };

    // grab the api_root and api_key environment variables
    let api_key: String = match env::var("SMTP2GO_API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => return Err(Smtp2goApiError::MissingAPIKey("Unable to find an api_key, please set the SMTP2GO_API_KEY environment variable".to_string())),
    };

    // create the header payload to
    let mut headers = Headers::new();
    headers.set(XSmtp2goApi(String::from("smtp2go-rust")).to_owned());
    headers.set(XSmtp2goApiVersion(String::from(::VERSION)).to_owned());
    headers.set(XSmtp2goApiKey(String::from(api_key)).to_owned());

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    // serialise the request into a json string
    let rjson: String =
        match serde_json::to_string(&request) {
            Ok(val) => val,
            Err(_) => return Err(Smtp2goApiError::InvalidJSON("Unable to serialise request into valid JSON".to_string())),
        };

    // create the hyper client
    let url: String = format!("{}/{}", api_root, endpoint.into());
    match client.post(&url)
        .body(&rjson)
        .headers(headers)
        .send() {
	        Ok(res) => parse_response(res),
	        Err(err) => { Err(Smtp2goApiError::RequestError(format!("Somthing went wrong with the request: {}", err))) }
    }
}
