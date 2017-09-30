pub mod info;
pub mod user;

use client::Client;


pub const DEFAULT_PAGER_PAGE: i64 = 0;
pub const DEFAULT_PAGER_LIMIT: i64 = 250;



#[derive(Debug, Clone)]
pub struct Api<'a> {
    pub client: Client<'a>
}


impl<'a> Api<'a> {
    pub fn info(self) -> info::Info<'a> {
        info::Info {
            client: self.client
        }
    }

    pub fn user(self) -> user::User<'a> {
        user::User {
            client: self.client
        }
    }
}
