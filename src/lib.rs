//! proxer api library
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate serde_urlencoded;
#[macro_use]
extern crate log;
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
use parameter::PageableParameter;
pub use prelude::*;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

use serde_json::Value;
use std::fmt;



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






pub trait PageableEndpoint<'de, E>
where
	E: Endpoint<'de> + Clone + fmt::Debug,
	<E as Endpoint<'de>>::Parameter: PageableParameter + Clone + fmt::Debug,
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
	<T as Endpoint<'de>>::Parameter: PageableParameter + Clone + std::fmt::Debug,
{
	client: Client,
	shifted: usize,
	endpoint: T,
	data: Vec<<<T as Endpoint<'de>>::ResponseType as IntoIterator>::Item>,
}




impl<'de, T: Endpoint<'de> + Clone + std::fmt::Debug> Pager<'de, T>
where
	<T as Endpoint<'de>>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
	<T as Endpoint<'de>>::Parameter: PageableParameter + Clone + std::fmt::Debug,
{
	pub fn new(
		client: Client,
		mut endpoint: T,
		mut start: Option<usize>,
		mut limit: Option<usize>
	) -> Self
	{

		match (endpoint.params_mut().page_mut(), start) {
			(&mut None, None) => *endpoint.params_mut().page_mut() = Some(0),
			(&mut None, Some(_)) => *endpoint.params_mut().page_mut() = start,
			_ => {}
		}

		match (endpoint.params_mut().limit_mut(), limit) {
			(&mut None, None) => *endpoint.params_mut().limit_mut() = Some(750),
			(&mut None, Some(_)) => *endpoint.params_mut().limit_mut() = limit,
			_ => {}
		}

		debug!(
			"initialize new pager: page: {}, limit {}",
			endpoint.params_mut().page_mut().unwrap(),
			endpoint.params_mut().limit_mut().unwrap(),
		);


		Self {
			client: client,
			data: Vec::new(),
			shifted: 0,
			endpoint: endpoint,
		}
	}

	fn page_mut(&mut self) -> usize {
		self.endpoint.params_mut().page_mut().unwrap()
	}

	fn limit_mut(&mut self) -> usize {
		self.endpoint.params_mut().limit_mut().unwrap()
	}
}





impl<'de, E> Iterator for Pager<'de, E>
where
	E: Endpoint<'de>,
	E: Clone,
	E: std::fmt::Debug,
	<E as Endpoint<'de>>::Parameter: PageableParameter + Clone + fmt::Debug,
	<E as Endpoint<'de>>::ResponseType: IntoIterator,
{
	type Item = Result<<<E as Endpoint<'de>>::ResponseType as IntoIterator>::Item, error::Error>;

	fn next(&mut self) -> Option<Self::Item>
	{
		match self.data.pop()
		{
			Some(i) => {
				debug!("returning from buffer");
				self.shifted += 1;

				debug!("buffer: remaining/shifted: {}/{}",
					self.data.len(),
					self.shifted,
				);
				Some(Ok(i))
			}
			None => {
				debug!("expected shifted: {}", self.page_mut() * self.limit_mut());
				debug!("actually shifted: {}", self.shifted);

				if self.page_mut() * self.limit_mut() > self.shifted {
					debug!("reached the last page");
					return None;
				}



				debug!("fetching new data");
				let res = self.client.execute(self.endpoint.clone()).unwrap();



				for var in res.into_iter() {
					self.data.push(var);
				}

				debug!("filled buffer with {} entries", self.data.len());


				*self.endpoint
					.params_mut()
					.page_mut() = Some(self.page_mut() + 1);

				self.next()

			}
		}
	}
}
