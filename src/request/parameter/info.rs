#![allow(dead_code)]
#![allow(unused_imports)]
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
use std::result::Result;
use std::thread;
use std::time;
use prelude::*;





#[derive(Debug, Clone)]
pub struct GetFullEntry {
	pub id: InfoID
}


impl ToString for GetFullEntry {
	fn to_string(&self) -> String
	{
		self.id.to_string()
	}
}




#[derive(Debug, Clone)]
pub struct GetEntry(pub InfoID);

impl ToString for GetEntry {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}



#[derive(Debug, Clone)]
pub struct GetNames(pub InfoID);

impl ToString for GetNames {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}



#[derive(Debug, Clone)]
pub struct GetGate(pub InfoID);

impl ToString for GetGate {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}



#[derive(Debug, Clone)]
pub struct GetLang(pub InfoID);

impl ToString for GetLang {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}



#[derive(Debug, Clone)]
pub struct GetSeason(pub InfoID);

impl ToString for GetSeason {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}



#[derive(Debug, Clone)]
pub struct GetGroups(pub InfoID);

impl ToString for GetGroups {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}



#[derive(Debug, Clone)]
pub struct GetPublisher(pub InfoID);

impl ToString for GetPublisher {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}



#[derive(Debug, Clone)]
pub struct GetListinfo {
	pub id: InfoID,
	pub p: Option<i64>,
	pub limit: Option<i64>,
}



#[derive(Debug, Clone)]
pub struct GetComments {
	pub id: InfoID,
	pub p: Option<i64>,
	pub limit: Option<i64>,
	pub sort: Option<String>,
}



#[derive(Debug, Clone)]
pub struct GetRelations {
	pub info_id: InfoID,
	pub is_h: bool,
}



#[derive(Debug, Clone)]
pub struct EntryTags(pub InfoID);

impl ToString for EntryTags {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}


#[derive(Debug, Clone)]
pub struct GetTranslatorgroup(pub InfoID);

impl ToString for GetTranslatorgroup {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}


#[derive(Debug, Clone)]
pub struct GetIndustry(pub InfoID);

impl ToString for GetIndustry {
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}


#[derive(Debug, Clone)]
pub struct SetUserInfo {
	pub id: InfoID,
	pub typ: String,
}
