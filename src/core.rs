use std::env;
use std::io::Read;
use serde_json;
use hyper::client::{Client, Response};
use hyper::net::HttpsConnector;
use hyper::header::Headers;
use hyper_native_tls::NativeTlsClient;

/// X-Custom header macros for hyper
header! { (XSmtp2goApi, "X-Smtp2go-Api") => [String] }
header! { (XSmtp2goApiVersion, "X-Smtp2go-Api-Version") => [String] }

pub trait Smtp2goApiRequest {
	fn set_api_key(&mut self, api_key: String) -> ();
	fn to_json(&self) -> String;
}

#[derive(Debug)]
struct Smtp2goApi {
	api_root: String,
	api_key: String
}

#[derive(Debug, Deserialize)]
pub struct Smtp2goApiResponse {
	request_id: String,
	data: serde_json::Value
}

#[derive(Debug)]
pub enum Smtp2goApiError {
	MissingAPIKey(String),
	MissingRequiredField(String),
	RequestError(String),
	EndpointError(String),
	InvalidJSON(String)
}

pub type Smtp2goApiResult = Result<Smtp2goApiResponse, Smtp2goApiError>;

impl Default for Smtp2goApi {

 	fn default() -> Smtp2goApi {
		Smtp2goApi { 
			api_root: env::var("SMTP2GO_API_ROOT").unwrap_or("https://api.smtp2go.com/v3".to_string()),
			api_key: env::var("SMTP2GO_API_KEY").unwrap_or("".to_string())
		}
	}
}

impl Smtp2goApiResponse {

	pub fn new<T: Into<String>>(request_id: T, data: serde_json::Value) -> Smtp2goApiResponse {
		Smtp2goApiResponse { request_id: request_id.into(), data: data }
	}
}

fn parse_response(mut response: Response) -> Smtp2goApiResult {

	// read all the response data into a string
	let mut text: String = String::new();
	response.read_to_string(&mut text).unwrap();

	// deserialize the response text into the response struct
	let response: Smtp2goApiResponse = match serde_json::from_str(&text) {
		Ok(val) => val,
		Err(_) => return Err(Smtp2goApiError::InvalidJSON("Unable to parse response JSON".to_string()))
	};

	Ok(response)
}	

pub fn api_request<T: Into<String>, U: Smtp2goApiRequest>(endpoint: T, request: &mut U) -> Smtp2goApiResult {

	// grab the api_root and api_key environment variables
	let api_root: String = match env::var("SMTP2GO_API_ROOT") {
		Ok(api_root) => api_root,
		Err(_) => "https://test-api.smtp2go.com/v3".to_string()
	};

	// grab the api_root and api_key environment variables
	let api_key:String = match env::var("SMTP2GO_API_KEY") {
		Ok(api_key) => api_key,
		Err(_) => return Err(Smtp2goApiError::MissingAPIKey("Unable to find an api_key, please set the SMTP2GO_API_KEY environment variable".to_string()))
	};

	// create the header payload to 
	let mut headers = Headers::new();
	headers.set(XSmtp2goApi(String::from("smtp2go.api-rust")).to_owned());
	headers.set(XSmtp2goApiVersion(String::from(::VERSION)).to_owned());

	let ssl = NativeTlsClient::new().unwrap();
	let connector = HttpsConnector::new(ssl);
	let client = Client::with_connector(connector);

	// set the api key onto the api request
	request.set_api_key(api_key);

	// serialise the request into a json string
	let rjson: String = request.to_json();
	if rjson.is_empty(){ 
		return Err(Smtp2goApiError::InvalidJSON("Unable to serialise request into valid JSON".to_string()))
	}

	// create the hyper client
	let url: String = format!("{}/{}", api_root, endpoint.into());
	let response =  client.post(&url)
		.body(&rjson)
		.headers(headers).send();

	match response {
		Ok(res) => parse_response(res),
		Err(_) => Err(Smtp2goApiError::RequestError("Somthing went wrong with the request ??".to_string()))
	}
}