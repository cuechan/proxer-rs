pub mod info;
pub mod user;
pub mod list;

use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, Deserializer, Visitor, Unexpected};




pub fn stringly_int<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>
{

	struct IntVisitor(PhantomData<i64>);


	impl<'a> Visitor<'a> for IntVisitor {
		type Value = i64;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("\"int\"")
		}


		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			match v.parse::<i64>() {
				Ok(int) => Ok(int),
				Err(_) => Err(de::Error::invalid_value(Unexpected::Str(v), &self))
			}
		}
	}

	deserializer.deserialize_any(IntVisitor(PhantomData))
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
