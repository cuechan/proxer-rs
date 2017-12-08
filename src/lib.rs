//! proxer api library


// #![warn(missing_docs)]



#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate colored;
extern crate reqwest;
extern crate serde_json;
extern crate serde;

pub mod error;
pub mod response;
pub mod request;
pub mod api;
pub mod prelude;
pub mod client;

pub use client::Client;
pub use prelude::*;


use std::collections::HashMap;
use serde_json::Value;



/// Every struct that is an endpoint, implements this trait.
pub trait Endpoint {
	type ResponseType: std::fmt::Debug + Clone;

	fn params_mut(&mut self) -> &mut HashMap<String, String>;
	fn client(&self) -> Client;
	fn url(&self) -> String;
	fn parse(&self, Value) -> Result<Self::ResponseType, error::Error>;

	fn params(&mut self) -> HashMap<String, String>
	{
		self.params_mut().clone()
	}

	fn send(&mut self) -> Result<Self::ResponseType, error::Error>
	{
		match self.client().execute(self.url(), self.params())
		{
			Err(e) => Err(e),
			Ok(r) => self.parse(r),
		}
	}
}






pub trait Pageable<E: Endpoint + Clone + std::fmt::Debug>
where
	<E as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
{
	fn pager(self) -> Pager<E>;
}



//#[derive(Debug, Clone)]
pub struct Pager<T: Endpoint + Clone + std::fmt::Debug>
where
	<T as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
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
{
	pub fn new(mut endpoint: T, mut start: Option<usize>, mut limit: Option<usize>) -> Self
	{
		if limit.is_none() {
			limit = Some(500);
		}

		if start.is_none() {
			start = Some(0);
		}


		endpoint
			.params_mut()
			.insert(
				"p".to_string(),
				start.unwrap().to_string()
			);


		endpoint
			.params_mut()
			.insert(
				"limit".to_string(),
				limit.unwrap().to_string()
			);



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
				//if false {
				if self.shifted < self.limit {
					return None;
				}
				else {
					self.endpoint
						.params_mut()
						.insert("p".to_string(), self.current_page.to_string());

					self.current_page += 1;

					// println!("{}", format!("send request").cyan().bold());
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
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn api_structure() {
		// is the api structured as we want it to be?

		let client = client::Client::new("my_api_key".to_string());


		








	}
}
