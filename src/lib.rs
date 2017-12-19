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
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt;



/// Every struct that is an endpoint, implements this trait.
pub trait Endpoint {
	type Parameter: Serialize + fmt::Debug + Clone;
	type ResponseType: std::fmt::Debug + Clone + DeserializeOwned;
	const URL: &'static str;


	fn new(Self::Parameter) -> Self;
	fn params_mut(&mut self) -> &mut Self::Parameter;
}






pub trait PageableEndpoint<E>
where
	E: Endpoint + Clone + fmt::Debug,
	<E as Endpoint>::Parameter: PageableParameter + Clone + fmt::Debug,
	<E as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
{
	fn pager(self, client: Client) -> Pager<E>;

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
pub struct Pager<T>
where
	T: Endpoint + Clone + std::fmt::Debug,
	<T as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
	<T as Endpoint>::Parameter: PageableParameter + Clone + std::fmt::Debug,
{
	client: Client,
	shifted: usize,
	endpoint: T,
	data: Vec<<<T as Endpoint>::ResponseType as IntoIterator>::Item>,
}




impl<T: Endpoint + Clone + std::fmt::Debug> Pager<T>
where
	<T as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
	<T as Endpoint>::Parameter: PageableParameter + Clone + std::fmt::Debug,
{
	pub fn new(
		client: Client,
		mut endpoint: T,
		start: Option<usize>,
		limit: Option<usize>
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





impl<E> Iterator for Pager<E>
where
	E: Endpoint,
	E: Clone,
	E: std::fmt::Debug,
	<E as Endpoint>::Parameter: PageableParameter + Clone + fmt::Debug,
	<E as Endpoint>::ResponseType: IntoIterator,
{
	type Item = Result<<<E as Endpoint>::ResponseType as IntoIterator>::Item, error::Error>;

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


				*self.endpoint.params_mut().page_mut() = Some(self.page_mut() + 1);

				self.next()

			}
		}
	}
}
