use Endpoint;
use client::Client;
use error;
use parameter;
use response;
use serde_json;
use serde_json::Value;





pub struct Userinfo {
	client: Client,
	url: String,
	data: parameter::UserUserinfo,
}

impl Userinfo {
	pub fn new(client: &Client, vars: parameter::UserUserinfo) -> Self
	{
		Self {
			client: client.clone(),
			data: vars,
			url: "user/userinfo".to_string(),
		}
	}
}


impl Endpoint for Userinfo {
	type Parameter = parameter::UserUserinfo;
	type ResponseType = response::user::Userinfo;


	fn new(client: Client, vars: Self::Parameter) -> Self
	{
		Self {
			client: client.clone(),
			data: vars,
			url: "user/userinfo".to_string(),
		}
	}


	fn client(&self) -> Client
	{
		self.client.to_owned()
	}

	fn url(&self) -> String
	{
		self.url.to_owned()
	}

	fn params_mut(&mut self) -> &mut Self::Parameter
	{
		&mut self.data
	}

	fn parse(&self, json: Value) -> Result<Self::ResponseType, error::Error>
	{
		let data: Self::ResponseType = serde_json::from_value(json).unwrap();

		Ok(data)
	}
}
