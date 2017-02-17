extern crate smtp2go;

use std::env;

#[test]
fn test_send() {
	
	env::set_var("SMTP2GO_API_KEY", "api-06AA9CCC552B11E68E9F90B11C30B754");

	match smtp2go::Email::new()
		.from("Goofy <goofy@clubhouse.com>")
		.to(&[
			"Mickey <mickey@clubhouse.com>".to_string()
		])
		.subject("Trying out SMTP2Go")
		.text_body("Test message")
		.send() {
			Ok(response) => println!("Message Successfully Sent - {:?}", response),
			Err(error) => println!("Message failed: Error: {:?}", error)
		};
}