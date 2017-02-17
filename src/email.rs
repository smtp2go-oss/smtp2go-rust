use super::core::*;
use serde_json;

#[derive(Serialize, Debug)]
pub struct Email {
	api_key: String,
	#[serde(rename="sender")]
	from: String,
	to: Vec<String>,
	subject: String,
	text_body: String,
	html_body: String
}

impl super::core::Smtp2goApiRequest for Email {
	fn set_api_key(&mut self, api_key: String){ self.api_key = api_key; }
	fn to_json(&self) -> String {
		match serde_json::to_string(&self){
			Ok(val) => val,
			Err(_) => "".to_string()
		}
	}
}

impl Default for Email {
	fn default() -> Email {
		Email { 
			api_key: String::new(),
			from: String::new(),
			to: Vec::new(),
			subject: String::new(),
			text_body: String::new(),
			html_body: String::new()
		}
	}
}

impl Email {
	
	pub fn new() -> Email {
		Email { ..Default::default() }
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

	pub fn send(&mut self) -> Smtp2goApiResult {

		// warn on missing required parameters
		if self.from.is_empty() { return Err(Smtp2goApiError::MissingRequiredField("From is a required field.".to_string())); }
		if self.to.is_empty() { return Err(Smtp2goApiError::MissingRequiredField("To is a required field and requires at least one email address.".to_string())); }
		if self.subject.is_empty() { return Err(Smtp2goApiError::MissingRequiredField("Subject is a required field.".to_string())); }

		api_request("/email/send", self)
	}
}