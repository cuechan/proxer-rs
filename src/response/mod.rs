use std::fmt;

pub mod info;
pub mod user;
pub mod list;



/// `S`ring/`I`nteger
/// a temporary type for strings that are integers
/// if a field with an integer as string is used, just use `.into()`

/// `S`ring/`I`nteger
/// a temporary type for strings that are integers
/// if a field with an integer as string is used, just use `.into()`
#[derive(Debug, Clone, Deserialize, Hash)]
#[serde(untagged)]
pub enum SI {
	I(i64),
	S(String),
}



impl Into<String> for SI {
	fn into(self) -> String
	{
		match self
		{
			SI::I(i) => i.to_string(),
			SI::S(s) => s,
		}
	}
}



impl From<SI> for u64 {
	fn from(si: SI) -> Self
	{
		match si
		{
			SI::I(i) => i as u64,
			SI::S(s) => s.parse().unwrap(),
		}
	}
}


impl From<SI> for u32 {
	fn from(si: SI) -> Self
	{
		match si
		{
			SI::I(i) => i as u32,
			SI::S(s) => s.parse().unwrap(),
		}
	}
}


impl From<SI> for i64 {
	fn from(si: SI) -> Self
	{
		match si
		{
			SI::I(i) => i as Self,
			SI::S(s) => s.parse().unwrap(),
		}
	}
}









impl fmt::Display for SI {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let x: String = self.to_owned().into();
		write!(f, "{}", x)
	}
}





#[derive(Debug, Clone, Deserialize, Hash)]
pub enum Kat {
	Anime,
	Manga,
}


#[derive(Debug, Clone, Deserialize, Hash)]
pub enum Medium {
	#[serde(rename = "animeseries")]
	AnimeSeries,
	#[serde(rename = "movie")]
	Movie,
	#[serde(rename = "ova")]
	OVA,
	#[serde(rename = "hentai")]
	Hentai,
	#[serde(rename = "mangaseries")]
	Manga,
	#[serde(rename = "oneshot")]
	OneShot,
	#[serde(rename = "doujin")]
	Doujin,
	#[serde(rename = "hmanga")]
	HManga,
}


#[derive(Debug, Clone, Deserialize, Hash)]
pub enum State {
	#[serde(rename = "0")]
	PreAiring,
	#[serde(rename = "1")]
	Finished,
	#[serde(rename = "2")]
	Airing,
	#[serde(rename = "3")]
	Cancelled,
	#[serde(rename = "4")]
	NoSub,
}

#[derive(Debug, Clone, Deserialize, Hash)]
pub enum Season {
	#[serde(rename = "0")]
	Unknown,
	#[serde(rename = "1")]
	Winter,
	#[serde(rename = "2")]
	Spring,
	#[serde(rename = "3")]
	Summer,
	#[serde(rename = "4")]
	Autumn,
}
