pub mod api;
pub mod http;

use std;
use std::fmt;


#[derive(Debug)]
pub enum Error {
	Api(api::Api),
	Json,
	Http,
	Unknown,
}


impl Error {
	pub fn is_api_error(self) -> bool
	{
		match self
		{
			Error::Api(_) => true,
			Error::Json => false,
			Error::Http => false,
			_ => false,
		}
	}

	pub fn is_http_err(self) -> bool
	{
		match self
		{
			Error::Api(_) => false,
			Error::Json => false,
			Error::Http => true,
			_ => false,
		}
	}

	pub fn is_json_error(self) -> bool
	{
		match self
		{
			Error::Api(_) => false,
			Error::Json => true,
			Error::Http => false,
			_ => false,
		}
	}
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}







impl std::error::Error for Error {
	fn description(&self) -> &str {
		"general error"
	}
}
