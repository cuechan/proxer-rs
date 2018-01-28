use prelude::*;
use serde_json::Value;
use chrono::NaiveDateTime;
use super::Kat;
use super::stringly_int;
use super::stringly_array_spaces;
use super::stringly_timestamp_weird;
use super::stringly_timestamp_unix;


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
	#[serde(deserialize_with = "stringly_timestamp_weird")]
	pub timestamp: NaiveDateTime,
}


#[derive(Debug, Clone, Deserialize)]
pub struct GetList {
	#[serde(deserialize_with = "stringly_int")]
	pub id: i64,
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
	#[serde(deserialize_with = "stringly_timestamp_weird")]
	pub timestamp: NaiveDateTime,
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
pub enum License {
	Unknown,
	Unlicensed,
	Licensed,
}
