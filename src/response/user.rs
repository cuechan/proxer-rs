#![allow(unused)]

use chrono;
use chrono::DateTime;
use error;
use serde_json;
use serde_json::Value;
use serde;
use std::convert::From;
use std::fmt;
use std::thread;
use std::time;
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


impl From<Value> for GetList {
	fn from(serde_value: serde_json::Value) -> Self
	{
		let data: GetListRaw = serde_json::from_value(serde_value).unwrap();


		Self {
			id: data.id.parse().unwrap(),
			name: data.name,
			count: data.count.parse().unwrap(),
			medium: data.medium,
			estate: data.estate,
			cid: data.cid.parse().unwrap(),
			comment: data.comment,
			state: data.state,
			episode: data.episode.parse().unwrap(),
			data: data.data,
			rating: data.rating.parse().unwrap(),
			timestamp: data.timestamp.parse().unwrap(),
		}
	}
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


impl From<Value> for Userinfo {
	fn from(serde_json: Value) -> Self
	{
		let data = serde_json.as_object().unwrap();



		Self {
			uid: data.get("uid").unwrap().as_str().unwrap().parse().unwrap(),

			username: data.get("username").unwrap().as_str().unwrap().to_string(),

			avatar: data.get("avatar").unwrap().as_str().unwrap().to_string(),

			status: data.get("status").unwrap().as_str().unwrap().to_string(),

			status_time: data.get("status_time")
				.expect("json doesnt contain 'status_time'")
				.as_i64()
				.unwrap(),

			points_upload: data.get("points_uploads")
				.expect("json does not contain 'points_upload")
				.as_str()
				.expect("'points_uploads' is not a string")
				.parse()
				.expect("cant parse 'points_uploads' as u64"),

			points_anime: data.get("points_anime")
				.unwrap()
				.as_str()
				.unwrap()
				.parse()
				.unwrap(),

			points_manga: data.get("points_manga")
				.unwrap()
				.as_str()
				.unwrap()
				.parse()
				.unwrap(),

			points_info: data.get("points_info")
				.unwrap()
				.as_str()
				.unwrap()
				.parse()
				.unwrap(),

			points_forum: data.get("points_forum")
				.unwrap()
				.as_str()
				.unwrap()
				.parse()
				.unwrap(),

			points_misc: data.get("points_misc")
				.unwrap()
				.as_str()
				.unwrap()
				.parse()
				.unwrap(),
		}
	}
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
