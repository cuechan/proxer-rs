use Endpoint;
use client::Client;
use error;
use parameter;
use response;
use serde_json;
use serde_json::Value;





pub struct Userinfo {
	data: parameter::UserUserinfo,
}




impl Endpoint for Userinfo {
	type Parameter = parameter::UserUserinfo;
	type ResponseType = response::user::Userinfo;
	const URL: &'static str = "user/userinfo";


	fn new(vars: Self::Parameter) -> Self
	{
		Self {
			data: vars
		}
	}

	fn params_mut(&mut self) -> &mut Self::Parameter
	{
		&mut self.data
	}
}
