use Endpoint;
use PageableEndpoint;
use Pager;
use client::Client;
use error;
use parameter;
use response;
use serde_json;
use serde_json::Value;






#[derive(Debug, Clone)]
pub struct GetFullEntry {
	client: Client,
	data: parameter::InfoGetFullEntry,
}






impl Endpoint for GetFullEntry {
	type Parameter = parameter::InfoGetFullEntry;
	type ResponseType = response::info::Fullentry;


	fn new(client: Client, vars: Self::Parameter) -> Self
	{
		Self {
			client: client.clone(),
			data: vars,
		}
	}

	fn params_mut(&mut self) -> &mut Self::Parameter
	{
		&mut self.data
	}

	fn client(&self) -> Client
	{
		self.client.to_owned()
	}

	fn url(&self) -> String
	{
		warn!("depricated url");
		String::from("foobar")
	}

	fn parse(&self, json: Value) -> Result<Self::ResponseType, error::Error>
	{
		match serde_json::from_value::<Self::ResponseType>(json.clone())
		{
			Ok(data) => Ok(data),
			Err(e) => Err(error::Error::Json(e)),
		}
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
		let res: Self::ResponseType = serde_json::from_value(json).unwrap();

		Ok(res)
	}
}



impl<'a> PageableEndpoint<'a, GetComments> for GetComments {
	fn pager(self, client: Client) -> Pager<'a, GetComments>
	{
		debug!("new pager with data: {:?}", self.data);
		Pager::new(client, self, Some(0), Some(3))
	}
}
