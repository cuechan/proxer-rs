use prelude::*;






#[derive(Debug, Clone, Deserialize)]
struct GetListRaw {
	pub id: String,
	pub name: String,
	pub count: String,
	pub medium: String,
	pub estate: String,
	pub cid: String,
	pub comment: String,
	pub state: String,
	pub episode: String,
	pub data: String,
	pub rating: String,
	pub timestamp: String, //Todo: use chrono here
}


#[derive(Debug, Clone)]
pub struct GetList {
	pub id: InfoID,
	pub name: String,
	pub count: i64,
	pub medium: String,
	pub estate: String,
	pub cid: i64,
	pub comment: String,
	pub state: String,
	pub episode: i64,
	pub data: String,
	pub rating: i64,
	pub timestamp: String, //Todo: use chrono here
}


#[derive(Deserialize, Debug, Clone)]
pub struct Userinfo {
	pub uid: u64,
	pub username: String,
	pub avatar: String, // use some sort of uri type here
	pub status: String,
	// status_time is sometimes a negative number. using i64
	pub status_time: i64,
	pub points_upload: u64,
	pub points_anime: u64,
	pub points_manga: u64,
	pub points_info: u64,
	pub points_forum: u64,
	pub points_misc: u64,
}


#[derive(Debug, Clone, Deserialize)]
pub enum Kat {
	Anime,
	Manga,
}


#[derive(Debug, Clone, Deserialize)]
pub enum License {
	Unknown,
	Unlicensed,
	Licensed,
}
