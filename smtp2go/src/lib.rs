extern crate regex;

#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate serde_json;

use std::env;
use regex::Regex;
use hyper::client::{Client, Response};
use hyper::net::HttpsConnector;
use hyper::header::Headers;
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

pub const VERSION: &'static str = "0.0.1-alpha";

header! { (XSmtp2goApi, "X-Smtp2go-Api") => [String] }
header! { (XSmtp2goApiVersion, "X-Smtp2go-Api-Version") => [String] }

pub struct Smtp2goApi { 
	api_root: String,
	api_key: String 
}

pub struct Smtp2goApiResponse {
	response: hyper::client::Response,
    json: serde_json::Value,
}

impl Smtp2goApi {
	
	pub fn new<T: Into<Option<String>>>(api_key: T) -> Smtp2goApi {

		// either passed api key or ""
		let mut api_key = api_key.into().unwrap_or(String::from(""));

		// if the key is empty, attempt to grab it from the environment
		if api_key.is_empty() {

			api_key = match env::var("SMTP2GO_API_KEY") {
				Ok(val) => val,
				Err(_) => panic!("SMTP2GO requires SMTP2GO_API_KEY Environment Variable to be set or api_key passed to the SMTP2Go constructor")
			};
		}

		// check it for validity
		let check = r"^api-[0-9a-zA-Z]{32}$";
		let re = Regex::new(check).unwrap();
		
		if !re.is_match(&api_key) {
			panic!("SMTP2GO found a malformed api_key, should be in format '{}'", check);
		}

		// all good to return
		Smtp2goApi { 
			api_root: match env::var("SMTP2GO_API_ROOT") {
				Ok(val) => val,
				Err(_) => String::from("https://test-api.smtp2go.com/v3")
			}, 
			api_key: api_key 
		}
	}

	#[allow(dead_code)]
	pub fn send(&self, sender: &String, recipients: &Vec<String>, subject: &String, message: &String) -> Result<Smtp2goApiResponse, ()> {

		// create the header payload to 
		let mut headers = Headers::new();
		headers.set(XSmtp2goApi(String::from("smtp2go-rust")).to_owned());
		headers.set(XSmtp2goApiVersion(String::from(::VERSION)).to_owned());

		let body = json!({
			"api_key": self.api_key,
			"sender": sender,
			"to": recipients,
			"subject": subject,
			"text_body": message			
		});

		let body2 = body.to_string();

		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		let client = Client::with_connector(connector);

		// create the hyper client
		let endpoint: String = [&self.api_root, "/email/send"].join("/");
		let response =  client.post(&endpoint)
			.body(&body2)
			.headers(headers).send();
		match response {
			Ok(res) => Ok(Smtp2goApiResponse::new(res)),
			Err(_) => Err(())
		}
	}
}

impl Smtp2goApiResponse {

	pub fn new(mut response: hyper::client::Response) -> Smtp2goApiResponse {

		// read all the response data into a string
		let mut text: String = String::new();
		response.read_to_string(&mut text).unwrap();

		// process the response data into a serde_json::Value
		let json = json!(text);

		Smtp2goApiResponse {
			json: json,
			response: response
		}
	}

	pub fn json(&self) -> &serde_json::Value {
		return &self.json;
	}
}