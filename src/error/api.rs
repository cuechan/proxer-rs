use std::fmt;
use serde_json;
use client;



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Errcode {
	// UNKNOWN_ERROR
	UnknownError,

	// 1xxx errors
	ApiVersionNotFound,
	ApiVersionDeprecated,
	ApiClassNotFound,
	ApiFunctionNotFound,
	NoApiPermissions,
	InvalidToken,
	ApiFunctionDisabled,
	ProxerMaintenance,
	ApiMaintenance,

	// 2xxx errors
	IpBlocked,
	NewsError,

	// 3xxx errors
	MissingLoginCredentials,
	InvalidLoginCredentials,
	InvalidId,
	InvalidUid,
	UserNotLoggedIn,
	UserNotFound,
	UserAlreadyLoggedIn,
}


impl Errcode {
	pub fn from_code(code: i64) -> Self
	{

		match code
		{
			1000 => Errcode::ApiVersionNotFound,
			1001 => Errcode::ApiVersionDeprecated,
			1002 => Errcode::ApiClassNotFound,
			1003 => Errcode::ApiFunctionNotFound,
			1004 => Errcode::NoApiPermissions,
			1005 => Errcode::InvalidToken,
			1006 => Errcode::ApiFunctionDisabled,
			1007 => Errcode::ProxerMaintenance,
			1008 => Errcode::ApiMaintenance,

			2000 => Errcode::IpBlocked,
			2001 => Errcode::NewsError,

			3000 => Errcode::MissingLoginCredentials,
			3001 => Errcode::InvalidLoginCredentials,
			3002 => Errcode::UserNotLoggedIn,
			3003 => Errcode::UserNotFound,
			3004 => Errcode::UserNotLoggedIn,
			3007 => Errcode::InvalidId,
			3009 => Errcode::UserNotLoggedIn,
			3012 => Errcode::UserAlreadyLoggedIn,
			3023 => Errcode::UserNotLoggedIn,
			3034 => Errcode::UserNotLoggedIn,

			_ => Errcode::UnknownError,
		}
	}
}





impl From<i64> for Errcode {
	fn from(code: i64) -> Errcode
	{
		Errcode::from_code(code)
	}
}



impl fmt::Display for Errcode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{:?}", self)
	}
}










#[derive(Debug, Clone)]
pub struct Api {
	code: i64,
	message: String,
	error: Errcode,
}

impl Api {
	pub fn new(code: i64, msg: String) -> Self
	{
		Api {
			code: code,
			message: msg,
			error: Errcode::from(code),
		}
	}

	pub fn error(&self) -> Errcode {
		self.error
	}
}


impl From<serde_json::Value> for Api {
	fn from(json: serde_json::Value) -> Self
	{

		let error = json.get("code").unwrap().as_i64().unwrap();
		let msg = json.get("message").unwrap().as_str().unwrap();


		Self {
			error: Errcode::from_code(error),
			code: error,
			message: msg.to_string(),
		}
	}
}


impl From<client::ApiResponse> for Api {
	fn from(res: client::ApiResponse) -> Self
	{
		Self {
			code: res.code.unwrap(),
			error: Errcode::from_code(res.code.unwrap()),
			message: res.message.to_string(),
		}
	}
}



impl fmt::Display for Api {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", Errcode::from(self.code))
	}
}
