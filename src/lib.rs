#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate regex;

#[macro_use]
extern crate hyper;
extern crate hyper_native_tls;

pub const VERSION: &'static str = "0.1.0";

mod core;
mod email;

pub use email::Email;