use serde_json::Value;
// use response::string_as_i64;



#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SI {
	I(i64),
	S(String),
}



#[derive(Debug, Clone, Deserialize)]
pub struct Fullentry {
	//#[serde(deserialize_with = "string_as_i64")]
	pub id: SI,
	pub names: Vec<Name>,
	pub genre: String,
	pub fsk: String,
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
pub struct Info {
	pub id: String,
	pub name: String,
	pub genre: String,
	pub fsk: String,
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
pub enum SpoilerFlag {
	#[serde(rename = "0")]
	NoSpoiler,
	#[serde(rename = "1")]
	Spoiler
}


#[derive(Debug, Clone, Deserialize)]
pub enum RateFlag {
	#[serde(rename = "0")]
	NoMatch,
	#[serde(rename = "1")]
	Match
}


#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
	pub info_id: Option<i64>,
	pub id: SI,
	#[serde(rename = "tid")]
	pub tag_id: SI,
	pub timestamp: String,
	pub rate_flag: RateFlag,
	pub spoiler_flag: SpoilerFlag,
	pub tag: String,
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
