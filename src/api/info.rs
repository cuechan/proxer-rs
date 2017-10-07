#![allow(missing_docs)]


use client;
use client::Client;
use Endpoint;
use error;
use Pageable;
use Pager;
use Request;
use request::parameter as p;
use response;
use std;
use std::collections::HashMap;






#[derive(Debug, Clone)]
pub struct GetFullEntry<'a> {
	client: Client<'a>,
	data: HashMap<String, String>,
	url: String,
}


impl<'a> GetFullEntry<'a> {
	pub fn new(client: &Client<'a>, vars: p::info::GetFullEntry) -> Self
	{
		let mut data = HashMap::new();

		data.insert("id".to_string(), vars.0.to_string());

		Self {
			client: client.clone(),
			url: "info/fullentry".to_string(),
			data: data,
		}
	}
}




impl<'a> Endpoint for GetFullEntry<'a> {
	type ResponseType = response::info::FullEntry;

	fn get_params_mut(&mut self) -> &mut HashMap<String, String>
	{
		&mut self.data
	}

	fn send(self) -> Result<Self::ResponseType, error::Error>
	{
		match self.client.execute(self.url, self.data)
		{
			Err(e) => Err(e),
			Ok(r) => Ok(Self::ResponseType::from(r)),
		}
	}
}






















#[derive(Debug, Clone)]
pub struct GetComments<'a> {
	client: Client<'a>,
	data: HashMap<String, String>,
	url: String,
}




impl<'a> GetComments<'a> {
	// type ResponseType = response::info::Comment;

	pub fn new(client: Client<'a>, vars: p::info::GetComments) -> Self
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
			client: client,
			data: data,
			url: "info/comments".to_string(),
		}
	}


	// pub fn iterator(self) -> Pager<Self>
	// {
	// 	Pager::new(self.clone(), 100)
	// }
}




impl<'a> Endpoint for GetComments<'a> {
	type ResponseType = Vec<response::info::Comment>;


	fn get_params_mut(&mut self) -> &mut HashMap<String, String>
	{
		&mut self.data
	}


	fn send(self) -> Result<Self::ResponseType, error::Error>
	{
		match self.client.execute(self.url, self.data)
		{
			Err(e) => Err(e),
			Ok(r) => {
				let mut res = Self::ResponseType::new();
				let array = r.as_array().unwrap();

				for comment in array {
					res.insert(0, response::info::Comment::from(comment.clone()));
				}

				Ok(res)
			}
		}
	}
}



impl<'a> Pageable<GetComments<'a>> for GetComments<'a> {
	fn pager(self) -> Pager<Self>
	{
		Pager::new(self.clone(), 0, 10)
	}
}
