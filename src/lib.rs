//! proxer api library


#![warn(missing_docs)]
#![allow(missing_docs)]



#[macro_use]
extern crate log;
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
use serde_json::Value;
use std::collections::HashMap;
use colored::Colorize;


/// Trait that represents an api endpoint
pub trait Request {
	/// parameter
	type RequestType;
	/// response type
	type ResponseType: From<Value>;

	/// gets the json, returns the actual data as struct
	fn parse(&self, json: Value) -> Self::ResponseType
	{
		Self::ResponseType::from(json)
	}

	fn get_url(self) -> String;

	/// returns the api endpoint url (eg. "info/fullentry")
	fn send(&self) -> client::ApiResponse;

	/// returns the payload
	fn get_data(self) -> HashMap<String, String>;
}

// Trait that requeres a function that parses the request to
// to an freely choosable datatype




pub trait Endpoint {
	type ResponseType: std::fmt::Debug + Clone;

	fn get_params_mut(&mut self) -> &mut HashMap<String, String>;
	fn send(self) -> Result<Self::ResponseType, error::Error>;
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
	pub fn new(mut endpoint: T, start: usize, limit: usize) -> Self
	{
		endpoint
			.get_params_mut()
			.insert("limit".to_string(), limit.to_string());

		endpoint
			.get_params_mut()
			.insert("p".to_string(), start.to_string());


		Self {
			data: Vec::new(),
			shifted: limit,
			limit: limit,
			endpoint: endpoint,
			current_page: start,
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
				std::thread::sleep(std::time::Duration::new(0, 000_500_000));
				println!("Iter: returning from buffer");
				self.shifted += 1;
				Some(Ok(i))
			},
			None => {
				//if false {
				if self.shifted < self.limit {
					println!("Iter: last fetch was smaller than limit");
					return None;
				}
				else {
					println!("Iter: fetching new data");
					self.endpoint
						.get_params_mut()
						.insert("p".to_string(), self.current_page.to_string());

					self.current_page += 1;

					println!("{}", format!("{:#?}", self.endpoint).cyan().bold());
					let res = self.endpoint.clone().send().unwrap();


					for var in res.into_iter() {
						self.data.push(var);
					}
					self.shifted = 0;

					std::thread::sleep(std::time::Duration::new(1, 5_000_000));
					self.next()
				}
			}
		}
	}
}
