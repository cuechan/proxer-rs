#![allow(unused)]
#![allow(unstable)]

use chrono;
use chrono::DateTime;
use error;
use serde_json;
use serde_json::Value;
use serde;
use std::convert::From;
use std::fmt;
use std::thread;
use std::time;






#[derive(Debug, Clone, Serialize)]
pub struct FullEntry {
    pub tags: Vec<Tag>,
    pub names: Vec<Name>,
}

/// This currently DO NOT support error handling.
/// I am wating for the TryFrom trait

impl From<serde_json::Value> for FullEntry {
    fn from(serde_value: serde_json::Value) -> Self {
        let data = serde_value.as_object().unwrap();


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
        let id: i64 = data.get("id").unwrap().as_str().unwrap().parse().unwrap();


        // parse description
        let description = data.get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();


        Self {
            id: id,
            description: description,
        }
    }
}















#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub class: &'static str,
    pub info_id: Option<u64>,
    pub id: u64,
    pub tag_id: u64,
    pub added: String,
    pub matches: bool,
    pub spoiler: bool,
    pub name: String,
    pub description: String,
    pub tag_category: Option<String>,
}


impl From<Value> for Tag {
    fn from(serde_value: serde_json::Value) -> Self {
        let data = serde_value.as_object().unwrap();



        // parse id
        let id: u64 = data.get("id").unwrap().as_str().unwrap().parse().unwrap();

        // parse tagid
        let tag_id: u64 = data.get("tid").unwrap().as_str().unwrap().parse().unwrap();


        let info_id = match data.get("info_id") {
            Some(r) => Some(r.as_u64().unwrap()),
            None => None,
        };


        let matches: bool = match data.get("rate_flag").unwrap().as_str().unwrap() {
            "0" => false,
            "1" => true,
            _ => false,
        };


        let spoiler: bool = match data.get("spoiler_flag").unwrap().as_str().unwrap() {
            "0" => false,
            "1" => true,
            _ => false,
        };



        // parse description
        let description = data.get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();


        // parse timestamp
        let added = data.get("timestamp").unwrap().as_str().unwrap().to_string();


        // parse tagname
        let name = data.get("tag").unwrap().as_str().unwrap().to_string();



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
            tag_category: None,
        }
    }
}













#[derive(Debug, Clone, Serialize)]
pub struct Name {
    pub reference_id: i64,
    pub id: i64,
    pub class: &'static str,
    pub name: String,
    pub nametype: String,
}


impl From<Value> for Name {
    fn from(serde_value: serde_json::Value) -> Self {
        let data = serde_value.as_object().unwrap();



        // parse id
        let id: i64 = data.get("id").unwrap().as_str().unwrap().parse().unwrap();


        // parse id
        let reference_id: i64 = data.get("eid").unwrap().as_str().unwrap().parse().unwrap();


        // parse description
        let name = data.get("name").unwrap().as_str().unwrap().to_string();


        // parse description
        let nametype = data.get("type").unwrap().as_str().unwrap().to_string();




        // Dude, pass me the data!
        Self {
            class: "Name",
            id: id,
            name: name,
            nametype: nametype,
            reference_id: reference_id,
        }
    }
}


















#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub info_id: u64,
    pub comment_type: String,
    pub state: u64, // Todo: use enum for state
    pub data: String,
    pub comment: String,
    pub rating: u64,
    pub episode: u64,
    pub positive: u64,
    pub timestamp: u64, //Todo: use chrono here
    pub username: String,
    pub uid: u64,
    pub avatar: String,
}


impl From<Value> for Comment {
    fn from(serde_value: serde_json::Value) -> Self {
        let data = serde_value.as_object().unwrap();


        Self {
            id: data.get("id").unwrap().as_str().unwrap().parse().unwrap(),

            info_id: data.get("tid").unwrap().as_str().unwrap().parse().unwrap(),

            comment_type: data.get("type").unwrap().as_str().unwrap().to_string(),

            // Todo: use enum for state
            state: data.get("state")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            data: data.get("data").unwrap().as_str().unwrap().to_string(),

            comment: data.get("comment").unwrap().as_str().unwrap().to_string(),

            rating: data.get("rating")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            episode: data.get("episode")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            positive: data.get("positive")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            //Todo: use chrono here
            timestamp: data.get("timestamp")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            username: data.get("username").unwrap().as_str().unwrap().to_string(),

            uid: data.get("uid").unwrap().as_str().unwrap().parse().unwrap(),

            avatar: data.get("id").unwrap().as_str().unwrap().to_string(),
        }
    }
}









#[derive(Deserialize, Debug, Clone)]
pub struct Userinfo {
    pub uid: u64,
    pub username: String,
    pub avatar: String, // use some sort of uri type here
    pub status: String,
    // status_time is sometimes a negative number. using i64
    pub status_time: i64,
    pub points_upload: u64,
    pub points_anime: u64,
    pub points_manga: u64,
    pub points_info: u64,
    pub points_forum: u64,
    pub points_misc: u64,
}


impl From<Value> for Userinfo {
    fn from(serde_json: Value) -> Self {
        let data = serde_json.as_object().unwrap();



        Self {
            uid: data.get("uid").unwrap().as_str().unwrap().parse().unwrap(),

            username: data.get("username").unwrap().as_str().unwrap().to_string(),

            avatar: data.get("avatar").unwrap().as_str().unwrap().to_string(),

            status: data.get("status").unwrap().as_str().unwrap().to_string(),

            status_time: data.get("status_time")
                .expect("json doesnt contain 'status_time'")
                .as_i64()
                .unwrap(),

            points_upload: data.get("points_uploads")
                .expect("json does not contain 'points_upload")
                .as_str()
                .expect("'points_uploads' is not a string")
                .parse()
                .expect("cant parse 'points_uploads' as u64"),

            points_anime: data.get("points_anime")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            points_manga: data.get("points_manga")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            points_info: data.get("points_info")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            points_forum: data.get("points_forum")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),

            points_misc: data.get("points_misc")
                .unwrap()
                .as_str()
                .unwrap()
                .parse()
                .unwrap(),
        }
    }
}
