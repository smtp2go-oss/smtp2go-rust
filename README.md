[![Crates.io](https://img.shields.io/crates/v/smtp2go.svg)](https://crates.io/crates/smtp2go)
[![Build Status](https://travis-ci.org/smtp2go-oss/smtp2go-rust.svg?branch=master)](https://travis-ci.org/smtp2go-oss/smtp2go-rust)
[![license](https://img.shields.io/github/license/smtp2go-oss/smtp2go-rust.svg)]()

# SMTP2GO API

Rust wrapper around the SMTP2GO [/email/send](https://apidoc.smtp2go.com/documentation/#/POST%20/email/send) API endpoint.

## Installation

Add this line to your Cargo.toml in the [dependencies] block

`smtp2go = "0.1.6"`

## Usage

Sign up for a free account [here](https://www.smtp2go.com/pricing) and once logged in navigate
to the `Settings -> Api Keys` page, create a new API key and make sure the `/email/send` endpoint
is enabled:

Once you have an API key you need to export it into the environment where your Rust application is
going to be executed, this can be done on the terminal like so:

    `$ export SMTP2GO_API_KEY="<your_API_key>"`

Or alternatively you can set it in code using the `std::env::set_var` function. 

Then sending mail is as simple as:

```
	match smtp2go::Email::new()
		.from("Matt <matt@example.com>")
		.to(&[
			"Dave <dave@example.com>".to_string()
		])
		.subject("Trying out SMTP2Go")
		.text_body("Test message")
		.send().await {
			Ok(response) => println!("Message Successfully Sent - {:?}", response),
			Err(error) => println!("Message failed: Error: {:?}", error)
		};
```

## Development

Clone repo. Run tests with `cargo test`.

## Contributing

Bug reports and pull requests are welcome on GitHub [here](https://github.com/smtp2go-oss/smtp2go-rust)

## License

The package is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
