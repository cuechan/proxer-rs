pub mod info;
pub mod user;
pub mod list;

use chrono;
use chrono::NaiveDateTime;
use chrono::format;
use serde::de::{self, Deserializer, Visitor, Unexpected};
use std::fmt;
use std::marker::PhantomData;




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



pub fn stringly_array_spaces<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>
{

	struct IntVisitor(PhantomData<Vec<String>>);


	impl<'a> Visitor<'a> for IntVisitor {
		type Value = Vec<String>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("\"string\"")
		}


		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {

			let mut list: Vec<String> = Vec::new();

			// for item in v.to_string().split_whitespace() {
			// 	list.push(item.to_string());
			// }


			v.to_string()
				.split_whitespace()
				.for_each(|x| list.push(x.to_string()));



			Ok(list)
			// Err(_) => Err(de::Error::invalid_value(Unexpected::Str(v), &self))
		}
	}

	deserializer.deserialize_any(IntVisitor(PhantomData))
}



/// total clusterfuck
pub fn stringly_timestamp_weird<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>
{

	struct IntVisitor(PhantomData<NaiveDateTime>);


	impl<'a> Visitor<'a> for IntVisitor {
		type Value = NaiveDateTime;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("\"stringed datetime\"")
		}


		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			// timestamp: "2016-06-19 14:13:26",
			let time = NaiveDateTime::parse_from_str(v, "%F %T")
				.expect(&format!("failed to parse: {}", v));

			Ok(time)
		}
	}

	deserializer.deserialize_any(IntVisitor(PhantomData))
}




/// better but still fucked up clusterfuck
pub fn stringly_timestamp_unix<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>
{

	struct IntVisitor(PhantomData<NaiveDateTime>);


	impl<'a> Visitor<'a> for IntVisitor {
		type Value = NaiveDateTime;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("\"stringed datetime\"")
		}


		fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
			// timestamp: "2016-06-19 14:13:26",
			let time = NaiveDateTime::parse_from_str(v, "%s").unwrap();

			Ok(time)
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
