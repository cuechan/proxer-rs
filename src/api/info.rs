use client;
use client::Client;
use Endpoint;
use error;
use Pageable;
use Pager;
use request::parameter as p;
use response;
use std::collections::HashMap;
use serde_json::Value;






// impl<'a> Info<'a> {
// 	pub fn get_full_entry(self, vars: p::info::GetFullEntry) -> GetFullEntry<'a> {
// 		GetFullEntry::new(self.client, vars)
// 	}
// }






















#[derive(Debug, Clone)]
pub struct GetFullEntry {
	client: Client,
	data: HashMap<String, String>,
	url: String,
}


impl GetFullEntry {
	pub fn new(client: &Client, vars: p::info::GetFullEntry) -> Self
	{
		let mut data = HashMap::new();

		data.insert("id".to_string(), vars.id.to_string());

		Self {
			client: client.clone(),
			url: "info/fullentry".to_string(),
			data: data,
		}
	}
}




impl Endpoint for GetFullEntry {
	type ResponseType = response::info::FullEntry;

	fn params_mut(&mut self) -> &mut HashMap<String, String>
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
	data: HashMap<String, String>,
	url: String,
}




impl GetComments {
	// type ResponseType = response::info::Comment;

	pub fn new(client: &Client, vars: p::info::GetComments) -> Self
	{
		let mut data = HashMap::new();

		data.insert("id".to_string(), vars.id.to_string());

		if let Some(p) = vars.p {
			data.insert("p".to_string(), p.to_string());
		};

		if let Some(limit) = vars.limit {
			data.insert("limit".to_string(), limit.to_string());
		};

		if let Some(sort) = vars.sort {
			data.insert("sort".to_string(), sort.to_string());
		};






		Self {
			client: client.clone(),
			data: data,
			url: "info/comments".to_string(),
		}
	}


	// pub fn iterator(self) -> Pager<Self>
	// {
	// 	Pager::new(self.clone(), 100)
	// }
}




impl Endpoint for GetComments {
	type ResponseType = Vec<response::info::Comment>;


	fn client(&self) -> Client
	{
		self.client.to_owned()
	}

	fn params_mut(&mut self) -> &mut HashMap<String, String>
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
		Pager::new(self.clone(), 0, 10)
	}
}
