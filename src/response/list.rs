use super::Medium;
use super::State;
use super::Season;
use super::stringly_int;




#[derive(Debug, Clone, Deserialize, Hash)]
pub struct EntryList {
	#[serde(deserialize_with = "stringly_int")]
	pub id: i64,
	pub name: String,
	pub genre: String,
	pub medium: Medium,
	#[serde(deserialize_with = "stringly_int")]
	pub count: i64,
	pub state: State,
	#[serde(deserialize_with = "stringly_int")]
	pub rate_sum: i64,
	#[serde(deserialize_with = "stringly_int")]
	pub rate_count: i64,
	pub language: String,
	#[serde(deserialize_with = "stringly_int")]
	pub year: i64,
	pub season: Season,
	#[serde(rename = "type")]
	pub type_: Option<String>,
}
