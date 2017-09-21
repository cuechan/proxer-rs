#![allow(dead_code)]
#![deny(unused_imports)]
#![allow(warnings)]
#![allow(unused)]


use std;


#[derive(Debug, Clone, Copy)]
pub struct UserID(u64);

pub trait IsUserID<T> {
    fn as_userid(self) -> UserID;
}


impl IsUserID<i32> for u64 {
    fn as_userid(self) -> UserID {
        UserID(self as u64)
    }
}

impl From<i32> for UserID {
    fn from(x: i32) -> Self {
        UserID(x as u64)
    }
}

impl From<u32> for UserID {
    fn from(x: u32) -> Self {
        UserID(x as u64)
    }
}

impl From<u64> for UserID {
    fn from(x: u64) -> Self {
        UserID(x as u64)
    }
}


impl ToString for UserID {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}




#[derive(Debug, Clone)]
pub struct InfoID(u64);

pub trait IsInfoID {
    fn as_infoid(self) -> InfoID;
}


impl From<i32> for InfoID {
    fn from(x: i32) -> Self {
        InfoID(x as u64)
    }
}

impl From<i64> for InfoID {
    fn from(x: i64) -> Self {
        InfoID(x as u64)
    }
}

impl From<u32> for InfoID {
    fn from(x: u32) -> Self {
        InfoID(x as u64)
    }
}

impl From<u64> for InfoID {
    fn from(x: u64) -> Self {
        InfoID(x as u64)
    }
}



impl ToString for InfoID {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}


impl std::str::FromStr for InfoID {
    type Err = std::num::ParseIntError;

    fn from_str(string: &str) -> Result<InfoID, Self::Err> {
        match string.parse::<i64>() {
            Ok(i) => Ok(InfoID::from(i)),
            Err(e) => Err(e)
        }
    }
}
