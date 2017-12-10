

//! All request parameters are here
//! all these structs can be serialized into an urlencoded string
//!
//! __b-but why n-not in different mods, anon? :(__
//!
//! because the api is so shitty, there is no logical distiction between the api classes.
//! it's just a huge clusterfuck
//!
//! __why usize instead of i64?__
//!
//! when you know whats coming, why don't be prepared? (an id will never be < 0)




#[derive(Serialize, Debug, Clone)]
pub struct InfoGetFullEntry {
	pub id: usize
}



#[derive(Serialize, Debug, Clone)]
pub struct InfoGetEntry {
	pub id: usize
}


#[derive(Serialize, Debug, Clone)]
pub struct InfoGetNames {
	pub id: usize
}



#[derive(Serialize, Debug, Clone)]
pub struct InfoGetGate{
	pub id: usize
}





#[derive(Serialize, Debug, Clone)]
pub struct InfoGetLang{
	pub id: usize
}



#[derive(Serialize, Debug, Clone)]
pub struct InfoGetSeason{
	pub id: usize
}


#[derive(Serialize, Debug, Clone)]
pub struct InfoGetGroups{
	pub id: usize
}



#[derive(Serialize, Debug, Clone)]
pub struct InfoGetPublisher{
	pub id: usize
}




#[derive(Serialize, Debug, Clone)]
pub struct InfoGetListinfo {
	pub id: usize,
	pub p: Option<i64>,
	pub limit: Option<i64>,
}



#[derive(Serialize, Debug, Clone)]
pub struct InfoGetComments {
	pub id: usize,
	pub p: Option<i64>,
	pub limit: Option<i64>,
	pub sort: Option<String>,
}

impl Iterator for InfoGetComments {
	type Item = Self;

	fn next(&mut self) -> Option<Self::Item> {
		self.p = match self.p {
			Some(p) => Some(p + 1),
			None => Some(0)
		};

		Some(self.clone())
	}
}




#[derive(Serialize, Debug, Clone)]
pub struct InfoGetRelations {
	pub info_id: usize,
	pub is_h: Option<bool>,
}



#[derive(Serialize, Debug, Clone)]
pub struct InfoEntryTags{
	pub id: usize
}



#[derive(Serialize, Debug, Clone)]
pub struct GetTranslatorgroup{
	pub id: usize
}



#[derive(Serialize, Debug, Clone)]
pub struct InfoGetIndustry{
	pub id: usize
}


#[derive(Serialize, Debug, Clone)]
pub struct InfoSetUserInfo {
	pub id: usize,
	/// since 'type' is a rust keyword, this parameter is renamed
	#[serde(rename = "type")]
	pub type_: String,
}



#[derive(Serialize, Debug, Clone)]
pub struct ListGetList {
	pub kat: Option<String>,
	pub medium: Option<String>,
	pub is_h: Option<String>,
	pub start: Option<String>,
	pub sort: Option<String>,
	pub sort_type: Option<String>,
	pub p: Option<i64>,
	pub limit: Option<i64>,
}


impl Iterator for ListGetList {
	type Item = Self;

	fn next(&mut self) -> Option<Self::Item> {
		self.p = match self.p {
			Some(p) => Some(p + 1),
			None => Some(0)
		};

		Some(self.clone())
	}
}



#[derive(Serialize, Debug, Clone)]
pub struct UserLogin {
	pub username: String,
	pub password: String,
	pub secretkey: Option<String>,
}



#[derive(Serialize, Debug, Clone)]
pub struct UserLogout {}



#[derive(Serialize, Debug, Clone)]
pub struct UserUserinfo {
	pub uid: Option<usize>,
	pub username: Option<String>,
}



#[derive(Serialize, Debug, Clone)]
pub struct UserGetTopten {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub is_h: Option<bool>,
}



#[derive(Serialize, Debug, Clone)]
pub struct UserGetList {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub search: Option<String>,
	pub search_start: Option<String>,
	pub is_h: Option<bool>,
	pub filter: Option<u64>,
	pub sort: Option<String>,

	pub p: Option<i64>,
	pub limit: Option<u64>,
}



#[derive(Serialize, Debug, Clone)]
pub struct UserGetLatestComments {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub length: Option<u64>,

	pub p: Option<u64>,
	pub limit: Option<u64>,
}



#[derive(Serialize, Debug, Clone)]
pub struct UserGetHistory {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub is_h: Option<bool>,
	pub filter: Option<u64>,

	pub p: Option<u64>,
	pub limit: Option<u64>,
}
