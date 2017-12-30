use Endpoint;
use PageableEndpoint;
use Pager;
use client::Client;
use response;
use serde_json;
use serde_json::Value;






#[derive(Serialize, Debug, Clone)]
pub struct GetFullEntry {
	pub id: usize,
}


impl Endpoint for GetFullEntry {
	type ResponseType = response::info::Fullentry;
	const URL: &'static str = "info/fullentry";
}



#[derive(Serialize, Debug, Clone)]
pub struct GetEntry {
	pub id: usize,
}


#[derive(Serialize, Debug, Clone)]
pub struct GetNames {
	pub id: usize,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetGate {
	pub id: usize,
}





#[derive(Serialize, Debug, Clone)]
pub struct GetLang {
	pub id: usize,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetSeason {
	pub id: usize,
}


#[derive(Serialize, Debug, Clone)]
pub struct GetGroups {
	pub id: usize,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetPublisher {
	pub id: usize,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetListinfo {
	pub id: usize,
	pub p: Option<i64>,
	pub limit: Option<i64>,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetComments {
	pub id: usize,
	pub p: Option<usize>,
	pub limit: Option<usize>,
	pub sort: Option<String>,
}



impl Endpoint for GetComments {
	type ResponseType = Vec<response::info::Comment>;
	const URL: &'static str = "info/comments";
}



impl PageableEndpoint for GetComments {
	fn pager(self, client: Client) -> Pager<GetComments>
	{
		debug!("new pager with data: {:?}", self);
		Pager::new(client, self, Some(0), Some(3))
	}

	fn page_mut(&mut self) -> &mut Option<usize>
	{
		&mut self.p
	}

	fn limit_mut(&mut self) -> &mut Option<usize>
	{
		&mut self.limit
	}
}


















#[derive(Serialize, Debug, Clone)]
pub struct GetRelations {
	pub info_id: usize,
	pub is_h: Option<bool>,
}



#[derive(Serialize, Debug, Clone)]
pub struct EntryTags {
	pub id: usize,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetTranslatorgroup {
	pub id: usize,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetIndustry {
	pub id: usize,
}






#[derive(Serialize, Debug, Clone)]
pub struct SetUserInfo {
	pub id: usize,
	/// since 'type' is a rust keyword, this parameter is renamed
	#[serde(rename = "type")]
	pub type_: String,
}
