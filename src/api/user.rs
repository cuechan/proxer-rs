use client::Client;
use std::collections::HashMap;
use request::parameter;
use Endpoint;
use response;
use error;
use serde_json::Value;





pub struct Userinfo {
	client: Client,
	url: String,
	data: HashMap<String, String>,
}

impl Userinfo {
	pub fn new(client: &Client, vars: parameter::user::Userinfo) -> Self
	{
		let mut data = HashMap::new();

		match (vars.uid, vars.username)
		{
			(Some(i), None) => data.insert("uid".to_string(), i.to_string()),
			(None, Some(i)) => data.insert("username".to_string(), i.to_string()),
			_ => panic!("either username nor uid are given"),
		};

		Self {
			client: client.clone(),
			data: data,
			url: "user/userinfo".to_string(),
		}
	}
}


impl Endpoint for Userinfo {
	type ResponseType = response::user::Userinfo;


	fn client(&self) -> Client
	{
		self.client.to_owned()
	}

	fn url(&self) -> String
	{
		self.url.to_owned()
	}

	fn params_mut(&mut self) -> &mut HashMap<String, String>
	{
		&mut self.data
	}

	fn parse(&self, json: Value) -> Result<Self::ResponseType, error::Error>
	{
		Ok(Self::ResponseType::from(json))
	}
}
