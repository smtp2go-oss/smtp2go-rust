# SMTP2Go API

Rust wrapper over the [SMTP2GO](https://www.smtp2go.com) API.

## Installation

Add this line to your Cargo.toml in the [dependencies] block

`smtp2go = "0.1.0"`

## Usage

Sign up for a free account [here](https://www.smtp2go.com/pricing) and once logged in navigate
to the `Settings -> Api Keys` page, create a new API key and make sure the `/email/send` endpoint
is enabled:

Once you have an API key you need to export it into the environment where your Rust application is
going to be executed, this can be done on the terminal like so:

    `$ export API_KEY="<your_API_key>"`

Or alternatively you can set it in code using the `std::env::set_var` function. 

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