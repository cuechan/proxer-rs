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



/// Every struct that is an endpoint, implements this trait.
pub trait Endpoint {
	type Parameter: Serialize + fmt::Debug + Clone;
	type ResponseType: std::fmt::Debug + Clone;


	fn new(Client, Self::Parameter) -> Self;

	fn params_mut(&mut self) -> &mut Self::Parameter;
	fn client(&self) -> Client;
	fn url(&self) -> String;
	fn parse(&self, Value) -> Result<Self::ResponseType, error::Error>;

	fn send(&mut self) -> Result<Self::ResponseType, error::Error>
	{
		match self.client().execute(self.url(), self.params_mut().clone())
		{
			Err(e) => Err(e),
			Ok(r) => self.parse(r),
		}
	}
}






pub trait Pageable<E>
where
	E: Endpoint + Clone + fmt::Debug,
	<E as Endpoint>::Parameter: Iterator + Clone + fmt::Debug,
	<E as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
{
	fn pager(self) -> Pager<E>;
}



//#[derive(Debug, Clone)]
pub struct Pager<T>
where
	T: Endpoint + Clone + std::fmt::Debug,
	<T as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
	<T as Endpoint>::Parameter: Iterator + Clone + std::fmt::Debug,
{
	limit: usize,
	current_page: usize,
	shifted: usize,
	endpoint: T,
	data: Vec<<<T as Endpoint>::ResponseType as IntoIterator>::Item>,
}




impl<T: Endpoint + Clone + std::fmt::Debug> Pager<T>
where
	<T as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
	<T as Endpoint>::Parameter: Iterator + Clone + std::fmt::Debug,
{
	pub fn new(mut endpoint: T, mut start: Option<usize>, mut limit: Option<usize>) -> Self
	{
		if limit.is_none() {
			limit = Some(500);
		}

		if start.is_none() {
			start = Some(0);
		}



		Self {
			data: Vec::new(),
			shifted: limit.unwrap(),
			limit: limit.unwrap(),
			endpoint: endpoint,
			current_page: start.unwrap(),
		}
	}
}





impl<E> Iterator for Pager<E>
where
	E: Endpoint,
	E: Clone,
	E: std::fmt::Debug,
	<E as Endpoint>::Parameter: Iterator + Clone + fmt::Debug,
	<E as Endpoint>::ResponseType: IntoIterator,
{
	type Item = Result<<<E as Endpoint>::ResponseType as IntoIterator>::Item, error::Error>;

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



				let res = self.endpoint.clone().send().unwrap();


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
