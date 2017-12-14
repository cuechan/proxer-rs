//! proxer api library
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate serde_urlencoded;
#[macro_use]
extern crate hyper;

pub mod error;
pub mod response;
pub mod api;
pub mod prelude;
pub mod client;
pub mod tests;
pub mod parameter;

pub use client::Client;
pub use prelude::*;


use serde_json::Value;
use serde::Serialize;
use std::fmt;
use serde::Deserialize;
use serde::de::DeserializeOwned;



/// Every struct that is an endpoint, implements this trait.
pub trait Endpoint<'de> {
	type Parameter: Serialize + fmt::Debug + Clone;
	type ResponseType: std::fmt::Debug + Clone + DeserializeOwned;
	const URL: &'static str;


	fn new(Client, Self::Parameter) -> Self;

	fn params_mut(&mut self) -> &mut Self::Parameter;
	fn client(&self) -> Client;
	fn url(&self) -> String;
	fn parse(&self, Value) -> Result<Self::ResponseType, error::Error>;
}






pub trait Pageable<'de, E>
where
	E: Endpoint<'de> + Clone + fmt::Debug,
	<E as Endpoint<'de>>::Parameter: Iterator + Clone + fmt::Debug,
	<E as Endpoint<'de>>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
{
	fn pager(self, client: Client) -> Pager<'de, E>;

	// fn pager(_self: E, client: Client) -> Pager<'de, E> {
	// 	Pager::new(
	// 		client,
	// 		_self.clone(),
	// 		None,
	// 		Some(1_000)
	// 	)
	// }
}



//#[derive(Debug, Clone)]
pub struct Pager<'de, T>
where
	T: Endpoint<'de> + Clone + std::fmt::Debug,
	<T as Endpoint<'de>>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
	<T as Endpoint<'de>>::Parameter: Iterator + Clone + std::fmt::Debug,
{
	client: Client,
	limit: usize,
	current_page: usize,
	shifted: usize,
	endpoint: T,
	data: Vec<<<T as Endpoint<'de>>::ResponseType as IntoIterator>::Item>,
}




impl<'de, T: Endpoint<'de> + Clone + std::fmt::Debug> Pager<'de, T>
where
	<T as Endpoint<'de>>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
	<T as Endpoint<'de>>::Parameter: Iterator + Clone + std::fmt::Debug,
{
	pub fn new(client: Client, endpoint: T, mut start: Option<usize>, mut limit: Option<usize>) -> Self
	{
		if limit.is_none() {
			limit = Some(750);
		}

		if start.is_none() {
			start = Some(0);
		}



		Self {
			client: client,
			data: Vec::new(),
			shifted: limit.unwrap(),
			limit: limit.unwrap(),
			endpoint: endpoint,
			current_page: start.unwrap(),
		}
	}
}





impl<'de, E> Iterator for Pager<'de, E>
where
	E: Endpoint<'de>,
	E: Clone,
	E: std::fmt::Debug,
	<E as Endpoint<'de>>::Parameter: Iterator + Clone + fmt::Debug,
	<E as Endpoint<'de>>::ResponseType: IntoIterator,
{
	type Item = Result<<<E as Endpoint<'de>>::ResponseType as IntoIterator>::Item, error::Error>;

	fn next(&mut self) -> Option<Self::Item>
	{
		match self.data.pop()
		{
			Some(i) => {
				self.shifted += 1;
				Some(Ok(i))
			}
			None => {
				if self.shifted < self.limit {
					return None;
				}


				self.endpoint
					.params_mut()
					.next();


				self.current_page += 1;



				let res = self
					.client
					.execute(self.endpoint.clone())
					.unwrap();


				for var in res.into_iter() {
					self.data.push(var);
				}
				self.shifted = 0;

				// std::thread::sleep(std::time::Duration::new(1, 5_000_000));
				self.next()

			}
		}
	}
}
