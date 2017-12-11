use serde_json::Value;


#[derive(Debug, Clone, Deserialize)]
pub struct Fullentry {
	id: String,
	names: Vec<Name>,
	genre: String,
	fsk: String,
	description: String,
	medium: String,
	count: String,
	state: String,
	rate_sum: String,
	rate_count: String,
	clicks: String,
	kat: String,
	license: String,
	tags: Vec<Tag>,
}


#[derive(Debug, Clone, Deserialize)]
struct Info {
	id: String,
	name: String,
	genre: String,
	fsk: String,
	description: String,
	medium: String,
	count: String,
	state: String,
	rate_sum: String,
	rate_count: String,
	clicks: String,
	kat: Value,
	license: Value,
}


#[derive(Debug, Clone, Deserialize)]
pub struct UserList {
	pub id: i64,
	pub name: String,
	pub description: String,
	pub genre: Vec<String>,
	pub fsk: Vec<String>,
	pub medium: String,
	pub count: i64,
	pub state: i64,
	pub rate_sum: i64,
	pub rate_count: i64,
	pub clicks: i64,
	pub kat: Kat,
	pub license: License,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
	pub info_id: Option<i64>,
	pub id: i64,
	pub tag_id: i64,
	pub added: String,
	pub matches: bool,
	pub spoiler: bool,
	pub name: String,
	pub description: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Name {
	id: String,
	eid: String,
	#[serde(rename = "type")]
	type_: String,
	name: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
	pub id: i64,
	pub info_id: i64,
	pub comment_type: String,
	pub state: i64, // Todo: use enum for state
	pub data: String,
	pub comment: String,
	pub rating: i64,
	pub episode: i64,
	pub positive: i64,
	pub timestamp: i64, //Todo: use chrono here
	pub username: String,
	pub uid: i64,
	pub avatar: String,
}






#[derive(Deserialize, Debug, Clone)]
pub struct Userinfo {
	pub uid: i64,
	pub username: String,
	pub avatar: String, // use some sort of uri type here
	pub status: String,
	// status_time is sometimes a negative number. using i64
	pub status_time: i64,
	pub points_upload: i64,
	pub points_anime: i64,
	pub points_manga: i64,
	pub points_info: i64,
	pub points_forum: i64,
	pub points_misc: i64,
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
