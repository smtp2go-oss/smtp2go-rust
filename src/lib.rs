use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub trait Smtp2goApiRequest: Serialize {}

#[derive(Debug, Deserialize)]
pub struct Smtp2goApiResponse {
    request_id: String,
    data: serde_json::Value,
}

#[derive(Debug)]
pub enum Smtp2goApiError {
    MissingAPIKey(String),
    IncorrectAPIKeyFormat(String),
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

pub async fn api_request<T: Into<String>, U: Smtp2goApiRequest>(
    endpoint: T,
    request: &mut U,
) -> Smtp2goApiResult {
    // grab the api_root and api_key environment variables
    let api_root: String = match env::var("SMTP2GO_API_ROOT") {
        Ok(api_root) => api_root,
        Err(_) => "https://api.smtp2go.com/v3".to_string(),
    };

    // grab the api_root and api_key environment variables
    let api_key: String =
        match env::var("SMTP2GO_API_KEY") {
            Ok(api_key) => api_key,
            Err(_) => return Err(Smtp2goApiError::MissingAPIKey(
                "Unable to find an api_key, please set the SMTP2GO_API_KEY environment variable"
                    .to_string(),
            )),
        };

    // check if the key is correctly formatted prior to the http request
    let re = Regex::new(r"^api-[a-zA-Z0-9]{32}$").unwrap();
    if !re.is_match(&api_key) {
        return Err(Smtp2goApiError::IncorrectAPIKeyFormat(format!("The value of SMTP2GO_API_KEY '{}' does not match the api key format of ^api-[a-zA-Z0-9]{{32}}$, please correct it", api_key)));
    }

    // serialise the request into a json string
    let rjson: String = match serde_json::to_string(&request) {
        Ok(val) => val,
        Err(_) => {
            return Err(Smtp2goApiError::InvalidJSON(
                "Unable to serialise request into valid JSON".to_string(),
            ))
        }
    };

    let client = reqwest::Client::new();
    match client
        .post(format!("{}/{}", api_root, endpoint.into()))
        .body(rjson)
        .header("Content-Type", "application/json")
        .header("X-Smtp2go-Api", "smtp2go-rust")
        .header("X-Smtp2go-Api-Version", VERSION)
        .header("X-Smtp2go-Api-Key", api_key)
        .send()
        .await
    {
        Ok(res) => match res.json::<Smtp2goApiResponse>().await {
            Ok(r) => Ok(r),
            Err(err) => Err(Smtp2goApiError::RequestError(format!(
                "Unable to deserialize API response: {}",
                err
            ))),
        },
        Err(err) => Err(Smtp2goApiError::RequestError(format!(
            "Somthing went wrong with the request: {}",
            err
        ))),
    }
}

#[derive(Serialize, Debug)]
pub struct Email {
    #[serde(rename = "sender")]
    from: String,
    to: Vec<String>,
    subject: String,
    text_body: String,
    html_body: String,
}

impl Smtp2goApiRequest for Email {}

impl Default for Email {
    fn default() -> Email {
        Email {
            from: String::new(),
            to: Vec::new(),
            subject: String::new(),
            text_body: String::new(),
            html_body: String::new(),
        }
    }
}

impl Email {
    pub fn new() -> Email {
        Email {
            ..Default::default()
        }
    }

    pub fn from<'a, T: Into<String>>(&'a mut self, from: T) -> &'a mut Email {
        self.from = from.into();
        self
    }

    pub fn to<'a>(&'a mut self, recipients: &[String]) -> &'a mut Email {
        self.to.extend(recipients.iter().cloned());
        self
    }

    pub fn subject<'a, T: Into<String>>(&'a mut self, subject: T) -> &'a mut Email {
        self.subject = subject.into();
        self
    }

    pub fn text_body<'a, T: Into<String>>(&'a mut self, body: T) -> &'a mut Email {
        self.text_body = body.into();
        self
    }

    pub fn html_body<'a, T: Into<String>>(&'a mut self, body: T) -> &'a mut Email {
        self.html_body = body.into();
        self
    }

    pub async fn send(&mut self) -> Smtp2goApiResult {
        // from email address is mandatory
        if self.from.is_empty() {
            return Err(Smtp2goApiError::MissingRequiredField(
                "From is a required field.".to_string(),
            ));
        }

        // need at least 1 to address - check it
        if self.to.is_empty() {
            return Err(Smtp2goApiError::MissingRequiredField(
                "To is a required field and requires at least one email address.".to_string(),
            ));
        }

        // subject is a required field - check it
        if self.subject.is_empty() {
            return Err(Smtp2goApiError::MissingRequiredField(
                "Subject is a required field.".to_string(),
            ));
        }

        // text_body is a required field - check it
        if self.text_body.is_empty() {
            return Err(Smtp2goApiError::MissingRequiredField(
                "Text Body is a required field.".to_string(),
            ));
        }

        // finally fire off the api request
        api_request("/email/send", self).await
    }
}
