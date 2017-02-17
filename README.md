# SMTP2Go API

Rust wrapper over the [SMTP2Go](https://www.smtp2go.com) API.

## Installation

Add this line to your Cargo.toml

`smtp2go = "0.1.0"`

## Usage

Sign up for a free account [here](https://www.smtp2go.com/pricing) and get an API key. At your shell, run:

    `$ export API_KEY="<your_API_key>"`

Then sending mail is as simple as:

```
	extern crate smtp2go;

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
```

Full documentation can be found [here](https://apidoc.smtp2go.com/documentation/#/README)

## Development

Clone repo. Run tests with `cargo test`.

## Contributing

Bug reports and pull requests are welcome on GitHub [here](https://github.com/smtp2go/smtp2go.api-rust)

## License

The package is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).