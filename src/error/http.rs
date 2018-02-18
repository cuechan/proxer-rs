use reqwest;
use std::fmt;

#[derive(Debug)]
pub struct Http {
	err: reqwest::Error,
}

impl Http {
	pub fn new(error: reqwest::Error) -> Self {
		Self { err: error }
	}
}

impl fmt::Display for Http {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "http error")
	}
}
