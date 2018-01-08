pub mod info;
pub mod user;
pub mod list;



/// `S`ring/`I`nteger
/// a temporary type for strings that are integers
/// if a field with an integer as string is used, just use `.into()`

/// `S`ring/`I`nteger
/// a temporary type for strings that are integers
/// if a field with an integer as string is used, just use `.into()`
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SI {
	I(i64),
	S(String),
}


#[derive(Debug, Clone, Deserialize)]
pub enum Kat {
	Anime,
	Manga,
}


#[derive(Debug, Clone, Deserialize)]
pub enum Medium {
	#[serde(rename = "animerseries")]
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


#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
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
