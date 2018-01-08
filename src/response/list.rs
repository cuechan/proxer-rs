use super::SI;
use super::Kat;
use super::Medium;
use super::State;
use super::Season;


#[derive(Debug, Clone, Deserialize)]
pub struct EntryList {
	pub id: SI,
	pub name: String,
	pub genre: String,
	pub medium: Medium,
	pub count: i64,
	pub state: State,
	pub rate_sum: i64,
	pub rate_count: SI,
	pub language: String,
	pub year: SI,
	pub season: Season,
	#[serde(rename = "type")]
	pub type_: Option<String>,
}
