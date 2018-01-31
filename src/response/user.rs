#![allow(unused_imports)]
use prelude::*;
use serde_json::Value;
use chrono::NaiveDateTime;
use super::Kat;
use super::stringly_int;
use super::stringly_array_spaces;
use super::stringly_timestamp_weird;
use super::stringly_timestamp_unix;
use super::timestamp_unix;





#[derive(Debug, Clone, Deserialize)]
pub struct Login {
	#[serde(deserialize_with = "stringly_int")]
	pub uid: i64,
	pub avatar: String,
	pub token: String,
}



#[derive(Debug, Clone, Deserialize)]
pub struct Logout {}




#[derive(Debug, Clone, Deserialize)]
pub struct GetList {
	#[serde(deserialize_with = "stringly_int")]
	pub id: i64,
	pub name: String,
	#[serde(deserialize_with = "stringly_int")]
	pub count: i64,
	pub medium: String,
	pub estate: String,
	#[serde(deserialize_with = "stringly_int")]
	pub cid: i64,
	pub comment: String,
	pub state: String,
	#[serde(deserialize_with = "stringly_int")]
	pub episode: i64,
	pub data: String,
	#[serde(deserialize_with = "stringly_int")]
	pub rating: i64,
	#[serde(deserialize_with = "stringly_timestamp_weird")]
	pub timestamp: NaiveDateTime,
}


#[derive(Deserialize, Debug, Clone)]
pub struct Userinfo {
	#[serde(deserialize_with = "stringly_int")]
	pub uid: i64,
	pub username: String,
	pub avatar: String,
	pub status: String,
	#[serde(deserialize_with = "timestamp_unix")]
	pub status_time: NaiveDateTime,
	#[serde(deserialize_with = "stringly_int")]
	pub points_uploads: i64,
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



#[derive(Deserialize, Debug, Clone)]
pub struct TopTen {
	#[serde(deserialize_with = "stringly_int")]
	pub eid: i64,
	pub name: String,
	pub kat: String,
	pub medium: String,
}
