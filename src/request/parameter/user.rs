#![allow(dead_code)]
#![allow(unused)]


use error;
use reqwest;
use reqwest::IntoUrl;
use serde_derive;
use serde_json;
use serde;
use serde::Serializer;
use std;
use std::collections::HashMap;
use std::process::exit;
use std::thread;
use std::time;
use prelude::*;






#[derive(Debug, Clone)]
pub struct Login {
	pub username: String,
	pub password: String,
	pub secretkey: Option<String>,
}



#[derive(Debug, Clone)]
pub struct Logout {}



#[derive(Debug, Clone)]
pub struct Userinfo {
	pub uid: Option<UserID>,
	pub username: Option<String>,
}



#[derive(Debug, Clone)]
pub struct GetTopten {
	pub uid: Option<UserID>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub is_h: Option<bool>,
}



#[derive(Debug, Clone)]
pub struct GetList {
	pub uid: Option<UserID>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub search: Option<String>,
	pub search_start: Option<String>,
	pub is_h: Option<bool>,
	pub filter: Option<u64>,
	pub sort: Option<String>,

	pub p: Option<i64>,
	pub limit: Option<u64>,
}



#[derive(Debug, Clone)]
pub struct GetLatestComments {
	pub uid: Option<InfoID>,
	pub username: Option<String>,
	pub kat: Option<String>,
	pub length: Option<u64>,

	pub p: Option<u64>,
	pub limit: Option<u64>,
}



#[derive(Debug, Clone)]
pub struct GetHistory {
	pub uid: Option<InfoID>,
	pub username: Option<String>,
	pub is_h: Option<bool>,
	pub filter: Option<u64>,

	pub p: Option<u64>,
	pub limit: Option<u64>,
}
