use super::core::*;

#[derive(Serialize, Debug)]
/// The 'Email' type. See [the module level documentation](index.html) for more info.
pub struct Email {

    #[serde(rename="sender")]
    /// The senders email address
    from: String,
    to: Vec<String>,
    subject: String,
    text_body: String,
    html_body: String,
}

impl super::core::Smtp2goApiRequest for Email {}

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

    ///
    /// Perform an api request given the information on self, will
    /// warn on missing required fields prior to sending
    /// @return Smtp2goApiResult
    ///
    pub fn send(&mut self) -> Smtp2goApiResult {

    	// from email address is mandatory
        if self.from.is_empty() { 
        	return Err(Smtp2goApiError::MissingRequiredField("From is a required field.".to_string())); 
        }

        // need at least 1 to address - check it
        if self.to.is_empty() { 
        	return Err(Smtp2goApiError::MissingRequiredField("To is a required field and requires at least one email address.".to_string())); 
       	}

       	// subject is a required field - check it
        if self.subject.is_empty() {
            return Err(Smtp2goApiError::MissingRequiredField("Subject is a required field.".to_string()));
        }

        // text_body is a required field - check it
        if self.text_body.is_empty() {
            return Err(Smtp2goApiError::MissingRequiredField("Text Body is a required field.".to_string()));
        }

        // finally fire off the api request
        api_request("/email/send", self)
    }
}
