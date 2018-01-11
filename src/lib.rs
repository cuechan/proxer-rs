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


pub use client::Client;
pub use prelude::*;
use serde::Serialize;
use serde::de::DeserializeOwned;



/// Every struct that is an endpoint, implements this trait.
pub trait Endpoint: Serialize {
	type ResponseType: std::fmt::Debug + Clone + DeserializeOwned;
	const URL: &'static str;
}






pub trait PageableEndpoint
where
	Self: Endpoint + std::marker::Sized,
	<Self as Endpoint>::ResponseType: IntoIterator + Clone,
{
	fn pager(self, client: Client) -> Pager<Self>;

	fn page_mut(&mut self) -> &mut Option<usize>;
	fn limit_mut(&mut self) -> &mut Option<usize>;
}



//#[derive(Debug, Clone)]
pub struct Pager<T>
where
	T: Endpoint,
	<T as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
{
	client: Client,
	shifted: usize,
	endpoint: T,
	data: Vec<<<T as Endpoint>::ResponseType as IntoIterator>::Item>,
}




impl<T> Pager<T>
where
	T: Endpoint + PageableEndpoint,
	T: Clone + std::fmt::Debug,
	<T as Endpoint>::ResponseType: IntoIterator + Clone + std::fmt::Debug,
{
	pub fn new(client: Client, mut endpoint: T, start: Option<usize>, limit: Option<usize>)
		-> Self
	{

		match (endpoint.page_mut(), start)
		{
			(&mut None, None) => *endpoint.page_mut() = Some(0),
			(&mut None, Some(_)) => *endpoint.page_mut() = start,
			_ => {}
		}

		match (endpoint.limit_mut(), limit)
		{
			(&mut None, None) => *endpoint.limit_mut() = Some(750),
			(&mut None, Some(_)) => *endpoint.limit_mut() = limit,
			_ => {}
		}

		debug!(
			"initialize new pager: page: {}, limit {}",
			endpoint.page_mut().unwrap(),
			endpoint.limit_mut().unwrap(),
		);


		Self {
			client: client,
			data: Vec::new(),
			shifted: 0,
			endpoint: endpoint,
		}
	}

	fn page_mut(&mut self) -> usize
	{
		self.endpoint.page_mut().unwrap()
	}

	fn limit_mut(&mut self) -> usize
	{
		self.endpoint.limit_mut().unwrap()
	}

	fn next_page(&mut self) {
		*self.endpoint.page_mut() = Some(self.page_mut() + 1);
	}
}





impl<T> Iterator for Pager<T>
where
	T: Endpoint,
	T: PageableEndpoint + Clone + std::fmt::Debug,
	<T as Endpoint>::ResponseType: IntoIterator,
{
	type Item = Result<<<T as Endpoint>::ResponseType as IntoIterator>::Item, error::Error>;

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
				let res = self.client
					.execute(self.endpoint.clone());

				self.next_page();

				
				match res {
					Ok(res) => {
						for var in res.into_iter() {
							self.data.push(var);
						}
						debug!("filled buffer with {} entries", self.data.len());

						self.next()
					},
					Err(e) => {
						Some(Err(e))
					}
				}
			}
		}
	}
}
