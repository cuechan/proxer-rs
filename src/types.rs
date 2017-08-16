#![allow(unused)]
#![allow(unstable)]

use serde;
use serde_json;
use serde_json::Value;
use std::fmt;
use std::thread;
use std::time;
use error;
use chrono::DateTime;
use std::convert::From;






#[derive(Debug, Clone, Serialize)]
pub struct FullInfo {
    pub id: i64,
    pub info: Info,
    pub tags: Vec<Tag>,
    pub names: Vec<Name>,
}

/// This currently DO NOT support error handling.
/// I am wating for the TryFrom trait

impl From<serde_json::Value> for FullInfo {
    fn from(serde_value: serde_json::Value) -> Self {
        let data = serde_value.as_object().unwrap();


        // get the id
        let id: i64 = data.get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .parse()
            .unwrap();



        // get the info
        let info = Info::from(serde_value.to_owned());


        // get the tags
        let mut tags = Vec::new();

        for tag in serde_value.get("tags").unwrap().as_array().unwrap() {
            tags.insert(0, Tag::from(tag.clone()));
        }



        // get the tags
        let mut names = Vec::new();

        for name in serde_value.get("names").unwrap().as_array().unwrap() {
            names.insert(0, Name::from(name.clone()));
        }



        Self {
            id: id,
            info: info,
            names: names,
            tags: tags,
        }
    }
}















#[derive(Debug, Clone, Serialize)]
pub struct Info {
    pub id: i64,
    pub description: String,
}


impl From<Value> for Info {
    fn from(serde_value: serde_json::Value) -> Self {
        let data = serde_value.as_object().unwrap();


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


        Self {
            id: id,
            description: description
        }
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


impl From<Value> for Tag {
    fn from(serde_value: serde_json::Value) -> Self{
        let data = serde_value.as_object().unwrap();



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



        // return the data

        Self {
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
        }
    }
}













#[derive(Debug, Clone, Serialize)]
pub struct Name {
    pub reference_id: i64,
    pub id: i64,
    pub class: &'static str,
    pub name: String,
    pub nametype: String
}


impl From<Value> for Name {
    fn from(serde_value: serde_json::Value) -> Self {
        let data = serde_value.as_object().unwrap();



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




        // Dude, pass me the data!
        Self {
            class: "Name",
            id: id,
            name: name,
            nametype: nametype,
            reference_id: reference_id
        }
    }
}


















#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub info_id: i64,
    pub comment_type: String,
    pub state: i64, // Todo: use enum for state
    pub data: String,
    pub comment: String,
    pub rating: i64,
    pub episode: i64,
    pub positive: i64,
    pub timestamp: i64, //Todo: use chrono here
    pub username: String,
    pub uid: i64,
    pub avatar: String,
}


impl From<Value> for Comment {
    fn from(serde_value: serde_json::Value) -> Self {
        let data = serde_value.as_object().unwrap();


        Self {
            id: data
                .get("id")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            info_id: data
                .get("tid")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),


            comment_type: data
                .get("type")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),


            // Todo: use enum for state
            state: data
                .get("state")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            data: data
                .get("data")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),


            comment: data
                .get("comment")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),


            rating: data
                .get("rating")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),


            episode: data
                .get("episode")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),


            positive: data
                .get("positive")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),


            //Todo: use chrono here
            timestamp: data
                .get("timestamp")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            username: data
                .get("username")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),


            uid: data
                .get("uid")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),


            avatar: data
                .get("id")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),

        }
    }
}
