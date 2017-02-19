extern crate serde;

#[macro_use] // custom derive crates need this
extern crate serde_derive;

extern crate serde_json;
extern crate regex;

#[macro_use] // needed for header! macro
extern crate hyper;
extern crate hyper_native_tls;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// import the client modules in to the smtp2go umbrella module
mod core;
mod email;

// import a few things into the smtp2go module
pub use core::{Smtp2goApiResult, Smtp2goApiError};
pub use email::Email;
