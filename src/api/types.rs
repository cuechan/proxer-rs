#![allow(unused)]

use serde;
use serde_json;
use std::fmt;
use std::thread;
use std::time;
use api::error;






#[derive(Debug, Clone, Serialize)]
pub struct FullInfo {
    pub id: i64,
    pub info: Info,
    pub tags: Vec<Tag>,
    pub names: Vec<Name>,
}


impl FullInfo {
    pub fn from_api(serde_value: serde_json::Value) -> Result<Self, error::Error> {
        let data = match serde_value.as_object() {
            Some(e) => e,
            None => return Err(error::Error::Json)
        };


        // get the id
        let id: i64 = data.get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();


        // get the info
        let info = match Info::from_api(serde_value.to_owned()) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };


        // get the tags
        let mut tags = Vec::new();

        for tag in serde_value.get("tags").unwrap().as_array().unwrap() {
            tags.insert(0, match Tag::from_api(tag.clone()) {
                Ok(r) => r,
                Err(e) => return Err(e)
            });
        }



        // get the tags
        let mut names = Vec::new();

        for name in serde_value.get("names").unwrap().as_array().unwrap() {
            names.insert(0, match Name::from_api(name.clone()) {
                Ok(r) => r,
                Err(e) => return Err(e)
            });
        }



        Ok(Self {
            id: id,
            info: info,
            names: names,
            tags: tags,
        })
    }
}












#[derive(Debug, Clone, Serialize)]
pub struct Info {
    pub id: i64,
    pub description: String,
}


impl Info {
    pub fn from_api(serde_value: serde_json::Value) -> Result<Self, error::Error> {
        let data = match serde_value.as_object() {
            Some(e) => e,
            None => return Err(error::Error::Json)
        };


        // parse id
        let id: i64 = data
            .get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();


        // parse description
        let description = data
            .get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();


        Ok(Self {
            id: id,
            description: description
        })
    }
}












#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub class: &'static str,
    pub info_id: Option<i64>,
    pub id: i64,
    pub tag_id: i64,
    pub added: String,
    pub matches: bool,
    pub spoiler: bool,
    pub name: String,
    pub description: String,
    pub tag_category: Option<String>,
}


impl Tag {
    pub fn from_api(serde_value: serde_json::Value) -> Result<Self, error::Error> {
        let data = match serde_value.as_object() {
            Some(e) => e,
            None => return Err(error::Error::Json)
        };


        // parse id
        let id: i64 = data
            .get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();

        // parse tagid
        let tag_id: i64 = data
            .get("tid")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();


        let info_id = match data.get("info_id") {
            Some(r) => Some(r.as_i64().unwrap()),
            None => None
        };


        let matches: bool = match data.get("rate_flag").unwrap().as_str().unwrap() {
            "0" => false,
            "1" => true,
            _ => false
        };


        let spoiler: bool = match data.get("spoiler_flag").unwrap().as_str().unwrap() {
            "0" => false,
            "1" => true,
            _ => false
        };



        // parse description
        let description = data
            .get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();


        // parse timestamp
        let added = data
            .get("timestamp")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();


        // parse tagname
        let name = data
            .get("tag")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();


        Ok(Self {
            class: "Tag",
            id: id,
            tag_id: tag_id,
            info_id: info_id,
            description: description,
            matches: matches,
            spoiler: spoiler,
            added: added,
            name: name,
            tag_category: None
        })
    }

    pub fn name(self) -> String {
        self.name
    }
}




#[derive(Debug, Clone, Serialize)]
pub struct Name {
    reference_id: i64,
    id: i64,
    class: &'static str,
    name: String,
    nametype: String
}


impl Name {
    pub fn from_api(serde_value: serde_json::Value) -> Result<Self, error::Error> {
        let data = match serde_value.as_object() {
            Some(e) => e,
            None => return Err(error::Error::Json)
        };



        // parse id
        let id: i64 = data
            .get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();


        // parse id
        let reference_id: i64 = data
            .get("eid")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();


        // parse description
        let name = data
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();


        // parse description
        let nametype = data
            .get("type")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();


        Ok(Self {
            class: "Name",
            id: id,
            name: name,
            nametype: nametype,
            reference_id: reference_id
        })
    }
}
