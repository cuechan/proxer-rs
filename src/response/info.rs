use serde_json::Value;
use chrono::NaiveDateTime;
use super::Kat;
use super::stringly_int;
use super::stringly_array_spaces;
use super::stringly_timestamp_weird;
use super::stringly_timestamp_unix;



#[derive(Debug, Clone, Deserialize)]
pub struct Fullentry {
	#[serde(deserialize_with = "stringly_int")]
	pub id: i64,
	pub names: Vec<Name>,
	#[serde(deserialize_with = "stringly_array_spaces")]
	pub genre: Vec<String>,
	#[serde(deserialize_with = "stringly_array_spaces")]
	pub fsk: Vec<String>,
	pub description: String,
	pub medium: String,
	pub count: String,
	pub state: String,
	pub rate_sum: String,
	pub rate_count: String,
	pub clicks: String,
	pub kat: String,
	pub license: String,
	pub tags: Vec<Tag>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct GetEntry {
	pub id: String,
	pub name: String,
	#[serde(deserialize_with = "stringly_array_spaces")]
	pub genre: Vec<String>,
	#[serde(deserialize_with = "stringly_array_spaces")]
	pub fsk: Vec<String>,
	pub description: String,
	pub medium: String,
	pub count: String,
	pub state: String,
	pub rate_sum: String,
	pub rate_count: String,
	pub clicks: String,
	pub kat: Value,
	pub license: Value,
}


#[derive(Debug, Clone, Deserialize)]
pub struct UserList {
	#[serde(deserialize_with = "stringly_int")]
	pub id: i64,
	pub name: String,
	pub description: String,
	#[serde(deserialize_with = "stringly_array_spaces")]
	pub genre: Vec<String>,
	#[serde(deserialize_with = "stringly_array_spaces")]
	pub fsk: Vec<String>,
	pub medium: String,
	#[serde(deserialize_with = "stringly_int")]
	pub count: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub state: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub rate_sum: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub rate_count: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub clicks: i64,
	pub kat: Kat,
	pub license: License,
}



#[derive(Debug, Clone, Deserialize)]
pub enum SpoilerFlag {
	#[serde(rename = "0")]
	NoSpoiler,
	#[serde(rename = "1")]
	Spoiler,
}


#[derive(Debug, Clone, Deserialize)]
pub enum RateFlag {
	#[serde(rename = "0")]
	NoMatch,
	#[serde(rename = "1")]
	Match,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
	pub info_id: Option<i64>,
	#[serde(deserialize_with = "stringly_int")]
	pub id: i64,
	#[serde(rename = "tid", deserialize_with = "stringly_int")]
	pub tag_id: i64,
	#[serde(deserialize_with = "stringly_timestamp_weird")]
	pub timestamp: NaiveDateTime,
	pub rate_flag: RateFlag,
	pub spoiler_flag: SpoilerFlag,
	pub tag: String,
	pub description: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Name {
	#[serde(deserialize_with = "stringly_int")]
	pub id: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub eid: i64,
	#[serde(rename = "type")]
	pub type_: String,
	pub name: String,
}


#[derive(Debug, Clone, Deserialize)]
pub enum WatchState {
	#[serde(rename = "0")]
	Watched,
	#[serde(rename = "1")]
	Watching,
	#[serde(rename = "2")]
	WillWatch,
	#[serde(rename = "3")]
	Cancelled,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
	#[serde(deserialize_with = "stringly_int")]
	pub id: i64,
	#[serde(rename = "tid", deserialize_with = "stringly_int")]
	pub info_id: i64,
	#[serde(rename = "type")]
	pub comment_type: String, // i have no idea what this is
	pub state: WatchState,
	pub data: String,
	pub comment: String,
	#[serde(deserialize_with = "stringly_int")]
	pub rating: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub episode: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub positive: i64,
	#[serde(deserialize_with = "stringly_timestamp_unix")]
	pub timestamp: NaiveDateTime, //Todo: use chrono here
	pub username: String,
	#[serde(deserialize_with = "stringly_int")]
	pub uid: i64,
	pub avatar: String,
}






#[derive(Deserialize, Debug, Clone)]
pub struct Userinfo {
	#[serde(deserialize_with = "stringly_int")]
	pub uid: i64,
	pub username: String,
	pub avatar: String, // use some sort of uri type here
	pub status: String,
	// status_time is sometimes a negative number. using i64
	#[serde(deserialize_with = "stringly_int")]
	pub status_time: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub points_upload: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub points_anime: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub points_manga: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub points_info: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub points_forum: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub points_misc: i64,
}


#[derive(Debug, Clone, Deserialize)]
pub enum License {
	Unknown,
	Unlicensed,
	Licensed,
}
