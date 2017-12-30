#[allow(unused_imports)]
use Endpoint;


#[derive(Serialize, Debug, Clone)]
pub struct Login {
	pub username: String,
	pub password: String,
	pub secretkey: Option<String>,
}



#[derive(Serialize, Debug, Clone)]
pub struct Logout {}



#[derive(Serialize, Debug, Clone)]
pub struct Userinfo {
	pub uid: Option<usize>,
	pub username: Option<String>,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetTopten {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub is_h: Option<bool>,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetList {
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
pub struct GetLatestComments {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub length: Option<u64>,

	pub p: Option<u64>,
	pub limit: Option<u64>,
}



#[derive(Serialize, Debug, Clone)]
pub struct GetHistory {
	pub uid: Option<usize>,
	pub username: Option<String>,
	pub is_h: Option<bool>,
	pub filter: Option<u64>,

	pub p: Option<u64>,
	pub limit: Option<u64>,
}
