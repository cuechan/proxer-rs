use client::Client;
use Endpoint;
use error;
use Pageable;
use Pager;
use parameter;
use response;
use std::collections::HashMap;
use serde_json::Value;






#[derive(Debug, Clone)]
pub struct GetFullEntry {
	client: Client,
	data: parameter::InfoGetFullEntry,
	url: String,
}


impl GetFullEntry {
	pub fn new(client: &Client, vars: parameter::InfoGetFullEntry) -> Self
	{
		Self {
			client: client.clone(),
			data: vars,
			url: "info/fullentry".to_string(),
		}
	}
}




impl Endpoint for GetFullEntry {
	type Parameter = parameter::InfoGetFullEntry;
	type ResponseType = response::info::FullEntry;

	fn new(client: Client, vars: Self::Parameter) -> Self {
		Self {
			client: client.clone(),
			data: vars,
			url: "info/fullentry".to_string(),
		}
	}

	fn params_mut(&mut self) -> &mut Self::Parameter
	{
		&mut self.data
	}

	fn client(&self) -> Client {
		self.client.to_owned()
	}

	fn url(&self) -> String {
		self.url.to_owned()
	}

	fn parse(&self, json: Value) -> Result<Self::ResponseType, error::Error>
	{
		Ok(Self::ResponseType::from(json))
	}
}








#[derive(Debug, Clone)]
pub struct GetComments {
	client: Client,
	data: parameter::InfoGetComments,
	url: String,
}




impl GetComments {
	// type ResponseType = response::info::Comment;

	pub fn new(client: &Client, vars: parameter::InfoGetComments) -> Self
	{
		Self {
			client: client.clone(),
			data: vars,
			url: "info/comments".to_string(),
		}
	}
}




impl Endpoint for GetComments {
	type Parameter = parameter::InfoGetComments;
	type ResponseType = Vec<response::info::Comment>;


	fn new(client: Client, vars: parameter::InfoGetComments) -> Self
	{
		Self {
			client: client.clone(),
			data: vars,
			url: "info/comments".to_string(),
		}
	}


	fn client(&self) -> Client
	{
		self.client.to_owned()
	}

	fn params_mut(&mut self) -> &mut Self::Parameter
	{
		&mut self.data
	}

	fn url(&self) -> String
	{
		self.url.to_owned()
	}


	fn parse(&self, json: Value) -> Result<Self::ResponseType, error::Error>
	{
		let mut res = Self::ResponseType::new();
		let array = json.as_array().unwrap();

		for comment in array {
			res.insert(0, response::info::Comment::from(comment.clone()));
		}

		Ok(res)
	}
}



impl Pageable<GetComments> for GetComments {
	fn pager(self) -> Pager<Self>
	{
		Pager::new(self.clone(), None, Some(1_000))
	}
}
