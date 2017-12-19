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
	data: parameter::InfoGetFullEntry,
}






impl Endpoint for GetFullEntry {
	type Parameter = parameter::InfoGetFullEntry;
	type ResponseType = response::info::Fullentry;
	const URL: &'static str = "info/fullentry";


	fn new(vars: Self::Parameter) -> Self
	{
		Self {
			data: vars,
		}
	}

	fn params_mut(&mut self) -> &mut Self::Parameter
	{
		&mut self.data
	}
}








#[derive(Debug, Clone)]
pub struct GetComments {
	data: parameter::InfoGetComments,
}



impl Endpoint for GetComments {
	type Parameter = parameter::InfoGetComments;
	type ResponseType = Vec<response::info::Comment>;
	const URL: &'static str = "info/comments";


	fn new(vars: parameter::InfoGetComments) -> Self
	{
		Self {
			data: vars,
		}
	}

	fn params_mut(&mut self) -> &mut Self::Parameter
	{
		&mut self.data
	}
}



impl PageableEndpoint<GetComments> for GetComments {
	fn pager(self, client: Client) -> Pager<GetComments>
	{
		debug!("new pager with data: {:?}", self.data);
		Pager::new(client, self, Some(0), Some(3))
	}
}
